use exn::Exn;
use exn::ResultExt;
use fs_err as fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use swegov_opendata_preprocess::corpusinfo;

use preprocess_progress::prodash::{Count, Progress};
use sparv_extension::make_corpus_config;
use sparv_extension::SparvConfig;
use sparv_extension::SparvMetadata;

use crate::preprocess_sfs;
use crate::preprocess_sfs::sparv_source::SfsSparvSourceOptions;

#[derive(Debug, Clone)]
pub struct PreprocessSfsCorpuraOptions<'a> {
    pub input: &'a Path,
    pub output: &'a Path,
}

#[derive(Debug, thiserror::Error)]
pub enum PreprocessSfsCorpuraError {
    #[error("failed to preprocess sfs corpora")]
    Failure,
    // #[error("failed to preprocess sfs corpora: {message}")]
    // WithMsg { message: String },
}

pub fn preprocess_sfs_corpus(
    input_path: &Path,
    output_path: &Path,
    _out: impl std::io::Write,
    _err: impl std::io::Write,
    mut progress: impl preprocess_progress::NestedProgress,
    _options: PreprocessSfsCorpuraOptions<'_>,
) -> Result<(), Exn<PreprocessSfsCorpuraError>> {
    tracing::info!("preprocess SFS corpus from {}", input_path.display());
    let make_error = || PreprocessSfsCorpuraError::Failure;
    let start = std::time::Instant::now();
    let _config_progress = progress.add_child("create config");
    const CORPUS_ID_ACTUAL: &str = "sfs-aktuella";
    const CORPUS_ID_FORMER: &str = "sfs-upphävda";
    for corpus_id in [CORPUS_ID_ACTUAL, CORPUS_ID_FORMER] {
        let corpus = corpusinfo(corpus_id).or_raise(make_error)?;
        let mut sparv_config = SparvConfig::with_parent_and_metadata(
            "../config.yaml",
            SparvMetadata::new(corpus.id)
                .names(corpus.names)
                .short_descriptions(corpus.descriptions),
        );
        if let Some(doi) = corpus.doi {
            sparv_config = sparv_config.doi(doi);
        }
        make_corpus_config(&sparv_config, &output_path.join(corpus_id)).or_raise(make_error)?;
    }
    let mut progress = progress.add_child("traverse input path");
    let mut years: Vec<PathBuf> = Vec::default();
    for year in fs::read_dir(input_path).or_raise(make_error)? {
        let year = year.or_raise(make_error)?.path();
        years.push(year);
    }
    progress.init(years.len().into(), preprocess_progress::count("folders"));
    let count = progress.counter();

    for year in years {
        preprocess_sfs::build_sparv_source(
            year.as_path(),
            SfsSparvSourceOptions {
                source_dir_actual: &output_path
                    .join(CORPUS_ID_ACTUAL)
                    .join("source")
                    .join(year.file_stem().unwrap()),
                source_dir_former: &output_path
                    .join(CORPUS_ID_FORMER)
                    .join("source")
                    .join(year.file_stem().unwrap()),
            },
        )
        .or_raise(make_error)?;
        count.fetch_add(1, Ordering::Relaxed);
    }
    progress.show_throughput(start);

    Ok(())
}
