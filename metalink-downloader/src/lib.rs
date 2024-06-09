use clap::Parser;

pub use error::{MetalinkDownloadError, Result};

mod cli;
mod commands;
mod error;
mod http;
mod types;

use cli::{Cli, Commands};

pub struct App {}

impl App {
    pub async fn run(self) -> Result<()> {
        let cli = Cli::parse();
        match cli.command {
            Commands::Plan {
                metalink_file,
                target_dir,
            } => Ok(commands::plan(metalink_file, target_dir).await?),
            Commands::DownloadFile {
                url,
                target_dir,
                user_agent,
                max_threads,
            } => Ok(commands::download_file(url, target_dir, user_agent, max_threads).await?),
            Commands::DownloadMetalink {
                metalink_file,
                target_dir,
                user_agent,
                max_threads_per_file,
                max_parallel_files,
            } => Ok(commands::download_metalink(
                metalink_file,
                target_dir,
                user_agent,
                max_threads_per_file,
                max_parallel_files,
            )
            .await?),
        }
    }
}
