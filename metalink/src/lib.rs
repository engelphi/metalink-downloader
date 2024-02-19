use chrono::{DateTime, Utc};
use serde::Deserialize;
use validator::{Validate, ValidationError};

// ----------------------------------------------------------------------------

mod rfc3339_to_datetime_utc {
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let offset_time = DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)?;
        Ok(Some(DateTime::from_naive_utc_and_offset(
            offset_time.naive_utc(),
            Utc,
        )))
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum HashAlgorithm {
    #[serde(rename = "md5")]
    Md5,
    #[serde(rename = "sha-1")]
    Sha1,
    #[serde(rename = "sha-256")]
    Sha256,
    #[serde(rename = "$text")]
    Other(String),
}

// ----------------------------------------------------------------------------

#[derive(Debug, Deserialize, PartialEq)]
pub struct Hash {
    #[serde(rename = "@type")]
    r#type: Option<HashAlgorithm>,
    #[serde(rename = "$text")]
    value: String,
}

impl Hash {
    pub fn hash_type(&self) -> Option<HashAlgorithm> {
        self.r#type.clone()
    }

    pub fn value(&self) -> &str {
        self.value.as_ref()
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, Deserialize, Validate)]
pub struct FileUrl {
    #[validate(range(
        min = 1,
        max = 999999,
        message = "priority needs to be between 1 and 999999"
    ))]
    #[serde(rename = "@priority")]
    priority: Option<u32>,
    #[validate(length(
        equal = 2,
        message = "location needs to be a two character country code"
    ))]
    #[serde(rename = "@location")]
    location: Option<String>,
    #[serde(rename = "$text")]
    url: url::Url,
}

impl FileUrl {
    pub fn priority(&self) -> Option<u32> {
        self.priority.clone()
    }

    pub fn location(&self) -> Option<String> {
        self.location.clone()
    }

    pub fn url(&self) -> url::Url {
        self.url.clone()
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct Pieces {
    #[serde(rename = "@type")]
    r#type: HashAlgorithm,
    #[serde(rename = "@length")]
    length: u64,
    #[serde(default, rename = "$value")]
    hashes: Vec<Hash>,
}

impl Pieces {
    pub fn hash_type(&self) -> HashAlgorithm {
        self.r#type.clone()
    }

    pub fn length(&self) -> u64 {
        self.length
    }

    pub fn hashes(&self) -> &Vec<Hash> {
        &self.hashes
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct File {
    #[serde(rename = "@name")]
    name: String,
    description: Option<String>,
    pieces: Option<Pieces>,
}

// ----------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct Origin {
    #[serde(rename = "@dynamic")]
    dynamic: Option<bool>,
    #[serde(rename = "$text")]
    url: url::Url,
}

impl Origin {
    pub fn is_dynamic(&self) -> bool {
        match self.dynamic {
            Some(true) => true,
            _ => false,
        }
    }

    pub fn url(&self) -> &url::Url {
        &self.url
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct Metalink {
    generator: Option<String>,
    origin: Option<Origin>,
    #[serde(with = "rfc3339_to_datetime_utc")]
    published: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(with = "rfc3339_to_datetime_utc")]
    updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "$value")]
    files: Vec<File>,
}

impl Metalink {
    pub fn generator(&self) -> Option<&String> {
        self.generator.as_ref()
    }

    pub fn origin(&self) -> Option<&Origin> {
        self.origin.as_ref()
    }

    pub fn published(&self) -> Option<&DateTime<Utc>> {
        self.published.as_ref()
    }

    pub fn updated(&self) -> Option<&DateTime<Utc>> {
        self.updated.as_ref()
    }

    pub fn files(&self) -> &Vec<File> {
        &self.files
    }
}

// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn read_hash_with_type() {
        const HASH: &str = r#"
            <hash type="sha-1">abc</hash>
        "#;

        let hash: Hash = from_str(HASH).unwrap();

        assert_eq!(hash.hash_type(), Some(HashAlgorithm::Sha1));
        assert_eq!(hash.value(), String::from("abc"));
    }

    #[test]
    fn read_hash_without_type() {
        const HASH: &str = r#"
            <hash>abc</hash>
        "#;

        let hash: Hash = from_str(HASH).unwrap();

        assert_eq!(hash.hash_type(), None);
        assert_eq!(hash.value(), String::from("abc"));
    }

    #[test]
    fn read_file_url_without_attributes() {
        const URL: &str = r#"
            <url>https://www.rfc-editor.org/rfc/rfc5854</url>
        "#;

        let url: FileUrl = from_str(URL).unwrap();
        assert_eq!(url.location(), None);
        assert_eq!(url.priority(), None);
        assert_eq!(
            url.url(),
            url::Url::from_str("https://www.rfc-editor.org/rfc/rfc5854").unwrap()
        );
        url.validate().unwrap();
    }

    #[test]
    fn read_file_url_with_attributes() {
        const URL: &str = r#"
            <url priority="1" location="us">https://www.rfc-editor.org/rfc/rfc5854</url>
        "#;

        let url: FileUrl = from_str(URL).unwrap();
        assert_eq!(url.location(), Some(String::from("us")));
        assert_eq!(url.priority(), Some(1));
        assert_eq!(
            url.url(),
            url::Url::from_str("https://www.rfc-editor.org/rfc/rfc5854").unwrap()
        );
        url.validate().unwrap();
    }

    #[test]
    fn read_pieces() {
        const PIECES: &str = r#"
            <pieces type="sha-1" length="50">
                <hash>abc</hash>
                <hash>def</hash>
            </pieces>
        "#;

        let pieces: Pieces = from_str(PIECES).unwrap();
        assert_eq!(pieces.hash_type(), HashAlgorithm::Sha1);
        assert_eq!(pieces.length(), 50);
        assert_eq!(
            *pieces.hashes(),
            vec![
                Hash {
                    r#type: None,
                    value: String::from("abc")
                },
                Hash {
                    r#type: None,
                    value: String::from("def")
                }
            ]
        );
    }
}
