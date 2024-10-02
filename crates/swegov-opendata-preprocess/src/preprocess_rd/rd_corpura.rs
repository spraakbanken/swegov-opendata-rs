use std::{
    borrow::Cow,
    collections::HashMap,
    fs,
    io::{self, Read},
    path::Path,
    sync::atomic::Ordering,
};

use preprocess_progress::prodash::{Count, NestedProgress, Progress};
use regex::Regex;
use sparv_extension::{make_corpus_config, SparvConfig, SparvMetadata, XmlSourceWriter};
use zip::ZipArchive;

use crate::{corpusinfo, preprocess_rd::xml::preprocess_xml, PreprocessError};

#[derive(Debug, Clone)]
pub struct PreprocessRdCorpuraOptions<'a> {
    pub corpura: &'a [&'a str],
    pub skip_files: &'a [&'a str],
    pub processed_json_path: &'a Path,
    pub verbose: bool,
}

/// Preprocess corpora.
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
) -> Result<(), PreprocessError> {
    // let path = RAWDIR;
    // let output = "data/material";
    // let processed_json_path = PROCESSED_JSON;
    writeln!(out, "preprocess_corpora")?;
    // Get previously processed data
    let mut processed_json: HashMap<String, HashMap<String, String>> =
        match fs::File::open(processed_json_path) {
            Ok(file) => {
                let reader = io::BufReader::new(file);
                serde_json::from_reader(reader).map_err(|error| {
                    PreprocessError::CouldNotReadJson {
                        path: processed_json_path.to_path_buf(),
                        error,
                    }
                })?
            }

            Err(_) => HashMap::new(),
        };

    let corpus_re = Regex::new(r"(\S+)-\d{4}-.+").expect("valid regex");

    let mut zippaths = Vec::new();
    for zippath in fs::read_dir(input).map_err(|error| PreprocessError::CouldNotReadDir {
        path: input.to_path_buf(),
        error,
    })? {
        let zippath = zippath?;
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

        let prefix = if let Some(matches) = corpus_re.captures(zippath_name) {
            if let Some(prefix) = matches.get(1) {
                prefix.as_str()
            } else {
                return Err(PreprocessError::custom("No prefix"));
            }
        } else {
            return Err(PreprocessError::custom("No prefix"));
        };

        writeln!(out, "prefix={prefix}")?;
        let (corpus_id, name, descr) = corpusinfo(prefix)?;
        // Process only if in 'corpora'
        if !corpura.is_empty() && !corpura.contains(&corpus_id) {
            if verbose {
                eprintln!("skipping corpus '{corpus_id}'");
            }
            continue;
        }

        writeln!(out, "Processing {} ...", zippath.display())?;
        let corpus_source_base = Path::new(zippath.file_stem().unwrap())
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        let corpus_source_dir = Path::new(output)
            .join(corpus_id)
            .join("source")
            .join(&corpus_source_base);
        let sparv_config = SparvConfig::with_parent_and_metadata(
            "../config.yaml",
            SparvMetadata::new(corpus_id)
                .name("swe", format!("Riksdagens öppna data: {}", name))
                .description("swe", descr),
        );
        make_corpus_config(&sparv_config, &output.join(corpus_id))?;
        let mut processed_zip_dict = processed_json.remove(zippath_name).unwrap_or_default();

        let child_progress = progress.add_child("Building sparv source");

        build_sparv_source(
            &mut processed_zip_dict,
            zippath_name,
            &zippath,
            verbose,
            &mut out,
            child_progress,
            corpus_source_dir,
            corpus_source_base,
        )?;
        count.fetch_add(1, Ordering::Relaxed);
    }
    Ok(())
}

#[tracing::instrument(skip(out, progress))]
fn build_sparv_source(
    processed_zip_dict: &mut HashMap<String, String>,
    zippath_name: &str,
    zippath: &std::path::PathBuf,
    verbose: bool,
    out: &mut impl std::io::Write,
    mut progress: impl preprocess_progress::NestedProgress,
    corpus_source_dir: std::path::PathBuf,
    corpus_source_base: &str,
) -> Result<(), PreprocessError> {
    let counter = processed_zip_dict.len() + 1;
    let mut source_writer = XmlSourceWriter::with_target_and_counter(&corpus_source_dir, counter);
    let zip_file = fs::File::open(&zippath).map_err(|error| PreprocessError::CouldNotReadFile {
        path: zippath.to_owned(),
        error,
    })?;
    let mut zipf =
        ZipArchive::new(zip_file).map_err(|error| PreprocessError::CouldNotReadZipArchive {
            path: zippath.to_path_buf(),
            error,
        })?;

    progress.init(zipf.len().into(), preprocess_progress::count("files"));
    let count = progress.counter();
    for i in 0..zipf.len() {
        let mut zipobj = zipf
            .by_index(i)
            .map_err(|error| PreprocessError::CouldNotGetZipObjByIndex { index: i, error })?;
        if verbose {
            writeln!(out, "  {}: {}", i, zipobj.name())?;
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

        let mut filecontents = String::new();
        zipobj.read_to_string(&mut filecontents).map_err(|error| {
            PreprocessError::CouldNotReadZipFile {
                archive: zippath.to_path_buf(),
                path: zipobj.name().into(),
                error,
            }
        })?;

        let xmlstring =
            preprocess_xml(&filecontents, Cow::from(zipobj.name())).map_err(|error| {
                PreprocessError::XmlError {
                    path: zipobj.name().into(),
                    error,
                }
            })?;
        if xmlstring.is_empty() {
            tracing::warn!("'{}' generated empty xml", zipobj.name());
            continue;
        }
        source_writer.write(xmlstring)?;
        count.fetch_add(1, Ordering::Relaxed);
    }
    source_writer.flush()?;
    Ok(())
}
