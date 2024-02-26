use std::str::FromStr;

use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{utils::*, MetalinkError};
use crate::{File, Origin};

/// Representation of the metalink:metalink element according to
/// [RFC5854 Section 4.1.1](https://www.rfc-editor.org/rfc/rfc5854#section-4.1.1)
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Metalink {
    generator: Option<String>,
    origin: Option<Origin>,
    #[serde(default)]
    #[serde(with = "rfc3339_to_datetime_utc")]
    published: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default)]
    #[serde(with = "rfc3339_to_datetime_utc")]
    updated: Option<chrono::DateTime<chrono::Utc>>,
    file: Vec<File>,
}

impl Metalink {
    /// Load a metalink from the file specified by file_path
    pub fn load_from_file<P: AsRef<std::path::Path>>(
        file_path: P,
    ) -> Result<Metalink, MetalinkError> {
        Ok(quick_xml::de::from_reader(std::io::BufReader::new(
            std::fs::File::open(file_path).context("Failed to open file")?,
        ))?)
    }

    /// Returns the value of the metalink:generator element
    /// if the field exists. See [RFC5854 Section 4.2.3](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.3)
    pub fn generator(&self) -> Option<&String> {
        self.generator.as_ref()
    }

    /// Returns the metalink:origin element if the field
    /// exists. See [RFC5854 Section 4.2.9](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.9)
    pub fn origin(&self) -> Option<&Origin> {
        self.origin.as_ref()
    }

    /// Returns the value of the metalink:published element
    /// if the field exists. See [RFC5854 Section 4.2.11](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.11)
    pub fn published(&self) -> Option<&DateTime<Utc>> {
        self.published.as_ref()
    }

    /// Returns the value of the metalink:updated element
    /// if the field exists. See [RFC5854 Section 4.2.15](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.15)
    pub fn updated(&self) -> Option<&DateTime<Utc>> {
        self.updated.as_ref()
    }

    /// Returns the list of metalink:file elements.
    /// See [RFC5854 Section 4.1.2](https://www.rfc-editor.org/rfc/rfc5854#section-4.1.2)
    pub fn files(&self) -> &Vec<File> {
        &self.file
    }
}

impl FromStr for Metalink {
    type Err = crate::MetalinkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(crate::utils::from_str::<Metalink>(s)?)
    }
}

// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    // use std::str::FromStr;
    //
    use super::*;
    use crate::{FileBuilder, FileUrl, Hash, Pieces};
    use chrono::DateTime;
    use iana_registry_enums::HashFunctionTextualName;

    #[test]
    fn parse_full_metalink() {
        const METALINK: &str = r#"
            <metalink>
                <generator>TestGenerator</generator>
                <published>2010-05-01T12:15:02Z</published>
                <updated>2010-05-02T12:15:02Z</updated>
                <origin dynamic="true">https://www.google.com</origin>
                <file name="abc/def">
                    <hash type="sha-1">abc</hash>
                    <hash type="sha-256">def</hash>
                    <hash type="sha-512">ghi</hash>
                    <pieces type="sha-1" length="50">
                        <hash>abc</hash>
                        <hash>def</hash>
                    </pieces>
                    <url priority="1" location="de">https://www.google.de</url>
                </file>
                <file name="def/ghi">
                    <hash type="sha-1">abc</hash>
                    <hash type="sha-256">def</hash>
                    <hash type="sha-512">ghi</hash>
                    <pieces type="sha-1" length="50">
                        <hash>abc</hash>
                        <hash>def</hash>
                    </pieces>
                    <url priority="1" location="de">https://www.google.de</url>
                </file>
            </metalink>
        "#;
        let metalink: Metalink = METALINK.parse().unwrap();

        let expected_generator = Some(String::from("TestGenerator"));
        let expected_published = Some(DateTime::from_naive_utc_and_offset(
            DateTime::parse_from_rfc3339("2010-05-01T12:15:02Z")
                .unwrap()
                .naive_utc(),
            Utc,
        ));
        let expected_updated = Some(DateTime::from_naive_utc_and_offset(
            DateTime::parse_from_rfc3339("2010-05-02T12:15:02Z")
                .unwrap()
                .naive_utc(),
            Utc,
        ));
        let expected_origin = Some(Origin::new(
            Some(true),
            url::Url::parse("https://www.google.com").unwrap(),
        ));

        let expected_files = vec![
            FileBuilder::new()
                .with_name("abc/def")
                .with_hashes(vec![
                    Hash::new(Some(HashFunctionTextualName::Sha1), "abc"),
                    Hash::new(Some(HashFunctionTextualName::Sha256), "def"),
                    Hash::new(Some(HashFunctionTextualName::Sha512), "ghi"),
                ])
                .with_pieces(Pieces::new(
                    HashFunctionTextualName::Sha1,
                    50,
                    vec![Hash::new(None, "abc"), Hash::new(None, "def")],
                ))
                .with_urls(vec![FileUrl::new(
                    url::Url::parse("https://www.google.de").unwrap(),
                    Some(1),
                    Some(isocountry::CountryCode::DEU),
                )])
                .build()
                .unwrap(),
            FileBuilder::new()
                .with_name("def/ghi")
                .with_hashes(vec![
                    Hash::new(Some(HashFunctionTextualName::Sha1), "abc"),
                    Hash::new(Some(HashFunctionTextualName::Sha256), "def"),
                    Hash::new(Some(HashFunctionTextualName::Sha512), "ghi"),
                ])
                .with_pieces(Pieces::new(
                    HashFunctionTextualName::Sha1,
                    50,
                    vec![Hash::new(None, "abc"), Hash::new(None, "def")],
                ))
                .with_urls(vec![FileUrl::new(
                    url::Url::parse("https://www.google.de").unwrap(),
                    Some(1),
                    Some(isocountry::CountryCode::DEU),
                )])
                .build()
                .unwrap(),
        ];

        let expected = Metalink {
            generator: expected_generator.clone(),
            published: expected_published.clone(),
            updated: expected_updated.clone(),
            origin: expected_origin.clone(),
            file: expected_files.clone(),
        };

        assert_eq!(metalink, expected);
        assert_eq!(metalink.generator(), expected_generator.as_ref());
        assert_eq!(metalink.published(), expected_published.as_ref());
        assert_eq!(metalink.updated(), expected_updated.as_ref());
        assert_eq!(metalink.origin(), expected_origin.as_ref());
        assert_eq!(*metalink.files(), expected_files);
    }

    #[test]
    fn parse_minimal_metalink() {
        const METALINK: &str = r#"
            <metalink>
                <file name="abc/def">
                    <hash type="sha-1">abc</hash>
                    <hash type="sha-256">def</hash>
                    <hash type="sha-512">ghi</hash>
                    <pieces type="sha-1" length="50">
                        <hash>abc</hash>
                        <hash>def</hash>
                    </pieces>
                    <url priority="1" location="de">https://www.google.de</url>
                </file>
            </metalink>
        "#;
        let metalink: Metalink = from_str(METALINK).unwrap();
        let expected_files = vec![FileBuilder::new()
            .with_name("abc/def")
            .with_hashes(vec![
                Hash::new(Some(HashFunctionTextualName::Sha1), "abc"),
                Hash::new(Some(HashFunctionTextualName::Sha256), "def"),
                Hash::new(Some(HashFunctionTextualName::Sha512), "ghi"),
            ])
            .with_pieces(Pieces::new(
                HashFunctionTextualName::Sha1,
                50,
                vec![Hash::new(None, "abc"), Hash::new(None, "def")],
            ))
            .with_urls(vec![FileUrl::new(
                url::Url::parse("https://www.google.de").unwrap(),
                Some(1),
                Some(isocountry::CountryCode::DEU),
            )])
            .build()
            .unwrap()];
        let expected = Metalink {
            generator: None,
            published: None,
            updated: None,
            origin: None,
            file: expected_files.clone(),
        };

        assert_eq!(metalink, expected);
        assert_eq!(metalink.generator(), None);
        assert_eq!(metalink.published(), None);
        assert_eq!(metalink.updated(), None);
        assert_eq!(metalink.origin(), None);
        assert_eq!(*metalink.files(), expected_files);
    }
}
