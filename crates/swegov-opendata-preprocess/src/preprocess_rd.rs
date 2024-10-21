mod html;
mod rd_corpura;
mod rd_json;
mod rd_segreg_corpus;
mod shared;
mod xml;

pub use self::html::process_html;
pub use self::rd_corpura::{preprocess_rd_corpura, PreprocessRdCorpuraOptions};
pub use self::rd_json::{preprocess_json, PreprocessJsonError};
pub use self::rd_segreg_corpus::{preprocess_rd_segreg_corpus, PreprocessRdSegregCorpusOptions};
pub use self::xml::{preprocess_xml, XmlError};
