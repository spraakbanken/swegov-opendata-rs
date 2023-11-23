use std::{
    fs,
    io::{self, Read},
    path::Path,
};

use crate::PreprocessError;

use super::sfs_json;
use error_stack::ResultExt;
use flate2::read::GzDecoder;
use sparv_extension::XmlSourceWriter;

pub fn build_sparv_source(
    path: &Path,
    corpus_source_dir: &Path,
) -> error_stack::Result<(), PreprocessError> {
    tracing::debug!("creating '{}'", corpus_source_dir.display());
    fs::create_dir_all(corpus_source_dir).change_context(PreprocessError)?;
    let mut source_writer = XmlSourceWriter::new(corpus_source_dir);
    for file_path in fs::read_dir(path).change_context(PreprocessError)? {
        let file_path = file_path.change_context(PreprocessError)?.path();
        tracing::info!(file_path = ?file_path, "reading a file");
        let file_span = tracing::info_span!("reading file", file_path = ?file_path);
        let _enter = file_span.enter();
        let filecontents = read_text(&file_path).change_context(PreprocessError)?;
        let xmlstring = sfs_json::preprocess_json(&filecontents)
            .change_context(PreprocessError)
            .attach_printable_lazy(|| format!("reading file {}", file_path.display()))?;
        source_writer
            .write(xmlstring)
            .change_context(PreprocessError)?;
    }
    source_writer.flush().change_context(PreprocessError)?;
    Ok(())
}

#[tracing::instrument()]
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
