use swegov_opendata_preprocess::{preprocess_corpura, PreprocessError};
use tracing_subscriber::EnvFilter;

fn main() -> error_stack::Result<(), PreprocessError> {
    let subscriber = tracing_subscriber::fmt()
        // .with(fmt::layer())
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("rd_preprocess=debug,info"))
                .expect("telemetry: valid envfilter"),
        )
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("telemetry: setting subscriber");
    preprocess_corpura(&["rd-bet"], &[])?;
    Ok(())
}
