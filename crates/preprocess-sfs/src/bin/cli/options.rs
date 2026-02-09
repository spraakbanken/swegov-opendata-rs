use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[clap(author,version,about,long_about=None)]
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

    #[clap(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    Preprocess(Preprocess),
    SyncDownloads(SyncDownloads),
}

#[derive(Debug, clap::Args)]
pub struct Preprocess {
    /// The directory to read input files from.
    pub input: PathBuf,
    /// The directory to output preprocessed corpus.
    pub output: PathBuf,
}

#[derive(Debug, clap::Args)]
pub struct SyncDownloads {
    /// The directory to read input files from.
    pub input: PathBuf,
    /// The directory to output preprocessed corpus.
    pub output: Option<PathBuf>,
}
