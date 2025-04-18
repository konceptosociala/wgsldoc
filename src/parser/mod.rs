use std::collections::{HashMap, HashSet};
use pest::iterators::Pair;
use pest_derive::Parser;
use thiserror::Error;

pub mod import;
pub mod module;
pub mod structure;
pub mod types;

#[derive(Debug, Error)]
#[error("Invalid pest rule for parsed type: expected `{expected:?}`, found {found:?}")]
pub struct InvalidPestRule {
    pub expected: Rule,
    pub found: Rule,
}

pub trait FromPest {
    fn from_pest(element: Pair<'_, Rule>) -> Result<Self, InvalidPestRule> 
        where 
            Self: Sized;
}

#[derive(Parser)]
#[grammar = "wgsldoc.pest"]
pub struct WgslParser;