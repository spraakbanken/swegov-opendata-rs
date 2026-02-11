use std::path::{Path, PathBuf};

use clap::Parser;
use preprocess_ui::ui::pretty::{prepare_and_run, AppError};
use swegov_opendata_preprocess::{
    preprocess_rd::{preprocess_rd_corpura, PreprocessRdCorpuraOptions},
    ALL_CORPUSES,
};

use crate::options::Args;

mod options;

fn main() -> exn::Result<(), AppError> {
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
        preprocess_ui::ui::STANDARD_RANGE,
        |progress, out, err| {
            preprocess_rd_corpura(
                &input,
                &output,
                out,
                err,
                progress,
                PreprocessRdCorpuraOptions {
                    corpura: ALL_CORPUSES,
                    skip_files: &[],
                    processed_json_path: Path::new("processed.json"),
                    verbose,
                },
            )
        },
    )?;
    Ok(())
}
