use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum Error {
    Internal(String),
    XmlDe(quick_xml::DeError, Option<usize>),
    Xml(quick_xml::Error),
    Json(serde_json::Error),
    DateParsing(chrono::ParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Xml(_err) => write!(f, "xml deserialization error "),
            Self::XmlDe(_err, num) => write!(f, "xml deserialization error on page {:?}", num),
            Self::Json(_err) => write!(f, "json deserializing error"),
            Self::Internal(msg) => write!(f, "Internal error: {}", msg),
            Self::DateParsing(_err) => write!(f, "date parsing error"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Xml(err) => Some(err),
            Self::XmlDe(err, _) => Some(err),
            Self::Json(err) => Some(err),
            Self::DateParsing(err) => Some(err),
            _ => None,
        }
    }
}

impl From<quick_xml::DeError> for Error {
    fn from(value: quick_xml::DeError) -> Self {
        Self::XmlDe(value, None)
    }
}

impl From<quick_xml::Error> for Error {
    fn from(value: quick_xml::Error) -> Self {
        Self::Xml(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl From<chrono::ParseError> for Error {
    fn from(value: chrono::ParseError) -> Self {
        Self::DateParsing(value)
    }
}
