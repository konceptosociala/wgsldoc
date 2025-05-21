use thiserror::Error;

use super::{
    types::{InvalidPrimitiveType, InvalidVectorDimension}, 
    Rule,
};

#[derive(Debug, Error)]
pub enum ParsingError {
    #[error("Invalid pest rule for parsed type: expected `{expected:?}`, found {found:?}")]
    InvalidPestRule {
        expected: Rule,
        found: Rule,
    },
    #[error(transparent)]
    InvalidPrimitiveType(#[from] InvalidPrimitiveType),
    #[error(transparent)]
    InvalidVectorDimension(#[from] InvalidVectorDimension),
    #[error("Error parsing shader input")]
    InputParsingError(#[from] Box<pest::error::Error<Rule>>)
}