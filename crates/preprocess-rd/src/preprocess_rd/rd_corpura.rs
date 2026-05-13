use std::path::PathBuf;
use std::{borrow::Cow, collections::HashMap, io::Read, path::Path, sync::atomic::Ordering};

use exn::Exn;
use exn::ResultExt;
use fs_err as fs;

use preprocess_progress::prodash::{Count, NestedProgress, Progress};
use sparv_extension::{make_corpus_config, SparvConfig, SparvMetadata, XmlSourceWriter};
use zip::ZipArchive;

use crate::preprocess_rd::shared;
use crate::preprocess_rd::xml::preprocess_xml;
use swegov_opendata_preprocess::corpusinfo;
use swegov_opendata_preprocess::PreprocessError;

use super::shared::read_json_or_default;

#[derive(Debug, Clone)]
pub struct PreprocessRdCorpuraOptions<'a> {
    pub corpura: &'a [&'a str],
    pub skip_files: &'a [&'a str],
    pub processed_json_path: &'a Path,
    pub verbose: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum PreprocessRdCorpuraError {
    #[error("failed to preprocess sfs corpora")]
    Failure,
    // #[error("failed to preprocess sfs corpora: {message}")]
    // WithMsg { message: String },
}

/// Preprocess RD corpora.
///
/// corpora: List that specifies which corpora (corpus-IDs) to process (default: all)
/// skip_files: Zip files which should not be processed.
/// testfile: Parse only 'testfile' and output result to 'test.xml'.
pub fn preprocess_rd_corpura(
    input: &Path,
    output: &Path,
    mut out: impl std::io::Write,
    _err: impl std::io::Write,
    mut progress: impl preprocess_progress::NestedProgress,
    PreprocessRdCorpuraOptions {
        corpura,
        skip_files,
        processed_json_path,
        verbose,
    }: PreprocessRdCorpuraOptions<'_>,
) -> Result<(), Exn<PreprocessRdCorpuraError>> {
    let make_error = || PreprocessRdCorpuraError::Failure;
    // let path = RAWDIR;
    // let output = "data/material";
    // let processed_json_path = PROCESSED_JSON;
    writeln!(out, "preprocess_corpora").or_raise(make_error)?;
    // Get previously processed data
    let mut processed_json: HashMap<String, HashMap<String, String>> =
        read_json_or_default(processed_json_path).or_raise(make_error)?;

    let mut zippaths = Vec::new();
    for zippath in fs::read_dir(input).or_raise(make_error)? {
        let zippath = zippath.or_raise(make_error)?;
        let zippath = zippath.path();
        if zippath.is_file() {
            let zippath_name = zippath
                .file_name()
                .expect("a file")
                .to_str()
                .expect("valid utf8");

            if zippath_name.starts_with(".") || !zippath_name.ends_with(".zip") {
                tracing::info!("skipping '{}' ...", zippath.display());
                continue;
            }

            // Don't process if in 'skip_files'
            if !skip_files.is_empty() && skip_files.contains(&zippath_name) {
                tracing::info!("found '{}' in `skip_files`, skipping ...", zippath_name);
                continue;
            }
            zippaths.push(zippath);
        } else {
            tracing::info!("'{}' is not a file, skipping ...", zippath.display());
        }
    }
    let mut progress = progress.add_child("traverse input path");
    progress.init(
        zippaths.len().into(),
        preprocess_progress::count("zip files"),
    );
    let count = progress.counter();

    for zippath in zippaths {
        let zippath_name = zippath
            .file_name()
            .expect("a file")
            .to_str()
            .expect("valid utf8");

        let Some(prefix) = shared::find_prefix(zippath_name).or_raise(make_error)? else {
            tracing::warn!(
                "Filename '{}' contains no valid corpus prefix: skipping ...",
                zippath_name
            );
            return Ok(());
        };

        writeln!(out, "prefix={prefix}").or_raise(make_error)?;
        let corpus = corpusinfo(&prefix).or_raise(make_error)?;

        // Process only if in 'corpora'
        if !corpura.is_empty() && !corpura.contains(&corpus.id) {
            if verbose {
                eprintln!("skipping corpus '{}'", corpus.id);
            }
            continue;
        }

        writeln!(out, "Processing {} ...", zippath.display()).or_raise(make_error)?;

        let mut sparv_config = SparvConfig::with_parent_and_metadata(
            "../config.yaml",
            SparvMetadata::new(corpus.id)
                .names(corpus.names)
                .short_descriptions(corpus.descriptions),
        );
        if let Some(doi) = corpus.doi {
            sparv_config = sparv_config.doi(doi);
        }
        make_corpus_config(&sparv_config, &output.join(corpus.id)).or_raise(make_error)?;

        let mut processed_zip_dict = processed_json.remove(zippath_name).unwrap_or_default();

        let child_progress = progress.add_child("Building sparv source");

        let corpus_source_base = Path::new(zippath.file_stem().unwrap())
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        let corpus_source_dir = Path::new(output)
            .join(corpus.id)
            .join("source")
            .join(corpus_source_base);

        build_sparv_source(
            &mut processed_zip_dict,
            zippath_name,
            &zippath,
            verbose,
            &mut out,
            child_progress,
            corpus_source_dir,
            corpus_source_base,
        )
        .or_raise(make_error)?;
        count.fetch_add(1, Ordering::Relaxed);
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum BuildSparvSourceError {
    #[error("failed to build sparv source from path '{path}'")]
    Failure { path: PathBuf },
    #[error("failed to build sparv source from path '{path}': could not read zip obj with index {index}")]
    CouldNotGetZipObjByIndex { path: PathBuf, index: usize },
}

#[tracing::instrument(skip(out, progress))]
#[allow(clippy::too_many_arguments)]
fn build_sparv_source(
    processed_zip_dict: &mut HashMap<String, String>,
    zippath_name: &str,
    zippath: &std::path::PathBuf,
    verbose: bool,
    out: &mut impl std::io::Write,
    mut progress: impl preprocess_progress::NestedProgress,
    corpus_source_dir: std::path::PathBuf,
    corpus_source_base: &str,
) -> Result<(), Exn<BuildSparvSourceError>> {
    let make_error = || BuildSparvSourceError::Failure {
        path: zippath.clone(),
    };
    let counter = processed_zip_dict.len() + 1;
    let mut source_writer = XmlSourceWriter::with_target_and_counter(&corpus_source_dir, counter);
    let zip_file = fs::File::open(zippath).or_raise(make_error)?;
    let mut zipf = ZipArchive::new(zip_file).or_raise(make_error)?;

    progress.init(zipf.len().into(), preprocess_progress::count("files"));
    let count = progress.counter();
    let mut filecontents = String::new();
    for i in 0..zipf.len() {
        let mut zipobj =
            zipf.by_index(i)
                .or_raise(|| BuildSparvSourceError::CouldNotGetZipObjByIndex {
                    path: zippath.clone(),
                    index: i,
                })?;
        if verbose {
            writeln!(out, "  {}: {}", i, zipobj.name()).or_raise(make_error)?;
        }

        // Skip if already processed
        if processed_zip_dict.contains_key(zipobj.name()) {
            if verbose {
                let _ = writeln!(
                    out,
                    "  Skipping file '{}' (already processed)",
                    zipobj.name()
                );
            }
            continue;
        }
        filecontents.clear();
        zipobj
            .read_to_string(&mut filecontents)
            .map_err(|error| PreprocessError::CouldNotReadZipFile {
                archive: zippath.to_path_buf(),
                path: zipobj.name().into(),
                error,
            })
            .or_raise(make_error)?;

        let filecontents = filecontents.replace("{/* RESERVATIONSTEXT */}", r#""""#);

        let xmlstring =
            preprocess_xml(&filecontents, Cow::from(zipobj.name())).or_raise(make_error)?;
        if xmlstring.is_empty() {
            tracing::warn!("'{}' generated empty xml", zipobj.name());
            continue;
        }
        source_writer.write(xmlstring).or_raise(make_error)?;
        count.fetch_add(1, Ordering::Relaxed);
    }
    source_writer.flush().or_raise(make_error)?;
    Ok(())
}
