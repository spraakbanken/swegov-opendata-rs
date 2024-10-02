use std::{
    fs,
    io::{self, Read},
    path::Path,
};

use crate::PreprocessError;

use super::sfs_json;
use flate2::read::GzDecoder;
use sparv_extension::XmlSourceWriter;

#[tracing::instrument()]
pub fn build_sparv_source(path: &Path, corpus_source_dir: &Path) -> Result<(), PreprocessError> {
    tracing::info!("creating '{}'", corpus_source_dir.display());
    fs::create_dir_all(corpus_source_dir).map_err(|error| PreprocessError::CouldNotCreateDir {
        path: corpus_source_dir.to_path_buf(),
        error,
    })?;
    let mut source_writer = XmlSourceWriter::new(corpus_source_dir);
    for file_path in fs::read_dir(path).map_err(|error| PreprocessError::CouldNotReadDir {
        path: path.to_path_buf(),
        error,
    })? {
        let file_path = file_path?.path();
        let file_span = tracing::info_span!("reading file", file_path = ?file_path);
        let _enter = file_span.enter();
        let filecontents =
            read_text(&file_path).map_err(|error| PreprocessError::CouldNotReadFile {
                path: file_path.clone(),
                error,
            })?;
        let xmlstring = sfs_json::preprocess_json(&filecontents).map_err(|error| {
            PreprocessError::SfsPreprocessError {
                path: file_path.clone(),
                error,
            }
        })?;
        source_writer.write(xmlstring)?;
    }
    source_writer.flush()?;
    Ok(())
}

pub fn read_text(path: &Path) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut text = String::new();

    if path.extension().is_some_and(|ext| ext == "gz") {
        let mut gz = GzDecoder::new(&file);
        gz.read_to_string(&mut text)?;
    } else {
        file.read_to_string(&mut text)?;
    }
    Ok(text)
}
