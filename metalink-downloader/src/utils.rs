use anyhow::Result;
use digest::generic_array::ArrayLength;
use digest::{Digest, OutputSizeUser};
use iana_registry_enums::HashFunctionTextualName;
use std::path::PathBuf;
use std::time::Duration;

pub fn make_http_client(user_agent: String) -> Result<reqwest::Client> {
    Ok(reqwest::ClientBuilder::new()
        .https_only(true)
        //.http2_prior_knowledge()
        .gzip(true)
        .timeout(Duration::from_secs(20))
        .user_agent(user_agent)
        .build()?)
}

#[derive(Debug, PartialEq)]
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
}

#[derive(Debug, PartialEq)]
pub struct CheckSum {
    hash_type: HashFunctionTextualName,
    checksum: String,
}

fn calculate_checksum<D: Digest>(data: &bytes::Bytes) -> String
where
    <D as OutputSizeUser>::OutputSize: std::ops::Add,
    <<D as OutputSizeUser>::OutputSize as std::ops::Add>::Output: ArrayLength<u8>,
{
    format!("{:x}", D::digest(&data))
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
            HashFunctionTextualName::Md2 => calculate_checksum::<md2::Md2>(&data),
            HashFunctionTextualName::Md5 => calculate_checksum::<md5::Md5>(&data),
            HashFunctionTextualName::Sha1 => calculate_checksum::<sha1_checked::Sha1>(&data),
            HashFunctionTextualName::Sha224 => calculate_checksum::<sha2::Sha224>(&data),
            HashFunctionTextualName::Sha256 => calculate_checksum::<sha2::Sha256>(&data),
            HashFunctionTextualName::Sha384 => calculate_checksum::<sha2::Sha384>(&data),
            HashFunctionTextualName::Sha512 => calculate_checksum::<sha2::Sha512>(&data),
            _ => unimplemented!(),
        }
    }

    pub fn validate_checksum(&self, data: &bytes::Bytes) -> bool {
        self.calculate_checksum(&data) == self.checksum
    }
}

pub fn calculate_ranges(total_size: u64, block_size: u64, filename: PathBuf) -> Vec<ChunkMetaData> {
    let mut remaining_size = total_size;
    let mut current_pos = 0;

    let mut ranges: Vec<ChunkMetaData> = Vec::new();
    while remaining_size > block_size {
        ranges.push(ChunkMetaData::new(
            current_pos,
            current_pos + block_size - 1,
            filename.clone(),
        ));
        current_pos += block_size;
        remaining_size -= block_size;
    }
    ranges.push(ChunkMetaData::new(
        current_pos,
        current_pos + remaining_size - 1,
        filename.clone(),
    ));

    ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calulate_ranges_handle_total_size_smaller_than_block_size() {
        let chunks = calculate_ranges(5, 10, "/x".into());
        assert_eq!(chunks.len(), 1);
        assert_eq!(
            chunks.first(),
            Some(ChunkMetaData::new(0, 4, "/x".into())).as_ref()
        );
    }

    #[test]
    fn calculate_ranges_handles_total_size_equal_block_size() {
        let chunks = calculate_ranges(10, 10, "/x".into());
        assert_eq!(chunks.len(), 1);
        assert_eq!(
            chunks.first(),
            Some(ChunkMetaData::new(0, 9, "/x".into())).as_ref()
        );
    }

    #[test]
    fn calculate_ranges_handles_total_size_bigger_block_size() {
        let chunks = calculate_ranges(15, 10, "/x".into());
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
