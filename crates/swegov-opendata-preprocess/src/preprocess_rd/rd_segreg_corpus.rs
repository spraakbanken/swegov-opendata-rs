use std::{
    borrow::Cow, collections::HashMap, fs, io::Read, path::Path, sync::atomic::Ordering,
    time::Duration,
};

use once_cell::sync::Lazy;
use preprocess_progress::{
    prodash::{Count, Progress},
    NestedProgress,
};
use regex::Regex;
use sparv_extension::{make_corpus_config, SparvConfig, SparvMetadata, XmlSourceWriter};
use swegov_opendata::DataSet;
use zip::ZipArchive;

use crate::{corpusinfo, preprocess_rd::shared::write_json, shared::io_ext, PreprocessError};

use super::{rd_json::preprocess_json, shared::read_json_or_default};

#[derive(Debug, Clone)]
pub struct PreprocessRdSegregCorpusOptions<'a> {
    pub skip_files: &'a [&'a str],
    pub processed_json_path: &'a Path,
}

const CORPUS_ID: &'static str = "rd-segreg";
/// Preprocess RD-SEGREG corpus.
///
pub fn preprocess_rd_segreg_corpus(
    input: &Path,
    output: &Path,
    out: impl std::io::Write,
    err: impl std::io::Write,
    mut progress: impl preprocess_progress::NestedProgress + 'static,
    PreprocessRdSegregCorpusOptions {
        skip_files,
        processed_json_path,
    }: PreprocessRdSegregCorpusOptions<'_>,
) -> Result<(), PreprocessError>
// where
    // <impl preprocess_progress::NestedProgress as NestedProgress>::SubProgress: 'static,
{
    tracing::info!(
        "preprocess rd-segreg from {} to {}",
        input.display(),
        output.display()
    );
    let mut processed_json: HashMap<String, HashMap<String, String>> =
        read_json_or_default(processed_json_path)?;
    let output_dir = output.join(CORPUS_ID);
    // {
    //     let _config_progress = progress.add_child("creating config");
    //     let sparv_config = SparvConfig::with_metadata(
    //         SparvMetadata::new(CORPUS_ID)
    //             .name("swe", "Segregationens språk")
    //             .description("swe", "Texter som handlar om segregation")
    //             .description("eng", "Texts that treat segregation"),
    //     );
    //     sparv_extension::make_corpus_config(&sparv_config, &output_dir)?;
    // }
    let folder_progress = progress.add_child("traverse folders");

    let mut ctx = Context {
        base_output: output,
        corpus_source_dir: &output_dir,
        processed_json: &mut processed_json,
    };
    let res = process_folders(
        input,
        &output_dir,
        &out,
        &err,
        &mut preprocess_progress::BoxedDynNestedProgress::new(folder_progress),
        &mut ctx,
    );
    write_json(processed_json_path, &processed_json)?;
    res
}
#[derive(Debug)]
pub struct Context<'a> {
    pub base_output: &'a Path,
    pub corpus_source_dir: &'a Path,
    pub processed_json: &'a mut HashMap<String, HashMap<String, String>>,
}
#[tracing::instrument(skip(out, err, progress, ctx))]
fn process_folders(
    input_path: &Path,
    output_path: &Path,
    out: &dyn std::io::Write,
    err: &dyn std::io::Write,
    progress: &mut preprocess_progress::BoxedDynNestedProgress,
    ctx: &mut Context,
) -> Result<(), PreprocessError> {
    tracing::trace!("processing '{}'", input_path.display());
    let start = std::time::Instant::now();
    let mut progress = progress.add_child("traverse path");

    let mut folders = Vec::default();
    for folder in fs::read_dir(input_path).map_err(|error| PreprocessError::CouldNotReadFolder {
        path: input_path.to_path_buf(),
        error,
    })? {
        let folder = folder
            .map_err(|error| PreprocessError::CouldNotAccessDirEntry {
                path: input_path.to_path_buf(),
                error,
            })?
            .path();
        // tracing::debug!("queuing {}", folder.display());
        folders.push(folder);
    }
    progress.init(folders.len().into(), preprocess_progress::count("folders"));
    let count = progress.counter();
    for folder in folders {
        // tracing::debug!("processing path '{}'", folder.display());
        let new_output_path = update_output_path(output_path, &folder);
        // if let Some(file_stem) = folder.file_stem() {
        // let new_output_path = output_path.join(file_stem);
        if build_source(folder.as_path()) {
            let folder_str = folder.as_path().display().to_string();
            if !ctx.processed_json.contains_key(&folder_str) {
                let mut processed_zip_dict =
                    ctx.processed_json.remove(&folder_str).unwrap_or_default();

                build_sparv_source(
                    folder.as_path(),
                    &new_output_path,
                    &mut progress,
                    &mut processed_zip_dict,
                    ctx,
                )?;
                ctx.processed_json.insert(folder_str, processed_zip_dict);
            }
        } else if folder.is_dir() && process_folder(folder.as_path()) {
            process_folders(
                folder.as_path(),
                &new_output_path,
                out,
                err,
                &mut progress,
                ctx,
            )?;
        }
        // }

        std::thread::sleep(Duration::from_millis(10));
        count.fetch_add(1, Ordering::Relaxed);
    }
    progress.show_throughput(start);
    Ok(())
}

// const SUPPORTED_TYPES: &'static [&'static str] = &["mot", "bet", "prot", "sou", "prop"];

fn update_output_path<'a>(orig_output: &'a Path, curr_input: &'a Path) -> Cow<'a, Path> {
    let _ = curr_input;
    Cow::Borrowed(orig_output)
}
fn build_source(path: &Path) -> bool {
    // tracing::trace!("analyzing path = '{}'", path.display());
    if let Some(ext) = path.extension() {
        if ext == "zip" {
            return true;
        }
    }

    // tracing::trace!("not building source from '{}'", path.display());
    false
}

fn process_folder(path: &Path) -> bool {
    !path.ends_with("anforande")
}
#[tracing::instrument()]
fn preprocess_file(path: &Path) -> Result<(), PreprocessError> {
    todo!()
}

#[tracing::instrument(skip(progress, ctx))]
fn build_sparv_source(
    path: &Path,
    output: &Path,
    progress: &mut preprocess_progress::BoxedDynNestedProgress,
    processed_zip_dict: &mut HashMap<String, String>,
    ctx: &Context,
) -> Result<(), PreprocessError> {
    tracing::debug!("building sparv source from {}", path.display());
    static SEGREG: Lazy<Regex> = Lazy::new(|| Regex::new(r"[Ss][Ee][Gg][Rr][Ee][Gg]").unwrap());
    static CORPUS_RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(\S+(\s\S+)?)\s?-\d{4}-.+").expect("valid regex"));

    let mut progress = progress.add_child("traverse zip archive");
    let mut metadata_path = path.to_path_buf();
    for _ in 0..2 {
        if metadata_path.extension().is_some() {
            metadata_path.set_extension("");
        }
    }
    metadata_path.set_extension("metadata.json");

    let metadata: DataSet =
        serde_json::from_str(&io_ext::read_text(&metadata_path).map_err(|error| {
            PreprocessError::CouldNotReadFile {
                path: metadata_path.clone(),
                error,
            }
        })?)
        .map_err(|error| PreprocessError::CouldNotReadJson {
            path: metadata_path,
            error,
        })?;
    let zippath_name = path
        .file_name()
        .expect("a filename")
        .to_str()
        .expect("valid utf8");
    let prefix = if let Some(matches) = CORPUS_RE.captures(zippath_name) {
        if let Some(prefix) = matches.get(1) {
            prefix.as_str().replace(' ', "+")
        } else {
            return Err(PreprocessError::NoValidPrefix(zippath_name.to_string()));
        }
    } else {
        tracing::warn!(
            "Filename '{}' contains no valid corpus prefix: skipping ...",
            path.display()
        );
        return Ok(());
        // return Err(PreprocessError::NoValidPrefix(zippath_name.to_string()));
    };
    let (corpus_id, names, short_descrs) = match corpusinfo(&prefix) {
        Ok(corpus_info) => corpus_info,
        Err(err) => {
            tracing::error!(
                "Failed to get corpusinfo for prefix='{}' for file='{}': error={}: skipping ...",
                prefix,
                path.display(),
                err,
            );
            return Ok(());
        }
    };
    let corpus_id = format!("segreg-{corpus_id}");
    let corpus_source_base = Path::new(path.file_stem().unwrap()).file_stem().unwrap();
    // todo!("handle {:?} from {:?}", corpus_source_base, path);
    let corpus_source_dir = output
        .join(&corpus_id)
        .join("source")
        .join(corpus_source_base);
    let sparv_config = SparvConfig::with_parent_and_metadata(
        "../config.yaml",
        SparvMetadata::new(&corpus_id)
            .names(names)
            .short_descriptions(short_descrs),
    );
    make_corpus_config(&sparv_config, &output.join(corpus_id))?;
    let counter = processed_zip_dict.len() + 1;
    let mut source_writer = XmlSourceWriter::with_target_and_counter(&corpus_source_dir, counter);
    let zip_file = fs::File::open(path).map_err(|error| PreprocessError::CouldNotReadFile {
        path: path.to_owned(),
        error,
    })?;
    let mut zipf =
        ZipArchive::new(zip_file).map_err(|error| PreprocessError::CouldNotReadZipArchive {
            path: path.to_owned(),
            error,
        })?;
    progress.init(zipf.len().into(), preprocess_progress::count("files"));
    let count = progress.counter();
    for i in 0..zipf.len() {
        let mut zipobj = zipf
            .by_index(i)
            .map_err(|error| PreprocessError::CouldNotGetZipObjByIndex { index: i, error })?;
        tracing::trace!(zip_archive = ?path,"Processing file {}: {}",i,zipobj.name());
        let mut filecontents = String::new();
        zipobj.read_to_string(&mut filecontents).map_err(|error| {
            PreprocessError::CouldNotReadZipFile {
                archive: path.to_path_buf(),
                path: zipobj.name().into(),
                error,
            }
        })?;
        if SEGREG.is_match(&filecontents) {
            let filecontents = filecontents.replace("{/* RESERVATIONSTEXT */}", r#""""#);
            // let xmlstring = preprocess_json(&filecontents, &metadata).map_err(|error| {
            //     PreprocessError::RdPreprocessJsonError {
            //         path: format!("{}:{}", path.display(), zipobj.name()),
            //         error,
            //     }
            // })?;
            let xmlstring = match preprocess_json(&filecontents, &metadata) {
                Ok(xmlstring) => xmlstring,
                Err(err) => {
                    tracing::error!(
                        "Failed to convert document '{}:{}'. Error: {:?}",
                        path.display(),
                        zipobj.name(),
                        err
                    );
                    count.fetch_add(1, Ordering::Relaxed);
                    continue;
                }
            };
            source_writer.write(xmlstring)?;
        } else {
            // tracing::trace!("didn't find SEGREG regex in {}", zipobj.name());
        }
        processed_zip_dict.insert(zipobj.name().to_string(), source_writer.current_filename());
        count.fetch_add(1, Ordering::Relaxed);
    }

    source_writer.flush()?;
    Ok(())
}
