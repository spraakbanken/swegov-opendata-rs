mod options;

use clap::Parser;
use preprocess_ui::ui::pretty::prepare_and_run;
use swegov_opendata_preprocess::preprocess_sfs::{
    preprocess_sfs_corpus, PreprocessSfsCorpuraOptions,
};

use crate::options::Args;

pub fn main() -> miette::Result<()> {
    let args = Args::parse();
    let trace = args.trace;
    let verbose = args.verbose;
    let input = args.input;
    let output = args.output;
    prepare_and_run(
        "preprocess-sfs",
        trace,
        verbose,
        preprocess_ui::ui::STANDARD_RANGE,
        |progress, out, err| {
            preprocess_sfs_corpus(
                &input,
                &output,
                out,
                err,
                progress,
                PreprocessSfsCorpuraOptions {
                    input: &input,
                    output: &output,
                },
            )
        },
    )
}
