use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Download file
    DownloadFile {
        /// url to download
        #[arg(short, long)]
        url: url::Url,

        /// target_dir to download
        #[arg(short, long)]
        target_dir: PathBuf,

        /// overwrite user agent
        #[arg(long, default_value=concat!("metalink-downloader/", env!("CARGO_PKG_VERSION")))]
        user_agent: String,
    },

    /// Dryrun the planning phase
    Plan {
        /// the metalink to plan the download for
        #[arg(short, long)]
        metalink_file: PathBuf,

        /// The target or download directory
        #[arg(short, long)]
        target_dir: PathBuf,
    },

    /// Download Metalink
    DownloadMetalink {
        /// the metalink to plan the download for
        #[arg(short, long)]
        metalink_file: PathBuf,

        /// The target or download directory
        #[arg(short, long)]
        target_dir: PathBuf,

        /// overwrite user agent
        #[arg(long, default_value=concat!("metalink-downloader/", env!("CARGO_PKG_VERSION")))]
        user_agent: String,
    },
}
