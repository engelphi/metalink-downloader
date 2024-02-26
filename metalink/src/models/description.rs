use serde::Deserialize;

/// Representation of the metalink:description element
/// according to [RFC5854 Section 4.2.2](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.2)
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Description {
    #[serde(rename = "$text")]
    description: String,
}

impl Description {
    /// Create a new description element
    pub fn new(description: &str) -> Self {
        Self {
            description: String::from(description),
        }
    }

    /// Returns the description
    pub fn description(&self) -> &str {
        self.description.as_ref()
    }
}

impl std::str::FromStr for Description {
    type Err = crate::MetalinkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(crate::utils::from_str::<Description>(s)?)
    }
}

impl std::convert::TryFrom<&str> for Description {
    type Error = crate::MetalinkError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_description_works() {
        const DESCRIPTION: &str = r#"
            <Description>Test</Description>
        "#;
        let description = Description::try_from(DESCRIPTION).unwrap();
        assert_eq!(Description::new("Test"), description);
        assert_eq!(description.description(), "Test");
    }
}
