use std::{fs, io};
use swegov_opendata::DokumentStatusPage;

fn main() -> anyhow::Result<()> {
    parse_html("data/sfs/output/sfs/2023/sfs-2023-105.json.gz")?;
    Ok(())
}

fn parse_html(path: &str) -> anyhow::Result<()> {
    println!("path: {}", path);
    let file = fs::File::open(path)?;
    let buf_reader = io::BufReader::new(file);
    let reader = flate2::bufread::GzDecoder::new(buf_reader);
    let DokumentStatusPage { dokumentstatus } = serde_json::from_reader(reader)?;
    // println!("{:#?}", dokumentstatus.dokument.html);
    get_div(&dokumentstatus.dokument.html.unwrap())?;
    Ok(())
}

fn get_div(html: &str) -> anyhow::Result<()> {
    println!("{} ...", &html[0..1000]);
    Ok(())
}
