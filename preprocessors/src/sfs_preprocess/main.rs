use clap::Parser;
use preprocessors::shared::pretty::prepare_and_run;
use swegov_opendata_preprocess::{
    preprocess_sfs::{preprocess_sfs_corpus, PreprocessSfsCorpuraOptions},
    PreprocessError,
};

use crate::sfs_preprocess::options::Args;

pub fn main() -> error_stack::Result<(), PreprocessError> {
    let args = Args::parse();
    let trace = args.trace;
    let verbose = args.verbose;
    let input = args.input;
    let output = args.output;
    prepare_and_run(
        "preprocess-sfs",
        trace,
        verbose,
        None,
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
