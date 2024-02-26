#![warn(missing_docs)]
//! This crate provides enum definitions iana registry names
//!
//! It provides serialization and deserialisation of names using serde
//! and provides trait implementations for std::fmt::Display, std::str::FromStr,
//! TryFrom<&str> and From<> for &'static str for the enumerations.
mod error;
mod hash_function_textual_names;
mod operating_system_names;

pub use error::IANARegistryError;
pub use hash_function_textual_names::HashFunctionTextualName;
pub use operating_system_names::OperatingSystemName;
