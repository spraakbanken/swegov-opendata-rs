use std::collections::HashMap;
use std::path::Path;
use std::{fs, io};

use error_stack::ResultExt;

use crate::SparvConfigError;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct SparvConfig {
    parent: Option<String>,
    metadata: SparvMetadata,
}

impl SparvConfig {
    pub fn new(parent: Option<String>, metadata: SparvMetadata) -> SparvConfig {
        Self { parent, metadata }
    }

    pub fn with_metadata(metadata: SparvMetadata) -> SparvConfig {
        Self::new(None, metadata)
    }

    pub fn with_parent_and_metadata<S: Into<String>>(
        parent: S,
        metadata: SparvMetadata,
    ) -> SparvConfig {
        Self::new(Some(parent.into()), metadata)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct SparvMetadata {
    id: String,
    name: HashMap<String, String>,
    description: HashMap<String, String>,
    short_description: HashMap<String, String>,
}

impl SparvMetadata {
    pub fn new<S: Into<String>>(id: S) -> SparvMetadata {
        Self {
            id: id.into(),
            name: Default::default(),
            description: Default::default(),
            short_description: Default::default(),
        }
    }
    pub fn name<S: Into<String>>(mut self, lang: &str, name: S) -> Self {
        self.name.insert(lang.to_string(), name.into());
        self
    }
    pub fn description<S: Into<String>>(mut self, lang: &str, description: S) -> Self {
        self.description
            .insert(lang.to_string(), description.into());
        self
    }
    pub fn short_description<S: Into<String>>(mut self, lang: &str, description: S) -> Self {
        self.short_description
            .insert(lang.to_string(), description.into());
        self
    }
}

/// Write Sparv corpus config file for sub corpus.
pub fn make_corpus_config(
    sparv_config: &SparvConfig,
    path: &Path,
) -> error_stack::Result<(), SparvConfigError> {
    fs::create_dir_all(&path).change_context(SparvConfigError)?;
    let path = path.join("config.yaml");
    //     if config_file.is_file():
    //         return
    //     path.mkdir(parents=True, exist_ok=True)
    let file = fs::File::create(&path)
        .change_context(SparvConfigError)
        .attach_printable_lazy(|| format!("failed creating '{}'", path.display()))?;
    let writer = io::BufWriter::new(file);
    serde_yaml::to_writer(writer, &sparv_config)
        .change_context(SparvConfigError)
        .attach_printable_lazy(|| format!("failed writing to '{}'", path.display()))?;
    tracing::info!(path = ?path, "  Config written",);
    Ok(())
}
