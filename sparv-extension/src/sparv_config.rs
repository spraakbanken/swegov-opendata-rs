use std::io::Write;
use std::path::Path;
use std::{fs, io};

use error_stack::ResultExt;

use crate::SparvConfigError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SparvConfig {
    parent: Option<String>,
    metadata: SparvMetadata,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SparvConfigRef<'a> {
    parent: Option<&'a str>,
    metadata: &'a SparvMetadataRef<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SparvMetadata {
    id: String,
    name: Vec<LangAndValue>,
    description: Vec<LangAndValue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SparvMetadataRef<'a> {
    id: &'a str,
    name: &'a [LangAndValueRef<'a>],
    description: &'a [LangAndValueRef<'a>],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LangAndValue {
    lang: String,
    value: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LangAndValueRef<'a> {
    lang: &'a str,
    value: &'a str,
}
/// Write Sparv corpus config file for sub corpus.
pub fn make_corpus_config(
    corpus_id: &str,
    name: &str,
    descr: &str,
    path: &Path,
) -> error_stack::Result<(), SparvConfigError> {
    fs::create_dir_all(&path).change_context(SparvConfigError)?;
    let path = path.join("config.yaml");
    //     if config_file.is_file():
    //         return
    //     path.mkdir(parents=True, exist_ok=True)
    let file = fs::File::create(&path)
        .change_context(SparvConfigError)
        .attach_printable_lazy(|| format!("failed to create {}", path.display()))?;
    let mut writer = io::BufWriter::new(file);
    writer
        .write("parent: ../config.yaml\n".as_bytes())
        .change_context(SparvConfigError)?;
    writer
        .write("\n".as_bytes())
        .change_context(SparvConfigError)?;
    writer
        .write("metadata:\n".as_bytes())
        .change_context(SparvConfigError)?;
    writer
        .write(format!("  id: {}\n", corpus_id).as_bytes())
        .change_context(SparvConfigError)?;
    writer
        .write("  name:\n".as_bytes())
        .change_context(SparvConfigError)?;
    writer
        .write(format!("    swe: 'Riksdagens öppna data: {}'\n", name).as_bytes())
        .change_context(SparvConfigError)?;
    if !descr.is_empty() {
        writer
            .write("  description:\n".as_bytes())
            .change_context(SparvConfigError)?;
        writer
            .write(format!("    swe: '{}'\n", descr).as_bytes())
            .change_context(SparvConfigError)?;
    }
    //     with open(config_file, "w") as f:
    //         f.write(config_content)
    eprintln!("  Config {} written", path.display());
    Ok(())
}
