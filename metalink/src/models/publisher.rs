use serde::Deserialize;
/// Representation of the metalink:publisher field according to
/// [RFC5854 Section 4.2.12](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.12)
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Publisher {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@url")]
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

impl std::str::FromStr for Publisher {
    type Err = crate::MetalinkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(crate::utils::from_str::<Publisher>(s)?)
    }
}

impl std::convert::TryFrom<&str> for Publisher {
    type Error = crate::MetalinkError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_full_publisher() {
        const PUBLISHER: &str = r#"
            <Publisher name="Company Inc." url="https://www.google.com"/>
        "#;

        let publisher = Publisher::try_from(PUBLISHER).unwrap();
        assert_eq!(
            Publisher::new_with_url(
                "Company Inc.",
                url::Url::parse("https://www.google.com").unwrap()
            ),
            publisher
        );
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
        let publisher: Publisher = PUBLISHER.parse().unwrap();
        assert_eq!(Publisher::new("Company Inc.",), publisher);
        assert_eq!(*publisher.name(), String::from("Company Inc."));
        assert_eq!(publisher.url(), None);
    }
}
