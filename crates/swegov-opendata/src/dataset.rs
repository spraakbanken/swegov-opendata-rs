use chrono::NaiveDateTime;

use crate::date_formats::{self, SweDateTime};

#[cfg(test)]
mod tests;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde::Deserialize,
    serde::Serialize,
    yaserde::YaSerialize,
    yaserde::YaDeserialize,
)]
#[serde(rename = "datasetlista")]
#[yaserde(rename = "datasetlista")]
pub struct DatasetLista {
    pub dataset: Vec<DataSet>,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    yaserde::YaSerialize,
    yaserde::YaDeserialize,
)]
#[serde(deny_unknown_fields, rename = "dataset")]
#[yaserde(rename = "dataset")]
pub struct DataSet {
    pub namn: String,
    pub typ: String,
    pub samling: String,
    pub rm: String,
    pub filnamn: String,
    #[serde(rename = "storlek")]
    #[yaserde(rename = "storlek")]
    pub storlek_bytes: u64,
    // #[serde(with = "quick_xml::serde_helpers::text_content")]
    pub format: DataFormat,
    pub filformat: FilFormat,
    // #[serde(with = "date_formats::swe_date_format")]
    #[yaserde(rename = "uppdaterad")]
    pub uppdaterad: date_formats::SweDateTime,
    pub url: String,
    pub description: Option<String>,
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
    uppdaterad: Option<SweDateTime>,
    filformat: Option<FilFormat>,
    format: Option<DataFormat>,
    storlek: Option<u64>,
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
    pub fn storlek(&mut self, storlek: u64) {
        self.storlek = Some(storlek);
    }
    pub fn format(&mut self, format: DataFormat) {
        self.format = Some(format);
    }
    pub fn filformat(&mut self, filformat: FilFormat) {
        self.filformat = Some(filformat);
    }
    pub fn uppdaterad(&mut self, uppdaterad: NaiveDateTime) {
        self.uppdaterad = Some(uppdaterad.into());
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
            namn: namn.ok_or("field 'namn' is missing")?,
            typ: typ.ok_or("field 'typ' is missing")?,
            upplysning,
            beskrivning,
            description,
            url: url.ok_or("field 'url' is missing")?,
            uppdaterad: uppdaterad.ok_or("field 'uppdaterad' is missing")?,
            filformat: filformat.ok_or("field 'filformat' is missing")?,
            format: format.ok_or("field 'format' is missing")?,
            storlek_bytes: storlek.ok_or("field 'storlek' is missing")?,
            filnamn: filnamn.ok_or("field 'filnamn' is missing")?,
            rm: rm.ok_or("field 'rm' is missing")?,
            samling: samling.ok_or("field 'samling' is missing")?,
        })
    }
}

#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename = "upplysning")]
// #[yaserde(rename = "upplysning")]
pub struct Upplysning {
    upplysning: String,
    year_comment: YearCommentMap,
}

impl Upplysning {
    pub fn upplysning(&self) -> &str {
        self.upplysning.as_str()
    }
}

#[derive(
    Debug, Default, Clone, PartialEq, serde::Deserialize, serde::Serialize, yaserde::YaSerialize,
)]
pub struct YearCommentMap {
    year_comments: Vec<YearComment>,
}

impl YearCommentMap {
    pub fn new(year_comments: Vec<YearComment>) -> Self {
        Self { year_comments }
    }
}

impl yaserde::YaDeserialize for Upplysning {
    fn deserialize<R: std::io::Read>(
        reader: &mut yaserde::de::Deserializer<R>,
    ) -> Result<Self, String> {
        let expected_name = "upplysning";
        if let xml::reader::XmlEvent::StartElement { name, .. } = reader.peek()? {
            if name.local_name != expected_name {
                return Err(format!(
                    "Wrong StartElement name: '{}', expected: '{}'",
                    name, expected_name
                ));
            }
            let _next = reader.next_event();
        } else {
            return Err("StartElement missing".to_string());
        }

        let mut upplysning = String::default();
        let mut year_comment = YearCommentMap::default();
        let mut found_br = false;
        loop {
            match reader.peek()? {
                xml::reader::XmlEvent::StartElement { name, .. } => {
                    match name.local_name.as_ref() {
                        "br" => {
                            found_br = true;
                        }
                        _ => todo!(),
                    }
                    let _res = reader.next_event()?;
                }
                xml::reader::XmlEvent::Characters(raw_text) => {
                    if found_br {
                        let mut parts = raw_text.split(':');
                        let year = parts.next().unwrap().to_string();
                        let comment = parts.next().unwrap().trim_start().to_string();
                        year_comment
                            .year_comments
                            .push(YearComment { year, comment });
                    } else {
                        upplysning = raw_text.to_string();
                    }
                    let _res = reader.next_event()?;
                }
                xml::reader::XmlEvent::EndElement { name } => {
                    if name.local_name == expected_name {
                        break;
                    } else if name.local_name == "br" {
                        let _next = reader.next_event()?;
                        continue;
                    } else {
                        todo!("handle end='{}'", name)
                    }
                }
                e => todo!("handle event={:?}", e),
            }
        }
        Ok(Self {
            upplysning,
            year_comment,
        })
    }
}

impl yaserde::YaSerialize for Upplysning {
    fn serialize<W: std::io::Write>(
        &self,
        writer: &mut yaserde::ser::Serializer<W>,
    ) -> Result<(), String> {
        let start_element = xml::writer::XmlEvent::start_element("upplysning");
        writer.write(start_element).map_err(|e| e.to_string())?;
        if !self.upplysning().is_empty() {
            writer
                .write(xml::writer::XmlEvent::characters(&self.upplysning))
                .map_err(|e| e.to_string())?;
        }
        for year_comment in &self.year_comment.year_comments {
            writer
                .write(xml::writer::XmlEvent::start_element("br"))
                .map_err(|e| e.to_string())?;
            writer
                .write(xml::writer::XmlEvent::end_element())
                .map_err(|e| e.to_string())?;
            writer
                .write(xml::writer::XmlEvent::characters(&format!(
                    "{}: {}",
                    year_comment.year, year_comment.comment
                )))
                .map_err(|e| e.to_string())?;
        }
        writer
            .write(xml::writer::XmlEvent::end_element())
            .map_err(|e| e.to_string())?;
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

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize, yaserde::YaSerialize)]
pub struct YearComment {
    pub year: String,
    pub comment: String,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    yaserde::YaSerialize,
    yaserde::YaDeserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum DataFormat {
    #[yaserde(rename = "csv")]
    Csv,
    #[yaserde(rename = "csvt")]
    CsvT,
    #[yaserde(rename = "html")]
    Html,
    #[yaserde(rename = "json")]
    Json,
    #[yaserde(rename = "sql")]
    Sql,
    #[yaserde(rename = "text")]
    Text,
    #[yaserde(rename = "xml")]
    Xml,
}

impl Default for DataFormat {
    fn default() -> Self {
        Self::Csv
    }
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

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    yaserde::YaSerialize,
    yaserde::YaDeserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum FilFormat {
    #[yaserde(rename = "zip")]
    Zip,
}

impl Default for FilFormat {
    fn default() -> Self {
        FilFormat::Zip
    }
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
