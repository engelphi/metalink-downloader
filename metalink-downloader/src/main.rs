use anyhow::{anyhow, Context};
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

use metalink_downloader::*;

#[tokio::main]
async fn main() -> Result<()> {
    let logfile = FileAppender::builder()
        //.encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .context(MetalinkDownloadError::Other(anyhow!("")))?;

    log4rs::init_config(config).context(MetalinkDownloadError::Other(anyhow!(
        "Failed to init logging"
    )))?;

    let app = App {};
    app.run().await
}
