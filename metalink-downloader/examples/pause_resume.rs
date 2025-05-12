use clap::Parser;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::path::PathBuf;

use std::fmt::Write;

use metalink_downloader::{Download, EventHandler};
pub use metalink_downloader::{MetalinkDownloadError, Result};

#[derive(Debug, Parser)]
pub struct Cli {
    /// the metalink to plan the download for
    #[arg(short, long)]
    metalink_file: PathBuf,

    /// The target or download directory
    #[arg(short, long)]
    target_dir: PathBuf,

    /// overwrite user agent
    #[arg(long, default_value=concat!("metalink-downloader/", env!("CARGO_PKG_VERSION")))]
    user_agent: String,

    /// do verify chunk checksums if available
    #[arg(short, long, default_value_t = false)]
    verify_chunk_checksums: bool,

    /// Number of concurrent tasks to be used for downloading
    #[arg(long, default_value_t = 4)]
    max_concurrent_tasks: u64,
}

#[derive(Debug)]
struct PBEventHandler {
    pb: ProgressBar,
}

impl PBEventHandler {
    pub fn new() -> Self {
        let pb = ProgressBar::no_length();
        pb.set_style(
                    ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                        .unwrap()
                        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
                        .progress_chars("#>-"));

        Self { pb }
    }
}

impl EventHandler for PBEventHandler {
    fn on_download_initialized(&self, total_download_size: u64) {
        self.pb.set_length(total_download_size);
    }

    fn on_download_started(&self) {
        self.pb.suspend(|| println!("Download Started"));
    }

    fn on_download_progressed(&self, bytes_done: u64, _total_bytes: u64) {
        self.pb.set_position(bytes_done);
    }

    fn on_download_paused(&self) {
        self.pb.suspend(|| println!("Paused"));
    }

    fn on_download_resumed(&self) {
        self.pb.suspend(|| println!("Resumed"));
    }

    fn on_download_failed(&self, error: MetalinkDownloadError) {
        self.pb
            .finish_with_message(format!("Download failed: {}", error));
    }

    fn on_download_succeeded(&self) {
        self.pb.finish_with_message("Download succeeded");
    }

    fn on_download_cancelled(&self) {
        self.pb.finish_with_message("Download Cancelled");
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut builder = Download::builder()
        .with_target_dir(cli.target_dir)
        .with_metalink_file(cli.metalink_file)
        .with_user_agent(&cli.user_agent)
        .with_max_retries(5)
        .with_max_concurrent_threads(cli.max_concurrent_tasks)
        .with_event_handler(PBEventHandler::new());
    if cli.verify_chunk_checksums {
        builder = builder.with_chunk_checksum_verification();
    }

    let mut download = builder.build()?;
    println!("Trigger Download Start");
    download.start().await?;

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    println!("Pausing download");
    download.pause().await?;

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    println!("Resuming download");
    download.resume().await?;

    println!("Waiting for download to finish");
    Ok(download.wait_for_completion().await?)
}
