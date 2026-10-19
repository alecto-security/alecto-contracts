//! Contract-shape errors. A `ContractError` means a field or field *pair*
//! violates the shape the spec fixes; message text mirrors the producing
//! implementation so operator-facing wording stays consistent across repos.

use std::fmt;

/// A contract violation. The string is the human-readable reason.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractError(pub String);

impl ContractError {
    pub fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl fmt::Display for ContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ContractError {}

pub type Result<T> = std::result::Result<T, ContractError>;
