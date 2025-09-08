use std::{collections::BTreeMap, io::BufRead};

use chrono::NaiveDateTime;

use crate::date_formats;

#[cfg(test)]
mod tests;

#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename = "datasetlista")]
pub struct DatasetLista {
    pub dataset: Vec<DataSet>,
}

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


#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename = "upplysning")]
pub struct Upplysning {
    upplysning: String,
    year_comment: BTreeMap<String, String>,
}

impl Upplysning {
    pub fn upplysning(&self) -> &str {
        self.upplysning.as_str()
    }
}

}

        }
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
