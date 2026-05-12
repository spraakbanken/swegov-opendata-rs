use std::path::{Path, PathBuf};

use clap::Parser;

use preprocess_ui::ui::pretty::{prepare_and_run, AppError};
use swegov_opendata_preprocess::ALL_CORPORA;

use preprocess_rd::cli::options::Args;
use preprocess_rd::preprocess_rd::{
    preprocess_rd_segreg_corpura, PreprocessRdSegregCorpuraOptions,
};

fn main() -> exn::Result<(), AppError> {
    let args = Args::parse();

    let trace = args.trace;
    let verbose = args.verbose;
    let input = args
        .input
        .unwrap_or_else(|| PathBuf::from("./data/rd-segreg/material"));
    let output = args
        .output
        .unwrap_or_else(|| PathBuf::from("./data/material"));
    prepare_and_run(
        "preprocess-rd-segreg",
        trace,
        verbose,
        preprocess_ui::ui::STANDARD_RANGE,
        |progress, out, err| {
            preprocess_rd_segreg_corpura(
                &input,
                &output,
                out,
                err,
                progress,
                PreprocessRdSegregCorpuraOptions {
                    corpura: ALL_CORPORA,
                    processed_json_path: Path::new("processed-rd-segreg.json"),
                    verbose,
                },
            )
        },
    )?;
    Ok(())
}
