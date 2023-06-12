use quick_xml::{events::BytesStart, NsReader};
mod error;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub fn xml_to_json<R>(reader: &mut NsReader<R>) -> Result<serde_json::Value>
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
                let obj = xml_to_json_value(reader, &start)?;
                let key = String::from_utf8(start.name().as_ref().to_vec()).unwrap();
                json[key] = obj
            }
            Event::Empty(start) => {
                let obj = json_object_from_attributes(&start)?;
                let key = String::from_utf8(start.name().as_ref().to_vec()).unwrap();
                json[key] = obj
            }
            Event::Eof => break,
            evt => todo!("handle {:?}", evt),
        }
    }
    Ok(json)
}

use serde_json::{Map, Value};

pub fn xml_to_json_value<R>(
    reader: &mut NsReader<R>,
    start: &BytesStart,
) -> Result<serde_json::Value>
where
    R: std::io::BufRead,
{
    use quick_xml::events::Event;
    use serde_json::{Map, Value};

    let mut obj = json_object_from_attributes(start)?;
    let mut buf = Vec::new();
    let expected_end = start.to_end().to_owned();
    dbg!(&start);
    dbg!(String::from_utf8_lossy(start.name().as_ref()));
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::End(end) if end.name() == start.name() => break,
            Event::Empty(start) => {
                let val = json_object_from_attributes(&start)?;
                let key = String::from_utf8(start.name().as_ref().to_vec()).unwrap();
                if obj.is_null() {
                    obj = Value::Object(Map::<String, Value>::new());
                }
                obj[key] = val;
            }
            Event::Start(start) => {
                let val = xml_to_json_value(reader, &start)?;
                let key = String::from_utf8(start.name().as_ref().to_vec()).unwrap();
                if obj.is_null() {
                    obj = Value::Object(Map::<String, Value>::new());
                }
                if obj.get(&key).is_some() {
                    if !obj[&key].is_array() {
                        obj[&key] = Value::Array(vec![obj[&key].take()]);
                    }
                    obj[key].as_array_mut().unwrap().push(val);
                } else {
                    obj[key] = val;
                }
            }
            Event::Text(text) => {
                let val = text.unescape()?.to_string();
                if obj.is_null() {
                    obj = Value::String(val);
                } else {
                    todo!("text={:?} and obj={:?}", text, obj)
                }
            }
            Event::End(end) => {
                dbg!(&end);
                dbg!(String::from_utf8_lossy(end.name().as_ref()));
                dbg!(String::from_utf8_lossy(start.name().as_ref()));
                // dbg!(expected_end.name().as_ref());
                if end.name().as_ref() == expected_end.name().as_ref() {
                    break;
                }
                todo!()
            } //if end.name() == start.name() => break,
            evt => todo!("handle {:?}", evt),
        }
    }
    dbg!(&obj);
    Ok(obj)
}

fn json_object_from_attributes(start: &BytesStart) -> Result<serde_json::Value> {
    let mut obj = Value::Null;
    for attr in start.attributes() {
        let attr = attr?;
        let attr_key = String::from_utf8(attr.key.as_ref().to_vec()).unwrap();
        let attr_value = attr.unescape_value().unwrap().to_string();
        let attr_key = format!("@{attr_key}");
        if obj.is_null() {
            obj = Value::Object(Map::<String, Value>::new());
        }
        obj[attr_key] = Value::String(attr_value);
    }
    Ok(obj)
}
