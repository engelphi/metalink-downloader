use iana_registry_enums::OperatingSystemName;
use serde::{Deserialize, Serialize};

/// Representation of the metalink:os element according to
/// [RFC5854 Section 4.2.10](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.10)
#[derive(Debug, Deserialize, Serialize, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;
    use quick_xml::se::to_string;

    #[test]
    fn construction_works() {
        let os = OS::new(OperatingSystemName::MacOS);
        assert_eq!(os.name(), OperatingSystemName::MacOS);
    }

    #[test]
    fn read_works() {
        const OS: &str = r#"<OS>MACOS</OS>"#;
        let os: OS = from_str(OS).unwrap();
        assert_eq!(os.name(), OperatingSystemName::MacOS);
    }

    #[test]
    fn write_works() {
        let expected_os = String::from(r#"<OS>MACOS</OS>"#);
        let os = OS::new(OperatingSystemName::MacOS);
        assert_eq!(to_string::<OS>(&os).unwrap(), expected_os);
    }
}
