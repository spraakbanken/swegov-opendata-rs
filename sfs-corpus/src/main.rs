use std::{
    fmt::Write,
    fs,
    io::{self, Read, Write as IoWrite},
    path::Path,
};

use anyhow::Context;
use swegov_opendata::{Dokument, DokumentStatus, DokumentStatusPage};
use walkdir::{DirEntry, WalkDir};
mod corpus;
use corpus::{Corpus, Text};

fn main() {
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

pub fn walk_and_build_sparv_xml(input: &str, output: &str) -> anyhow::Result<()> {
    println!(
        "walk_and_build_sparv_xml(input='{}', output='{}')",
        input, output
    );
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
        build_sparv_xml_from_year(year.path().as_path(), Path::new(output))?;
    }
    Ok(())
}

pub fn build_sparv_xml_from_year(path: &Path, output_base: &Path) -> anyhow::Result<()> {
    println!("processing {:?}", path);
    fs::create_dir_all(output_base)?;
    let mut corpus = Corpus::new("sfs");
    for file_path in fs::read_dir(path)? {
        let file_path = file_path?;
        corpus.add_text(read_text(file_path.path().as_path())?);
    }

    let year_str = path.display().to_string();
    let year = year_str.rsplit_once('/').unwrap().1;
    let year_filename = format!("sfs-{year}.xml");
    let year_filename = output_base.join(year_filename);
    println!("filename = {:#?}", year_filename);
    // println!("sfs_year = {:#?}", sfs_year);
    write_corpus(&corpus, &year_filename)?;
    Ok(())
}

pub fn read_text(path: &Path) -> anyhow::Result<Text> {
    println!("reading text from '{}'", path.display());
    let file = fs::File::open(path)?;
    let file_reader = io::BufReader::new(file);
    let mut reader = flate2::read::GzDecoder::new(file_reader);

    let mut text = String::new();
    reader.read_to_string(&mut text)?;
    let DokumentStatusPage { dokumentstatus } = serde_json::from_str(&text)
        .map_err(|err| {
            // println!("text={}", text);
            err
        })
        .with_context(|| format!("Failed to deserialize file: {}", path.display()))?;
    // let DokumentStatusPage { dokumentstatus } = dokumentstatus;
    Ok(Text::try_from(dokumentstatus)
        .with_context(|| format!("Failed to parse Text from '{}'", path.display()))?)
}

pub fn write_corpus(corpus: &Corpus, path: &Path) -> anyhow::Result<()> {
    let mut buffer = String::new();
    quick_xml::se::to_writer(&mut buffer, corpus)?;
    let mut file = fs::File::create(path)?;
    file.write_all(&buffer.as_bytes())?;
    Ok(())
}
