use std::collections::{HashMap, HashSet};

use function::Function;
use import::Import;
use module::Module;
use structure::Structure;

pub mod structure;
pub mod import;
pub mod module;
pub mod types;
pub mod function;

pub struct Wgsl {
    pub modules: HashMap<String, Module>,
    pub imports: HashSet<Import>,
    pub functions: HashSet<Function>,
    pub structures: HashSet<Structure>,
}