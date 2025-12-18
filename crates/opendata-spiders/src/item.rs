use swegov_opendata::{DokumentLista, DokumentStatus};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub enum Item {
    #[serde(rename = "dokumentstatus")]
    // #[yaserde(rename = "dokumentstatus")]
    DokumentStatus(DokumentStatus),
    #[serde(rename = "dokumentlista")]
    // #[yaserde(rename = "dokumentlista")]
    DokumentLista(DokumentLista),
    // Other(String),
    Div(String),
}

impl yaserde::YaDeserialize for Item {
    fn deserialize<R: std::io::Read>(
        reader: &mut yaserde::de::Deserializer<R>,
    ) -> Result<Self, String> {
        if let xml::reader::XmlEvent::StartElement { name, .. } = reader.peek()? {
            match name.local_name.as_str() {
                "dokumentlista" => {
                    return Ok(Item::DokumentLista(DokumentLista::deserialize(reader)?));
                }
                "dokumentstatus" => {
                    return Ok(Item::DokumentStatus(DokumentStatus::deserialize(reader)?));
                }
                tag => return Err(format!("Unknown root '{tag}'")),
            }
        }
        Err("Unable to parse opendata_spiders::item::Item".to_string())
    }
}
