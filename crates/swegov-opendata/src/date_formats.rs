use std::str::FromStr;

use chrono::NaiveDateTime;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SweDateTime(NaiveDateTime);

impl From<NaiveDateTime> for SweDateTime {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}

impl FromStr for SweDateTime {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(swe_date_format::parse_from_str(s)?))
    }
}

impl SweDateTime {
    pub fn as_inner(&self) -> NaiveDateTime {
        self.0
    }
}
impl serde::Serialize for SweDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&swe_date_format::to_string(&self.0))
    }
}

impl<'de> serde::Deserialize<'de> for SweDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let date = swe_date_format::parse_from_str(&s).map_err(serde::de::Error::custom)?;
        Ok(SweDateTime(date))
    }
}

impl yaserde::YaSerialize for SweDateTime {
    fn serialize<W: std::io::Write>(
        &self,
        writer: &mut yaserde::ser::Serializer<W>,
    ) -> Result<(), String> {
        let name = writer
            .get_start_event_name()
            .unwrap_or_else(|| "SweDateTime".to_string());
        if !writer.skip_start_end() {
            let event = xml::writer::XmlEvent::start_element(name.as_str());
            writer.write(event).map_err(|e| e.to_string())?;
        }
        let content = swe_date_format::to_string(&self.0);
        let event = xml::writer::XmlEvent::characters(&content);
        writer.write(event).map_err(|e| e.to_string())?;

        if !writer.skip_start_end() {
            let event = xml::writer::XmlEvent::end_element();
            writer.write(event).map_err(|e| e.to_string())?;
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

impl yaserde::YaDeserialize for SweDateTime {
    fn deserialize<R: std::io::Read>(
        reader: &mut yaserde::de::Deserializer<R>,
    ) -> Result<Self, String> {
        loop {
            match reader.next_event()? {
                xml::reader::XmlEvent::StartElement { .. } => {}
                xml::reader::XmlEvent::Characters(ref text_content) => {
                    return SweDateTime::from_str(text_content).map_err(|e| e.to_string());
                }
                _ => {
                    break;
                }
            }
        }
        Err("Unable to parse attribute".to_string())
    }
}

pub mod swe_date_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};
    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn to_string(date: &NaiveDateTime) -> String {
        date.format(FORMAT).to_string()
    }

    pub fn parse_from_str(s: &str) -> chrono::ParseResult<NaiveDateTime> {
        NaiveDateTime::parse_from_str(s, FORMAT)
    }

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&to_string(date))
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        parse_from_str(&s).map_err(serde::de::Error::custom)
    }
}

pub mod option_swe_date_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    use super::swe_date_format;
    pub fn serialize<S>(opt_date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match opt_date {
            None => serializer.serialize_none(),
            Some(date) => serializer.serialize_str(&swe_date_format::to_string(date)),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let option_s = Option::<String>::deserialize(deserializer)?;
        match option_s {
            None => Ok(None),
            Some(s) => {
                let date = swe_date_format::parse_from_str(&s).map_err(serde::de::Error::custom)?;
                Ok(Some(date))
            }
        }
    }
}

pub mod swe_date_format_or_empty_to_option {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    use super::swe_date_format;
    pub fn serialize<S>(opt_date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match opt_date {
            None => serializer.serialize_none(),
            Some(date) => serializer.serialize_str(&swe_date_format::to_string(date)),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let option_s = Option::<String>::deserialize(deserializer)?;
        match option_s {
            None => Ok(None),
            Some(s) if s.is_empty() => Ok(None),
            Some(s) => {
                let date = swe_date_format::parse_from_str(&s).map_err(serde::de::Error::custom)?;
                Ok(Some(date))
            }
        }
    }
}
