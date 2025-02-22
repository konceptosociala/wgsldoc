use std::collections::{HashMap, HashSet};

use module::Module;
use pest::iterators::Pair;
use pest_derive::Parser;

use import::Import;
use structure::Structure;
use thiserror::Error;

pub mod import;
pub mod module;
pub mod structure;
pub mod types;

#[derive(Debug, Error)]
#[error("Invalid pest rule for this type: expected `{expected:?}`, found {found:?}")]
pub struct InvalidPestRule {
    pub expected: Rule,
    pub found: Rule,
}

pub trait FromPest {
    fn from_pest(element: Pair<'_, Rule>) -> Result<Self, InvalidPestRule> 
        where 
            Self: Sized;
}

pub struct Wgsl {
    pub modules: HashMap<String, Module>,
    pub imports: HashSet<Import>,
    pub functions: HashSet<Function>,
    pub structures: HashSet<Structure>,
}

pub struct Function {

}

#[derive(Parser)]
#[grammar = "wgsldoc.pest"]
pub struct WgslParser;