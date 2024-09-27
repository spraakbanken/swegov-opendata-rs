use chrono::NaiveDateTime;
use serde::de::Visitor;

use crate::date_formats;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DatasetLista {
    pub dataset: Vec<DataSet>,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DataSet {
    pub namn: String,
    pub typ: String,
    pub samling: String,
    pub rm: String,
    pub filnamn: String,
    #[serde(rename = "storlek")]
    pub storlek_bytes: usize,
    pub format: DataFormat,
    pub filformat: FilFormat,
    #[serde(with = "date_formats::swe_date_format")]
    pub uppdaterad: NaiveDateTime,
    pub url: String,
    pub description: String,
    pub beskrivning: String,
    pub upplysning: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataFormat {
    Csv,
    CsvT,
    Html,
    Json,
    Sql,
    Text,
    Xml,
}

impl serde::Serialize for DataFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            Self::Csv => "csv",
            Self::CsvT => "csvt",
            Self::Html => "html",
            Self::Json => "json",
            Self::Sql => "sql",
            Self::Text => "text",
            Self::Xml => "xml",
        };
        serializer.serialize_str(value)
    }
}
impl<'de> serde::Deserialize<'de> for DataFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct DataFormatVisitor;

        impl<'de> Visitor<'de> for DataFormatVisitor {
            type Value = DataFormat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("one of 'csv', 'csvt', 'html', 'json', 'sql', 'text' or 'xml'")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let format = match v {
                    "csv" => DataFormat::Csv,
                    "csvt" => DataFormat::CsvT,
                    "html" => DataFormat::Html,
                    "json" => DataFormat::Json,
                    "sql" => DataFormat::Sql,
                    "text" => DataFormat::Text,
                    "xml" => DataFormat::Xml,
                    x => return Err(E::custom(format!("unknown format: {}", x))),
                };
                Ok(format)
            }
        }
        deserializer.deserialize_str(DataFormatVisitor)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FilFormat {
    Zip,
}

impl serde::Serialize for FilFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            Self::Zip => "zip",
        };
        serializer.serialize_str(value)
    }
}
impl<'de> serde::Deserialize<'de> for FilFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct FilFormatVisitor;

        impl<'de> Visitor<'de> for FilFormatVisitor {
            type Value = FilFormat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("one of 'zip'")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let format = match v {
                    "zip" => FilFormat::Zip,
                    x => return Err(E::custom(format!("unknown filformat: {}", x))),
                };
                Ok(format)
            }
        }
        deserializer.deserialize_str(FilFormatVisitor)
    }
}
