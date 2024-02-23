use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::utils::*;
use crate::{File, Origin};

/// Representation of the metalink:metalink element according to
/// [RFC5854 Section 4.1.1](https://www.rfc-editor.org/rfc/rfc5854#section-4.1.1)
#[derive(Debug, Deserialize, Serialize)]
pub struct Metalink {
    generator: Option<String>,
    origin: Option<Origin>,
    #[serde(with = "rfc3339_to_datetime_utc")]
    published: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(with = "rfc3339_to_datetime_utc")]
    updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "$value")]
    files: Vec<File>,
}

impl Metalink {
    /// Returns the value of the metalink:generator element
    /// if the field exists. See [RFC5854 Section 4.2.3](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.3)
    pub fn generator(&self) -> Option<&String> {
        self.generator.as_ref()
    }

    /// Returns the metalink:origin element if the field
    /// exists. See [RFC5854 Section 4.2.9](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.9)
    pub fn origin(&self) -> Option<&Origin> {
        self.origin.as_ref()
    }

    /// Returns the value of the metalink:published element
    /// if the field exists. See [RFC5854 Section 4.2.11](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.11)
    pub fn published(&self) -> Option<&DateTime<Utc>> {
        self.published.as_ref()
    }

    /// Returns the value of the metalink:updated element
    /// if the field exists. See [RFC5854 Section 4.2.15](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.15)
    pub fn updated(&self) -> Option<&DateTime<Utc>> {
        self.updated.as_ref()
    }

    /// Returns the list of metalink:file elements.
    /// See [RFC5854 Section 4.1.2](https://www.rfc-editor.org/rfc/rfc5854#section-4.1.2)
    pub fn files(&self) -> &Vec<File> {
        &self.files
    }
}

// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    // use std::str::FromStr;
    //
    // use super::*;
    // use iana_registry_enums::OperatingSystemName;
    // use quick_xml::de::from_str;

    #[test]
    fn parse_full_metalink() {}
}
