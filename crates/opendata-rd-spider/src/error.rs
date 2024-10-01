use std::io;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum Error {
    #[diagnostic(code(opendata_rd_spider::CouldNotCreateFile))]
    #[error("Could not create file'{path}'.")]
    CouldNotCreateFile {
        path: String,
        #[source]
        source: io::Error,
    },
    #[diagnostic(code(opendata_rd_spider::CouldNotCreateFolder))]
    #[error("Could not create folder '{path}'.")]
    CouldNotCreateFolder {
        path: String,
        #[source]
        source: io::Error,
    },
    #[diagnostic(code(opendata_rd_spider::CouldNotParseJson))]
    #[error("Could not parse JSON from file '{path}'.")]
    CouldNotParseJson {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[diagnostic(code(opendata_rd_spider::CouldNotParseXml))]
    #[error("Could not parse XML from src.")]
    CouldNotParseXml {
        #[source_code]
        src: String,
        #[source]
        source: deserx::DeXmlError,
    },

    #[diagnostic(code(opendata_rd_spider::CouldNotReadFile))]
    #[error("Could not read file'{path}'.")]
    CouldNotReadFile {
        path: String,
        #[source]
        source: io::Error,
    },
    #[error("Could not serialize JSON to '{path}'")]
    #[diagnostic(code(opendata_rd_spider::CouldNotSerializeJson))]
    CouldNotSerializeJson {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[diagnostic(code(opendata_rd_spider::CouldNotWriteFile))]
    #[error("Could not write to file'{path}'.")]
    CouldNotWriteFile {
        path: String,
        #[source]
        source: io::Error,
    },
    #[diagnostic(code(opendata_rd_spider::GeneralIoError))]
    #[error("General I/O error for '{path}'.")]
    GeneralIoError {
        path: String,
        #[source]
        source: io::Error,
    },
    #[error("Request read status_code={0}")]
    #[diagnostic(code(opendata_rd_spider::RequestReturnedError))]
    RequestReturnedError(reqwest::StatusCode),
    #[error(transparent)]
    #[diagnostic(code(opendata_rd_spider::reqwest_error))]
    ReqwestError(#[from] reqwest::Error),
    #[error("Failed to parse '{url}'")]
    #[diagnostic(code(opendata_rd_spider::UrlParseError))]
    UrlParseError {
        url: String,
        #[source]
        source: url::ParseError,
    },
}
