use deserx::DeXml;
use opendata_spiders::item::Item;
use quick_xml::{events::BytesStart, NsReader};
use std::{collections::HashMap, error::Error, fs, io};

fn main() -> Result<(), Box<dyn Error>> {
    // read_dokumentlista_json("assets/dokumentlista.json")?;

    // test_dokumentstatus("assets/sfs-1880-48_s_1.json")?;

    // test_dokumentstatus("assets/sfs-1880-cds0riksb.json")?;

    // test_dokumentstatus("assets/sfs-1976-114.json")?;
    // test_dokumentstatus("assets/sfs-1909-bih__29_s_1.json")?;
    read_xml("assets/dokumentlista.xml")?;
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
    let mut reader = quick_xml::reader::NsReader::from_reader(buf_reader);
    reader.trim_text(true);
    // let item = Item::deserialize_xml(&mut reader)?;
    let item = xml_to_json(&mut reader)?;
    println!("{:#?}", item);

    // let string = quick_xml::se::to_string(&dokumentlista)?;
    // println!("serialized: '{}'", string);

    // println!(" === Re-deserialization");
    // let dokumentlista: DokumentStatus = quick_xml::de::from_str(&string)?;
    // println!("{:#?}", dokumentlista);
    Ok(())
}

pub fn xml_to_json<R>(reader: &mut NsReader<R>) -> anyhow::Result<serde_json::Value>
where
    R: std::io::BufRead,
{
    use quick_xml::events::Event;
    use serde_json::{Map, Value};

    let mut json = Value::Object(Map::new());
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Decl(_) => {}
            Event::Start(start) => {
                // let mut obj = Value::Object(Map::<String, Value>::new());
                // for attr in start.attributes() {
                //     let attr = attr?;
                //     let attr_key = String::from_utf8(attr.key.as_ref().to_vec()).unwrap();
                //     let attr_value = attr.unescape_value().unwrap().to_string();
                //     let attr_key = format!("@{attr_key}");
                //     obj[attr_key] = Value::String(attr_value);
                // }
                let obj = xml_to_json_object(reader, &start)?;
                let key = String::from_utf8(start.name().as_ref().to_vec()).unwrap();
                json[key] = obj
            }
            Event::Empty(start) => {
                let mut obj = Value::Object(Map::<String, Value>::new());
                for attr in start.attributes() {
                    let attr = attr?;
                    let attr_key = String::from_utf8(attr.key.as_ref().to_vec()).unwrap();
                    let attr_value = attr.unescape_value().unwrap().to_string();
                    let attr_key = format!("@{attr_key}");
                    obj[attr_key] = Value::String(attr_value);
                }
                let key = String::from_utf8(start.name().as_ref().to_vec()).unwrap();
                json[key] = obj
            }
            evt => todo!("handle {:?}", evt),
        }
    }
    Ok(json)
}

use serde_json::{Map, Value};

pub fn xml_to_json_object<R>(
    reader: &mut NsReader<R>,
    start: &BytesStart,
) -> anyhow::Result<serde_json::Value>
where
    R: std::io::BufRead,
{
    use quick_xml::events::Event;
    use serde_json::{Map, Value};
    // let mut obj = Value::Object(Map::<String, Value>::new());
    // for attr in start.attributes() {
    //     let attr = attr?;
    //     let attr_key = String::from_utf8(attr.key.as_ref().to_vec()).unwrap();
    //     let attr_value = attr.unescape_value().unwrap().to_string();
    //     let attr_key = format!("@{attr_key}");
    //     obj[attr_key] = Value::String(attr_value);
    // }
    let mut obj = json_object_from_attributes(start)?;
    let mut buf = Vec::new();
    dbg!(String::from_utf8_lossy(start.name().as_ref()));
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::End(end) if end.name() == start.name() => break,
            Event::Empty(start) => {
                let val = json_object_from_attributes(&start)?;
                let key = String::from_utf8(start.name().as_ref().to_vec()).unwrap();
                obj[key] = val;
            }
            Event::Start(start) => {
                let val = xml_to_json_object(reader, &start)?;
                let key = String::from_utf8(start.name().as_ref().to_vec()).unwrap();
                obj[key] = val;
            }
            evt => todo!("handle {:?}", evt),
        }
    }
    Ok(obj)
}

fn json_object_from_attributes(start: &BytesStart) -> anyhow::Result<serde_json::Value> {
    let mut obj = Value::Object(Map::<String, Value>::new());
    for attr in start.attributes() {
        let attr = attr?;
        let attr_key = String::from_utf8(attr.key.as_ref().to_vec()).unwrap();
        let attr_value = attr.unescape_value().unwrap().to_string();
        let attr_key = format!("@{attr_key}");
        obj[attr_key] = Value::String(attr_value);
    }
    Ok(obj)
}
