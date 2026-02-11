use exn::Exn;
use minidom_extension::minidom::{quick_xml::Reader, Element};

#[derive(Debug, thiserror::Error)]
pub enum ProcessHtmlUglyError {
    #[error("failed to process html ugly")]
    Failure,
}

pub fn process_html_ugly(
    reader: &mut Reader<&[u8]>,
) -> Result<Vec<Element>, Exn<ProcessHtmlUglyError>> {
    todo!()
}
