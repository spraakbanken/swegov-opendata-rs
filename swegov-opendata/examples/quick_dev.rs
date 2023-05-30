use std::error::Error;
use std::fs;
use swegov_opendata::{DokumentListaPage, DokumentStatusPage};

fn main() -> Result<(), Box<dyn Error>> {
    let dokumentlista_path = "assets/dokumentlista.json";

    let dokumentlista_file = fs::File::open(dokumentlista_path)?;
    let dokumentlista: DokumentListaPage = serde_json::from_reader(dokumentlista_file)?;
    println!("{:#?}", dokumentlista);

    let dokumentstatus_path = "assets/sfs-1880-48_s_1.json";

    let dokumentstatus_file = fs::File::open(dokumentstatus_path)?;
    let dokumentstatus: DokumentStatusPage = serde_json::from_reader(dokumentstatus_file)?;
    println!("{:#?}", dokumentstatus);
    Ok(())
}
