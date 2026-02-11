mod options;

use clap::Parser;
use preprocess_sfs::{
    preprocess_sfs::{preprocess_sfs_corpus, PreprocessSfsCorpuraOptions},
    sync_sfs_downloads::sync_sfs_downloads,
};
use preprocess_ui::ui::pretty::{prepare_and_run, AppError};

use crate::options::Args;

pub fn main() -> exn::Result<(), AppError> {
    let args = Args::parse();
    let trace = args.trace;
    let verbose = args.verbose;
    match args.cmd {
        options::Subcommand::Preprocess(cmd) => prepare_and_run(
            "preprocess",
            trace,
            verbose,
            preprocess_ui::ui::STANDARD_RANGE,
            |progress, out, err| {
                preprocess_sfs_corpus(
                    &cmd.input,
                    &cmd.output,
                    out,
                    err,
                    progress,
                    PreprocessSfsCorpuraOptions {
                        input: &cmd.input,
                        output: &cmd.output,
                    },
                )
            },
        ),
        options::Subcommand::SyncDownloads(cmd) => prepare_and_run(
            "sync-downloads",
            trace,
            verbose,
            preprocess_ui::ui::STANDARD_RANGE,
            |progress, out, err| {
                sync_sfs_downloads(&cmd.input, cmd.output.as_ref(), out, err, progress)
            },
        ),
    }
}
