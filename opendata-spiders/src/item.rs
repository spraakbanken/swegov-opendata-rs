use deserx::DeXml;
use swegov_opendata::{DokumentLista, DokumentStatus};

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

impl DeXml for Item {
    fn deserialize_xml<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
    ) -> Result<Self, quick_xml::Error> {
        Self::deserialize_xml_from_tag(reader, "Item")
    }
    fn deserialize_xml_from_tag<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
        tag: &str,
    ) -> Result<Self, quick_xml::Error> {
        use quick_xml::events::Event;
        let mut buf = Vec::new();
        let item = 'item: loop {
            match reader.read_event_into(&mut buf)? {
                Event::Decl(_) => {}
                Event::Start(start) if start.name().as_ref() == b"dokumentlista" => {
                    let dokumentlista = DokumentLista::deserialize_xml_from_body_with_end(
                        reader,
                        &start,
                        start.to_end(),
                    )?;
                    break 'item Item::DokumentLista(dokumentlista);
                }
                evt => todo!("handle {:?}", evt),
            }
        };
        todo!("whats next")
    }
    fn deserialize_xml_from_body<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
        start: &quick_xml::events::BytesStart,
    ) -> Result<Self, quick_xml::Error> {
        todo!("body")
    }
}
