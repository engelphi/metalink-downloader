use anyhow::Result;
use clap::Parser;

mod cli;
mod commands;
mod utils;

use cli::{Commands, CLI};

pub struct App {}

impl App {
    pub async fn run(self) -> Result<()> {
        let cli = CLI::parse();
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
        }
    }
}
