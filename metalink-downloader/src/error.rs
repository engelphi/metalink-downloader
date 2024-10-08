use miette::Diagnostic;
use thiserror::Error;

#[derive(Diagnostic, Debug, Error)]
pub enum MetalinkDownloadError {
    #[error(transparent)]
    MetalinkParsingError(#[from] metalink::MetalinkError),

    #[error(transparent)]
    RequestError(#[from] reqwest::Error),

    #[error(transparent)]
    RequestMiddlewareError(#[from] reqwest_middleware::Error),

    #[error(transparent)]
    HeaderError(#[from] reqwest::header::InvalidHeaderValue),

    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, MetalinkDownloadError>;
