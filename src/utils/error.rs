use thiserror::Error;

use crate::parser::error::ParsingError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("WGSL parsing error")]
    ParsingError(#[from] ParsingError),
    #[error("I/O error")]
    IoError(#[from] std::io::Error),
}