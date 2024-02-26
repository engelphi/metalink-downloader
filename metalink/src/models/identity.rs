use serde::Deserialize;

/// Representation of the metalink:identity element
/// according to [RFC5854 Section 4.2.5](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.5)
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Identity {
    #[serde(rename = "$text")]
    identity: String,
}

impl Identity {
    /// Create a new identity element
    pub fn new(identity: &str) -> Self {
        Self {
            identity: String::from(identity),
        }
    }

    /// Returns the identity
    pub fn identity(&self) -> &str {
        self.identity.as_ref()
    }
}

impl std::str::FromStr for Identity {
    type Err = crate::MetalinkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(crate::utils::from_str::<Identity>(s)?)
    }
}

impl std::convert::TryFrom<&str> for Identity {
    type Error = crate::MetalinkError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_identity_works() {
        const IDENTITY: &str = r#"
            <Identity>Test</Identity>
        "#;
        let identity = Identity::try_from(IDENTITY).unwrap();
        assert_eq!(Identity::new("Test"), identity);
        assert_eq!(identity.identity(), "Test");
    }
}
