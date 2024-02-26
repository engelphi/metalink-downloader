use serde::Deserialize;

/// Representation of the metalink:size element
/// according to [RFC5854 Section 4.2.14](https://www.rfc-editor.org/rfc/rfc5854#section-4.2.14)
#[derive(Debug, Deserialize, PartialEq, Clone)]
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
    use crate::utils::from_str;

    #[test]
    fn read_size_works() {
        const SIZE: &str = r#"
            <Size>50</Size>
        "#;
        let size: Size = from_str(SIZE).unwrap();
        assert_eq!(Size::new(50), size);
        assert_eq!(size.size(), 50);
    }
}
