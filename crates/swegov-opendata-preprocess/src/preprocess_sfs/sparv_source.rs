use crate::{shared::io_ext, PreprocessError};
use fs_err as fs;
use std::path::Path;

use super::sfs_json;
use sparv_extension::XmlSourceWriter;

#[tracing::instrument()]
pub fn build_sparv_source(path: &Path, corpus_source_dir: &Path) -> Result<(), PreprocessError> {
    tracing::info!("creating '{}'", corpus_source_dir.display());
    fs::create_dir_all(corpus_source_dir).map_err(|error| {
        PreprocessError::CouldNotCreateFolder {
            path: corpus_source_dir.to_path_buf(),
            error,
        }
    })?;
    let mut source_writer = XmlSourceWriter::new(corpus_source_dir);
    for file_path in fs::read_dir(path).map_err(|error| PreprocessError::CouldNotReadFolder {
        path: path.to_path_buf(),
        error,
    })? {
        let file_path = file_path?.path();
        let file_span = tracing::info_span!("reading file", file_path = ?file_path);
        let _enter = file_span.enter();
        let filecontents =
            io_ext::read_text(&file_path).map_err(|error| PreprocessError::CouldNotReadFile {
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
