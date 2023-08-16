use std::{
    fmt::Write,
    fs,
    io::{self, Read, Write as IoWrite},
    path::Path,
};

use anyhow::Context;
use swegov_opendata::{Dokument, DokumentStatus, DokumentStatusPage};
use tracing::Level;
use tracing_subscriber::EnvFilter;
use walkdir::{DirEntry, WalkDir};
mod corpus;
pub mod error;
use corpus::{Corpus, Text};

fn main() {
    let file_appender = tracing_appender::rolling::never("logs", "sfs-corpus.jsonl");
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::fmt()
        .json()
        // .with_span_list(true)
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
    let mut args = std::env::args();
    let mut args = args.skip(1);
    let input = args.next().expect("`INPUT` required");
    let output = args.next().expect("`OUTPUT` required");

    walk_and_build_sparv_xml(&input, &output)?;
    Ok(())
}

#[tracing::instrument]
pub fn walk_and_build_sparv_xml(input: &str, output: &str) -> anyhow::Result<()> {
    let sfs_folder = if let Some(sfs_folder) = fs::read_dir(input)?.find(|e| {
        e.as_ref()
            .unwrap()
            .file_name()
            .to_str()
            .unwrap()
            .ends_with("sfs")
    }) {
        sfs_folder?
    } else {
        return Err(anyhow::anyhow!("Can't find `sfs` folder"));
    };
    println!("sfs_folder: {:?}", sfs_folder);

    for (i, year) in fs::read_dir(sfs_folder.path())?.enumerate() {
        let year = year?;
        // if i > 0 {
        //     continue;
        // }
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
        println!("reading text from {}", file_path.path().display());
        let text = read_text(file_path.path().as_path())?;

        println!("adding text to corpus");
        corpus.add_text(text);
    }

    let year_str = path.display().to_string();
    let year = year_str.rsplit_once('/').unwrap().1;
    let year_filename = format!("sfs-{year}.xml");
    let year_filename = output_base.join(year_filename);
    // tracing::event!("filename = {:#?}", year_filename);
    // println!("sfs_year = {:#?}", sfs_year);
    println!("writing corpus");
    write_corpus(&corpus, &year_filename)
        .with_context(|| format!("error when writing corpus to '{}'", year_filename.display()))?;
    Ok(())
}

#[tracing::instrument]
pub fn read_text(path: &Path) -> anyhow::Result<Text> {
    let file = fs::File::open(path)?;
    let file_reader = io::BufReader::new(file);
    let mut reader = flate2::read::GzDecoder::new(file_reader);

    println!("[main.read_text] parsing JSON");
    let mut text = String::new();
    reader.read_to_string(&mut text)?;
    let DokumentStatusPage { dokumentstatus } = serde_json::from_str(&text)
        .map_err(|err| {
            // println!("text={}", text);
            err
        })
        .with_context(|| format!("Failed to deserialize file: {}", path.display()))?;
    // let DokumentStatusPage { dokumentstatus } = dokumentstatus;
    println!("[main.read_text] create Text");
    Ok(Text::try_from(dokumentstatus)
        .with_context(|| format!("Failed to parse Text from '{}'", path.display()))?)
}

#[tracing::instrument]
pub fn write_corpus(corpus: &Corpus, path: &Path) -> anyhow::Result<()> {
    // let mut buffer = String::new();
    // quick_xml::se::to_writer(&mut buffer, corpus)?;
    let file = fs::File::create(path)?;
    let mut writer = io::BufWriter::new(file);

    println!("building elem from corpus");
    let elem = corpus.build_elem();
    println!("built elem from corpus, writing ...");
    // dbg!(&elem);
    elem.write_to(&mut writer)?;
    // file.write_all(&buffer.as_bytes())?;
    Ok(())
}
