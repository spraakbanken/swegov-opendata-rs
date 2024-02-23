use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[clap(name = "sfs-preprocess",author,version,about,long_about=None)]
pub struct Args {
    /// Display verbose messages and progress information
    #[clap(long, short = 'v')]
    pub verbose: bool,

    /// Display structured `tracing` output in a tree-like structure.
    #[clap(long)]
    pub trace: bool,

    /// Turn off verbose message display for commands where these are shown by default.
    #[clap(long, conflicts_with("verbose"))]
    pub no_verbose: bool,

    /// The directory to read input files from.
    pub input: Option<PathBuf>,
    /// The directory to output preprocessed corpus.
    pub output: Option<PathBuf>,
}
