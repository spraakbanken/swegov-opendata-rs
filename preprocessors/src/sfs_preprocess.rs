use std::{path::Path, time::Instant};

use swegov_opendata_preprocess::{
    preprocess_sfs::{preprocess_sfs_corpus, PreprocessSfsCorpuraOptions},
    PreprocessError,
};
use tracing_subscriber::EnvFilter;

fn main() {
    let start = Instant::now();
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::fmt()
        .json()
        // .with_span_list(true)
        // .with_current_span(true)
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| {
                    EnvFilter::try_new(
                        "sfs_preprocess2=debug,swegov_opendata_preprocess=trace,info",
                    )
                })
                .expect("telemetry: Creating EnvFilter"),
        )
        // .with_writer(io::stderr)
        // .with_writer(file_appender)
        .finish();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).expect("telemetry: setting subscriber");

    let res = try_main();
    let time_elapsed = start.elapsed();
    tracing::info!(?time_elapsed, "elapsed time");
    eprintln!("running time: {:?}", time_elapsed);
    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
        tracing::error!("Error: {:#}", err);
        std::process::exit(1);
    }
}

fn try_main() -> error_stack::Result<(), PreprocessError> {
    let args = std::env::args();
    let mut args = args.skip(1);
    let input: String = args.next().expect("`INPUT` required");
    let output = args.next().expect("`OUTPUT` required");
    preprocess_sfs_corpus(PreprocessSfsCorpuraOptions {
        input: Path::new(&input),
        output: Path::new(&output),
    })?;
    // let num_files_read = walk_and_build_sparv_xml(&input, &output)?;
    // tracing::info!(count = num_files_read, "files read");
    Ok(())
}
