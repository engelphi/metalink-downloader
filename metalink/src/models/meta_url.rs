use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use validator::Validate;

use crate::TorrentOrMime;

/// Representation of the metalink:metaurl element according to
/// [RFC5854 Section 4.2.8](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.8)
#[serde_as]
#[derive(Debug, Deserialize, Validate, PartialEq, Clone)]
pub struct MetaUrl {
    #[validate(range(
        min = 1,
        max = 999999,
        message = "priority needs to be between 1 and 999999"
    ))]
    #[serde(rename = "@priority")]
    priority: Option<u32>,
    // TODO needs validation for valid MIME type or the string torrent of bittorrent urls
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "@mediatype")]
    media_type: TorrentOrMime,
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "$text")]
    url: url::Url,
}

impl MetaUrl {
    /// Create a new meta url element.
    pub fn new(
        url: url::Url,
        media_type: TorrentOrMime,
        priority: Option<u32>,
        name: Option<String>,
    ) -> Self {
        Self {
            url,
            media_type,
            priority,
            name,
        }
    }

    /// Returns priority attribute of the metaurl if the attribute is set
    pub fn priority(&self) -> Option<u32> {
        self.priority
    }

    /// Returns mediatype attribute of the metaurl
    pub fn mediatype(&self) -> &TorrentOrMime {
        &self.media_type
    }

    /// Returns the name attribute of the meta url if the attribute is set
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Returns the url of the meta url
    pub fn url(&self) -> &url::Url {
        &self.url
    }
}

impl std::str::FromStr for MetaUrl {
    type Err = crate::MetalinkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(crate::utils::from_str::<MetaUrl>(s)?)
    }
}

impl std::convert::TryFrom<&str> for MetaUrl {
    type Error = crate::MetalinkError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn read_full_metaurl() {
        const METAURL: &str = r#"
            <metaurl priority="1" mediatype="torrent" name="test/test2/test.tar.gz">https://www.rfc-editor.org/rfc/rfc5854</metaurl>
        "#;

        let meta_url = MetaUrl::try_from(METAURL).unwrap();

        assert_eq!(
            *meta_url.url(),
            url::Url::from_str("https://www.rfc-editor.org/rfc/rfc5854").unwrap()
        );
        assert_eq!(*meta_url.mediatype(), TorrentOrMime::Torrent);
        assert_eq!(meta_url.priority(), Some(1));
        assert_eq!(
            *meta_url.name().unwrap(),
            String::from("test/test2/test.tar.gz")
        );
    }

    #[test]
    fn read_metaurl_with_required_fields() {
        const METAURL: &str = r#"
            <metaurl mediatype="application/json">https://www.rfc-editor.org/rfc/rfc5854</metaurl>
        "#;

        let meta_url: MetaUrl = METAURL.parse().unwrap();

        assert_eq!(
            *meta_url.url(),
            url::Url::from_str("https://www.rfc-editor.org/rfc/rfc5854").unwrap()
        );
        assert_eq!(
            *meta_url.mediatype(),
            TorrentOrMime::Mime(mime::Mime::from_str("application/json").unwrap())
        );
        assert_eq!(meta_url.priority(), None);
        assert_eq!(meta_url.name(), None);
    }
}
