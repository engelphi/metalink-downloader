use serde::Deserialize;

/// Representation of the metalink:language element
/// according to [RFC5854 Section 4.2.6](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.6)
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Language {
    #[serde(rename = "$text")]
    language: String,
}

impl Language {
    /// Create a new language element
    pub fn new(language: &str) -> Self {
        Self {
            language: String::from(language),
        }
    }

    /// Returns the language
    pub fn language(&self) -> &str {
        self.language.as_ref()
    }
}

impl std::str::FromStr for Language {
    type Err = crate::MetalinkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(crate::utils::from_str::<Language>(s)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_language_works() {
        const LANGUAGE: &str = r#"
            <Language>Test</Language>
        "#;
        let language: Language = LANGUAGE.parse().unwrap();
        assert_eq!(Language::new("Test"), language);
        assert_eq!(language.language(), "Test");
    }
}
