use fs_err as fs;
use std::path::{Path, PathBuf};

use exn::{Exn, ResultExt};

#[derive(Debug, thiserror::Error)]
#[error("failed to sync SFS downloads")]
pub struct SyncSfsDownloadsError;

pub fn sync_sfs_downloads(
    input_path: &Path,
    output_path: Option<&PathBuf>,
    out: impl std::io::Write,
    err: impl std::io::Write,
    mut progress: impl preprocess_progress::NestedProgress,
) -> Result<(), Exn<SyncSfsDownloadsError>> {
    let make_error = || SyncSfsDownloadsError;

    let mut downloads = Vec::new();
    for download in fs::read_dir(input_path).or_raise(make_error)? {
        let download = download.or_raise(make_error)?.path();
        let Some(stem) = download.file_stem().map(|s| s.to_string_lossy()) else {
            continue;
        };
        if download.is_dir() && stem.starts_with(|c: char| c.is_numeric()) {
            downloads.push(download);
        }
    }
    downloads.sort_by(|a, b| b.cmp(a));
    dbg!(&downloads);
    progress.init(
        downloads.len().into(),
        preprocess_progress::count("folders"),
    );
    let count = progress.counter();
    let start = std::time::Instant::now();

    for download in downloads {
        count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    progress.show_throughput(start);
    Ok(())
}
