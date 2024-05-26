use metalink::*;

use anyhow::Result;
use std::path::PathBuf;

pub async fn plan(metalink_file: PathBuf, target_dir: PathBuf) -> Result<()> {
    println!("File: {:?}, Target: {:?}", metalink_file, target_dir);
    let plan = Plan::new(metalink_file, target_dir)?;
    println!("{:?}", plan);
    Ok(())
}

#[derive(Debug)]
pub struct Plan {
    files: Vec<FilePlan>,
}

impl Plan {
    pub fn new(metalink_file: PathBuf, target_dir: PathBuf) -> Result<Self> {
        let mut files: Vec<FilePlan> = Vec::new();
        let loaded_metalink = Metalink::load_from_file(metalink_file)?;
        for file in loaded_metalink.files() {
            files.push(FilePlan::new(file, &target_dir));
        }

        Ok(Self { files })
    }
}

#[derive(Debug)]
pub struct FilePlan {
    target_file: PathBuf,
    url: url::Url,
    chunks: Option<Vec<FileChunk>>,
}

impl FilePlan {
    pub fn new(file: &metalink::File, base_download_dir: &PathBuf) -> Self {
        let chunks: Vec<FileChunk> = Vec::new();

        Self {
            target_file: base_download_dir.join(file.name()),
            url: file.urls().unwrap().first().unwrap().url(),
            chunks: Some(chunks),
        }
    }
}

#[derive(Debug)]
pub struct FileChunk {
    start: usize,
    end: usize,
    checksum: String,
    checksum_type: iana_registry_enums::HashFunctionTextualName,
}
//
// impl From<Option<&Pieces>> for Option<Vec<FileChunk>> {
//     fn from(value: Option<&Pieces>) -> Self {
//         todo!()
//     }
// }
