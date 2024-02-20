use iana_registry_enums::{HashFunctionTextualName, OperatingSystemName};

use chrono::{DateTime, Utc};
use serde::Deserialize;
use validator::Validate;

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

#[derive(Debug, Deserialize, PartialEq)]
pub struct Hash {
    #[serde(rename = "@type")]
    r#type: Option<HashFunctionTextualName>,
    #[serde(rename = "$text")]
    value: String,
}

impl Hash {
    pub fn hash_type(&self) -> Option<HashFunctionTextualName> {
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

#[derive(Debug, Deserialize, Validate)]
pub struct MetaUrl {
    #[validate(range(
        min = 1,
        max = 999999,
        message = "priority needs to be between 1 and 999999"
    ))]
    #[serde(rename = "@priority")]
    priority: Option<u32>,
    // TODO needs validation for valid MIME type or the string torrent of bittorrent urls
    #[serde(rename = "@mediatype")]
    media_type: String,
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "$text")]
    url: url::Url,
}

impl MetaUrl {
    pub fn priority(&self) -> Option<u32> {
        self.priority.clone()
    }

    pub fn mediatype(&self) -> &String {
        &self.media_type
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn url(&self) -> &url::Url {
        &self.url
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct Pieces {
    #[serde(rename = "@type")]
    r#type: HashFunctionTextualName,
    #[serde(rename = "@length")]
    length: u64,
    #[serde(default, rename = "$value")]
    hashes: Vec<Hash>,
}

impl Pieces {
    pub fn hash_type(&self) -> HashFunctionTextualName {
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
pub struct Publisher {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@url")]
    url: Option<url::Url>,
}

impl Publisher {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn url(&self) -> Option<&url::Url> {
        self.url.as_ref()
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct Signature {
    // TODO add validation: needs to be MIME type describing the signature type
    #[serde(rename = "@mediatype")]
    media_type: String,
    #[serde(rename = "$text")]
    signature: String,
}

impl Signature {
    pub fn media_type(&self) -> &String {
        &self.media_type
    }

    pub fn signature(&self) -> &String {
        &self.signature
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct OS {
    #[serde(rename = "$text")]
    name: OperatingSystemName,
}

#[derive(Debug, Deserialize)]
pub struct File {
    #[serde(rename = "@name")]
    name: String,
    copyright: Option<String>,
    description: Option<String>,
    hash: Option<Vec<Hash>>,
    identity: Option<String>,
    language: Option<Vec<String>>,
    logo: Option<url::Url>,
    metaurl: Option<Vec<MetaUrl>>,
    // TODO needs validation: values need to be IANA registry "Operating System Names"
    os: Option<Vec<OS>>,
    pieces: Option<Pieces>,
    publisher: Option<Publisher>,
    signature: Option<Signature>,
    size: Option<u64>,
    urls: Option<Vec<FileUrl>>,
    version: Option<String>,
}

impl File {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn copyright(&self) -> Option<&String> {
        self.copyright.as_ref()
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn hashes(&self) -> Option<&Vec<Hash>> {
        self.hash.as_ref()
    }

    pub fn identity(&self) -> Option<&String> {
        self.identity.as_ref()
    }

    pub fn languages(&self) -> Option<&Vec<String>> {
        self.language.as_ref()
    }

    pub fn logo(&self) -> Option<&url::Url> {
        self.logo.as_ref()
    }

    pub fn meta_urls(&self) -> Option<&Vec<MetaUrl>> {
        self.metaurl.as_ref()
    }

    pub fn oses(&self) -> Option<&Vec<OS>> {
        self.os.as_ref()
    }

    pub fn pieces(&self) -> Option<&Pieces> {
        self.pieces.as_ref()
    }

    pub fn publisher(&self) -> Option<&Publisher> {
        self.publisher.as_ref()
    }

    pub fn signature(&self) -> Option<&Signature> {
        self.signature.as_ref()
    }

    pub fn size(&self) -> Option<u64> {
        self.size.clone()
    }

    pub fn urls(&self) -> Option<&Vec<FileUrl>> {
        self.urls.as_ref()
    }

    pub fn version(&self) -> Option<&String> {
        self.version.as_ref()
    }
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

        assert_eq!(hash.hash_type(), Some(HashFunctionTextualName::Sha1));
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
        assert_eq!(pieces.hash_type(), HashFunctionTextualName::Sha1);
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

    #[test]
    fn read_file() {
        const FILE: &str = r#"
            <file name="abc/def">
                <hash type="sha-1">abc</hash>
                <hash type="sha-256">def</hash>
                <hash type="sha-512">ghi</hash>
                <description>Description</description>
                <copyright>Copyright</copyright>
                <identity>Test</identity>
                <language>German</language>
                <language>USA</language>
                <logo>https://www.google.com/images/branding/googlelogo/1x/googlelogo_light_color_272x92dp.png</logo>
                <metaurl priority="1" mediatype="torrent" name="test/test2/test.tar.gz">https://www.rfc-editor.org/rfc/rfc5854</metaurl>
                <os>MACOS</os>
                <os>LINUX</os>
                <pieces type="sha-1" length="50">
                    <hash>abc</hash>
                    <hash>def</hash>
                </pieces>
                <publisher name="Company Inc." url="https://www.google.com" />
                <signature mediatype="application/pgp-signature">
                   -----BEGIN PGP SIGNATURE-----
                   Version: GnuPG v1.4.10 (GNU/Linux)

                   iEYEABECAAYFAkrxdXQACgkQeOEcayedXJHqFwCfd1p/HhRf/iDvYhvFbTrQPz+p
                   p3oAoO9lKHoOqOE0EMB3zmMcLoYUrNkg
                   =ggAf
                   -----END PGP SIGNATURE-----
                </signature>
                <size>
                    50
                </size>
                <url priority="1" location="de">https://www.google.de</url>
                <url priority="1" location="us">https://www.google.com</url>
                <version>1.0.0</version>
            </file>
        "#;

        let file: File = from_str(FILE).unwrap();
        assert_eq!(*file.name(), String::from("abc/def"));
        println!("{:#?}", file);
    }
}
