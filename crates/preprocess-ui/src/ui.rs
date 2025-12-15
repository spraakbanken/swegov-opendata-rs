pub const DEFAULT_FRAME_RATE: f32 = 6.0;

#[allow(unused)]
pub type ProgressRange = std::ops::RangeInclusive<prodash::progress::key::Level>;
#[allow(unused)]
pub const STANDARD_RANGE: ProgressRange = 2..=2;

pub mod pretty {
    use miette::IntoDiagnostic;
    use std::error::Error;
    use std::io::{stderr, stdout};
    use std::time::Instant;
    use tracing_subscriber::EnvFilter;

    use crate::ui::ProgressRange;
    use preprocess_progress;

    pub fn init_tracing(
        enable: bool,
        progress: &preprocess_progress::prodash::tree::Root,
    ) -> miette::Result<()> {
        if enable {
            let processor = tracing_forest::Printer::new().formatter({
                let progress = std::sync::Mutex::new(progress.add_child("tracing"));
                move |tree: &tracing_forest::tree::Tree| -> Result<String, std::fmt::Error> {
                    use preprocess_progress::prodash::Progress;
                    use tracing_forest::Formatter;
                    let progress = &mut progress.lock().unwrap();
                    let tree = tracing_forest::printer::Pretty.fmt(tree)?;
                    for line in tree.lines() {
                        progress.info(line.into());
                    }
                    Ok(String::new())
                }
            });
            use tracing_subscriber::layer::SubscriberExt;
            let subscriber = tracing_subscriber::Registry::default()
                .with(tracing_forest::ForestLayer::from(processor));
            tracing::subscriber::set_global_default(subscriber).into_diagnostic()?;
        } else {
            let subscriber = tracing_subscriber::fmt()
                // .with(fmt::layer())
                .with_env_filter(
                    EnvFilter::try_from_default_env()
                        .or_else(|_| EnvFilter::try_new("warn"))
                        .expect("telemetry: valid envfilter"),
                )
                .finish();
            // tracing::subscriber::set_global_default(tracing_subscriber::Registry::default())
            tracing::subscriber::set_global_default(subscriber).into_diagnostic()?;
        }
        Ok(())
    }

    pub fn prepare_and_run<E: Error + Send + Sync + 'static>(
        name: &str,
        trace: bool,
        verbose: bool,
        range: impl Into<Option<ProgressRange>>,
        run: impl FnOnce(
            preprocess_progress::DoOrDiscard<prodash::tree::Item>,
            &mut dyn std::io::Write,
            &mut dyn std::io::Write,
        ) -> Result<(), E>,
    ) -> miette::Result<()> {
        let start = Instant::now();
        let res = match verbose {
            false => {
                let stdout = stdout();
                let mut stdout_lock = stdout.lock();
                let stderr = stderr();
                let mut stderr_lock = stderr.lock();
                run(
                    preprocess_progress::DoOrDiscard::from(None),
                    &mut stdout_lock,
                    &mut stderr_lock,
                )
            }
            true => {
                use crate::ui::{self, STANDARD_RANGE};
                let progress = ui::progress_tree(trace);
                let sub_progress = progress.add_child(name);
                init_tracing(trace, &progress)?;
                let handle = ui::setup_line_renderer_range(
                    &progress,
                    range.into().unwrap_or(STANDARD_RANGE),
                );

                let mut out = Vec::<u8>::new();
                let res = run(
                    preprocess_progress::DoOrDiscard::from(Some(sub_progress)),
                    &mut out,
                    &mut stderr(),
                );
                handle.shutdown_and_wait();
                std::io::Write::write_all(&mut stdout(), &out).into_diagnostic()?;
                res
            }
        };
        let time_elapsed = start.elapsed();
        tracing::info!(?time_elapsed, "elapsed time");
        res.into_diagnostic()
    }
}
pub fn progress_tree(trace: bool) -> std::sync::Arc<prodash::tree::Root> {
    prodash::tree::root::Options {
        message_buffer_capacity: if trace { 10_000 } else { 200 },
        ..Default::default()
    }
    .into()
}

#[allow(unused)]
pub fn setup_line_renderer_range(
    progress: &std::sync::Arc<prodash::tree::Root>,
    levels: std::ops::RangeInclusive<prodash::progress::key::Level>,
) -> prodash::render::line::JoinHandle {
    prodash::render::line(
        std::io::stderr(),
        std::sync::Arc::downgrade(progress),
        prodash::render::line::Options {
            level_filter: Some(levels),
            frames_per_second: DEFAULT_FRAME_RATE,
            initial_delay: Some(std::time::Duration::from_millis(1000)),
            timestamp: true,
            throughput: true,
            hide_cursor: true,
            ..prodash::render::line::Options::default()
        }
        .auto_configure(prodash::render::line::StreamKind::Stderr),
    )
}
