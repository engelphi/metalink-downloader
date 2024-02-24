use serde::{Deserialize, Serialize};

/// Representation of the metalink:description element
/// according to [RFC5854 Section 4.2.2](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.2)
#[derive(Debug, Deserialize, Serialize, PartialEq)]
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
    use quick_xml::de::from_str;
    use quick_xml::se::to_string;

    #[test]
    fn read_description_works() {
        const DESCRIPTION: &str = r#"
            <Description>Test</Description>
        "#;
        let description: Description = from_str(DESCRIPTION).unwrap();
        assert_eq!(Description::new("Test"), description);
    }

    #[test]
    fn write_description_works() {
        let description = Description::new("Test");

        let expected = String::from(r#"<Description>Test</Description>"#);

        assert_eq!(to_string::<Description>(&description).unwrap(), expected);
    }
}
