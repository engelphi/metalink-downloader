use iana_registry_enums::OperatingSystemName;
use serde::Deserialize;

/// Representation of the metalink:os element according to
/// [RFC5854 Section 4.2.10](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.10)
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct OS {
    #[serde(rename = "$text")]
    name: OperatingSystemName,
}

impl OS {
    /// Create a new os element
    pub fn new(name: OperatingSystemName) -> Self {
        Self { name }
    }
    /// Returns the name of the operating system
    pub fn name(&self) -> OperatingSystemName {
        self.name
    }
}

impl std::str::FromStr for OS {
    type Err = crate::MetalinkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(crate::utils::from_str::<OS>(s)?)
    }
}

impl std::convert::TryFrom<&str> for OS {
    type Error = crate::MetalinkError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction_works() {
        let os = OS::new(OperatingSystemName::MacOS);
        assert_eq!(os.name(), OperatingSystemName::MacOS);
    }

    #[test]
    fn read_works() {
        const OS: &str = r#"<OS>MACOS</OS>"#;
        let os = OS::try_from(OS).unwrap();
        assert_eq!(os.name(), OperatingSystemName::MacOS);
    }
}
