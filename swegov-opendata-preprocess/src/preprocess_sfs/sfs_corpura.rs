use std::{fs, path::Path};

use error_stack::ResultExt;
use sparv_extension::make_corpus_config;
use sparv_extension::SparvConfig;
use sparv_extension::SparvMetadata;

use crate::preprocess_sfs;
use crate::PreprocessError;

#[derive(Debug, Clone)]
pub struct PreprocessSfsCorpuraOptions<'a> {
    pub input: &'a Path,
    pub output: &'a Path,
}

pub fn preprocess_sfs_corpus(
    PreprocessSfsCorpuraOptions { input, output }: PreprocessSfsCorpuraOptions<'_>,
) -> error_stack::Result<(), PreprocessError> {
    tracing::info!("preprocess SFS corpus from {}", input.display());
    let corpus_id = "sfs";
    let sparv_config = SparvConfig::with_parent_and_metadata(
        "../config.yaml",
        SparvMetadata::new(corpus_id)
            .name("swe", "Riksdagens öppna data: Svensk Författningssamling")
            .description("swe", "Svensk Författningssamling")
            .description("eng", "Swedish Code of Statues")
            .short_description("swe", "Svensk Författningssamling")
            .short_description("eng", "Swedish Code of Statues"),
    );
    make_corpus_config(&sparv_config, &output.join(corpus_id)).change_context(PreprocessError)?;
    for year in fs::read_dir(input).change_context(PreprocessError)? {
        let year = year.change_context(PreprocessError)?.path();
        tracing::debug!("found path: {}", year.display());
        preprocess_sfs::build_sparv_source(
            year.as_path(),
            &output
                .join(corpus_id)
                .join("source")
                .join(year.file_stem().unwrap()),
        )?;
    }
    Ok(())
}
