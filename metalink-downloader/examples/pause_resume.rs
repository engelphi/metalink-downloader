use clap::Parser;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::path::PathBuf;

use std::fmt::Write;

use metalink_downloader::Download;
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

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let pb = ProgressBar::no_length();
    pb.set_style(
                    ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                        .unwrap()
                        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
                        .progress_chars("#>-"));
    let cloned_pb = pb.clone();
    let mut builder = Download::builder()
        .with_target_dir(cli.target_dir)
        .with_metalink_file(cli.metalink_file)
        .with_user_agent(&cli.user_agent)
        .with_max_retries(5)
        .with_max_concurrent_threads(cli.max_concurrent_tasks)
        .with_progress_callback(move |done, total| {
            if cloned_pb.length().is_none() {
                cloned_pb.set_length(total);
            }
            cloned_pb.set_position(done);
            if done == total {
                cloned_pb.finish();
            }
        });
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
