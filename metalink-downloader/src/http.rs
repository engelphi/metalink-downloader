use crate::types::*;
use crate::Result;

use anyhow::{anyhow, Context};
use log::info;
use std::future::Future;
use std::io::{Seek, Write};
use std::path::PathBuf;
use std::time::Duration;

use tokio::task::JoinHandle;

pub(crate) fn make_http_client(user_agent: String) -> Result<reqwest::Client> {
    Ok(reqwest::ClientBuilder::new()
        .https_only(true)
        //.http2_prior_knowledge()
        .gzip(true)
        .timeout(Duration::from_secs(20))
        .user_agent(user_agent)
        .build()?)
}

fn request_range(
    client: &reqwest::Client,
    url: &reqwest::Url,
    start: u64,
    end: u64,
) -> impl Future<Output = std::result::Result<reqwest::Response, reqwest::Error>> {
    client
        .get(url.clone())
        .header(
            reqwest::header::RANGE,
            reqwest::header::HeaderValue::from_str(&format!("bytes={}-{}", start, end))
                .expect("Failed to construct range header"),
        )
        .send()
}

pub(crate) async fn simple_download(
    client: &reqwest::Client,
    url: reqwest::Url,
    target_file: PathBuf,
) -> Result<()> {
    info!(
        "Simple Download: Target file={:?}, Url: {:?}",
        target_file, url
    );
    let response = client.get(url).send().await?;
    // Note proper error handling needed if parent is None
    std::fs::create_dir_all(target_file.parent().unwrap())?;
    let mut output_file = std::fs::File::create(target_file.clone()).context(format!(
        "Failed to create file simple download: {:#?}",
        target_file
    ))?;
    output_file
        .write_all(&response.bytes().await?)
        .context(format!(
            "Failed to write file simple download: {:#?}",
            output_file
        ))?;
    output_file.flush().context(format!(
        "Failed to flush file simple download: {:#?}",
        output_file
    ))?;

    Ok(())
}

pub(crate) async fn get_file_size(
    client: &reqwest::Client,
    url: reqwest::Url,
) -> Result<Option<u64>> {
    let mut response = client.head(url).send().await?;

    match response
        .headers_mut()
        .entry(reqwest::header::CONTENT_LENGTH)
    {
        reqwest::header::Entry::Occupied(entry) => Ok(Some(
            entry
                .get()
                .to_str()
                .context("Failed convert header Content-Length header value to string")?
                .parse()
                .context("Failed to parse Content-Length header")?,
        )),
        _ => Ok(None),
    }
}

async fn download_chunk(
    chunk: &ChunkMetaData,
    client: &reqwest::Client,
    url: &reqwest::Url,
    tx: &tokio::sync::mpsc::Sender<Command>,
) -> Result<()> {
    if !chunk.has_checksum() {
        let response = request_range(client, url, chunk.start, chunk.end).await?;
        let _ = tx
            .send(Command::WriteFileChunk {
                offset: chunk.start,
                downloaded_bytes: response.bytes().await?,
            })
            .await;
        return Ok(());
    } else {
        // retry at most three times
        for _ in 0..3 {
            let response = request_range(client, url, chunk.start, chunk.end).await?;
            let bytes = response.bytes().await?;
            log::debug!(
                "Validating checksum of {:?} for chunk starting at {}",
                chunk.filename,
                chunk.start
            );
            if let Some(true) = chunk.validate_checksum(&bytes) {
                log::debug!(
                    "Checksum validation of {:?} for chunk starting at {} succeeded",
                    chunk.filename,
                    chunk.start
                );
                let _ = tx
                    .send(Command::WriteFileChunk {
                        offset: chunk.start,
                        downloaded_bytes: bytes,
                    })
                    .await;
                return Ok(());
            }
            log::warn!(
                "Checksum validation for chunk of file {:?} starting at {}",
                chunk.filename,
                chunk.start
            );
        }
    }

    Err(anyhow!("Unable to download chunk").into())
}

async fn file_writer_task(
    target_file: &PathBuf,
    size: u64,
    mut rx: tokio::sync::mpsc::Receiver<Command>,
    prog_tx: Option<tokio::sync::mpsc::UnboundedSender<ProgressUpdate>>,
) -> Result<()> {
    // Note proper error handling needed if parent is None
    std::fs::create_dir_all(target_file.parent().unwrap())?;
    let mut file = std::fs::File::create(target_file.clone())
        .context(format!("Failed to create file: {:#?}", target_file))?;
    let mut bytes_written = 0;
    while let Some(cmd) = rx.recv().await {
        match cmd {
            Command::FinishWriting => break,
            Command::WriteFileChunk {
                offset,
                downloaded_bytes,
            } => {
                file.seek(std::io::SeekFrom::Start(offset))
                    .context(format!("Failed to seek file: {:#?}", file))?;
                let bytes = file
                    .write(&downloaded_bytes)
                    .context(format!("Failed to write file: {:#?}", file))?;
                bytes_written += bytes;
                file.flush()
                    .context(format!("Failed to flush file: {:#?}", file))?;
                info!(
                    "Progress: {}%",
                    (bytes_written as f64 / size as f64) * 100f64
                );

                if let Some(tx) = &prog_tx {
                    tx.send(ProgressUpdate::Progressed(bytes as u64))
                        .context("Failed to send progress update".to_string())?;
                }
            }
        }
    }
    Ok(())
}

pub(crate) async fn segregrated_download(
    client: &reqwest::Client,
    url: reqwest::Url,
    target_file: PathBuf,
    size: u64,
    ranges: &[ChunkMetaData],
    prog_tx: Option<tokio::sync::mpsc::UnboundedSender<ProgressUpdate>>,
) -> Result<()> {
    let available_parallelism = std::thread::available_parallelism()?.get() - 1;
    let (tx, rx) = tokio::sync::mpsc::channel::<Command>(available_parallelism);
    let file_writer_task: JoinHandle<Result<()>> =
        tokio::spawn(async move { file_writer_task(&target_file, size, rx, prog_tx).await });

    for chunk in ranges.chunks(available_parallelism) {
        let mut tasks: Vec<JoinHandle<Result<()>>> = Vec::new();
        for chunk_meta_data in chunk {
            let cloned_client = client.clone();
            let cloned_url = url.clone();
            let cloned_tx = tx.clone();
            let cloned_chunk_metadata = chunk_meta_data.clone();

            tasks.push(tokio::spawn(async move {
                download_chunk(
                    &cloned_chunk_metadata,
                    &cloned_client,
                    &cloned_url,
                    &cloned_tx,
                )
                .await
            }));
        }
        // NOTE: the results need to be checked for failed requests and retried if it make sense
        let _ = futures::future::join_all(tasks).await;
    }

    tx.send(Command::FinishWriting)
        .await
        .context("Failed to send finished command to file writer")?;
    let _ = file_writer_task.await.context("File writer task failed")?;

    Ok(())
}
