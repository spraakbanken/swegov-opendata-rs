use core::fmt;
use std::{io, path::PathBuf};

use crate::{
    corpusinfo::UnknownCorpus,
    preprocess_rd::{self, PreprocessJsonError},
    preprocess_sfs::SfsPreprocessError,
};

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum PreprocessError {
    #[error("{0}")]
    Custom(String),
    #[error("Filename '{0}' contains no valid prefix")]
    NoValidPrefix(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("Could not read JSON from '{path}'")]
    CouldNotReadJson {
        path: PathBuf,
        #[source]
        error: serde_json::Error,
    },
    #[error("Could not write JSON to '{path}'")]
    CouldNotWriteJson {
        path: PathBuf,
        #[source]
        error: serde_json::Error,
    },
    #[error("Could not create folder '{path}'")]
    CouldNotCreateFolder {
        path: PathBuf,
        #[source]
        error: io::Error,
    },
    #[error("Could not read folder '{path}'")]
    CouldNotReadFolder {
        path: PathBuf,
        #[source]
        error: io::Error,
    },
    #[error("Could not access DirEntry in folder '{path}'")]
    CouldNotAccessDirEntry {
        path: PathBuf,
        #[source]
        error: io::Error,
    },
    #[error("Could not read file '{path}'")]
    CouldNotReadFile {
        path: PathBuf,
        #[source]
        error: io::Error,
    },
    #[error("Could not read the zip archive '{path}'")]
    CouldNotReadZipArchive {
        path: PathBuf,
        #[source]
        error: zip::result::ZipError,
    },
    #[error("Could not read the zip file '{path}' from the archive '{archive:?}")]
    CouldNotReadZipFile {
        archive: PathBuf,
        path: String,
        #[source]
        error: io::Error,
    },
    #[error("Could not get zip object with index {index}")]
    CouldNotGetZipObjByIndex {
        index: usize,
        #[source]
        error: zip::result::ZipError,
    },
    #[error(transparent)]
    #[diagnostic(transparent)]
    UnknownCorpus(#[from] UnknownCorpus),
    #[error(transparent)]
    #[diagnostic(transparent)]
    SparvError(#[from] sparv_extension::SparvError),
    #[error("SFS processing error when processing {path}")]
    SfsPreprocessError {
        path: PathBuf,
        #[source]
        error: SfsPreprocessError,
    },
    #[error("Xml error when processing {path}")]
    XmlError {
        path: String,
        #[source]
        error: preprocess_rd::XmlError,
    },
    #[error("Error when preprocessing rd-json from path '{path}'")]
    RdPreprocessJsonError {
        path: String,
        #[source]
        error: PreprocessJsonError,
    },
}

impl PreprocessError {
    pub fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::Custom(msg.to_string())
    }
}
