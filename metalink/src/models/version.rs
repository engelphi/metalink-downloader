use serde::Deserialize;

/// Representation of the metalink:version element
/// according to [RFC5854 Section 4.2.17](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.17)
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Version {
    #[serde(rename = "$text")]
    version: String,
}

impl Version {
    /// Create a new version element
    pub fn new(version: &str) -> Self {
        Self {
            version: String::from(version),
        }
    }

    /// Returns the version
    pub fn version(&self) -> &str {
        self.version.as_ref()
    }
}

impl std::str::FromStr for Version {
    type Err = crate::MetalinkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(crate::utils::from_str::<Version>(s)?)
    }
}

impl std::convert::TryFrom<&str> for Version {
    type Error = crate::MetalinkError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_version_works() {
        const VERSION: &str = r#"
            <Version>1.0.0</Version>
        "#;
        let version = Version::try_from(VERSION).unwrap();
        assert_eq!(Version::new("1.0.0"), version);
        assert_eq!(version.version(), "1.0.0");
    }
}
