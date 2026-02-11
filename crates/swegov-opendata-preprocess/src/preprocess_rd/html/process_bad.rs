use exn::Exn;
use minidom_extension::minidom::{quick_xml::Reader, Element};

#[derive(Debug, thiserror::Error)]
pub enum ProcessHtmlBadError {
    #[error("failed to process html bad")]
    Failure,
}

pub fn process_html_bad(
    reader: &mut Reader<&[u8]>,
) -> Result<Vec<Element>, Exn<ProcessHtmlBadError>> {
    todo!()
}
