use anyhow::{anyhow, Context};
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};

use metalink_downloader::{App, MetalinkDownloadError, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let logfile = FileAppender::builder().build("log/output.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Debug),
        )
        .context(MetalinkDownloadError::Other(anyhow!("")))?;

    log4rs::init_config(config).context(MetalinkDownloadError::Other(anyhow!(
        "Failed to init logging"
    )))?;

    let app = App {};
    app.run().await
}
