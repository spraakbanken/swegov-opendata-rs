use opendata_spiders::item::Item;

use std::{error::Error, fs, io};

fn main() -> Result<(), Box<dyn Error>> {
    // read_dokumentlista_json("assets/dokumentlista.json")?;

    // test_dokumentstatus("assets/sfs-1880-48_s_1.json")?;

    // test_dokumentstatus("assets/sfs-1880-cds0riksb.json")?;

    // test_dokumentstatus("assets/sfs-1976-114.json")?;
    // test_dokumentstatus("assets/sfs-1909-bih__29_s_1.json")?;
    read_xml("assets/dokumentlista.xml")?;
    read_xml("assets/dokumentlista_no_nasta_sida.xml")?;
    read_xml("assets/dokumentlista_2021_2023.xml")?;
    // read_xml("assets/dokumentlista_2021_2023.xml")?;
    // read_dokumentstatus_xml("assets/sfs-1994-2076-serialized.xml")?;
    read_xml("assets/sfs-1994-2076.xml")?;

    Ok(())
}

fn read_xml(path: &str) -> Result<(), Box<dyn Error>> {
    println!(" === Reading '{}'", path);
    let file = fs::File::open(path)?;
    println!("{:<5}: {}", "DokumentStatus", path);
    let buf_reader = io::BufReader::new(file);
    // let mut reader = quick_xml::reader::NsReader::from_reader(buf_reader);
    // reader.trim_text(true);
    let item: Item = quick_xml::de::from_reader(buf_reader)?;
    // let item = Item::deserialize_xml(&mut reader)?;
    // let item = xml_to_json(&mut reader)?;
    println!("{:#?}", item);

    // let string = quick_xml::se::to_string(&dokumentlista)?;
    // println!("serialized: '{}'", string);

    // println!(" === Re-deserialization");
    // let dokumentlista: DokumentStatus = quick_xml::de::from_str(&string)?;
    // println!("{:#?}", dokumentlista);
    Ok(())
}
