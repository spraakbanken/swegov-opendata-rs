use std::path::Path;

use clap::Parser;
use preprocess_ui::ui::pretty::prepare_and_run;
use swegov_opendata_preprocess::preprocess_rd::{
    preprocess_rd_segreg_corpus, PreprocessRdSegregCorpusOptions,
};

use crate::options::Args;

mod options;

fn main() -> miette::Result<()> {
    let args = Args::parse();
    let trace = args.trace;
    let verbose = args.verbose;
    let input = args.input;
    let output = args.output;

    prepare_and_run(
        "preprocess-rd-segreg",
        trace,
        verbose,
        preprocess_ui::ui::STANDARD_RANGE,
        |progress, out, err| {
            preprocess_rd_segreg_corpus(
                &input,
                &output,
                out,
                err,
                progress,
                PreprocessRdSegregCorpusOptions {
                    skip_files: &[],
                    processed_json_path: Path::new("processed-rd-segreg.json"),
                },
            )
        },
    )
}
