#![warn(missing_docs)]
//! This crate provides serialisation and deserialisation code for
//! the metalink download metadata format as described in [RGC5854](https://www.rfc-editor.org/rfc/rfc5854)

pub use crate::error::MetalinkError;
pub use crate::models::{
    File, FileBuilder, FileUrl, Hash, MetaUrl, Metalink, Origin, Pieces, Publisher, Signature,
    TorrentOrMime, OS,
};

mod error;
mod models;
mod utils;
