mod corpusinfo;
mod error;
pub mod nodeinfo;
pub mod preprocess_rd;
pub mod preprocess_sfs;
pub mod shared;

pub use self::corpusinfo::corpusinfo;
pub use self::error::PreprocessError;
pub type PreprocessResult<T> = error_stack::Result<T, PreprocessError>;
