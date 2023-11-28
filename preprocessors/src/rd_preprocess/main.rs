use std::path::Path;

use preprocessors::shared::prepare_and_run;
use swegov_opendata_preprocess::{
    preprocess_rd::{preprocess_rd_corpura, PreprocessRdCorpuraOptions},
    PreprocessError,
};

pub fn main() -> error_stack::Result<(), PreprocessError> {
    prepare_and_run("preprocess-rd", || {
        preprocess_rd_corpura(PreprocessRdCorpuraOptions {
            input: Path::new("data/rd/rawdata"),
            output: Path::new("data/material"),
            corpura: &["rd-bet"],
            skip_files: &[],
            processed_json_path: Path::new("processed.json"),
        })
    })
}
