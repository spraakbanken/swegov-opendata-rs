use std::fs;
use std::{error::Error, io::BufReader};
use swegov_opendata::{DokumentLista, DokumentListaPage, DokumentStatus, DokumentStatusPage};

fn main() -> Result<(), Box<dyn Error>> {
    read_dokumentlista_json("assets/dokumentlista.json")?;

    test_dokumentstatus("assets/sfs-1880-48_s_1.json")?;

    test_dokumentstatus("assets/sfs-1880-cds0riksb.json")?;

    test_dokumentstatus("assets/sfs-1976-114.json")?;
    test_dokumentstatus("assets/sfs-1909-bih__29_s_1.json")?;
    read_dokumentlista_xml("assets/dokumentlista.xml")?;
    read_xml("assets/dokumentlista_2021_2023.xml")?;
    read_dokumentlista_xml("assets/dokumentlista_2021_2023.xml")?;
    // read_dokumentstatus_xml("assets/sfs-1994-2076-serialized.xml")?;
    read_dokumentstatus_xml("assets/sfs-1994-2076.xml")?;

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

fn read_dokumentstatus_xml(path: &str) -> Result<(), Box<dyn Error>> {
    println!(" === Reading '{}'", path);
    let file = fs::File::open(path)?;
    println!("{:<5}: {}", "DokumentStatus", path);
    let buf_reader = BufReader::new(file);
    let dokumentlista: DokumentStatus = quick_xml::de::from_reader(buf_reader)?;
    println!("{:#?}", dokumentlista);

    let string = quick_xml::se::to_string(&dokumentlista)?;
    println!("serialized: '{}'", string);

    println!(" === Re-deserialization");
    let dokumentlista: DokumentStatus = quick_xml::de::from_str(&string)?;
    println!("{:#?}", dokumentlista);
    Ok(())
}

fn read_xml(path: &str) -> Result<(), Box<dyn Error>> {
    use quick_xml::events::Event;
    println!(" === Reading '{}'", path);
    let file = fs::File::open(path)?;
    println!("{:<5}: {}", "DokumentStatus", path);
    let buf_reader = BufReader::new(file);
    let mut reader = quick_xml::reader::Reader::from_reader(buf_reader);
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Decl(_)) => println!("decl"),
            Ok(Event::Text(txt)) => println!("Text({:?}", txt),
            Ok(Event::Start(start)) => {
                println!(
                    "<{}>",
                    String::from_utf8(start.name().into_inner().to_vec())?
                )
            }
            Ok(Event::Empty(start)) => {
                println!(
                    "<{} />",
                    String::from_utf8(start.name().into_inner().to_vec())?
                )
            }
            Ok(Event::End(end)) => {
                println!(
                    "</{}>",
                    String::from_utf8(end.name().into_inner().to_vec())?
                )
            }
            Ok(Event::Eof) => break,
            evt => todo!("handle {:?}", evt),
        }
    }
    Ok(())
}

fn read_dokumentlista_xml(path: &str) -> Result<(), Box<dyn Error>> {
    println!(" === Reading '{}'", path);
    let file = fs::File::open(path)?;
    println!("{:<5}: {}", "DokumentLista", path);
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
