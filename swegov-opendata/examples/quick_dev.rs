use std::fs;
use std::{error::Error, io::BufReader};
use swegov_opendata::{DokumentLista, DokumentListaPage, DokumentStatusPage};

fn main() -> Result<(), Box<dyn Error>> {
    read_dokumentlista_json("assets/dokumentlista.json")?;

    test_dokumentstatus("assets/sfs-1880-48_s_1.json")?;

    test_dokumentstatus("assets/sfs-1880-cds0riksb.json")?;

    test_dokumentstatus("assets/sfs-1976-114.json")?;
    test_dokumentstatus("assets/sfs-1909-bih__29_s_1.json")?;
    read_dokumentlista_xml("assets/dokumentlista.xml")?;

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

fn read_dokumentlista_xml(path: &str) -> Result<(), Box<dyn Error>> {
    println!(" === Reading '{}'", path);
    let file = fs::File::open(path)?;
    println!("{:<5}: {}", "DokumentStatus", path);
    let buf_reader = BufReader::new(file);
    let dokumentlista: DokumentLista = quick_xml::de::from_reader(buf_reader)?;
    println!("{:#?}", dokumentlista);

    let string = quick_xml::se::to_string(&dokumentlista)?;
    println!("serialized: {}", string);
    let dokumentlista: DokumentLista = quick_xml::de::from_str(&string)?;
    println!("{:#?}", dokumentlista);
    Ok(())
}

fn read_dokumentlista_json(path: &str) -> Result<(), Box<dyn Error>> {
    println!(" === Reading '{}'", path);
    let dokumentlista_file = fs::File::open(path)?;
    let dokumentlista: DokumentListaPage = serde_json::from_reader(dokumentlista_file)?;
    println!("{:#?}", dokumentlista);

    let string = serde_json::to_string(&dokumentlista)?;
    println!("serialized: {}", string);
    Ok(())
}
