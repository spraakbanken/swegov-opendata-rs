use std::time::Instant;

use swegov_opendata_preprocess::PreprocessError;
use tracing_subscriber::EnvFilter;

pub fn init_tracing() {
    let subscriber = tracing_subscriber::fmt()
        // .with(fmt::layer())
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("info"))
                .expect("telemetry: valid envfilter"),
        )
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("telemetry: setting subscriber");
}

pub fn prepare_and_run(
    name: &str,
    run: impl FnOnce() -> error_stack::Result<(), PreprocessError>,
) -> error_stack::Result<(), PreprocessError> {
    init_tracing();
    let start = Instant::now();
    let res = run();
    let time_elapsed = start.elapsed();
    tracing::info!(?time_elapsed, "elapsed time");
    eprintln!("running time for '{}': {:?}", name, time_elapsed);
    res
}
