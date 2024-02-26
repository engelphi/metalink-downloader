use iana_registry_enums::HashFunctionTextualName;
use serde::Deserialize;

use crate::Hash;

/// Representation of the metalink:pieces element according to
/// [RFC5854 Section 4.1.3](https://www.rfc-editor.org/rfc/rfc5854#section-4.1.3)
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Pieces {
    #[serde(rename = "@type")]
    r#type: HashFunctionTextualName,
    #[serde(rename = "@length")]
    length: u64,
    #[serde(default, rename = "$value")]
    hashes: Vec<Hash>,
}

impl Pieces {
    /// Create a new metalink:pieces element where
    /// hash_type describes the hash algorithm used for each of piece hashes,
    /// length describes length of each of the pieces and hashes is the list
    /// of the piece-wise hashes.
    pub fn new(hash_type: HashFunctionTextualName, length: u64, hashes: Vec<Hash>) -> Self {
        Self {
            r#type: hash_type,
            length,
            hashes,
        }
    }

    /// Returns the type of the hash used for the piece-wise hashes
    pub fn hash_type(&self) -> HashFunctionTextualName {
        self.r#type
    }

    /// Returns the length of the pieces
    pub fn length(&self) -> u64 {
        self.length
    }

    /// Returns the list of piece-wise hashes
    pub fn hashes(&self) -> &Vec<Hash> {
        &self.hashes
    }
}

impl std::str::FromStr for Pieces {
    type Err = crate::MetalinkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(crate::utils::from_str::<Pieces>(s)?)
    }
}

impl std::convert::TryFrom<&str> for Pieces {
    type Error = crate::MetalinkError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_pieces() {
        const PIECES: &str = r#"
            <pieces type="sha-1" length="50">
                <hash>abc</hash>
                <hash>def</hash>
            </pieces>
        "#;

        let pieces = Pieces::try_from(PIECES).unwrap();
        assert_eq!(pieces.hash_type(), HashFunctionTextualName::Sha1);
        assert_eq!(pieces.length(), 50);
        assert_eq!(
            *pieces.hashes(),
            vec![Hash::new(None, "abc"), Hash::new(None, "def")]
        );
    }
}
