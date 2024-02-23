use serde::{Deserialize, Serialize};
/// Representation of the metalink:publisher field according to
/// [RFC5854 Section 4.2.12](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.12)
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Publisher {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@url", skip_serializing_if = "Option::is_none")]
    url: Option<url::Url>,
}

impl Publisher {
    /// Create a new publisher with the given name
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            url: None,
        }
    }

    /// Create a new publisher with the given name and url
    pub fn new_with_url(name: &str, url: url::Url) -> Self {
        Self {
            name: String::from(name),
            url: Some(url),
        }
    }

    /// Returns the name attribute of the metalink:publisher field
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns the url attribute of the metalink:publisher field
    /// It returns [None] if attribute was not set
    pub fn url(&self) -> Option<&url::Url> {
        self.url.as_ref()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quick_xml::de::from_str;
    use quick_xml::se::to_string;
    use std::str::FromStr;

    #[test]
    fn read_full_publisher() {
        const PUBLISHER: &str = r#"
            <Publisher name="Company Inc." url="https://www.google.com"/>
        "#;

        let publisher: Publisher = from_str(PUBLISHER).unwrap();
        assert_eq!(*publisher.name(), String::from("Company Inc."));
        assert_eq!(
            *publisher.url().unwrap(),
            url::Url::parse("https://www.google.com").unwrap()
        );
    }

    #[test]
    fn read_publisher_with_required_fields() {
        const PUBLISHER: &str = r#"
            <Publisher name="Company Inc."/>
        "#;
        let publisher: Publisher = from_str(PUBLISHER).unwrap();
        assert_eq!(*publisher.name(), String::from("Company Inc."));
        assert_eq!(publisher.url(), None);
    }

    #[test]
    fn write_publisher_with_only_required_fields() {
        let publisher = Publisher::new("Company Inc.");

        let expected = String::from(r#"<Publisher name="Company Inc."/>"#);
        assert_eq!(to_string::<Publisher>(&publisher).unwrap(), expected);
    }

    #[test]
    fn write_publisher_with_url() {
        let publisher = Publisher::new_with_url(
            "Company Inc.",
            url::Url::from_str("https://www.google.com").unwrap(),
        );

        let expected =
            String::from(r#"<Publisher name="Company Inc." url="https://www.google.com/"/>"#);
        assert_eq!(to_string::<Publisher>(&publisher).unwrap(), expected);
    }
}
