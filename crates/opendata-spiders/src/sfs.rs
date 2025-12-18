use fs_err::PathExt;
use std::{fmt::Debug, path::PathBuf};

use async_trait::async_trait;

use flate2::Compression;
use reqwest::Client;

// use std::fs;
use fs_err as fs;
use ulid::Ulid;

use crate::item::Item;
use crate::Error;

pub struct SfsSpider {
    http_client: Client,
    output_path: PathBuf,
}

impl SfsSpider {
    pub fn new(options: SfsSpiderOptions) -> Self {
        // println!("{:?}", options);
        let SfsSpiderOptions {
            user_agent: user_agent_opt,
            output_path,
        } = options;
        let user_agent = user_agent_opt.as_deref().unwrap_or(crate::APP_USER_AGENT);
        fs::create_dir_all(&output_path).expect("spiders/sfs: can't create output_path");
        let output_path = output_path
            .fs_err_canonicalize()
            .expect("spiders/sfs: output_path error");
        tracing::warn!(user_agent, "configuring SfsSpider {:?}", output_path);
        let http_client = reqwest::Client::builder()
            .user_agent(user_agent)
            .gzip(true)
            .build()
            .expect("spiders/sfs: Building HTTP client");
        Self {
            http_client,
            output_path,
        }
    }
}

impl Debug for SfsSpider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SfsSpider {{ /* omitted */ }}")
    }
}

impl Default for SfsSpider {
    fn default() -> Self {
        Self::new(SfsSpiderOptions::default())
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SfsSpiderOptions {
    pub user_agent: Option<String>,
    pub output_path: PathBuf,
}

impl Default for SfsSpiderOptions {
    fn default() -> Self {
        Self {
            user_agent: None,
            output_path: "./output".into(),
        }
    }
}
#[async_trait]
impl webcrawler::Spider for SfsSpider {
    type Item = Item;
    type Error = Error;

    fn name(&self) -> String {
        String::from("sfs")
    }

    fn start_urls(&self) -> Vec<String> {
        let base_url = "https://data.riksdagen.se/dokumentlista/?sok=&doktyp=SFS&rm=";
        // let from_to = "&from=1880-01-01&tom=1900-12-31";
        let base_suffix = "&ts=&bet=&tempbet=&nr=&org=&iid=&avd=&webbtv=&talare=&exakt=&planering=&facets=&sort=rel&sortorder=desc&rapport=&utformat=xml&a=s;#soktraff";
        // "https://data.riksdagen.se/dokumentlista/?sok=&doktyp=SFS&rm=&from=1880-01-01&tom=1900-12-31&ts=&bet=&tempbet=&nr=&org=&iid=&avd=&webbtv=&talare=&exakt=&planering=&facets=&sort=rel&sortorder=desc&rapport=&utformat=xml&a=s#soktraff";
        // let base_url = "https://data.riksdagen.se/dokumentlista/?sok=&doktyp=SFS&rm=&ts=&bet=&tempbet=&nr=&org=&iid=&avd=&webbtv=&talare=&exakt=&planering=&facets=&sort=rel&sortorder=desc&rapport=&utformat=xml&from=1890-01-01&tom=1899-12-31&a=s#soktraff";
        // let base_url = "https://data.riksdagen.se/dokumentlista/?sok=&doktyp=SFS&rm=&ts=&bet=&tempbet=&nr=&org=&iid=&avd=&webbtv=&talare=&exakt=&planering=&facets=&sort=rel&sortorder=desc&rapport=&utformat=xml";
        let mut urls = Vec::new();

        for (from_year, to_year) in [
            (1880, 1900),
            (1901, 1920),
            (1921, 1940),
            (1941, 1960),
            (1961, 1980),
            (1981, 2000),
            (2001, 2020),
            (2021, 2025),
        ] {
            urls.push(format!(
                "{base_url}&from={from_year}-01-01&tom={to_year}-12-31{base_suffix}"
            ))
        }
        urls
    }

    #[tracing::instrument]
    async fn scrape(&self, url: String) -> Result<(Vec<Self::Item>, Vec<String>), Error> {
        let mut new_urls = Vec::new();
        let mut items = Vec::new();

        let dokument_url = "https://data.riksdagen.se/dokument";
        tracing::info!("calling {}", url);
        let response = self.http_client.get(&url).send().await.map_err(|err| {
            tracing::error!("Failed fetching: {:?}", err);
            err
        })?;

        tracing::trace!("response status: {}", response.status());

        if !response.status().is_success() {
            let status_code = response.status();
            tracing::error!(
                "The request returned '{}': '{}",
                response.status(),
                response.text().await?
            );
            return Err(Error::RequestReturnedError(status_code));
        }
        let text = response.text().await.map_err(|err| {
            tracing::error!("Failed getting text: {}", err);
            err
        })?;
        // println!("{}", text);
        let item: Item = match yaserde::de::from_str(&text) {
            Err(err) if url.contains("dokument/") => {
                tracing::error!(error=?err,text=text,"Failed parsing XML");
                let new_url = url.replace("dokument", "dokumentstatus");
                tracing::info!("Trying {} instead", new_url);
                new_urls.push(new_url);
                if text.starts_with("<div") {
                    // items.push((format!("{url}_text"), Item::Div(text)));
                    items.push(Item::Div(text));
                }
                return Ok((items, new_urls));
            }
            Err(err) => {
                tracing::error!(error=?err,text=text,"Failed parsing XML");
                return Err(Error::XmlDe { msg: err });
            }
            Ok(item) => item,
        };
        // println!("item={:#?}", item);
        // println!("url={url}");

        if let Item::DokumentLista(dokumentlista) = &item {
            if let Some(nasta_sida) = &dokumentlista.nasta_sida {
                new_urls.push(nasta_sida.clone());
            }
            for dokument in &dokumentlista.dokument {
                let dok_id = dokument.dok_id.as_str();
                let new_url = format!("{dokument_url}/{dok_id}");
                new_urls.push(new_url);
            }
        }

        items.push(item);
        Ok((items, new_urls))
    }

    #[tracing::instrument(skip(item))]
    async fn process(&self, mut url: String, item: Self::Item) -> Result<String, Error> {
        // let (url, item) = item;
        let mut path = self.output_path.clone();
        let file_name;
        tracing::info!("analyzing url={}", url);
        match &item {
            Item::DokumentLista(dokumentlista) => {
                path.push("dokumentlista");
                file_name = dokumentlista.q.as_str().replace('&', "_");
            }
            Item::DokumentStatus(dokumentstatus) => {
                let dokument_typ = dokumentstatus.dokument.typ
                .as_str()
                // .unwrap_or("NO_TYP")
                ;
                path.push(dokument_typ);
                let dokument_rm = dokumentstatus.dokument.rm.as_str();
                path.push(dokument_rm);

                file_name = dokumentstatus
                    .dokument
                    .dok_id
                    .as_str()
                    .replace([' ', '.'], "_");
            }
            _ => {
                file_name = String::new();
                url = format!("{url}_text");
                path.push("unknown");
            }
        }
        fs_err::tokio::create_dir_all(&path)
            .await
            .inspect_err(|_err| {
                tracing::error!("failed creating path='{}', url={}", path.display(), url);
            })?;
        if file_name.is_empty() {
            path.push(
                format!("unknown-{}", Ulid::new()), // .replace(':', "")
                                                    // .replace([' ', '.', '/'], "_"),
            );
        } else {
            path.push(Ulid::new().to_string());
            // path.push(&file_name);
        }
        // let file_name = format!("{file_name}.json");
        path.set_extension("json.gz");
        let span = tracing::info_span!("writing output", "{}", path.display());
        let _enter = span.enter();
        tracing::info!("creating file");
        let file = fs::File::create(&path).inspect_err(|_err| {
            tracing::error!("failed creating file, url={}", url);
        })?;
        let compress_writer = flate2::write::GzEncoder::new(file, Compression::default());
        let writer = std::io::BufWriter::new(compress_writer);
        tracing::info!("writing JSON");
        serde_json::to_writer(writer, &item).inspect_err(|_err| {
            tracing::error!("failed writing JSON, url={}", url);
        })?;
        Ok(path.display().to_string())
    }
}
