use crate::http::*;
use crate::types::Plan;
use crate::Result;
use anyhow::Context;
use std::fmt::Write;
use std::path::PathBuf;
use tokio::task::JoinHandle;

use crate::types::ProgressUpdate;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};

pub async fn download_metalink(
    metalink_file: PathBuf,
    target_dir: PathBuf,
    user_agent: String,
) -> Result<()> {
    log::info!("==========Start Metalink Download==========");
    let plan = Plan::new(metalink_file, target_dir)?.minimize_plan()?;

    let client = make_http_client(user_agent)?;
    let total_size = plan.total_size;
    let (prog_tx, mut prog_rx) = tokio::sync::mpsc::unbounded_channel::<ProgressUpdate>();
    let progress_reporter: JoinHandle<Result<()>> = tokio::spawn(async move {
        let pb = ProgressBar::new(total_size);
        pb.set_style(
            ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .unwrap()
                .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
                .progress_chars("#>-"));
        let mut bytes_downloaded = 0;
        while let Some(cmd) = prog_rx.recv().await {
            match cmd {
                ProgressUpdate::Progressed(bytes) => {
                    bytes_downloaded += bytes;
                    pb.set_position(bytes_downloaded);
                }
                ProgressUpdate::Finished => break,
            }
        }

        pb.finish_with_message("Download Finished");
        Ok(())
    });

    for file in plan.files {
        log::info!("Start downloading: {:?}", file.target_file);
        if let Some(chunks) = file.chunks {
            segregrated_download(
                &client,
                file.url,
                file.target_file.clone(),
                file.file_size.unwrap(),
                &chunks,
                Some(prog_tx.clone()),
            )
            .await?;
        } else {
            simple_download(&client, file.url, file.target_file.clone()).await?;
        }
        log::info!("Finish downloading: {:?}", file.target_file);
    }

    prog_tx
        .send(ProgressUpdate::Finished)
        .context("Failed to send finish progress command")?;
    let _ = progress_reporter
        .await
        .context("Progress Reporter failed")?;

    Ok(())
}
