use function::Function;
use import::Import;
use linked_hash_set::LinkedHashSet;
use structure::Structure;

pub mod structure;
pub mod import;
pub mod types;
pub mod function;

#[derive(Debug)]
pub struct Wgsl {
    pub imports: LinkedHashSet<Import>,
    pub functions: LinkedHashSet<Function>,
    pub structures: LinkedHashSet<Structure>,
    // TODO: add entry points
    // TODO: add builtin imports
    // TODO: add constants
    // TODO: add bindings
    // TODO: add enums
}