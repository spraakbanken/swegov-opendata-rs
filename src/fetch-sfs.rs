mod fetch_sfs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    fetch_sfs::main().await
}
