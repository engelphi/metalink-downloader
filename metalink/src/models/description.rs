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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::from_str;

    #[test]
    fn read_description_works() {
        const DESCRIPTION: &str = r#"
            <Description>Test</Description>
        "#;
        let description: Description = from_str(DESCRIPTION).unwrap();
        assert_eq!(Description::new("Test"), description);
        assert_eq!(description.description(), "Test");
    }
}
