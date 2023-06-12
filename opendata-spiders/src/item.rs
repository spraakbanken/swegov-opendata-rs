use deserx::DeXml;
use swegov_opendata::{DokumentLista, DokumentStatus};

use crate::Error;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Item {
    #[serde(rename = "dokumentstatus")]
    DokumentStatus(DokumentStatus),
    #[serde(rename = "dokumentlista")]
    DokumentLista(DokumentLista),
    // Other(String),
    Div(String),
}

// pub fn deserialize_from_xml(text: &str) -> Result<Item, Error> {}
