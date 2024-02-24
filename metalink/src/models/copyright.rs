use serde::{Deserialize, Serialize};

/// Representation of the metalink:copyright element
/// according to [RFC5854 Section 4.2.1](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.1)
#[derive(Debug, Deserialize, Serialize, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;
    use quick_xml::se::to_string;

    #[test]
    fn read_copyright_works() {
        const COPYRIGHT: &str = r#"
            <Copyright>Test</Copyright>
        "#;
        let copyright: Copyright = from_str(COPYRIGHT).unwrap();
        assert_eq!(Copyright::new("Test"), copyright);
    }

    #[test]
    fn write_copyright_works() {
        let copyright = Copyright::new("Test");

        let expected = String::from(r#"<Copyright>Test</Copyright>"#);

        assert_eq!(to_string::<Copyright>(&copyright).unwrap(), expected);
    }
}
