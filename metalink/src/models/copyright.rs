use serde::Deserialize;

/// Representation of the metalink:copyright element
/// according to [RFC5854 Section 4.2.1](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.1)
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Copyright {
    #[serde(rename = "$text")]
    copyright: String,
}

impl Copyright {
    /// Create a new copyright element
    pub fn new(copyright: &str) -> Self {
        Self {
            copyright: String::from(copyright),
        }
    }

    /// Returns the description
    pub fn copyright(&self) -> &str {
        self.copyright.as_ref()
    }
}

impl std::str::FromStr for Copyright {
    type Err = crate::MetalinkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(crate::utils::from_str::<Copyright>(s)?)
    }
}

impl std::convert::TryFrom<&str> for Copyright {
    type Error = crate::MetalinkError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_copyright_works() {
        const COPYRIGHT: &str = r#"
            <Copyright>Test</Copyright>
        "#;
        let copyright = Copyright::try_from(COPYRIGHT).unwrap();
        assert_eq!(Copyright::new("Test"), copyright);
        assert_eq!(copyright.copyright(), "Test");
    }
}
