use crate::Result;
use anyhow::anyhow;
use digest::generic_array::ArrayLength;
use digest::{Digest, OutputSizeUser};
use iana_registry_enums::HashFunctionTextualName;
use std::io::{Read, Seek};
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::usize;

use crate::MetalinkDownloadError;

pub fn make_http_client(user_agent: String) -> Result<reqwest::Client> {
    Ok(reqwest::ClientBuilder::new()
        .https_only(true)
        //.http2_prior_knowledge()
        .gzip(true)
        .timeout(Duration::from_secs(20))
        .user_agent(user_agent)
        .build()?)
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
    Ok(format!("{:x}", digest))
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
        self.calculate_checksum(data) == self.checksum
    }

    pub fn validate_file_checksum(&self, file_on_disk: &std::path::Path) -> bool {
        match self.calculate_file_checksum(file_on_disk) {
            Ok(checksum) => checksum == self.checksum,
            _ => false,
        }
    }
}

pub fn to_chunk_metadata(
    pieces: &metalink::Pieces,
    filename: &Path,
    total_size: u64,
) -> Result<Vec<ChunkMetaData>> {
    let mut ranges = calculate_ranges(total_size, pieces.length(), filename);

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

pub fn calculate_ranges(total_size: u64, block_size: u64, filename: &Path) -> Vec<ChunkMetaData> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calulate_ranges_handle_total_size_smaller_than_block_size() {
        let file: PathBuf = "/x".into();
        let chunks = calculate_ranges(5, 10, &file);
        assert_eq!(chunks.len(), 1);
        assert_eq!(
            chunks.first(),
            Some(ChunkMetaData::new(0, 4, "/x".into())).as_ref()
        );
    }

    #[test]
    fn calculate_ranges_handles_total_size_equal_block_size() {
        let file: PathBuf = "/x".into();
        let chunks = calculate_ranges(10, 10, &file);
        assert_eq!(chunks.len(), 1);
        assert_eq!(
            chunks.first(),
            Some(ChunkMetaData::new(0, 9, "/x".into())).as_ref()
        );
    }

    #[test]
    fn calculate_ranges_handles_total_size_bigger_block_size() {
        let file: PathBuf = "/x".into();
        let chunks = calculate_ranges(15, 10, &file);
        assert_eq!(chunks.len(), 2);
        assert_eq!(
            chunks,
            vec![
                ChunkMetaData::new(0, 9, "/x".into()),
                ChunkMetaData::new(10, 14, "/x".into())
            ]
        );
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
