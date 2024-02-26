use serde::Deserialize;

/// Representation of the metalink:logo element
/// according to [RFC5854 Section 4.2.7](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.7)
#[derive(Debug, Deserialize, PartialEq, Clone)]
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

impl std::str::FromStr for Logo {
    type Err = crate::MetalinkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(crate::utils::from_str::<Logo>(s)?)
    }
}

impl std::convert::TryFrom<&str> for Logo {
    type Error = crate::MetalinkError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_logo_works() {
        const LOGO: &str = r#"
            <Logo>https://www.google.com/images/branding/googlelogo/1x/googlelogo_light_color_272x92dp.png</Logo>
        "#;
        let logo = Logo::try_from(LOGO).unwrap();
        assert_eq!(Logo::new(url::Url::parse("https://www.google.com/images/branding/googlelogo/1x/googlelogo_light_color_272x92dp.png").unwrap()), logo);
        assert_eq!(*logo.logo(), url::Url::parse("https://www.google.com/images/branding/googlelogo/1x/googlelogo_light_color_272x92dp.png").unwrap());
    }
}
