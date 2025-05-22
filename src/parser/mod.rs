use error::ParsingError;
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

        // TODO: parse global docs
        let mut global_docs = vec![];
        let mut imports = vec![];
        let mut functions = vec![];
        let mut structures = vec![];

        for shader_element in shader_elements {
            match shader_element.as_rule() {
                Rule::STRUCTURE => {
                    let structure = Structure::from_pest(shader_element)?;

                    if structures.contains(&structure) {
                        log::warn!("Structure with name `{}` already exists!", structure.name());
                    } else {
                        structures.push(structure);
                    }
                },
                Rule::FUNCTION => {
                    let function = Function::from_pest(shader_element)?;

                    if functions.contains(&function) {
                        log::warn!("Function with name `{}` already exists!", function.name());
                    } else {
                        functions.push(function);
                    }
                },
                Rule::IMPORT => {
                    let import = Import::from_pest(shader_element)?;

                    if imports.contains(&import) {
                        log::warn!("Import with name `{}` already exists!", import.name());
                    } else {
                        imports.push(import);
                    }
                },
                _ => {},
            }
        }

        Ok(Wgsl { global_docs, imports, functions, structures })
    }
}