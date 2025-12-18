use fs_err::{self as fs, PathExt};
use std::{
    collections::HashMap,
    fmt,
    path::{Path, PathBuf},
    sync::Arc,
};

use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use reqwest::Client;
use swegov_opendata::{DataFormat, DatasetLista};
use tokio::{io::AsyncWriteExt, sync::RwLock};

use crate::{Error, Item};

#[derive(Debug, Clone)]
pub struct RdSpiderOptions {
    pub user_agent: Option<String>,
    pub output_path: PathBuf,
}

impl Default for RdSpiderOptions {
    fn default() -> Self {
        Self {
            user_agent: None,
            output_path: PathBuf::from("./output"),
        }
    }
}

pub struct RdSpider {
    http_client: Client,
    output_path: PathBuf,
    metadata: Arc<RwLock<Metadata>>,
}

#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Metadata {
    metadata: HashMap<String, MetadataField>,
}

impl Metadata {
    pub fn open(path: &Path) -> Result<Self, Error> {
        tracing::debug!("loading metadata from {}", path.display());
        let file_data = fs::read_to_string(path).map_err(|source| Error::CouldNotReadFile {
            path: path.display().to_string(),
            source,
        })?;
        let metadata =
            serde_json::from_str(&file_data).map_err(|source| Error::CouldNotParseJson {
                path: path.display().to_string(),
                source,
            })?;
        Ok(Self { metadata })
    }
    pub fn open_or_default(path: &Path) -> Result<Self, Error> {
        match Self::open(path) {
            Err(Error::CouldNotReadFile { path, source }) => {
                tracing::info!(cause = ?source,"Could not read Metadata from '{}', creating default", path);
                Ok(Self::default())
            }
            Err(err) => {
                tracing::warn!(cause = ?err, "Error reading Metadata from '{}'", path.display());
                Err(err)
            }
            Ok(metadata) => {
                tracing::info!("Read Metadata from {}", path.display());
                Ok(metadata)
            }
        }
    }
    pub fn write(&self, path: &Path) -> Result<(), Error> {
        let file = std::fs::File::create(path).map_err(|source| Error::CouldNotCreateFile {
            path: path.display().to_string(),
            source,
        })?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, &self.metadata).map_err(|source| {
            Error::CouldNotSerializeJson {
                path: path.display().to_string(),
                source,
            }
        })?;
        tracing::info!("Wrote Metadata to {}", path.display());

        Ok(())
    }

    pub fn should_be_updated(&self, url: &str, uppdaterad: NaiveDateTime) -> bool {
        match self.metadata.get(url) {
            None => true,
            Some(meta) => meta.uppdated.naive_local() < uppdaterad,
        }
    }

    pub fn mark_as_updated(&mut self, url: &str, file_name: &Path) {
        self.metadata.insert(
            url.to_string(),
            MetadataField {
                file_name: file_name.display().to_string(),
                uppdated: Utc::now(),
            },
        );
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct MetadataField {
    file_name: String,
    uppdated: DateTime<Utc>,
}

impl RdSpider {
    const START_URL: &'static str = "https://data.riksdagen.se/dataset/katalog/dataset.Xml";
    const BASE_URL: &'static str = "https://data.riksdagen.se";

    fn metadata_path(output_path: &Path) -> PathBuf {
        output_path.join("metadata-dataset.json")
    }
    pub fn new(
        RdSpiderOptions {
            user_agent: user_agent_opt,
            output_path,
        }: RdSpiderOptions,
    ) -> Result<Self, Error> {
        let user_agent = user_agent_opt.as_deref().unwrap_or(crate::APP_USER_AGENT);

        fs::create_dir_all(&output_path).map_err(|source| Error::CouldNotCreateFolder {
            path: output_path.display().to_string(),
            source,
        })?;
        let output_path =
            output_path
                .fs_err_canonicalize()
                .map_err(|source| Error::GeneralIoError {
                    path: output_path.display().to_string(),
                    source,
                })?;
        tracing::warn!(user_agent, "configuring SfsSpider {:?}", output_path);
        let metadata = Metadata::open_or_default(&Self::metadata_path(&output_path))?;
        let http_client = reqwest::Client::builder()
            .user_agent(user_agent)
            .gzip(true)
            .build()?;

        Ok(Self {
            http_client,
            output_path,
            metadata: Arc::new(RwLock::new(metadata)),
        })
    }

    pub async fn close(&self) -> Result<(), Error> {
        self.metadata
            .read()
            .await
            .write(&Self::metadata_path(&self.output_path))?;
        Ok(())
    }
}

impl fmt::Debug for RdSpider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RdSpider {{ /* omitted */ }}")
    }
}

#[async_trait]
impl webcrawler::Spider for RdSpider {
    type Item = Item;
    type Error = Error;

    fn name(&self) -> String {
        String::from("swegov-opendata-rd")
    }

    fn start_urls(&self) -> Vec<String> {
        vec![Self::START_URL.to_string()]
    }

    #[tracing::instrument]
    async fn scrape(&self, url: String) -> Result<(Vec<Self::Item>, Vec<String>), Self::Error> {
        tracing::debug!("calling {}", url);
        let response = self.http_client.get(&url).send().await?;
        tracing::trace!("response status: {}", response.status());

        if !response.status().is_success() {
            let status_code = response.status();
            let text = response.text().await?;

            tracing::error!(
                response.status = ?status_code,
                response.text = text,
                url = url,
                "A request returned non-successful status"
            );
            return Err(Error::RequestReturnedError(status_code));
        }

        let mut new_urls = Vec::new();
        let mut items = Vec::new();
        if url == Self::START_URL {
            let text = response.text().await?;

            let text = text.replace("\r\n", "");
            let DatasetLista { dataset } = yaserde::de::from_str(&text)
                .map_err(|msg| Error::CouldNotParseXml { src: text, msg })?;
            for dataset in dataset {
                if dataset.format == DataFormat::Json
                    && self
                        .metadata
                        .read()
                        .await
                        .should_be_updated(&dataset.url, dataset.uppdaterad.as_inner())
                {
                    new_urls.push(format!("{}{}", Self::BASE_URL, dataset.url));
                    items.push(Item::Metadata(dataset));
                }
            }
        } else {
            let bytes = response.bytes().await?;

            items.push(Item::Raw(bytes.to_vec()));
        }
        Ok((items, new_urls))
    }

    #[tracing::instrument(skip(item))]
    async fn process(&self, url: String, item: Self::Item) -> Result<String, Self::Error> {
        let mut update_metadata = false;
        let (data, path) = match item {
            Item::Metadata(dataset) => {
                let mut path = self.output_path.join(&dataset.url[1..]);
                if path.extension().is_some() {
                    path.set_extension("");
                }
                if path.extension().is_some() {
                    path.set_extension("");
                }
                path.set_extension("metadata.json");
                (
                    serde_json::to_vec(&dataset).map_err(|source| {
                        Error::CouldNotSerializeJson {
                            path: path.display().to_string(),
                            source,
                        }
                    })?,
                    path,
                )
            }
            Item::Raw(data) => {
                // let url = Url::parse(&url).map_err(|source| Error::UrlParseError {
                //     url: url.clone(),
                //     source,
                // })?;
                let url_path = match url.strip_prefix(Self::BASE_URL) {
                    Some(url_path) => url_path,
                    None => todo!("Handle download from url={}", url),
                };
                let path = self.output_path.join(&url_path[1..]);
                update_metadata = true;
                (data, path)
            }
        };
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                tracing::info!("creating folder {}", parent.display());
                fs_err::tokio::create_dir_all(parent)
                    .await
                    .map_err(|source| Error::CouldNotCreateFolder {
                        path: parent.display().to_string(),
                        source,
                    })?;
            }
        }
        tracing::info!("writing to path {}", path.display());
        let mut file = fs_err::tokio::File::create(&path).await.map_err(|source| {
            Error::CouldNotCreateFile {
                path: path.display().to_string(),
                source,
            }
        })?;
        file.write_all(&data)
            .await
            .map_err(|source| Error::CouldNotWriteFile {
                path: path.display().to_string(),
                source,
            })?;
        file.flush().await.map_err(|source| Error::GeneralIoError {
            path: path.display().to_string(),
            source,
        })?;
        if update_metadata {
            self.metadata.write().await.mark_as_updated(&url, &path);
        }
        Ok(path.display().to_string())
    }
}
