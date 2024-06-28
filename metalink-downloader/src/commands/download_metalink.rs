use crate::http::{download, make_http_client, simple_download};
use crate::types::{FilePlan, Plan};
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
    max_threads_per_file: u16,
) -> Result<()> {
    log::info!("==========Start Metalink Download==========");
    let plan = Plan::new(metalink_file, &target_dir)?.minimize_plan()?;

    let client = make_http_client(user_agent)?;
    let total_size = plan.total_size;
    let (prog_tx, prog_rx) = tokio::sync::mpsc::unbounded_channel::<ProgressUpdate>();
    let progress_reporter: JoinHandle<Result<()>> =
        tokio::spawn(async move { progress_reporter_task(prog_rx, total_size).await });

    //let mut tasks: Vec<JoinHandle<Result<()>>> = Vec::new();
    for file in plan.files {
        let cloned_file = file.clone();
        let cloned_tx = prog_tx.clone();
        let cloned_client = client.clone();
        //tasks.push(tokio::spawn(async move {
        download_file_task(
            &cloned_client,
            &cloned_file,
            &cloned_tx,
            max_threads_per_file,
        )
        .await
        .with_context(|| format!("Failed to download file: {:?}", file.target_file))?;
        //}));
    }
    //let _ = futures::future::join_all(tasks).await;

    prog_tx
        .send(ProgressUpdate::Finished)
        .with_context(|| "Failed to send finish progress command")?;
    progress_reporter
        .await
        .with_context(|| "Progress Reporter failed")??;

    Ok(())
}

async fn download_file_task(
    client: &reqwest::Client,
    file: &FilePlan,
    tx: &tokio::sync::mpsc::UnboundedSender<ProgressUpdate>,
    max_threads_per_file: u16,
) -> Result<()> {
    log::info!("Start downloading: {:?}", file.target_file);
    if let Some(chunks) = file.chunks.as_ref() {
        download(
            client,
            file.url.clone(),
            file.target_file.clone(),
            file.file_size.unwrap(),
            chunks,
            Some(tx.clone()),
            max_threads_per_file,
        )
        .await
        .with_context(|| format!("Parallel download of {:?} failed", file.target_file))?;
    } else {
        simple_download(client, file.url.clone(), file.target_file.clone())
            .await
            .with_context(|| format!("Simple download of {:?} failed", file.target_file))?;
    }
    log::info!("Finish downloading: {:?}", file.target_file);
    Ok(())
}

async fn progress_reporter_task(
    mut prog_rx: tokio::sync::mpsc::UnboundedReceiver<ProgressUpdate>,
    total_size: u64,
) -> Result<()> {
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
}
