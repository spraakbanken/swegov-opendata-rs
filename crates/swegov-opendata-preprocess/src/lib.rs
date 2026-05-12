mod corpusinfo;
mod error;
pub mod nodeinfo;
pub mod shared;

pub use crate::corpusinfo::corpusinfo;
pub use crate::corpusinfo::ALL_CORPORA;
pub use crate::error::PreprocessError;
pub type PreprocessResult<T> = Result<T, PreprocessError>;
