use std::{
    fs,
    io::{self, Read},
    path::Path,
};

use anyhow::Context;
use swegov_opendata::DokumentStatusPage;
use tracing::Level;
use tracing_subscriber::EnvFilter;
mod corpus;
pub mod error;
use corpus::{Corpus, Text};

fn main() {
    let file_appender = tracing_appender::rolling::never("logs", "sfs-corpus.jsonl");
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::fmt()
        .json()
        .with_span_list(true)
        .with_current_span(true)
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("sfs_corpus=trace,info"))
                .expect("telemetry: Creating EnvFilter"),
        )
        // .with_writer(io::stderr)
        .with_writer(file_appender)
        .finish();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).expect("telemetry: setting subscriber");

    if let Err(err) = try_main() {
        eprintln!("Error: {:?}", err);
    }
}

fn try_main() -> anyhow::Result<()> {
    let args = std::env::args();
    let mut args = args.skip(1);
    let input = args.next().expect("`INPUT` required");
    let output = args.next().expect("`OUTPUT` required");

    walk_and_build_sparv_xml(&input, &output)?;
    Ok(())
}

#[tracing::instrument]
pub fn walk_and_build_sparv_xml(input: &str, output: &str) -> anyhow::Result<()> {
    println!("Reading from '{}' ...", input);

    for year in fs::read_dir(input)? {
        let year = year?;
        match build_sparv_xml_from_year(year.path().as_path(), Path::new(output)) {
            Ok(()) => {}
            Err(error) => {
                tracing::error!(?year, error = ?error, "Error when processing year");
                continue;
            }
        };
    }
    Ok(())
}

#[tracing::instrument]
pub fn build_sparv_xml_from_year(path: &Path, output_base: &Path) -> anyhow::Result<()> {
    fs::create_dir_all(output_base)?;
    let mut corpus = Corpus::new("sfs");
    for file_path in fs::read_dir(path)? {
        let file_path = file_path?;
        tracing::event!(
            Level::INFO,
            file_path = ?file_path,
            "reading a file"
        );
        tracing::debug!("reading text from {}", file_path.path().display());
        let text = read_text(file_path.path().as_path())?;

        tracing::debug!("adding text to corpus");
        corpus.add_text(text);
    }

    let year_str = path.display().to_string();
    let year = year_str.rsplit_once('/').unwrap().1;
    let year_filename = format!("sfs-{year}.xml");
    let year_filename = output_base.join(year_filename);
    // tracing::event!("filename = {:#?}", year_filename);
    // tracing::debug!("sfs_year = {:#?}", sfs_year);
    tracing::debug!("writing corpus");
    write_corpus(&corpus, &year_filename)
        .with_context(|| format!("error when writing corpus to '{}'", year_filename.display()))?;
    Ok(())
}

#[tracing::instrument]
pub fn read_text(path: &Path) -> anyhow::Result<Text> {
    let file = fs::File::open(path)?;
    let file_reader = io::BufReader::new(file);
    let mut reader = flate2::read::GzDecoder::new(file_reader);

    tracing::debug!("[main.read_text] parsing JSON");
    let mut text = String::new();
    reader.read_to_string(&mut text)?;
    let DokumentStatusPage { dokumentstatus } = serde_json::from_str(&text)
        .map_err(|err| {
            // tracing::debug!("text={}", text);
            err
        })
        .with_context(|| format!("Failed to deserialize file: {}", path.display()))?;
    // let DokumentStatusPage { dokumentstatus } = dokumentstatus;
    tracing::debug!("[main.read_text] create Text");
    Ok(Text::try_from(dokumentstatus)
        .with_context(|| format!("Failed to parse Text from '{}'", path.display()))?)
}

#[tracing::instrument]
pub fn write_corpus(corpus: &Corpus, path: &Path) -> anyhow::Result<()> {
    // let mut buffer = String::new();
    // quick_xml::se::to_writer(&mut buffer, corpus)?;
    let file = fs::File::create(path)?;
    let mut writer = io::BufWriter::new(file);

    tracing::debug!("building elem from corpus");
    let elem = corpus.build_elem();
    tracing::debug!("built elem from corpus, writing ...");
    // dbg!(&elem);
    elem.write_to(&mut writer)?;
    // file.write_all(&buffer.as_bytes())?;
    Ok(())
}
