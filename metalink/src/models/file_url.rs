use serde::Deserialize;
use validator::Validate;

/// Representation of the metalink:url element according to
/// [RFC5854 Section 4.2.16](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.16)
#[derive(Debug, Deserialize, Validate, PartialEq, Clone)]
pub struct FileUrl {
    #[validate(range(
        min = 1,
        max = 999999,
        message = "priority needs to be between 1 and 999999"
    ))]
    #[serde(rename = "@priority")]
    priority: Option<u32>,
    #[serde(rename = "@location")]
    location: Option<isocountry::CountryCode>,
    #[serde(rename = "$text")]
    url: url::Url,
}

impl FileUrl {
    /// Create a new FileUrl object
    /// The priority -- if set -- needs to be a value between 1 and 999999.
    /// The location is a ISO3166-1 alpha-2 two letter country code for the
    /// geographical location of the physical server.
    pub fn new(
        url: url::Url,
        priority: Option<u32>,
        location: Option<isocountry::CountryCode>,
    ) -> Self {
        Self {
            url,
            priority,
            location,
        }
    }

    /// Returns the priority attribute of the url element if set,
    /// lower means higher priority
    pub fn priority(&self) -> Option<u32> {
        self.priority
    }

    /// Returns the location attribute of the url element if set
    pub fn location(&self) -> Option<&isocountry::CountryCode> {
        self.location.as_ref()
    }

    /// Returns the url stored in the url element
    pub fn url(&self) -> url::Url {
        self.url.clone()
    }
}

impl std::str::FromStr for FileUrl {
    type Err = crate::MetalinkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(crate::utils::from_str::<FileUrl>(s)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn read_file_url_without_attributes() {
        const URL: &str = r#"
            <url>https://www.rfc-editor.org/rfc/rfc5854</url>
        "#;

        let url: FileUrl = URL.parse().unwrap();
        assert_eq!(url.location(), None);
        assert_eq!(url.priority(), None);
        assert_eq!(
            url.url(),
            url::Url::from_str("https://www.rfc-editor.org/rfc/rfc5854").unwrap()
        );
        url.validate().unwrap();
    }

    #[test]
    fn read_file_url_with_attributes() {
        const URL: &str = r#"
            <url priority="1" location="us">https://www.rfc-editor.org/rfc/rfc5854</url>
        "#;

        let url: FileUrl = URL.parse().unwrap();
        assert_eq!(*url.location().unwrap(), isocountry::CountryCode::USA);
        assert_eq!(url.priority(), Some(1));
        assert_eq!(
            url.url(),
            url::Url::from_str("https://www.rfc-editor.org/rfc/rfc5854").unwrap()
        );
        url.validate().unwrap();
    }
}
