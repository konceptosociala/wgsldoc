//! Module containing error types for parsing WGSL components using Pest.

use super::{
    types::{InvalidPrimitiveType, InvalidVectorDimension},
    Rule,
};
use thiserror::Error;

/// Error types for parsing WGSL components using Pest.
#[derive(Debug, Error)]
pub enum ParsingError {
    /// Error for invalid pest rules during parsing.
    #[error("Invalid pest rule for parsed type: expected `{expected:?}`, found {found:?}")]
    InvalidPestRule { 
        /// The expected pest rule.
        expected: Rule, 
        /// The found pest rule.
        found: Rule,
    },
    /// Error for invalid primitive types during parsing.
    #[error(transparent)]
    InvalidPrimitiveType(#[from] InvalidPrimitiveType),
    /// Error for invalid vector dimensions during parsing.
    #[error(transparent)]
    InvalidVectorDimension(#[from] InvalidVectorDimension),
    /// Error parsing shader input.
    #[error("Error parsing shader input")]
    InputParsingError(#[from] Box<pest::error::Error<Rule>>),
}
