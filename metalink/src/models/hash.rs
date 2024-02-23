use iana_registry_enums::HashFunctionTextualName;
use serde::{Deserialize, Serialize};

/// Representation of the metalink:hash element according to
/// [RFC5854 Section 4.2.4](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.4)
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Hash {
    #[serde(rename = "@type")]
    r#type: Option<HashFunctionTextualName>,
    #[serde(rename = "$text")]
    value: String,
}

impl Hash {
    /// Create a new hash field with the given hash_type and the corresponding
    /// hash value
    pub fn new(hash_type: Option<HashFunctionTextualName>, value: &str) -> Self {
        Self {
            r#type: hash_type,
            value: String::from(value),
        }
    }

    /// Returns the type of the hash field
    pub fn hash_type(&self) -> Option<HashFunctionTextualName> {
        self.r#type
    }

    /// Returns the hash value of the hash field
    pub fn value(&self) -> &str {
        self.value.as_ref()
    }
}

#[cfg(test)]
mod tests {
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
}
