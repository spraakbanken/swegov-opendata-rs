// copied from: https://gist.github.com/rust-play/e7320f84b125bcac0c41f349b03d2f46

use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::fmt::{self, Display};
use std::str::FromStr;
use xml::writer::XmlEvent;
use yaserde::{YaDeserialize, YaSerialize};

use crate::DokumentListaDokument;

#[derive(Debug)]
pub enum TryParse<T> {
    Parsed(T),
    Unparsed(Value),
    NotPresent,
}

impl<T: Clone> Clone for TryParse<T> {
    fn clone(&self) -> Self {
        match self {
            TryParse::NotPresent => TryParse::NotPresent,
            TryParse::Parsed(t) => TryParse::Parsed(t.clone()),
            TryParse::Unparsed(v) => TryParse::Unparsed(v.clone()),
        }
    }
}

// #[derive(Deserialize, Debug)]
// struct Foo4 {
//     inner: TryParse<Inner>,

//     #[serde(flatten)]
//     other: HashMap<String, Value>,
// }

// #[derive(Deserialize, Debug, Default)]
// struct Inner {
//     i1: i32,
//     i2: i32,
// }

impl<'de, T: DeserializeOwned> Deserialize<'de> for TryParse<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match Option::<Value>::deserialize(deserializer)? {
            None => Ok(TryParse::NotPresent),
            Some(value) => match T::deserialize(&value) {
                Ok(t) => Ok(TryParse::Parsed(t)),
                Err(_) => Ok(TryParse::Unparsed(value)),
            },
        }
    }
}

// impl<'de, T: Deserialize<'de> + Clone> Deserialize<'de> for TryParse<T> {
//     fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
//         match Option::<Value>::deserialize(deserializer)? {
//             None => Ok(TryParse::NotPresent),
//             Some(value) => match T::deserialize(&value) {
//                 Ok(t) => Ok(TryParse::Parsed(t.clone())),
//                 Err(_) => Ok(TryParse::Unparsed(value.clone())),
//             },
//         }
//     }
// }

impl<T: Serialize> Serialize for TryParse<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            TryParse::NotPresent => serializer.serialize_none(),
            TryParse::Parsed(t) => t.serialize(serializer),
            TryParse::Unparsed(v) => v.serialize(serializer),
        }
    }
}

impl<T: Display> YaSerialize for TryParse<T> {
    fn serialize<W: std::io::Write>(
        &self,
        writer: &mut yaserde::ser::Serializer<W>,
    ) -> Result<(), String> {
        let name = writer
            .get_start_event_name()
            .unwrap_or_else(|| "TryParse".to_string());
        if !writer.skip_start_end() {
            writer
                .write(XmlEvent::start_element(name.as_str()))
                .map_err(|e| e.to_string())?;
        }
        match self {
            TryParse::NotPresent => {}
            TryParse::Parsed(v) => {
                writer
                    .write(XmlEvent::characters(&v.to_string()))
                    .map_err(|e| e.to_string())?;
            }
            TryParse::Unparsed(v) => {
                writer
                    .write(XmlEvent::characters(&v.to_string()))
                    .map_err(|e| e.to_string())?;
            }
        }
        if !writer.skip_start_end() {
            writer
                .write(XmlEvent::end_element())
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    fn serialize_attributes(
        &self,
        attributes: Vec<xml::attribute::OwnedAttribute>,
        namespace: xml::namespace::Namespace,
    ) -> Result<
        (
            Vec<xml::attribute::OwnedAttribute>,
            xml::namespace::Namespace,
        ),
        String,
    > {
        Ok((attributes, namespace))
    }
}

impl<T: FromStr> YaDeserialize for TryParse<T>
where
    T: FromStr,
    T::Err: Display,
{
    fn deserialize<R: std::io::Read>(
        reader: &mut yaserde::de::Deserializer<R>,
    ) -> Result<Self, String> {
        loop {
            match reader.peek()? {
                xml::reader::XmlEvent::StartElement { .. } => {}
                xml::reader::XmlEvent::Characters(ref text) => {
                    return TryParse::<T>::from_str(text).map_err(|e| e.to_string());
                }
                _ => {
                    break;
                }
            }
            let _next = reader.next_event();
        }
        Err("Unable to parse TryParse".to_string())
    }
}

impl<T: FromStr> FromStr for TryParse<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match T::from_str(s) {
            Ok(t) => Ok(TryParse::Parsed(t)),
            Err(_) => Ok(TryParse::Unparsed(s.into())),
        }
    }
}

pub fn deserialize_tryparse_from_string<'de, T, D>(deserializer: D) -> Result<TryParse<T>, D::Error>
where
    D: Deserializer<'de>,
    // T: FromStr + serde::Deserialize<'de> + Clone,
    T: FromStr + DeserializeOwned,
    <T as FromStr>::Err: fmt::Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrTryParse<T> {
        String(String),
        Value(T),
    }

    match StringOrTryParse::<T>::deserialize(deserializer)? {
        StringOrTryParse::String(s) => s.parse::<TryParse<T>>().map_err(serde::de::Error::custom),
        StringOrTryParse::Value(t) => Ok(TryParse::Parsed(t)),
    }
}
