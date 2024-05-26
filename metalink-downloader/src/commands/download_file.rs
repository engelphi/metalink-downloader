use crate::utils::make_http_client;
use anyhow::{anyhow, Result};
use std::{io::Write, path::PathBuf};

const SIXTEEN_K: usize = 16384;
const TEST_SIZE: usize = 563000;

pub async fn download_file(url: url::Url, target_dir: PathBuf, user_agent: String) -> Result<()> {
    println!("{:#?}, {:#?}, {:#?}", url, target_dir, user_agent);

    let client = make_http_client(user_agent)?;
    let url = reqwest::Url::parse(url.as_str())?;
    let path = PathBuf::from(url.path());
    let file_name = path
        .file_name()
        .ok_or(anyhow!("Unable to extract file path from url"))?;
    let target_file = target_dir.join(file_name);

    match get_file_size(&client, url.clone()).await? {
        Some(size) => {
            println!("Size: {}", size);
            if size <= TEST_SIZE {
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

    Ok(())
}

pub async fn segregrated_download(
    client: &reqwest::Client,
    url: reqwest::Url,
    target_file: PathBuf,
    size: usize,
) -> Result<()> {
    println!("Target file: {:?}", target_file);
    Ok(())
}

pub async fn get_file_size(client: &reqwest::Client, url: reqwest::Url) -> Result<Option<usize>> {
    let mut response = client.head(url).send().await?;

    match response
        .headers_mut()
        .entry(reqwest::header::CONTENT_LENGTH)
    {
        reqwest::header::Entry::Occupied(entry) => Ok(Some(entry.get().to_str()?.parse()?)),
        _ => Ok(None),
    }
}
