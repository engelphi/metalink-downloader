use serde::{Deserialize, Serialize};

/// Representation of the metalink:logo element
/// according to [RFC5854 Section 4.2.7](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.7)
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Logo {
    #[serde(rename = "$text")]
    logo: url::Url,
}

impl Logo {
    /// Create a new logo element
    pub fn new(logo: url::Url) -> Self {
        Self { logo }
    }

    /// Returns the logo
    pub fn logo(&self) -> &url::Url {
        &self.logo
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;
    use quick_xml::se::to_string;

    #[test]
    fn read_logo_works() {
        const LOGO: &str = r#"
            <Logo>https://www.google.com/images/branding/googlelogo/1x/googlelogo_light_color_272x92dp.png</Logo>
        "#;
        let logo: Logo = from_str(LOGO).unwrap();
        assert_eq!(Logo::new(url::Url::parse("https://www.google.com/images/branding/googlelogo/1x/googlelogo_light_color_272x92dp.png").unwrap()), logo);
    }

    #[test]
    fn write_logo_works() {
        let logo = Logo::new(url::Url::parse("https://www.google.com/images/branding/googlelogo/1x/googlelogo_light_color_272x92dp.png").unwrap());

        let expected = String::from(
            r#"<Logo>https://www.google.com/images/branding/googlelogo/1x/googlelogo_light_color_272x92dp.png</Logo>"#,
        );

        assert_eq!(to_string::<Logo>(&logo).unwrap(), expected);
    }
}
