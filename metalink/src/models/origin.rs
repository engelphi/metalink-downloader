use serde::Deserialize;

/// Representation of the metalink:origin element according to
/// [RFC5854 Section 4.2.9](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.9)
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Origin {
    #[serde(rename = "@dynamic")]
    dynamic: Option<bool>,
    #[serde(rename = "$text")]
    url: url::Url,
}

impl Origin {
    /// Construct a new metalink:origin field
    pub fn new(dynamic: Option<bool>, url: url::Url) -> Self {
        Self { dynamic, url }
    }

    /// Returns whether the dynamic is set and true
    pub fn is_dynamic(&self) -> bool {
        matches!(self.dynamic, Some(true))
    }

    /// Returns the url of the metalink:origin element
    pub fn url(&self) -> &url::Url {
        &self.url
    }
}

impl std::str::FromStr for Origin {
    type Err = crate::MetalinkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(crate::utils::from_str::<Origin>(s)?)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn read_origin_with_dynamic_set_to_true() {
        const DYNAMIC: &str = r#"
            <origin dynamic="true">https://www.google.com</origin>
        "#;

        let origin: Origin = DYNAMIC.parse().unwrap();
        assert_eq!(origin.is_dynamic(), true);
        assert_eq!(
            *origin.url(),
            url::Url::from_str("https://www.google.com").unwrap()
        );
    }

    #[test]
    fn read_origin_with_dynamic_set_to_false() {
        const DYNAMIC: &str = r#"
            <origin dynamic="false">https://www.google.com</origin>
        "#;

        let origin: Origin = DYNAMIC.parse().unwrap();
        assert_eq!(origin.is_dynamic(), false);
        assert_eq!(
            *origin.url(),
            url::Url::from_str("https://www.google.com").unwrap()
        );
    }

    #[test]
    fn read_origin_with_dynamic_not_set() {
        const DYNAMIC: &str = r#"
            <origin>https://www.google.com</origin>
        "#;

        let origin: Origin = DYNAMIC.parse().unwrap();
        assert_eq!(origin.is_dynamic(), false);
        assert_eq!(
            *origin.url(),
            url::Url::from_str("https://www.google.com").unwrap()
        );
    }
}
