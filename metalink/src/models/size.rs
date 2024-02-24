use serde::{Deserialize, Serialize};

/// Representation of the metalink:size element
/// according to [RFC5854 Section 4.2.14](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.14)
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Size {
    #[serde(rename = "$text")]
    size: u64,
}

impl Size {
    /// Create a new size element
    pub fn new(size: u64) -> Self {
        Self { size }
    }

    /// Returns the size
    pub fn size(&self) -> u64 {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;
    use quick_xml::se::to_string;

    #[test]
    fn read_size_works() {
        const SIZE: &str = r#"
            <Size>50</Size>
        "#;
        let size: Size = from_str(SIZE).unwrap();
        assert_eq!(Size::new(50), size);
    }

    #[test]
    fn write_size_works() {
        let size = Size::new(50);

        let expected = String::from(r#"<Size>50</Size>"#);

        assert_eq!(to_string::<Size>(&size).unwrap(), expected);
    }
}
