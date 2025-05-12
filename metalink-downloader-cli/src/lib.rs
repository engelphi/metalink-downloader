use std::fmt::Write;

use clap::Parser;

use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use metalink_downloader::{Download, EventHandler};
pub use metalink_downloader::{MetalinkDownloadError, Result};

mod cli;

use cli::{Cli, Commands};

pub struct App {}

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

impl App {
    pub async fn run(self) -> Result<()> {
        let cli = Cli::parse();
        match cli.command {
            Commands::Plan {
                metalink_file,
                target_dir,
            } => Ok(metalink_downloader::plan(metalink_file, target_dir).await?),
            Commands::DownloadFile {
                url,
                target_dir,
                user_agent,
                max_threads,
            } => Ok(
                metalink_downloader::download_file(url, target_dir, user_agent, max_threads)
                    .await?,
            ),
            Commands::DownloadMetalink {
                metalink_file,
                target_dir,
                user_agent,
                verify_chunk_checksums,
                max_concurrent_tasks,
            } => {
                let mut builder = Download::builder()
                    .with_target_dir(target_dir)
                    .with_metalink_file(metalink_file)
                    .with_user_agent(&user_agent)
                    .with_max_retries(5)
                    .with_max_concurrent_threads(max_concurrent_tasks)
                    .with_event_handler(PBEventHandler::new());
                if verify_chunk_checksums {
                    builder = builder.with_chunk_checksum_verification();
                }

                let mut download = builder.build()?;
                log::info!("Trigger Download Start");
                download.start().await?;
                Ok(download.wait_for_completion().await?)
            }
        }
    }
}
