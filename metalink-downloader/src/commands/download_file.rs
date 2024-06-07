use crate::http::*;
use crate::types::ChunkMetaData;
use crate::Result;

use anyhow::anyhow;
use std::path::PathBuf;

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
                let ranges = ChunkMetaData::calculate_ranges(size, ONE_MB, &target_file);
                segregrated_download(&client, url.clone(), target_file, size, &ranges, None).await
            }
        }
        None => simple_download(&client, url.clone(), target_file).await,
    }
}
