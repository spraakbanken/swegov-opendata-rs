use std::{collections::BTreeMap, io::BufRead};

use chrono::NaiveDateTime;
use deserx::DeXmlError;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::NsReader;

use crate::date_formats;

#[cfg(test)]
mod tests;

#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename = "datasetlista")]
pub struct DatasetLista {
    pub dataset: Vec<DataSet>,
}

impl deserx::SerXml for DatasetLista {
    fn serialize_xml<W: std::io::Write>(
        &self,
        serializer: &mut quick_xml::Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.ser_as_element(serializer, "datasetlista")
    }

    fn ser_elem_body<W: std::io::Write>(
        &self,
        serializer: &mut quick_xml::Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.dataset.ser_elem_body(serializer)
    }
}

impl deserx::DeXml for DatasetLista {
    fn deserialize_xml<R: BufRead>(reader: &mut NsReader<R>) -> Result<Self, deserx::DeXmlError> {
        Self::deserialize_xml_from_tag(reader, "datasetlista")
    }

    fn deserialize_xml_from_body<R: BufRead>(
        reader: &mut NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, deserx::DeXmlError> {
        // dbg!(start);
        let mut buf = Vec::new();
        let mut dataset = Vec::new();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::End(e) => {
                    if e == start.to_end() {
                        break;
                    }
                    match e.name().as_ref() {
                        b"dataset" => (),
                        _ => todo!("handle {:?}", e),
                    }
                }

                Event::Start(e) => match e.name().as_ref() {
                    b"dataset" => {
                        let d = DataSet::deserialize_xml_from_body(reader, &e).unwrap(); //?;
                        dataset.push(d);
                    }
                    _ => todo!("handle {:?}", e),
                },
                // e => todo!("handle {:?}", e),
                Event::Text(e) => {
                    if e.unescape()?.trim().is_empty() {
                        continue;
                    } else {
                        return Err(DeXmlError::custom(format!("Unexpected text '{:?}'", e)));
                    }
                }
                e => {
                    return Err(DeXmlError::custom(format!(
                        "dataset.rs:67:22: handle {:?}",
                        e
                    )))
                }
            }
        }

        Ok(Self { dataset })
    }
    fn deserialize_xml_from_tag<R: BufRead>(
        reader: &mut NsReader<R>,
        tag: &str,
    ) -> Result<Self, DeXmlError> {
        // dbg!(tag);
        use quick_xml::events::Event;
        let mut buf = Vec::new();
        let self_: Self = match reader.read_event_into(&mut buf)? {
            Event::Empty(evt) if evt.name().as_ref() == tag.as_bytes() => Self::default(),
            Event::Start(evt) if evt.name().as_ref() == tag.as_bytes() => {
                Self::deserialize_xml_from_body(reader, &evt).unwrap() //?
                                                                       // Self::deserialize_xml_from_body_with_end(reader, &evt, evt.to_end())?
            }
            evt => {
                return Err(DeXmlError::UnexpectedTag {
                    tag: tag.to_string(),
                    event: format!("{:?}", evt),
                })
            }
        };

        Ok(self_)
    }
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, rename = "dataset")]
pub struct DataSet {
    pub namn: String,
    pub typ: String,
    pub samling: String,
    pub rm: String,
    pub filnamn: String,
    #[serde(rename = "storlek")]
    pub storlek_bytes: usize,
    // #[serde(with = "quick_xml::serde_helpers::text_content")]
    pub format: DataFormat,
    pub filformat: FilFormat,
    #[serde(with = "date_formats::swe_date_format")]
    pub uppdaterad: NaiveDateTime,
    pub url: String,
    pub description: String,
    pub beskrivning: Option<String>,
    // pub upplysning: String,
    pub upplysning: Option<Upplysning>,
}

#[derive(Debug, Clone, Default)]
pub struct DatasetBuilder {
    namn: Option<String>,
    typ: Option<String>,
    upplysning: Option<Upplysning>,
    beskrivning: Option<String>,
    description: Option<String>,
    url: Option<String>,
    uppdaterad: Option<NaiveDateTime>,
    filformat: Option<FilFormat>,
    format: Option<DataFormat>,
    storlek: Option<usize>,
    filnamn: Option<String>,
    rm: Option<String>,
    samling: Option<String>,
}

impl DatasetBuilder {
    pub fn samling(&mut self, samling: String) {
        self.samling = Some(samling);
    }
    pub fn rm(&mut self, rm: String) {
        self.rm = Some(rm);
    }
    pub fn filnamn(&mut self, filnamn: String) {
        self.filnamn = Some(filnamn);
    }
    pub fn storlek(&mut self, storlek: usize) {
        self.storlek = Some(storlek);
    }
    pub fn format(&mut self, format: DataFormat) {
        self.format = Some(format);
    }
    pub fn filformat(&mut self, filformat: FilFormat) {
        self.filformat = Some(filformat);
    }
    pub fn uppdaterad(&mut self, uppdaterad: NaiveDateTime) {
        self.uppdaterad = Some(uppdaterad);
    }
    pub fn url(&mut self, url: String) {
        self.url = Some(url);
    }
    pub fn description(&mut self, description: String) {
        self.description = Some(description);
    }
    pub fn beskrivning(&mut self, beskrivning: Option<String>) {
        self.beskrivning = beskrivning;
    }
    pub fn upplysning(&mut self, upplysning: Option<Upplysning>) {
        self.upplysning = upplysning;
    }
    pub fn namn(&mut self, namn: String) {
        self.namn = Some(namn);
    }
    pub fn typ(&mut self, typ: String) {
        self.typ = Some(typ);
    }
    pub fn build(self) -> Result<DataSet, &'static str> {
        let DatasetBuilder {
            namn,
            typ,
            upplysning,
            beskrivning,
            description,
            url,
            uppdaterad,
            filformat,
            format,
            storlek,
            filnamn,
            rm,
            samling,
        } = self;
        Ok(DataSet {
            namn: namn.ok_or_else(|| "field 'namn' is missing")?,
            typ: typ.ok_or_else(|| "field 'typ' is missing")?,
            upplysning,
            beskrivning,
            description: description.ok_or_else(|| "field 'description' is missing")?,
            url: url.ok_or_else(|| "field 'url' is missing")?,
            uppdaterad: uppdaterad.ok_or_else(|| "field 'uppdaterad' is missing")?,
            filformat: filformat.ok_or_else(|| "field 'filformat' is missing")?,
            format: format.ok_or_else(|| "field 'format' is missing")?,
            storlek_bytes: storlek.ok_or_else(|| "field 'storlek' is missing")?,
            filnamn: filnamn.ok_or_else(|| "field 'filnamn' is missing")?,
            rm: rm.ok_or_else(|| "field 'rm' is missing")?,
            samling: samling.ok_or_else(|| "field 'samling' is missing")?,
        })
    }
}

impl deserx::DeXml for DataSet {
    fn deserialize_xml<R: BufRead>(reader: &mut NsReader<R>) -> Result<Self, deserx::DeXmlError> {
        Self::deserialize_xml_from_tag(reader, "dataset")
    }
    fn deserialize_xml_from_body<R: BufRead>(
        reader: &mut NsReader<R>,
        _start: &BytesStart,
    ) -> Result<Self, deserx::DeXmlError> {
        let mut builder = DatasetBuilder::default();
        builder.namn(String::deserialize_xml_from_tag(reader, "namn")?);
        // dbg!(&builder);
        builder.typ(String::deserialize_xml_from_tag(reader, "typ")?);
        // dbg!(&builder);
        builder.samling(String::deserialize_xml_from_tag(reader, "samling")?);
        // dbg!(&builder);
        builder.rm(String::deserialize_xml_from_tag(reader, "rm")?);
        // dbg!(&builder);
        builder.filnamn(String::deserialize_xml_from_tag(reader, "filnamn")?);
        // dbg!(&builder);
        let s = String::deserialize_xml_from_tag(reader, "storlek")?;
        builder.storlek(s.parse::<usize>().map_err(DeXmlError::custom)?);
        // dbg!(&builder);
        builder.format(DataFormat::deserialize_xml_from_tag(reader, "format")?);
        // dbg!(&builder);
        builder.filformat(FilFormat::deserialize_xml_from_tag(reader, "filformat")?);
        // dbg!(&builder);
        let s = String::deserialize_xml_from_tag(reader, "uppdaterad")?;
        builder.uppdaterad(
            date_formats::swe_date_format::parse_from_str(&s).map_err(DeXmlError::custom)?,
        );
        // dbg!(&builder);
        builder.url(String::deserialize_xml_from_tag(reader, "url")?);
        // dbg!(&builder);
        builder.description(String::deserialize_xml_from_tag(reader, "description")?);
        // dbg!(&builder);
        let beskrivning = String::deserialize_xml_from_tag(reader, "beskrivning").unwrap(); //?;
                                                                                            // dbg!(&beskrivning);
        if !beskrivning.is_empty() {
            builder.beskrivning(Some(beskrivning));
        }
        // dbg!(&builder);
        let upplysning = Upplysning::deserialize_xml_from_tag(reader, "upplysning").unwrap(); //?;
        if upplysning.upplysning.is_empty() && upplysning.year_comment.is_empty() {
            builder.upplysning(None);
        } else {
            builder.upplysning(Some(upplysning));
        }

        builder.build().map_err(DeXmlError::custom)
    }
}

impl deserx::SerXml for DataSet {
    fn serialize_xml<W: std::io::Write>(
        &self,
        serializer: &mut quick_xml::Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.ser_as_element(serializer, "dataset")
    }

    fn ser_elem_body<W: std::io::Write>(
        &self,
        serializer: &mut quick_xml::Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.namn.ser_as_element(serializer, "namn")?;
        self.typ.ser_as_element(serializer, "typ")?;
        self.samling.ser_as_element(serializer, "samling")?;
        self.rm.ser_as_element(serializer, "rm")?;
        self.filnamn.ser_as_element(serializer, "filnamn")?;
        self.storlek_bytes.ser_as_element(serializer, "storlek")?;
        self.format.ser_as_element(serializer, "format")?;
        self.filformat.ser_as_element(serializer, "filformat")?;
        date_formats::swe_date_format::to_string(&self.uppdaterad)
            .ser_as_element(serializer, "uppdaterad")?;
        self.url.ser_as_element(serializer, "url")?;
        self.description.ser_as_element(serializer, "description")?;
        self.beskrivning.ser_as_element(serializer, "beskrivning")?;
        self.upplysning.ser_as_element(serializer, "upplysning")?;
        Ok(())
    }
}

#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename = "upplysning")]
pub struct Upplysning {
    upplysning: String,
    year_comment: BTreeMap<String, String>,
}

impl deserx::SerXml for Upplysning {
    fn serialize_xml<W: std::io::Write>(
        &self,
        serializer: &mut quick_xml::Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        if self.upplysning.is_empty() && self.year_comment.is_empty() {
            self.ser_as_element_empty(serializer, "upplysning")
        } else {
            self.ser_as_element(serializer, "upplysning")
        }
    }

    fn ser_elem_body<W: std::io::Write>(
        &self,
        serializer: &mut quick_xml::Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        if !self.upplysning.is_empty() {
            self.upplysning.ser_as_text(serializer)?;
        }
        for (year, comment) in self.year_comment.iter() {
            serializer.write_event(Event::Empty(BytesStart::new("br")))?;
            serializer.write_event(Event::Text(BytesText::new(&format!(
                "{}: {}",
                year, comment
            ))))?;
        }
        Ok(())
    }
}
impl deserx::DeXml for Upplysning {
    fn deserialize_xml<R: BufRead>(
        reader: &mut quick_xml::NsReader<R>,
    ) -> Result<Self, deserx::DeXmlError> {
        let mut buf = Vec::new();
        match reader.read_event_into(&mut buf)? {
            Event::Start(evt) => Self::deserialize_xml_from_body(reader, &evt),
            _ => todo!(),
        }
    }

    fn deserialize_xml_from_body<R: BufRead>(
        reader: &mut quick_xml::NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, deserx::DeXmlError> {
        let upplysning = String::deserialize_xml_from_text(reader)?;
        let mut year_comment = BTreeMap::default();
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Empty(e) => match e.name().as_ref() {
                    b"br" => {
                        // dbg!(e);
                    }
                    _ => todo!(),
                },
                Event::Text(e) => {
                    let raw_text = e.unescape()?;
                    let mut parts = raw_text.split(':');
                    let year = parts.next().unwrap().to_string();
                    let comment = parts.next().unwrap().trim_start().to_string();
                    year_comment.insert(year, comment);
                }
                Event::End(e) => match e.name().as_ref() {
                    tag if tag == start.name().as_ref() => break,
                    _ => todo!(),
                },
                Event::Eof => break,
                e => todo!("handle {:?}", e),
            }
        }
        Ok(Self {
            upplysning,
            year_comment,
        })
    }

    fn deserialize_xml_from_body_with_end<R: BufRead>(
        reader: &mut quick_xml::NsReader<R>,
        start: &BytesStart,
        end: BytesEnd,
    ) -> Result<Self, deserx::DeXmlError> {
        // dbg!(start);
        // dbg!(&end);
        let mut upplysning = String::default(); //?;
        let mut found_br = false;
        let mut year_comment = BTreeMap::default();
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Empty(e) => match e.name().as_ref() {
                    b"br" => {
                        // dbg!(e);
                        found_br = true;
                    }
                    _ => todo!(),
                },
                Event::Text(e) => {
                    let raw_text = e.unescape()?;
                    if found_br {
                        let mut parts = raw_text.split(':');
                        let year = parts.next().unwrap().to_string();
                        let comment = parts.next().unwrap().trim_start().to_string();
                        year_comment.insert(year, comment);
                    } else {
                        upplysning.push_str(raw_text.as_ref());
                    }
                }
                Event::End(e) if e == end => break,
                Event::Eof => break,
                e => todo!("handle {:?}", e),
            }
        }
        Ok(Self {
            upplysning,
            year_comment,
        })
    }
    fn deserialize_xml_from_tag<R: BufRead>(
        reader: &mut NsReader<R>,
        tag: &str,
    ) -> Result<Self, DeXmlError> {
        // dbg!(tag);
        use quick_xml::events::Event;
        let mut buf = Vec::new();
        let self_: Self = match reader.read_event_into(&mut buf)? {
            Event::Empty(evt) if evt.name().as_ref() == tag.as_bytes() => Self::default(),
            Event::Start(evt) if evt.name().as_ref() == tag.as_bytes() => {
                Self::deserialize_xml_from_body_with_end(reader, &evt, evt.to_end())?
            }
            evt => {
                return Err(DeXmlError::UnexpectedTag {
                    tag: tag.to_string(),
                    event: format!("{:?}", evt),
                })
            }
        };

        Ok(self_)
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataFormat {
    Csv,
    CsvT,
    Html,
    Json,
    Sql,
    Text,
    Xml,
}

impl DataFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Csv => "csv",
            Self::CsvT => "csvt",
            Self::Html => "html",
            Self::Json => "json",
            Self::Sql => "sql",
            Self::Text => "text",
            Self::Xml => "xml",
        }
    }

    pub fn from_str_opt(s: &str) -> Option<Self> {
        let format = match s {
            "csv" => DataFormat::Csv,
            "csvt" => DataFormat::CsvT,
            "html" => DataFormat::Html,
            "json" => DataFormat::Json,
            "sql" => DataFormat::Sql,
            "text" => DataFormat::Text,
            "xml" => DataFormat::Xml,
            _ => return None,
        };
        Some(format)
    }
}

impl deserx::SerXml for DataFormat {
    fn serialize_xml<W: std::io::Write>(
        &self,
        _serializer: &mut quick_xml::Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        todo!()
    }

    fn ser_elem_body<W: std::io::Write>(
        &self,
        serializer: &mut quick_xml::Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.ser_as_text(serializer)
    }

    fn ser_as_text<W: std::io::Write>(
        &self,
        serializer: &mut quick_xml::Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        serializer.write_event(Event::Text(BytesText::new(self.as_str())))
    }
}
impl deserx::DeXml for DataFormat {
    fn deserialize_xml<R: BufRead>(_reader: &mut NsReader<R>) -> Result<Self, deserx::DeXmlError> {
        todo!()
    }

    fn deserialize_xml_from_text<R: BufRead>(
        reader: &mut NsReader<R>,
    ) -> Result<Self, deserx::DeXmlError> {
        let mut buf = Vec::new();
        match reader.read_event_into(&mut buf)? {
            Event::Text(text) => {
                let s = text.unescape()?;
                match Self::from_str_opt(s.as_ref()) {
                    Some(res) => Ok(res),
                    None => Err(DeXmlError::custom(format!("Unknown format '{}'", s))),
                }
            }
            evt => {
                return Err(DeXmlError::UnexpectedEvent {
                    event: format!("{:?}", evt),
                })
            }
        }
    }

    fn deserialize_xml_from_body<R: BufRead>(
        reader: &mut NsReader<R>,
        _start: &BytesStart,
    ) -> Result<Self, deserx::DeXmlError> {
        Self::deserialize_xml_from_text(reader)
    }
}
// impl serde::Serialize for DataFormat {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         serializer.serialize_str(self.as_str())
//     }
// }
// impl<'de> serde::Deserialize<'de> for DataFormat {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         struct DataFormatVisitor;

//         impl<'de> Visitor<'de> for DataFormatVisitor {
//             type Value = DataFormat;

//             fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//                 formatter.write_str("one of 'csv', 'csvt', 'html', 'json', 'sql', 'text' or 'xml'")
//             }

//             fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
//             where
//                 E: serde::de::Error,
//             {
//                 match DataFormat::from_str_opt(v) {
//                     Some(format) => Ok(format),
//                     None => Err(E::custom(format!("unknown format: {}", v))),
//                 }
//             }
//         }
//         deserializer.deserialize_str(DataFormatVisitor)
//     }
// }

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FilFormat {
    Zip,
}

impl FilFormat {
    pub fn from_str_opt(s: &str) -> Option<Self> {
        match s {
            "zip" => Some(Self::Zip),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Zip => "zip",
        }
    }
}

impl deserx::SerXml for FilFormat {
    fn serialize_xml<W: std::io::Write>(
        &self,
        _serializer: &mut quick_xml::Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        todo!()
    }

    fn ser_elem_body<W: std::io::Write>(
        &self,
        serializer: &mut quick_xml::Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.ser_as_text(serializer)
    }

    fn ser_as_text<W: std::io::Write>(
        &self,
        serializer: &mut quick_xml::Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        serializer.write_event(Event::Text(BytesText::new(self.as_str())))
    }
}
impl deserx::DeXml for FilFormat {
    fn deserialize_xml<R: BufRead>(_reader: &mut NsReader<R>) -> Result<Self, deserx::DeXmlError> {
        todo!()
    }

    fn deserialize_xml_from_text<R: BufRead>(
        reader: &mut NsReader<R>,
    ) -> Result<Self, deserx::DeXmlError> {
        let mut buf = Vec::new();
        match reader.read_event_into(&mut buf)? {
            Event::Text(text) => {
                let s = text.unescape()?;
                match Self::from_str_opt(s.as_ref()) {
                    Some(res) => Ok(res),
                    None => Err(DeXmlError::custom(format!("Unknown format '{}'", s))),
                }
            }
            evt => {
                return Err(DeXmlError::UnexpectedEvent {
                    event: format!("{:?}", evt),
                })
            }
        }
    }
    fn deserialize_xml_from_body<R: BufRead>(
        reader: &mut NsReader<R>,
        _start: &BytesStart,
    ) -> Result<Self, deserx::DeXmlError> {
        Self::deserialize_xml_from_text(reader)
    }
}
// impl serde::Serialize for FilFormat {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         serializer.serialize_str(self.as_str())
//     }
// }
// impl<'de> serde::Deserialize<'de> for FilFormat {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         struct FilFormatVisitor;

//         impl<'de> Visitor<'de> for FilFormatVisitor {
//             type Value = FilFormat;

//             fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//                 formatter.write_str("one of 'zip'")
//             }

//             fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
//             where
//                 E: serde::de::Error,
//             {
//                 match FilFormat::from_str_opt(v) {
//                     Some(format) => Ok(format),
//                     None => return Err(E::custom(format!("unknown filformat: {}", v))),
//                 }
//             }
//         }
//         deserializer.deserialize_str(FilFormatVisitor)
//     }
// }
