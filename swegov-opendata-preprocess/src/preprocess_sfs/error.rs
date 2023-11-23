use minidom_extension::minidom;
use std::{fmt, string::FromUtf8Error};

use minidom::quick_xml;

#[derive(Debug)]
pub enum SfsPreprocessError {
    Internal(String),
    // XmlDe(quick_xml::DeError, Option<usize>),
    Xml(minidom::quick_xml::Error),
    Json,
    DateParsing(chrono::ParseError),
    Write,
    HtmlFieldIsEmpty,
    XmlParsingError {
        pos: usize,
        err: quick_xml::Error,
    },
    XmlParsingAttrError {
        pos: usize,
        err: quick_xml::events::attributes::AttrError,
    },
    XmlFromUtf8Error {
        pos: usize,
        err: FromUtf8Error,
    },
}

impl fmt::Display for SfsPreprocessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Xml(_err) => write!(f, "xml deserialization error "),
            // Self::XmlDe(_err, num) => write!(f, "xml deserialization error on page {:?}", num),
            Self::Json => write!(f, "json deserializing error"),
            Self::Internal(msg) => write!(f, "Internal error: {}", msg),
            Self::DateParsing(_err) => write!(f, "date parsing error"),
            Self::Write => write!(f, "Error writing xml"),
            Self::HtmlFieldIsEmpty => f.write_str("The 'html' field of the dokument is empty"),
            Self::XmlParsingError { pos, err } => f.write_fmt(format_args!(
                "xml parse error at position {}: {:?}",
                pos, err
            )),
            Self::XmlParsingAttrError { pos, err } => f.write_fmt(format_args!(
                "xml parse error for attribute at position {}: {:?}",
                pos, err
            )),
            Self::XmlFromUtf8Error { pos, err } => f.write_fmt(format_args!(
                "xml parse error for string at position {}: {:?}",
                pos, err
            )),
        }
    }
}

// impl StdError for SfsPreprocessError {
//     fn source(&self) -> Option<&(dyn StdError + 'static)> {
//         match self {
//             Self::Xml(err) => Some(err),
//             // Self::XmlDe(err, _) => Some(err),
//             Self::Json(err) => Some(err),
//             Self::DateParsing(err) => Some(err),
//             _ => None,
//         }
//     }
// }

// impl From<quick_xml::DeError> for Error {
//     fn from(value: quick_xml::DeError) -> Self {
//         Self::XmlDe(value, None)
//     }
// }

impl From<minidom::quick_xml::Error> for SfsPreprocessError {
    fn from(value: minidom::quick_xml::Error) -> Self {
        Self::Xml(value)
    }
}

impl From<chrono::ParseError> for SfsPreprocessError {
    fn from(value: chrono::ParseError) -> Self {
        Self::DateParsing(value)
    }
}

impl error_stack::Context for SfsPreprocessError {}
