use exn::{Exn, ResultExt};
use fs_err as fs;
use std::path::Path;

use sparv_extension::XmlSourceWriter;
use swegov_opendata_preprocess::shared::io_ext;

use crate::preprocess_sfs::{sfs_json::PreprocessJsonResponse, SfsPreprocessError};

use super::sfs_json;

#[derive(Debug)]
pub struct SfsSparvSourceOptions<'a> {
    pub source_dir_actual: &'a Path,
    pub source_dir_former: &'a Path,
}

#[derive(Debug, thiserror::Error)]
pub enum BuildSparvSourceError {
    #[error("failed to build sparv source")]
    Failure,
    // #[error]
}

#[tracing::instrument()]
pub fn build_sparv_source(
    path: &Path,
    options: SfsSparvSourceOptions,
) -> Result<(), Exn<BuildSparvSourceError>> {
    let make_error = || BuildSparvSourceError::Failure;
    tracing::info!("creating '{}'", options.source_dir_actual.display());
    fs::create_dir_all(options.source_dir_actual).or_raise(make_error)?;
    tracing::info!("creating '{}'", options.source_dir_former.display());
    fs::create_dir_all(options.source_dir_former).or_raise(make_error)?;
    let mut source_writer_actual = XmlSourceWriter::new(options.source_dir_actual);
    let mut source_writer_former = XmlSourceWriter::new(options.source_dir_former);
    for file_path in fs::read_dir(path).or_raise(make_error)? {
        let file_path = file_path.or_raise(make_error)?.path();
        let file_span = tracing::info_span!("reading file", file_path = ?file_path);
        let _enter = file_span.enter();
        let filecontents = io_ext::read_text(&file_path).or_raise(make_error)?;
        match sfs_json::preprocess_json(&filecontents).or_raise(make_error)? {
            PreprocessJsonResponse::Actual(xmlstring) => {
                source_writer_actual.write(xmlstring).or_raise(make_error)?;
            }
            PreprocessJsonResponse::Former(xmlstring) => {
                source_writer_former.write(xmlstring).or_raise(make_error)?;
            }
        }
    }
    source_writer_actual.flush().or_raise(make_error)?;
    source_writer_former.flush().or_raise(make_error)?;
    Ok(())
}
