use std::error::Error;
use std::fs;
use swegov_opendata::{DokumentListaPage, DokumentStatusPage};

fn main() -> Result<(), Box<dyn Error>> {
    let dokumentlista_path = "assets/dokumentlista.json";

    let dokumentlista_file = fs::File::open(dokumentlista_path)?;
    let dokumentlista: DokumentListaPage = serde_json::from_reader(dokumentlista_file)?;
    println!("{:#?}", dokumentlista);

    test_dokumentstatus("assets/sfs-1880-48_s_1.json")?;

    test_dokumentstatus("assets/sfs-1880-cds0riksb.json")?;

    test_dokumentstatus("assets/sfs-1976-114.json")?;
    Ok(())
}

fn test_dokumentstatus(path: &str) -> Result<(), Box<dyn Error>> {
    println!(" === Reading '{}'", path);
    let dokumentstatus_file = fs::File::open(path)?;
    println!("{:<5}: {}", "DokumentStatus", path);
    let dokumentstatus: DokumentStatusPage = serde_json::from_reader(dokumentstatus_file)?;
    println!("{:#?}", dokumentstatus);

    let string = serde_json::to_string(&dokumentstatus)?;
    println!("serialized: {}", string);
    Ok(())
}
