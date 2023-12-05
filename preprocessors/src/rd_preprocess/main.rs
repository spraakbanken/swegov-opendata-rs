use std::path::Path;

use clap::Parser;
use preprocessors::shared::pretty::prepare_and_run;
use swegov_opendata_preprocess::{
    preprocess_rd::{preprocess_rd_corpura, PreprocessRdCorpuraOptions},
    PreprocessError,
};

use crate::rd_preprocess::options::Args;

pub fn main() -> error_stack::Result<(), PreprocessError> {
    let args = Args::parse();

    let trace = args.trace;
    let verbose = args.verbose;
    prepare_and_run(
        "preprocess-rd",
        trace,
        verbose,
        None,
        |progress, out, err| {
            preprocess_rd_corpura(PreprocessRdCorpuraOptions {
                input: Path::new("data/rd/rawdata"),
                output: Path::new("data/material"),
                corpura: &["rd-bet"],
                skip_files: &[],
                processed_json_path: Path::new("processed.json"),
            })
        },
    )
}
