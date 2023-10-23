use std::fmt;

use minidom::quick_xml;

#[derive(Debug)]
pub enum SfsPreprocessError {
    Internal(String),
    // XmlDe(quick_xml::DeError, Option<usize>),
    Xml(quick_xml::Error),
    Json,
    DateParsing(chrono::ParseError),
    Write,
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

impl From<quick_xml::Error> for SfsPreprocessError {
    fn from(value: quick_xml::Error) -> Self {
        Self::Xml(value)
    }
}

impl From<chrono::ParseError> for SfsPreprocessError {
    fn from(value: chrono::ParseError) -> Self {
        Self::DateParsing(value)
    }
}

impl error_stack::Context for SfsPreprocessError {}
