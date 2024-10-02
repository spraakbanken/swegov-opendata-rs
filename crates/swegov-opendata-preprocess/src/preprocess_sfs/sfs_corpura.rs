use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::{fs, path::Path};

use preprocess_progress::prodash::{Count, Progress};
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
    input_path: &Path,
    output_path: &Path,
    _out: impl std::io::Write,
    _err: impl std::io::Write,
    mut progress: impl preprocess_progress::NestedProgress,
    _options: PreprocessSfsCorpuraOptions<'_>,
) -> Result<(), PreprocessError> {
    tracing::info!("preprocess SFS corpus from {}", input_path.display());
    let start = std::time::Instant::now();
    let _config_progress = progress.add_child("create config");
    let corpus_id = "sfs";
    let sparv_config = SparvConfig::with_parent_and_metadata(
        "../config.yaml",
        SparvMetadata::new(corpus_id)
            .name("swe", "Riksdagens öppna data: Svensk Författningssamling")
            .description("swe", "Svensk Författningssamling")
            .description("eng", "Swedish Code of Statues"),
    );
    make_corpus_config(&sparv_config, &output_path.join(corpus_id))?;
    let mut progress = progress.add_child("traverse input path");
    let mut years: Vec<PathBuf> = Vec::default();
    for year in fs::read_dir(input_path).map_err(|error| PreprocessError::CouldNotReadDir {
        path: input_path.to_path_buf(),
        error,
    })? {
        let year = year?.path();
        years.push(year);
    }
    progress.init(years.len().into(), preprocess_progress::count("folders"));
    let count = progress.counter();

    for year in years {
        preprocess_sfs::build_sparv_source(
            year.as_path(),
            &output_path
                .join(corpus_id)
                .join("source")
                .join(year.file_stem().unwrap()),
        )?;
        count.fetch_add(1, Ordering::Relaxed);
    }
    progress.show_throughput(start);
    Ok(())
}
