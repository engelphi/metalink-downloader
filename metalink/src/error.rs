// ----------------------------------------------------------------------------
/// General error type for errors encountered during the parsing of a meta link
/// file
#[derive(Debug, thiserror::Error)]
pub enum MetalinkError {
    /// An error occured when parsing a mediatype field
    /// Most likely a malformed mediatype was encountered
    #[error("Failed to parse mediatype")]
    MediaTypeParseError(#[from] mime::FromStrError),

    /// An error occured while parsing a url
    #[error("Failed to parse url")]
    UrlParseError(#[from] url::ParseError),

    /// An error occured while constructing a metalink
    #[error("Error constructing the Metalink: {0}")]
    MetalinkConstructionError(String),
}
