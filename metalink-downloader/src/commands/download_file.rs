use crate::http::{get_file_size, make_http_client, segregrated_download, simple_download};
use crate::types::ChunkMetaData;
use crate::Result;

use anyhow::anyhow;
use std::path::PathBuf;

const ONE_MB: u64 = 1_048_576;

pub async fn download_file(
    url: url::Url,
    target_dir: PathBuf,
    user_agent: String,
    max_threads: u16,
) -> Result<()> {
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
                segregrated_download(
                    &client,
                    url.clone(),
                    target_file,
                    size,
                    &ranges,
                    None,
                    max_threads,
                )
                .await
            }
        }
        None => simple_download(&client, url.clone(), target_file).await,
    }
}
