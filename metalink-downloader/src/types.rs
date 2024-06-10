use anyhow::anyhow;
use digest::{generic_array::ArrayLength, Digest, OutputSizeUser};
use iana_registry_enums::HashFunctionTextualName;
use log::info;
use metalink::Metalink;
use std::io::{Read, Seek};
use std::path::{Path, PathBuf};

use crate::{MetalinkDownloadError, Result};

#[derive(Debug)]
pub(crate) enum Command {
    WriteFileChunk {
        offset: u64,
        downloaded_bytes: bytes::Bytes,
    },
    FinishWriting,
}

#[derive(Debug)]
pub(crate) enum ProgressUpdate {
    // Download progressed by n bytes
    Progressed(u64),
    Finished,
}

#[derive(Debug, Default)]
pub struct Plan {
    pub files: Vec<FilePlan>,
    pub total_size: u64,
}

impl Plan {
    pub fn new(metalink_file: PathBuf, target_dir: &Path) -> Result<Self> {
        let mut files: Vec<FilePlan> = Vec::new();
        let loaded_metalink = Metalink::load_from_file(metalink_file)?;
        for file in loaded_metalink.files() {
            files.push(FilePlan::new(file, target_dir)?);
        }

        let total_size = files
            .iter()
            .fold(0, |acc, file| acc + file.file_size.unwrap_or(0));

        Ok(Self { files, total_size })
    }

    /// Shrink the plan so the only files and chunks that need to
    /// be downloaded are left
    pub fn minimize_plan(self) -> Result<Plan> {
        let mut minimized_plan = Plan::default();

        for file in self.files {
            if !file.target_file.exists() {
                minimized_plan.files.push(file);
            } else if let Some(chunks) = file.chunks {
                let file_on_disk = std::fs::File::open(&file.target_file)?;
                let mut minimized_chunks: Vec<ChunkMetaData> = Vec::new();
                for chunk in chunks {
                    if !chunk.is_valid_on_disk(&file_on_disk)? {
                        minimized_chunks.push(chunk);
                    }
                }

                if !minimized_chunks.is_empty() {
                    minimized_plan.files.push(FilePlan {
                        target_file: file.target_file,
                        url: file.url,
                        file_checksums: file.file_checksums,
                        chunks: Some(minimized_chunks),
                        file_size: file.file_size,
                    });
                }
            } else if let Some(checksum) = file.file_checksums.as_ref() {
                if !checksum.validate_file_checksum(&file.target_file) {
                    minimized_plan.files.push(file);
                }
            } else {
                // no checksums to validate need to assume that the file is broken an
                // redownload it
                minimized_plan.files.push(file);
            }
        }

        // After minimizing the plan total size calculation gets a bit complicated
        // If we have a file without chunks then we take the file size if the file
        // has chunks we need to sum up the size of the chunks. As only those parts
        // will be downloaded
        let mut total_size: u64 = 0;

        for file in &minimized_plan.files {
            if let Some(chunks) = file.chunks.as_ref() {
                for chunk in chunks {
                    total_size += chunk.chunk_size();
                }
            } else {
                // NOTE: if a file element in the metalink does not have a file size this
                // might fail
                total_size += file.file_size.unwrap();
            }
        }

        minimized_plan.total_size = total_size;

        Ok(minimized_plan)
    }
}

#[derive(Debug, Clone)]
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
        let file_size: Option<u64> = file.size().map(metalink::Size::size);

        let chunks: Option<Vec<ChunkMetaData>> = match file.pieces() {
            Some(pieces) => {
                if file_size.is_none() {
                    return Err(MetalinkDownloadError::Other(anyhow!(
                        "File size is required when having pieces"
                    )));
                }
                Some(ChunkMetaData::to_chunk_metadata(
                    pieces,
                    &target_file,
                    file_size.unwrap(),
                )?)
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
            target_file,
            url,
            file_checksums,
            chunks,
            file_size,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChunkMetaData {
    pub start: u64,
    pub end: u64,
    pub checksum: Option<CheckSum>,
    pub filename: PathBuf,
}

impl ChunkMetaData {
    pub fn new(start: u64, end: u64, filename: PathBuf) -> Self {
        Self {
            start,
            end,
            filename,
            checksum: None,
        }
    }

    pub fn has_checksum(&self) -> bool {
        self.checksum.is_some()
    }

    pub fn validate_checksum(&self, bytes: &bytes::Bytes) -> Option<bool> {
        self.checksum
            .as_ref()
            .map(|checksum| checksum.validate_checksum(bytes))
    }

    pub fn is_valid_on_disk(&self, mut file: &std::fs::File) -> Result<bool> {
        if let Some(checksum) = self.checksum.as_ref() {
            file.seek(std::io::SeekFrom::Start(self.start))?;
            info!("Chunk size: {}", self.chunk_size());
            let buffer_size: usize = self.chunk_size() as usize;
            let mut buffer: Vec<u8> = vec![0; buffer_size];
            let count = file.read(buffer.as_mut_slice())?;
            if count != buffer_size {
                return Ok(false);
            }

            return Ok(checksum.validate_checksum(&bytes::Bytes::from(buffer)));
        }

        Ok(false)
    }

    pub fn chunk_size(&self) -> u64 {
        self.end - self.start + 1
    }

    pub fn to_chunk_metadata(
        pieces: &metalink::Pieces,
        filename: &Path,
        total_size: u64,
    ) -> Result<Vec<ChunkMetaData>> {
        let mut ranges = Self::calculate_ranges(total_size, pieces.length(), filename);

        let hash_type = pieces.hash_type();

        if ranges.len() != pieces.hashes().len() {
            return Err(MetalinkDownloadError::Other(anyhow!(
                "Mismatch between chunk count({}) and pieces count({})",
                ranges.len(),
                pieces.hashes().len()
            )));
        }

        for (chunk, hash) in ranges.iter_mut().zip(pieces.hashes().iter()) {
            chunk.checksum = Some(CheckSum::new(hash_type, hash.value().to_owned()));
        }

        Ok(ranges)
    }

    pub fn calculate_ranges(
        total_size: u64,
        block_size: u64,
        filename: &Path,
    ) -> Vec<ChunkMetaData> {
        let mut remaining_size = total_size;
        let mut current_pos = 0;

        let mut ranges: Vec<ChunkMetaData> = Vec::new();
        while remaining_size > block_size {
            ranges.push(ChunkMetaData::new(
                current_pos,
                current_pos + block_size - 1,
                filename.to_path_buf(),
            ));
            current_pos += block_size;
            remaining_size -= block_size;
        }
        ranges.push(ChunkMetaData::new(
            current_pos,
            current_pos + remaining_size - 1,
            filename.to_path_buf(),
        ));

        ranges
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CheckSum {
    hash_type: HashFunctionTextualName,
    checksum: String,
}

fn calculate_checksum<D: Digest>(data: &bytes::Bytes) -> String
where
    <D as OutputSizeUser>::OutputSize: std::ops::Add,
    <<D as OutputSizeUser>::OutputSize as std::ops::Add>::Output: ArrayLength<u8>,
{
    format!("{:x}", D::digest(data))
}

fn calculate_file_checksum<D: Digest>(path: &std::path::Path) -> Result<String>
where
    <D as OutputSizeUser>::OutputSize: std::ops::Add,
    <<D as OutputSizeUser>::OutputSize as std::ops::Add>::Output: ArrayLength<u8>,
{
    let input_file = std::fs::File::open(path)?;
    let mut reader = std::io::BufReader::new(input_file);

    let digest = {
        let mut hasher = D::new();
        let mut buffer = [0; 1024];
        loop {
            let count = reader.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }
        hasher.finalize()
    };
    Ok(format!("{digest:x}"))
}

impl CheckSum {
    pub fn new(hash_type: HashFunctionTextualName, checksum: String) -> Self {
        Self {
            hash_type,
            checksum,
        }
    }

    fn calculate_checksum(&self, data: &bytes::Bytes) -> String {
        match self.hash_type {
            HashFunctionTextualName::Md2 => calculate_checksum::<md2::Md2>(data),
            HashFunctionTextualName::Md5 => calculate_checksum::<md5::Md5>(data),
            HashFunctionTextualName::Sha1 => calculate_checksum::<sha1_checked::Sha1>(data),
            HashFunctionTextualName::Sha224 => calculate_checksum::<sha2::Sha224>(data),
            HashFunctionTextualName::Sha256 => calculate_checksum::<sha2::Sha256>(data),
            HashFunctionTextualName::Sha384 => calculate_checksum::<sha2::Sha384>(data),
            HashFunctionTextualName::Sha512 => calculate_checksum::<sha2::Sha512>(data),
            _ => unimplemented!(),
        }
    }

    fn calculate_file_checksum(&self, path: &std::path::Path) -> Result<String> {
        match self.hash_type {
            HashFunctionTextualName::Md2 => calculate_file_checksum::<md2::Md2>(path),
            HashFunctionTextualName::Md5 => calculate_file_checksum::<md5::Md5>(path),
            HashFunctionTextualName::Sha1 => calculate_file_checksum::<sha1_checked::Sha1>(path),
            HashFunctionTextualName::Sha224 => calculate_file_checksum::<sha2::Sha224>(path),
            HashFunctionTextualName::Sha256 => calculate_file_checksum::<sha2::Sha256>(path),
            HashFunctionTextualName::Sha384 => calculate_file_checksum::<sha2::Sha384>(path),
            HashFunctionTextualName::Sha512 => calculate_file_checksum::<sha2::Sha512>(path),
            _ => unimplemented!(),
        }
    }

    pub fn validate_checksum(&self, data: &bytes::Bytes) -> bool {
        let res = self.calculate_checksum(data) == self.checksum;
        if res {
            log::info!("Checksum validation succeeded");
        } else {
            log::info!("Checksum validation failed");
        }
        res
    }

    pub fn validate_file_checksum(&self, file_on_disk: &std::path::Path) -> bool {
        match self.calculate_file_checksum(file_on_disk) {
            Ok(checksum) => checksum == self.checksum,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calulate_ranges_handle_total_size_smaller_than_block_size() {
        let file: PathBuf = "/x".into();
        let chunks = ChunkMetaData::calculate_ranges(5, 10, &file);
        assert_eq!(chunks.len(), 1);
        assert_eq!(
            chunks.first(),
            Some(ChunkMetaData::new(0, 4, "/x".into())).as_ref()
        );

        assert_eq!(Some(5), chunks.first().map(|chunk| chunk.chunk_size()));
    }

    #[test]
    fn calculate_ranges_handles_total_size_equal_block_size() {
        let file: PathBuf = "/x".into();
        let chunks = ChunkMetaData::calculate_ranges(10, 10, &file);
        assert_eq!(chunks.len(), 1);
        assert_eq!(
            chunks.first(),
            Some(ChunkMetaData::new(0, 9, "/x".into())).as_ref()
        );
        assert_eq!(Some(10), chunks.first().map(|chunk| chunk.chunk_size()));
    }

    #[test]
    fn calculate_ranges_handles_total_size_bigger_block_size() {
        let file: PathBuf = "/x".into();
        let chunks = ChunkMetaData::calculate_ranges(15, 10, &file);
        assert_eq!(chunks.len(), 2);
        assert_eq!(
            chunks,
            vec![
                ChunkMetaData::new(0, 9, "/x".into()),
                ChunkMetaData::new(10, 14, "/x".into())
            ]
        );
        assert_eq!(10, chunks[0].chunk_size());
        assert_eq!(5, chunks[1].chunk_size());
    }

    #[test]
    fn validate_checksum() {
        let checksum = CheckSum::new(
            HashFunctionTextualName::Sha256,
            String::from("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"),
        );

        let bytes = bytes::Bytes::from(&b"abc"[..]);
        assert_eq!(checksum.validate_checksum(&bytes), true);
    }
}
