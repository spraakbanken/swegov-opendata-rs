use std::str::FromStr;

use chrono::NaiveDate;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SweDate(NaiveDate);

impl From<NaiveDate> for SweDate {
    fn from(value: NaiveDate) -> Self {
        Self(value)
    }
}

impl FromStr for SweDate {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(swe_date_format::parse_from_str(s)?))
    }
}

impl SweDate {
    pub fn as_inner(&self) -> NaiveDate {
        self.0
    }
}
impl serde::Serialize for SweDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&swe_date_format::to_string(&self.0))
    }
}

impl<'de> serde::Deserialize<'de> for SweDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let date = swe_date_format::parse_from_str(&s).map_err(serde::de::Error::custom)?;
        Ok(SweDate(date))
    }
}

impl yaserde::YaSerialize for SweDate {
    fn serialize<W: std::io::Write>(
        &self,
        writer: &mut yaserde::ser::Serializer<W>,
    ) -> Result<(), String> {
        let name = writer
            .get_start_event_name()
            .unwrap_or_else(|| "SweDate".to_string());
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

impl yaserde::YaDeserialize for SweDate {
    fn deserialize<R: std::io::Read>(
        reader: &mut yaserde::de::Deserializer<R>,
    ) -> Result<Self, String> {
        loop {
            match reader.next_event()? {
                xml::reader::XmlEvent::StartElement { .. } => {}
                xml::reader::XmlEvent::Characters(ref text_content) => {
                    return SweDate::from_str(text_content).map_err(|e| e.to_string());
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
    use chrono::NaiveDate;
    const FORMAT: &str = "%Y-%m-%d";

    pub fn to_string(date: &NaiveDate) -> String {
        date.format(FORMAT).to_string()
    }

    pub fn parse_from_str(s: &str) -> chrono::ParseResult<NaiveDate> {
        NaiveDate::parse_from_str(s, FORMAT)
    }
}
