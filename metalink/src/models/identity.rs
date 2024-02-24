use serde::{Deserialize, Serialize};

/// Representation of the metalink:identity element
/// according to [RFC5854 Section 4.2.5](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.5)
#[derive(Debug, Deserialize, Serialize, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;
    use quick_xml::se::to_string;

    #[test]
    fn read_identity_works() {
        const IDENTITY: &str = r#"
            <Identity>Test</Identity>
        "#;
        let identity: Identity = from_str(IDENTITY).unwrap();
        assert_eq!(Identity::new("Test"), identity);
    }

    #[test]
    fn write_identity_works() {
        let identity = Identity::new("Test");

        let expected = String::from(r#"<Identity>Test</Identity>"#);

        assert_eq!(to_string::<Identity>(&identity).unwrap(), expected);
    }
}
