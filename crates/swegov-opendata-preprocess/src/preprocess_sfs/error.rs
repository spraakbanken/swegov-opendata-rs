use minidom_extension::minidom;
use std::string::FromUtf8Error;

use minidom::quick_xml;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum SfsPreprocessError {
    #[error("Internal error: {0}")]
    Internal(String),
    // XmlDe(quick_xml::DeError, Option<usize>),
    #[error("Xml error")]
    Xml(#[from] minidom::quick_xml::Error),
    #[error("Json deserialization error")]
    Json(#[from] serde_json::Error),
    #[error("date parsing error")]
    DateParsing(#[from] chrono::ParseError),
    #[error("Error writing xml")]
    FailedToWriteXml(#[from] minidom::Error),
    #[error("The 'html' field of the dokument is empty")]
    HtmlFieldIsEmpty,
    #[error("xml parse error for string at position {pos}: {err:?}")]
    XmlParsingError {
        pos: usize,
        #[source]
        err: quick_xml::Error,
    },
    #[error("xml parse error for string at position {pos}: {err:?}")]
    XmlParsingAttrError {
        pos: usize,
        #[source]
        err: quick_xml::events::attributes::AttrError,
    },
    #[error("[XML] bad UTF8 at pos {pos}: {err:?}")]
    XmlFromUtf8Error {
        pos: usize,
        #[source]
        err: FromUtf8Error,
    },
}
