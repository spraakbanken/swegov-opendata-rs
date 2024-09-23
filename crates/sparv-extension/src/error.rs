use std::io;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum SparvError {
    #[diagnostic(code(SparvErrorCode::CouldNotCreateFile))]
    #[error("Could not create file '{path}'.")]
    CouldNotCreateFile {
        path: String,
        #[source]
        source: io::Error,
    },
    #[diagnostic(code(SparvErrorCode::CouldNotCreateFolder))]
    #[error("Could not create folder '{path}'.")]
    CouldNotCreateFolder {
        path: String,
        #[source]
        source: io::Error,
    },
    #[diagnostic(code(SparvErrorCode::CouldNotWriteToFile))]
    #[error("Could not write to file '{path}'.")]
    CouldNotWriteToFile {
        path: String,
        #[source]
        source: io::Error,
    },
    #[diagnostic(code(SparvErrorCode::CouldNotWriteYaml))]
    #[error("Could not write yaml to file '{path}'.")]
    CouldNotWriteYaml {
        path: String,
        #[source]
        source: serde_yaml::Error,
    },
}
