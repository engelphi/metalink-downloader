use crate::IANARegistryError;

use serde::{Deserialize, Serialize};

// ================================================================================================
/// Represents list of hash function names from
/// [IANA Hash Function Textual Names](https://www.iana.org/assignments/hash-function-text-names/hash-function-text-names.xhtml) registry
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum HashFunctionTextualName {
    /// IANA name for the MD2 algorithm
    #[serde(rename = "md2")]
    Md2,
    /// IANA name for the MD5 algorithm
    #[serde(rename = "md5")]
    Md5,
    /// IANA name for the Sha-1 algorithm
    #[serde(rename = "sha-1")]
    Sha1,
    /// IANA name for the Sha-224 algorithm
    #[serde(rename = "sha-224")]
    Sha224,
    /// IANA name for the Sha-256 algorithm
    #[serde(rename = "sha-256")]
    Sha256,
    /// IANA name for the Sha-384 algorithm
    #[serde(rename = "sha-384")]
    Sha384,
    /// IANA name for the Sha-512 algorithm
    #[serde(rename = "sha-512")]
    Sha512,
    /// IANA name for the Shake128 algorithm
    #[serde(rename = "shake128")]
    Shake128,
    /// IANA name for the Shake256 algorithm
    #[serde(rename = "shake256")]
    Shake256,
}

impl std::str::FromStr for HashFunctionTextualName {
    type Err = IANARegistryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "md2" => Ok(Self::Md2),
            "md5" => Ok(Self::Md5),
            "sha-1" => Ok(Self::Sha1),
            "sha-224" => Ok(Self::Sha224),
            "sha-256" => Ok(Self::Sha256),
            "sha-384" => Ok(Self::Sha384),
            "sha-512" => Ok(Self::Sha512),
            "shake128" => Ok(Self::Shake128),
            "shake256" => Ok(Self::Shake256),
            _ => Err(Self::Err::HashParseError),
        }
    }
}

impl TryFrom<&str> for HashFunctionTextualName {
    type Error = IANARegistryError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "md2" => Ok(Self::Md2),
            "md5" => Ok(Self::Md5),
            "sha-1" => Ok(Self::Sha1),
            "sha-224" => Ok(Self::Sha224),
            "sha-256" => Ok(Self::Sha256),
            "sha-384" => Ok(Self::Sha384),
            "sha-512" => Ok(Self::Sha512),
            "shake128" => Ok(Self::Shake128),
            "shake256" => Ok(Self::Shake256),
            _ => Err(Self::Error::HashParseError),
        }
    }
}

impl From<HashFunctionTextualName> for &'static str {
    fn from(value: HashFunctionTextualName) -> Self {
        match value {
            HashFunctionTextualName::Md2 => "md2",
            HashFunctionTextualName::Md5 => "md5",
            HashFunctionTextualName::Sha1 => "sha-1",
            HashFunctionTextualName::Sha224 => "sha-224",
            HashFunctionTextualName::Sha256 => "sha-256",
            HashFunctionTextualName::Sha384 => "sha-384",
            HashFunctionTextualName::Sha512 => "sha-512",
            HashFunctionTextualName::Shake128 => "shake128",
            HashFunctionTextualName::Shake256 => "shake256",
        }
    }
}

impl std::fmt::Display for HashFunctionTextualName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;

    static HASH_NAMES: Lazy<Vec<(&'static str, HashFunctionTextualName)>> = Lazy::new(|| {
        vec![
            ("md2", HashFunctionTextualName::Md2),
            ("md5", HashFunctionTextualName::Md5),
            ("sha-1", HashFunctionTextualName::Sha1),
            ("sha-224", HashFunctionTextualName::Sha224),
            ("sha-256", HashFunctionTextualName::Sha256),
            ("sha-384", HashFunctionTextualName::Sha384),
            ("sha-512", HashFunctionTextualName::Sha512),
            ("shake128", HashFunctionTextualName::Shake128),
            ("shake256", HashFunctionTextualName::Shake256),
        ]
    });

    #[test]
    fn test_ord() {
        let mut hashes = vec![
            HashFunctionTextualName::Shake256,
            HashFunctionTextualName::Shake128,
            HashFunctionTextualName::Sha512,
            HashFunctionTextualName::Sha384,
            HashFunctionTextualName::Sha256,
            HashFunctionTextualName::Sha224,
            HashFunctionTextualName::Sha1,
            HashFunctionTextualName::Md5,
            HashFunctionTextualName::Md2,
        ];

        assert_eq!(
            hashes.iter().max(),
            Some(HashFunctionTextualName::Shake256).as_ref()
        );

        hashes.sort();
        assert_eq!(
            hashes,
            vec![
                HashFunctionTextualName::Md2,
                HashFunctionTextualName::Md5,
                HashFunctionTextualName::Sha1,
                HashFunctionTextualName::Sha224,
                HashFunctionTextualName::Sha256,
                HashFunctionTextualName::Sha384,
                HashFunctionTextualName::Sha512,
                HashFunctionTextualName::Shake128,
                HashFunctionTextualName::Shake256,
            ]
        );
    }

    #[test]
    fn test_from_str() {
        for (name_str, name_enum) in HASH_NAMES.iter() {
            assert_eq!(Ok(*name_enum), name_str.parse::<HashFunctionTextualName>());
        }
        assert_eq!(
            Err(IANARegistryError::HashParseError),
            "xyz".parse::<HashFunctionTextualName>()
        );
    }

    #[test]
    fn test_try_from() {
        for (name_str, name_enum) in HASH_NAMES.iter() {
            assert_eq!(Ok(*name_enum), HashFunctionTextualName::try_from(*name_str));
        }
        assert_eq!(
            Err(IANARegistryError::HashParseError),
            HashFunctionTextualName::try_from("xyz")
        );
    }

    #[test]
    fn test_from_for_str() {
        for (name_str, name_enum) in HASH_NAMES.iter() {
            let result: &'static str = From::<HashFunctionTextualName>::from(*name_enum);
            assert_eq!(result, *name_str);
        }
    }

    #[test]
    fn test_display() {
        for (name_str, name_enum) in HASH_NAMES.iter() {
            assert_eq!(format!("{}", *name_enum), *name_str);
        }
    }
}
