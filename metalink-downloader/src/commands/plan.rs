use anyhow::anyhow;
use log::info;
use metalink::*;

use crate::utils::{CheckSum, ChunkMetaData};
use crate::{utils::to_chunk_metadata, MetalinkDownloadError, Result};

use std::path::{Path, PathBuf};

pub async fn plan(metalink_file: PathBuf, target_dir: PathBuf) -> Result<()> {
    info!("File: {:?}, Target: {:?}", metalink_file, target_dir);
    let plan = Plan::new(metalink_file, target_dir)?;
    println!("{:#?}", plan);
    Ok(())
}

#[derive(Debug)]
pub struct Plan {
    pub files: Vec<FilePlan>,
    pub total_size: u64,
}

impl Plan {
    pub fn new(metalink_file: PathBuf, target_dir: PathBuf) -> Result<Self> {
        let mut files: Vec<FilePlan> = Vec::new();
        let loaded_metalink = Metalink::load_from_file(metalink_file)?;
        for file in loaded_metalink.files() {
            files.push(FilePlan::new(file, &target_dir)?);
        }

        let total_size = files
            .iter()
            .fold(0, |acc, file| acc + file.file_size.unwrap_or(0));

        Ok(Self { files, total_size })
    }
}

#[derive(Debug)]
pub struct FilePlan {
    pub target_file: PathBuf,
    pub url: url::Url,
    pub file_checksums: Option<CheckSum>,
    pub chunks: Option<Vec<ChunkMetaData>>,
    pub file_size: Option<u64>,
}

impl FilePlan {
    pub fn new(file: &metalink::File, base_download_dir: &Path) -> Result<Self> {
        let target_file = base_download_dir.join(file.name());
        let file_size: Option<u64> = file.size().map(|size| size.size());

        let chunks: Option<Vec<ChunkMetaData>> = match file.pieces() {
            Some(pieces) => {
                if file_size.is_none() {
                    return Err(MetalinkDownloadError::Other(anyhow!(
                        "File size is required when having pieces"
                    )));
                }
                Some(to_chunk_metadata(pieces, &target_file, file_size.unwrap())?)
            }
            None => None,
        };

        // If we have a checksum we want to use the one with strong hash
        let file_checksums: Option<CheckSum> = match file.hashes() {
            Some(hashes) => hashes
                .iter()
                .filter(|hash| hash.hash_type().is_some())
                .max_by_key(|hash| hash.hash_type().unwrap())
                .map(|hash| CheckSum::new(hash.hash_type().unwrap(), hash.value().to_owned())),
            None => None,
        };

        let url: url::Url = match file.urls() {
            Some(urls) if !urls.is_empty() => urls.first().unwrap().url(),
            Some(_) => {
                return Err(MetalinkDownloadError::Other(anyhow!(
                    "File urls should not be empty"
                )))
            }
            None => {
                return Err(MetalinkDownloadError::Other(anyhow!(
                    "Non-url based file defintions are not supported"
                )))
            }
        };

        Ok(Self {
            chunks,
            file_checksums,
            url,
            file_size,
            target_file,
        })
    }
}
