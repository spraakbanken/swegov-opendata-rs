use swegov_opendata::{DokumentLista, DokumentStatus};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub enum Item {
    #[serde(rename = "dokumentstatus")]
    DokumentStatus(DokumentStatus),
    #[serde(rename = "dokumentlista")]
    DokumentLista(DokumentLista),
    // Other(String),
    Div(String),
}
