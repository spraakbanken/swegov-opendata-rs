use std::{
    borrow::Cow,
    collections::HashMap,
    fmt, fs,
    io::{self, Read},
    path::Path,
};

use error_stack::{Context, ResultExt};
use regex::Regex;
use sparv_extension::{make_corpus_config, SparvConfig, SparvMetadata};
use zip::ZipArchive;

use crate::{corpusinfo, preprocess_rd::xml::preprocess_xml, PreprocessError};

const MAX_SIZE: usize = 10 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct PreprocessRdCorpuraOptions<'a> {
    pub input: &'a Path,
    pub output: &'a Path,
    pub corpura: &'a [&'a str],
    pub skip_files: &'a [&'a str],
    pub processed_json_path: &'a Path,
}

/// Preprocess corpora.
///
/// corpora: List that specifies which corpora (corpus-IDs) to process (default: all)
/// skip_files: Zip files which should not be processed.
/// testfile: Parse only 'testfile' and output result to 'test.xml'.
pub fn preprocess_rd_corpura(
    PreprocessRdCorpuraOptions {
        input,
        output,
        corpura,
        skip_files,
        processed_json_path,
    }: PreprocessRdCorpuraOptions<'_>,
) -> error_stack::Result<(), PreprocessError> {
    // let path = RAWDIR;
    let verbose = true;
    // let output = "data/material";
    // let processed_json_path = PROCESSED_JSON;
    eprintln!("preprocess_corpora");
    // Get previously processed data
    let mut processed_json: HashMap<String, HashMap<String, String>> =
        match fs::File::open(processed_json_path) {
            Ok(file) => {
                let reader = io::BufReader::new(file);
                serde_json::from_reader(reader)
                    .change_context(PreprocessError)
                    .attach_printable_lazy(|| {
                        format!("can't read {}", processed_json_path.display())
                    })?
            }

            Err(_) => HashMap::new(),
        };

    let corpus_re = Regex::new(r"(\S+)-\d{4}-.+").expect("valid regex");
    for zippath in fs::read_dir(input)
        .change_context(PreprocessError)
        .attach_printable_lazy(|| format!("could not read dir {}", input.display()))?
    {
        let zippath = zippath.change_context(PreprocessError)?;
        if zippath.path().is_file() {
            let zippath = zippath.path();
            dbg!(&zippath);
            let zippath_name = zippath
                .file_name()
                .expect("a file")
                .to_str()
                .expect("valid utf8");
            dbg!(&zippath_name);

            if zippath_name.starts_with(".") || !zippath_name.ends_with(".zip") {
                if verbose {
                    eprintln!("skipping '{}' ...", zippath.display());
                }
                continue;
            }

            // Don't process if in 'skip_files'
            if !skip_files.is_empty() && skip_files.contains(&zippath_name) {
                if verbose {
                    eprintln!("found '{}' in `skip_files`, skipping ...", zippath_name);
                }
                continue;
            }

            let prefix = if let Some(matches) = corpus_re.captures(zippath_name) {
                if let Some(prefix) = matches.get(1) {
                    prefix.as_str()
                } else {
                    return Err(PreprocessError.into());
                }
            } else {
                return Err(PreprocessError.into());
            };

            eprintln!("prefix={prefix}");
            let (corpus_id, name, descr) = corpusinfo(prefix).change_context(PreprocessError)?;
            // Process only if in 'corpora'
            if !corpura.is_empty() && !corpura.contains(&corpus_id) {
                if verbose {
                    eprintln!("skipping corpus '{corpus_id}'");
                }
                continue;
            }

            eprintln!("Processing {} ...", zippath.display());
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
                    .description("swe", descr)
                    .short_description("swe", ""),
            );
            make_corpus_config(&sparv_config, &output.join(corpus_id))
                .change_context(PreprocessError)?;

            let mut total_size = 0;
            let mut result: Vec<Vec<u8>> = vec![];
            let processed_zip_dict = processed_json.remove(zippath_name).unwrap_or_default();
            let mut counter = processed_zip_dict.len() + 1;

            let zip_file = fs::File::open(&zippath)
                .change_context(PreprocessError)
                .attach_printable_lazy(|| format!("Failed to open {}", zippath.display()))?;
            let mut zipf = ZipArchive::new(zip_file)
                .change_context(PreprocessError)
                .attach_printable_lazy(|| {
                    format!("Could not read the zip archive {}", zippath.display())
                })?;
            for i in 0..zipf.len() {
                let mut zipobj = zipf.by_index(i).change_context(PreprocessError)?;
                if verbose {
                    eprintln!("  {}: {}", i, zipobj.name());
                }

                // Skip if already processed
                if processed_zip_dict.contains_key(zipobj.name()) {
                    if verbose {
                        eprintln!("  Skipping file '{}' (already processed)", zipobj.name());
                    }
                    continue;
                }

                let mut filecontents = String::new();
                zipobj
                    .read_to_string(&mut filecontents)
                    .change_context(PreprocessError)
                    .attach_printable("failed to read zip file")?;

                let xmlstring = preprocess_xml(&filecontents, Cow::from(zipobj.name()))
                    .change_context(PreprocessError)?;
                if xmlstring.is_empty() {
                    tracing::warn!("'{}' generated empty xml", zipobj.name());
                    continue;
                }
                let this_size = xmlstring.len();
                // If adding the latest result would lead to the file size going over the limit, save
                if total_size + this_size > MAX_SIZE {
                    dbg!(&corpus_source_dir);
                    dbg!(&corpus_source_base);
                    let curr_file =
                        corpus_source_dir.join(format!("{}-{}.xml", corpus_source_base, counter));
                    dbg!(&curr_file);
                    write_xml(result.as_slice(), curr_file.as_path())?;
                    tracing::info!("wrote xml to '{}'", curr_file.display());
                    result.clear();
                    total_size = 0;
                    counter += 1;
                }

                result.push(xmlstring);
                total_size += this_size;
                // break;
            }
        }
    }
    Ok(())
}

fn write_xml(texts: &[Vec<u8>], xmlpath: &Path) -> error_stack::Result<(), PreprocessError> {
    use std::io::Write;
    let corpus_source_dir = xmlpath.parent().unwrap();
    fs::create_dir_all(corpus_source_dir).change_context(PreprocessError)?;
    let xmlfile = fs::File::create(xmlpath).change_context(PreprocessError)?;
    let mut writer = io::BufWriter::new(xmlfile);
    writer.write(b"<file>\n").change_context(PreprocessError)?;
    for text in texts {
        writer.write(text).change_context(PreprocessError)?;
        writer.write(b"\n").change_context(PreprocessError)?;
    }
    writer.write(b"\n</file>").change_context(PreprocessError)?;
    Ok(())
}
