//! Module defining custom error types for the documentation generator.
//! This includes errors related to WGSL parsing and I/O operations.

use thiserror::Error;

use crate::parser::error::ParsingError;

/// Custom error type for the documentation generator.
#[derive(Debug, Error)]
pub enum Error {
    /// WGSL parsing error.
    #[error("WGSL parsing error")]
    ParsingError(#[from] ParsingError),
    /// I/O error.
    #[error("I/O error")]
    IoError(#[from] std::io::Error),
}
