use std::{fs, path::Path};

use error_stack::ResultExt;

use crate::{core::component::preprocess::preprocess_sfs, PreprocessError};

#[derive(Debug, Clone)]
pub struct PreprocessCorpuraOptions<'a> {
    pub input: &'a Path,
    pub output: &'a Path,
}

pub fn preprocess_sfs_corpus(
    options: PreprocessCorpuraOptions,
) -> error_stack::Result<(), PreprocessError> {
    tracing::info!("preprocess SFS corpus from {}", options.input.display());
    for year in fs::read_dir(options.input).change_context(PreprocessError)? {
        let year = year.change_context(PreprocessError)?.path();
        tracing::debug!("found path: {}", year.display());
        preprocess_sfs::build_sparv_source(
            year.as_path(),
            &options
                .output
                .join("sfs")
                .join("source")
                .join(year.file_stem().unwrap()),
        )?;
    }
    Ok(())
}
