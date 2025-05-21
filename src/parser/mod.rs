use error::ParsingError;
use linked_hash_set::LinkedHashSet;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::models::{function::Function, import::Import, structure::Structure, Wgsl};

pub mod error;
pub mod function;
pub mod import;
pub mod structure;
pub mod types;

pub trait FromPest {
    fn from_pest(element: Pair<'_, Rule>) -> Result<Self, ParsingError> 
    where 
        Self: Sized;
}

#[derive(Parser)]
#[grammar = "parser/wgsldoc.pest"]
struct WgslParserInner;

pub struct WgslParser;

impl WgslParser {
    pub fn parse(shader: &str) -> Result<Wgsl, ParsingError> {
        let shader_elements = WgslParserInner::parse(Rule::SHADER, shader)
            .map_err(|e| ParsingError::InputParsingError(Box::new(e)))?;

        let mut imports = LinkedHashSet::new();
        let mut functions = LinkedHashSet::new();
        let mut structures = LinkedHashSet::new();

        for shader_element in shader_elements {
            match shader_element.as_rule() {
                Rule::STRUCTURE => {
                    let structure = Structure::from_pest(shader_element)?;
                    let name = structure.name().to_owned();
                    
                    if !structures.insert(structure) {
                        log::warn!("Structure with name `{name}` already exists!")
                    }
                },
                Rule::FUNCTION => {
                    let function = Function::from_pest(shader_element)?;
                    let name = function.name().to_owned();
                    
                    if !functions.insert(function) {
                        log::warn!("Function with name `{name}` already exists!")
                    }
                },
                Rule::IMPORT => {
                    let import = Import::from_pest(shader_element)?;
                    let name = import.name().to_owned();
                    
                    if !imports.insert(import) {
                        log::warn!("Import with name `{name}` already exists!")
                    }
                },
                _ => {},
            }
        }

        Ok(Wgsl { imports, functions, structures })
    }
}