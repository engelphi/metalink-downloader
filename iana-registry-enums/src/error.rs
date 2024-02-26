/// Errors related to parsing IANA registry entries
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum IANARegistryError {
    /// Failed to parse IANA Hash Function Textual Names
    #[error("Failed to parse iana hash function textual name")]
    HashParseError,
    /// Failed to parse IANA Operating System Names
    #[error("Failed to parse iana operating system name")]
    OsNameParseError,
}
