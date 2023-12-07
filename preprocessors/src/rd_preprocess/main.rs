use std::path::{Path, PathBuf};

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
    let input = args
        .input
        .unwrap_or_else(|| PathBuf::from("./data/rd/material"));
    let output = args
        .output
        .unwrap_or_else(|| PathBuf::from("./data/material"));
    prepare_and_run(
        "preprocess-rd",
        trace,
        verbose,
        None,
        |progress, out, err| {
            preprocess_rd_corpura(
                &input,
                &output,
                out,
                err,
                progress,
                PreprocessRdCorpuraOptions {
                    corpura: &["rd-bet"],
                    skip_files: &[],
                    processed_json_path: Path::new("processed.json"),
                    verbose,
                },
            )
        },
    )
}
