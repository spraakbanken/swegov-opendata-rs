mod corpusinfo;
pub mod nodeinfo;
pub mod preprocess;
mod sparv_config;

pub use self::corpusinfo::corpusinfo;
pub use self::preprocess::{preprocess_corpura, PreprocessError};

pub type PreprocessResult<T> = error_stack::Result<T, PreprocessError>;
