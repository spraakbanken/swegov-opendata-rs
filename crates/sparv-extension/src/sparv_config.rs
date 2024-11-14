use std::collections::HashMap;
use std::path::Path;
use std::{fs, io};

use crate::SparvError;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct SparvConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    description: HashMap<String, String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
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
pub fn make_corpus_config(sparv_config: &SparvConfig, path: &Path) -> Result<(), SparvError> {
    fs::create_dir_all(&path).map_err(|source| SparvError::CouldNotCreateFolder {
        path: path.display().to_string(),
        source,
    })?;
    let path = path.join("config.yaml");
    //     if config_file.is_file():
    //         return
    //     path.mkdir(parents=True, exist_ok=True)
    let file = fs::File::create(&path).map_err(|source| SparvError::CouldNotCreateFile {
        path: path.display().to_string(),
        source,
    })?;
    let writer = io::BufWriter::new(file);
    serde_yaml::to_writer(writer, &sparv_config).map_err(|source| {
        SparvError::CouldNotWriteYaml {
            path: path.display().to_string(),
            source,
        }
    })?;
    tracing::info!(path = ?path, "  Config written",);
    Ok(())
}
