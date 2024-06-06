use clap::Parser;

pub use error::{MetalinkDownloadError, Result};

mod cli;
mod commands;
mod error;
mod utils;

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
            } => Ok(commands::download_file(url, target_dir, user_agent).await?),
            Commands::DownloadMetalink {
                metalink_file,
                target_dir,
                user_agent,
            } => Ok(commands::download_metalink(metalink_file, target_dir, user_agent).await?),
        }
    }
}
