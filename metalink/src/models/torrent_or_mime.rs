use crate::MetalinkError;

/// Enumeration describing the mediatype attribute of
/// metalink:metaurl element according to [RFC5854 Section 4.2.8.2](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.8.2).
#[derive(Debug, PartialEq)]
pub enum TorrentOrMime {
    /// The mediatype is torrent meaning that the url of the metaurl
    /// points to a Bittorrent IRI.
    Torrent,
    /// The mediatype is a mimetype as described in [RFC4288](https://www.rfc-editor.org/rfc/rfc4288)
    Mime(mime::Mime),
}

impl std::fmt::Display for TorrentOrMime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TorrentOrMime::Torrent => write!(f, "torrent"),
            TorrentOrMime::Mime(mime) => write!(f, "{}", mime),
        }
    }
}

impl std::str::FromStr for TorrentOrMime {
    type Err = MetalinkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.to_lowercase() == "torrent" {
            return Ok(Self::Torrent);
        }

        Ok(Self::Mime(s.parse::<mime::Mime>()?))
    }
}
