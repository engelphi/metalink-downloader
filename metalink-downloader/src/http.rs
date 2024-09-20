use crate::types::{ChunkMetaData, Command, ProgressUpdate};
use crate::Result;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, Jitter, RetryTransientMiddleware};

use anyhow::{anyhow, Context};
use log::info;
use std::io::{Seek, Write};
use std::path::PathBuf;
use std::time::Duration;

use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::task::JoinHandle;

pub(crate) type Client = ClientWithMiddleware;

/// Creates a reqwest client to be used by the downloader tasks
pub(crate) fn make_http_client(user_agent: String) -> Result<Client> {
    let retry_policy = ExponentialBackoff::builder()
        .retry_bounds(Duration::from_secs(1), Duration::from_secs(60))
        .jitter(Jitter::Bounded)
        .base(2)
        .build_with_max_retries(5);
    Ok(ClientBuilder::new(
        reqwest::ClientBuilder::new()
            .https_only(true)
            .http2_prior_knowledge()
            .gzip(true)
            .zstd(true)
            .timeout(Duration::from_secs(20))
            .user_agent(user_agent)
            .build()?,
    )
    .with(RetryTransientMiddleware::new_with_policy(retry_policy))
    .build())
}

async fn request_range(
    client: &Client,
    url: &reqwest::Url,
    start: u64,
    end: u64,
) -> Result<reqwest::Response> {
    Ok(client
        .get(url.clone())
        .header(
            reqwest::header::RANGE,
            reqwest::header::HeaderValue::from_str(&format!("bytes={start}-{end}"))
                .expect("Failed to construct range header"),
        )
        .send()
        .await?)
}

pub(crate) async fn simple_download(
    client: &Client,
    url: reqwest::Url,
    target_file: PathBuf,
) -> Result<()> {
    info!("Simple Download: Target file={target_file:?}, Url: {url:?}");
    let response = client.get(url).send().await?;
    // Note proper error handling needed if parent is None
    std::fs::create_dir_all(target_file.parent().unwrap())?;
    let mut output_file = std::fs::File::create(target_file.clone())
        .with_context(|| format!("Failed to create file simple download: {target_file:#?}"))?;
    output_file
        .write_all(&response.bytes().await?)
        .with_context(|| format!("Failed to write file simple download: {output_file:#?}"))?;
    output_file
        .flush()
        .with_context(|| format!("Failed to flush file simple download: {output_file:#?}"))?;

    Ok(())
}

pub(crate) async fn get_file_size(client: &Client, url: reqwest::Url) -> Result<Option<u64>> {
    let mut response = client.head(url).send().await?;

    match response
        .headers_mut()
        .entry(reqwest::header::CONTENT_LENGTH)
    {
        reqwest::header::Entry::Occupied(entry) => Ok(Some(
            entry
                .get()
                .to_str()
                .with_context(|| "Failed convert header Content-Length header value to string")?
                .parse()
                .with_context(|| "Failed to parse Content-Length header")?,
        )),
        reqwest::header::Entry::Vacant(_) => Ok(None),
    }
}

async fn download_chunk(
    chunk: &ChunkMetaData,
    client: &Client,
    url: &reqwest::Url,
    tx: &tokio::sync::mpsc::UnboundedSender<Command>,
) -> Result<()> {
    if chunk.has_checksum() {
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
                tx.send(Command::WriteFileChunk {
                    offset: chunk.start,
                    downloaded_bytes: bytes,
                })
                .with_context(|| {
                    format!(
                        "Failed to send downloaded chunk of {:?} starting at: {}",
                        chunk.filename, chunk.start
                    )
                })?;
                return Ok(());
            }
            log::warn!(
                "Checksum validation for chunk of file {:?} starting at {} failed",
                chunk.filename,
                chunk.start
            );
        }
    } else {
        let response = request_range(client, url, chunk.start, chunk.end).await?;
        tx.send(Command::WriteFileChunk {
            offset: chunk.start,
            downloaded_bytes: response.bytes().await?,
        })
        .with_context(|| {
            format!(
                "Failed to send unvalidated downloaded chunk of {:?}, starting at: {}",
                chunk.filename, chunk.start
            )
        })?;
        return Ok(());
    }

    Err(anyhow!("Unable to download chunk").into())
}

async fn file_writer_task(
    target_file: &PathBuf,
    size: u64,
    mut rx: tokio::sync::mpsc::UnboundedReceiver<Command>,
    prog_tx: Option<tokio::sync::mpsc::UnboundedSender<ProgressUpdate>>,
) -> Result<()> {
    // Note proper error handling needed if parent is None
    std::fs::create_dir_all(target_file.parent().unwrap())?;
    let mut file = std::fs::File::create(target_file.clone())
        .with_context(|| format!("Failed to create file: {target_file:#?}"))?;
    file.set_len(size)?;
    let mut bytes_written = 0;
    while let Some(cmd) = rx.recv().await {
        match cmd {
            Command::FinishWriting => break,
            Command::WriteFileChunk {
                offset,
                downloaded_bytes,
            } => {
                file.seek(std::io::SeekFrom::Start(offset))
                    .with_context(|| format!("Failed to seek file: {file:#?}"))?;
                let bytes = file
                    .write(&downloaded_bytes)
                    .with_context(|| format!("Failed to write file: {file:#?}"))?;
                bytes_written += bytes;

                info!(
                    "Progress: {}%",
                    (bytes_written as f64 / size as f64) * 100f64
                );
                file.flush()
                    .with_context(|| format!("Failed to flush file: {file:#?}"))?;
                if let Some(tx) = &prog_tx {
                    tx.send(ProgressUpdate::Progressed(bytes as u64))
                        .with_context(|| "Failed to send progress update")?;
                }
            }
        }
    }
    Ok(())
}

pub(crate) async fn segregrated_download(
    client: &Client,
    url: reqwest::Url,
    target_file: PathBuf,
    size: u64,
    ranges: &[ChunkMetaData],
    prog_tx: Option<tokio::sync::mpsc::UnboundedSender<ProgressUpdate>>,
    max_threads: u16,
) -> Result<()> {
    let available_parallelism: usize = (max_threads - 1) as usize;
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Command>();
    let file_writer: JoinHandle<Result<()>> =
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
        .with_context(|| "Failed to send finished command to file writer")?;
    file_writer
        .await
        .with_context(|| "File writer task failed")??;

    Ok(())
}

pub(crate) async fn download(
    client: &Client,
    url: reqwest::Url,
    target_file: PathBuf,
    ranges: &[ChunkMetaData],
    prog_tx: Option<tokio::sync::mpsc::UnboundedSender<ProgressUpdate>>,
    verify_chunk_checksum: bool,
) -> Result<()> {
    std::fs::create_dir_all(target_file.parent().unwrap())?;
    let mut f = File::create(target_file.clone())
        .await
        .with_context(|| format!("Failed to create file {:?}", target_file))?;

    for chunk in ranges {
        if chunk.has_checksum() && verify_chunk_checksum {
            // retry at most three times
            for _ in 0..3 {
                let response = request_range(client, &url, chunk.start, chunk.end).await?;
                let bytes = response.bytes().await?;
                if let Some(true) = chunk.validate_checksum(&bytes) {
                    log::debug!(
                        "Checksum validation of {:?} for chunk starting at {} succeeded",
                        chunk.filename,
                        chunk.start
                    );
                    f.write_all(&bytes).await?;
                }
            }
        } else {
            let response = request_range(client, &url, chunk.start, chunk.end).await?;
            let bytes = response.bytes().await?;
            f.write_all(&bytes).await?;
        }

        if let Some(tx) = &prog_tx {
            tx.send(ProgressUpdate::Progressed(chunk.chunk_size()))
                .with_context(|| "Failed to send progress update")?;
        }
    }

    Ok(())
}
