use crate::utils::{calculate_ranges, make_http_client};
use anyhow::{anyhow, Result};
use std::{
    future::Future,
    io::{Seek, Write},
    path::PathBuf,
};
use tokio::task::JoinHandle;

const ONE_MB: u64 = 1048576;

pub async fn download_file(url: url::Url, target_dir: PathBuf, user_agent: String) -> Result<()> {
    let client = make_http_client(user_agent)?;
    let url = reqwest::Url::parse(url.as_str())?;
    let path = PathBuf::from(url.path());
    let file_name = path
        .file_name()
        .ok_or(anyhow!("Unable to extract file path from url"))?;
    let target_file = target_dir.join(file_name);

    match get_file_size(&client, url.clone()).await? {
        Some(size) => {
            if size <= ONE_MB {
                simple_download(&client, url.clone(), target_file).await
            } else {
                segregrated_download(&client, url.clone(), target_file, size).await
            }
        }
        None => simple_download(&client, url.clone(), target_file).await,
    }
}

pub async fn simple_download(
    client: &reqwest::Client,
    url: reqwest::Url,
    target_file: PathBuf,
) -> Result<()> {
    println!(
        "Simple Download: Target file={:?}, Url: {:?}",
        target_file, url
    );
    let response = client.get(url).send().await?;
    let mut output_file = std::fs::File::create(target_file.clone())?;
    output_file.write_all(&response.bytes().await?)?;
    output_file.flush()?;

    Ok(())
}

pub fn request_range(
    client: &reqwest::Client,
    url: &reqwest::Url,
    start: u64,
    end: u64,
) -> impl Future<Output = Result<reqwest::Response, reqwest::Error>> {
    client
        .get(url.clone())
        .header(
            reqwest::header::RANGE,
            reqwest::header::HeaderValue::from_str(&format!("bytes={}-{}", start, end))
                .expect("Failed to construct range header"),
        )
        .send()
}

#[derive(Debug)]
enum Command {
    WriteFileChunk {
        offset: u64,
        downloaded_bytes: bytes::Bytes,
    },
    FinishWriting,
}

pub async fn segregrated_download(
    client: &reqwest::Client,
    url: reqwest::Url,
    target_file: PathBuf,
    size: u64,
) -> Result<()> {
    let available_parallelism = std::thread::available_parallelism()?.get() - 1;
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Command>(available_parallelism);
    let mut file = std::fs::File::create(target_file.clone())?;
    let file_writer_task: JoinHandle<Result<()>> = tokio::spawn(async move {
        let mut bytes_written = 0;
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::FinishWriting => break,
                Command::WriteFileChunk {
                    offset,
                    downloaded_bytes,
                } => {
                    file.seek(std::io::SeekFrom::Start(offset))?;
                    bytes_written += file.write(&downloaded_bytes)?;
                    file.flush()?;
                    println!(
                        "Progress: {}%",
                        (bytes_written as f64 / size as f64) * 100f64
                    );
                }
            }
        }
        Ok(())
    });
    let ranges = calculate_ranges(size, ONE_MB, target_file);

    for chunk in ranges.chunks(available_parallelism) {
        let mut tasks: Vec<JoinHandle<Result<()>>> = Vec::new();
        for chunk_meta_data in chunk {
            let cloned_client = client.clone();
            let cloned_url = url.clone();
            let cloned_tx = tx.clone();
            let start = chunk_meta_data.start;
            let end = chunk_meta_data.end;

            tasks.push(tokio::spawn(async move {
                let response = request_range(&cloned_client, &cloned_url, start, end).await?;
                let _ = cloned_tx
                    .send(Command::WriteFileChunk {
                        offset: start,
                        downloaded_bytes: response.bytes().await?,
                    })
                    .await;
                Ok(())
            }));
        }
        // NOTE: the results need to be checked for failed requests and retried if it make sense
        let _ = futures::future::join_all(tasks).await;
    }

    tx.send(Command::FinishWriting).await?;
    let _ = file_writer_task.await?;

    Ok(())
}

pub async fn get_file_size(client: &reqwest::Client, url: reqwest::Url) -> Result<Option<u64>> {
    let mut response = client.head(url).send().await?;

    match response
        .headers_mut()
        .entry(reqwest::header::CONTENT_LENGTH)
    {
        reqwest::header::Entry::Occupied(entry) => Ok(Some(entry.get().to_str()?.parse()?)),
        _ => Ok(None),
    }
}
