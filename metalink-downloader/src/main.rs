use anyhow::Result;
use metalink_downloader::*;

#[tokio::main]
async fn main() -> Result<()> {
    let app = App {};
    app.run().await
}
