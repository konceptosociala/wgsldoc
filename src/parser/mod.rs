use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use error::ParsingError;
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
    pub fn parse(shader_name: &str, shader: &str) -> Result<Wgsl, ParsingError> {
        let shader_elements = WgslParserInner::parse(Rule::SHADER, shader)
            .map_err(|e| ParsingError::InputParsingError(Box::new(e)))?;

        let source_code = shader.to_owned();
        let mut global_docs = None;
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
                Rule::GLOBAL_DOCS => {
                    for docs_element in shader_element.into_inner() {
                        if global_docs.is_none() {
                            global_docs = Some(String::new());
                        }

                        if let Some(global_docs) = &mut global_docs {
                            if !global_docs.is_empty() {
                                global_docs.push('\n');
                            }

                            global_docs.push_str(docs_element.as_span().as_str());
                        }
                            
                        global_docs = global_docs.filter(|s| !s.is_empty());
                    }
                },
                _ => {},
            }
        }

        Ok(Wgsl {
            module_name: shader_name.to_string(),
            source_code,
            global_docs,
            imports,
            functions,
            structures,
        })
    }
}