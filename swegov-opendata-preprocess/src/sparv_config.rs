use std::io::Write;
use std::path::PathBuf;
use std::{fmt, fs, io};

use error_stack::{Context, ResultExt};

/// Write Sparv corpus config file for sub corpus.
pub fn make_corpus_config(
    corpus_id: &str,
    name: &str,
    descr: &str,
    mut path: PathBuf,
) -> error_stack::Result<(), SparvConfigError> {
    fs::create_dir_all(&path).change_context(SparvConfigError)?;
    path.set_file_name("config.yaml");
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
        .write(format!("    swe: Riksdagens öppna data: {}\n", name).as_bytes())
        .change_context(SparvConfigError)?;
    writer
        .write("  description:\n".as_bytes())
        .change_context(SparvConfigError)?;
    writer
        .write(format!("    swe: {}\n", descr).as_bytes())
        .change_context(SparvConfigError)?;
    //     with open(config_file, "w") as f:
    //         f.write(config_content)
    eprintln!("  Config {} written", path.display());
    Ok(())
}

#[derive(Debug)]
pub struct SparvConfigError;

impl fmt::Display for SparvConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Sparv Config error")
    }
}

impl Context for SparvConfigError {}
