use function::Function;
use import::Import;
use structure::Structure;

pub mod structure;
pub mod import;
pub mod types;
pub mod function;

#[derive(Debug)]
pub struct Wgsl {
    pub global_docs: Vec<String>,
    pub imports: Vec<Import>,
    pub functions: Vec<Function>,
    pub structures: Vec<Structure>,
    // TODO: add entry points
    // TODO: add builtin imports
    // TODO: add constants
    // TODO: add bindings
    // TODO: add enums
}