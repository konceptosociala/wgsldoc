use thiserror::Error;

use super::{types::{InvalidPrimitiveType, InvalidVectorDimension}, InvalidPestRule};

#[derive(Debug, Error)]
pub enum ParsingError {
    #[error(transparent)]
    InvalidPestRule(#[from] InvalidPestRule),
    #[error(transparent)]
    InvalidPrimitiveType(#[from] InvalidPrimitiveType),
    #[error(transparent)]
    InvalidVectorDimension(#[from] InvalidVectorDimension),
}