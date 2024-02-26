/// Enumeration describing the mediatype attribute of
/// metalink:metaurl element according to [RFC5854 Section 4.2.8.2](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.8.2).
#[derive(Debug, PartialEq, Clone)]
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
    type Err = crate::MetalinkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.to_lowercase() == "torrent" {
            return Ok(Self::Torrent);
        }

        Ok(Self::Mime(s.parse::<mime::Mime>()?))
    }
}

impl std::convert::TryFrom<&str> for TorrentOrMime {
    type Error = crate::MetalinkError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let torrent = TorrentOrMime::Torrent;
        let mime = TorrentOrMime::Mime("application/json".parse::<mime::Mime>().unwrap());

        assert_eq!(format!("{}", torrent), "torrent");
        assert_eq!(format!("{}", mime), "application/json");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(
            "torrent".parse::<TorrentOrMime>().unwrap(),
            TorrentOrMime::Torrent
        );
        assert_eq!(
            "application/json".parse::<TorrentOrMime>().unwrap(),
            TorrentOrMime::Mime("application/json".parse::<mime::Mime>().unwrap())
        );
    }

    #[test]
    fn test_try_from() {
        assert_eq!(
            TorrentOrMime::try_from("torrent").unwrap(),
            TorrentOrMime::Torrent
        );
        assert_eq!(
            TorrentOrMime::try_from("application/json").unwrap(),
            TorrentOrMime::Mime("application/json".parse::<mime::Mime>().unwrap())
        );
    }
}
