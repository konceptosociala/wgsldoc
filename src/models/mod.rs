use std::collections::HashSet;

use function::Function;
use import::Import;
use structure::Structure;

pub mod structure;
pub mod import;
pub mod types;
pub mod function;

pub struct Wgsl {
    pub imports: HashSet<Import>,
    pub functions: HashSet<Function>,
    pub structures: HashSet<Structure>,
    // TODO: add entry points
    // TODO: add builtin imports
    // TODO: add constants
    // TODO: add bindings
    // TODO: add enums
}