mod options;

use std::{io, path::PathBuf, sync::Arc, time::Duration};

use clap::Parser;
use miette::IntoDiagnostic;
use tokio::signal;
use tracing_subscriber::EnvFilter;
use webcrawler::{crawler, CrawlerOptions};

use crate::options::Args;

#[tokio::main]
async fn main() -> miette::Result<()> {
    let args = Args::parse();

    let delay_ms = args.delay_ms;
    let crawling_concurrency = args.crawling_concurrency;
    let processing_concurrency = args.processing_concurrency;
    let state_path = args.state;
    let output = args.output;

    init_tracing()?;

    let spider = Arc::new(opendata_spiders::sfs::SfsSpider::new(
        opendata_spiders::sfs::SfsSpiderOptions {
            user_agent: Some(APP_USER_AGENT.into()),
            output_path: output.unwrap_or_else(|| PathBuf::from("./output")),
        },
    ));
    crawler::run_with_options(
        spider,
        signal::ctrl_c(),
        CrawlerOptions {
            saved_state_path: Some(state_path),
            delay: Duration::from_millis(delay_ms),
            crawling_concurrency,
            processing_concurrency,
        },
    )
    .await;
    Ok(())
}

/// construct a subscriber that prints formatted traces to stdout
fn init_tracing() -> miette::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .json()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("fetch_sfs=info,info"))
                .expect("telemetry: Creating EnvFilter"),
        )
        .with_writer(io::stderr)
        .finish();
    tracing::subscriber::set_global_default(subscriber).into_diagnostic()?;
    Ok(())
}

// == Client ==
// Name your user agent after your app?
pub static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
