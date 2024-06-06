use crate::commands::download_file::{segregrated_download, simple_download};
use crate::commands::plan::Plan;
use crate::utils::make_http_client;
use crate::Result;
use std::path::PathBuf;

pub async fn download_metalink(
    metalink_file: PathBuf,
    target_dir: PathBuf,
    user_agent: String,
) -> Result<()> {
    let plan = Plan::new(metalink_file, target_dir)?;
    let client = make_http_client(user_agent)?;

    for file in plan.files {
        if file.chunks.is_some() {
            segregrated_download(
                &client,
                file.url,
                file.target_file,
                file.file_size.unwrap(),
                &file.chunks.unwrap(),
            )
            .await?;
        } else {
            simple_download(&client, file.url, file.target_file).await?;
        }
    }

    Ok(())
}
