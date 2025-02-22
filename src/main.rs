use std::str::FromStr;

use pest::Parser;
use wgsldoc::parser::{structure::{Field, Structure}, types::{PathType, Primitive, Type, Vector}, FromPest, Rule, WgslParser};

const SHADER: &str = include_str!("../test_shader.wgsl");

fn main() -> anyhow::Result<()> {
    let mut structures = vec![];
    let wgsl = WgslParser::parse(Rule::SHADER, SHADER)?;

    for shader_element in wgsl {
        match shader_element.as_rule() {
            Rule::STRUCTURE => {
                let mut structure = Structure::default();

                for struct_element in shader_element.into_inner() {
                    match struct_element.as_rule() {
                        Rule::DOCS => {
                            for docs in struct_element.into_inner() {
                                structure.docs = docs.as_span().as_str().to_owned();
                            }
                        },
                        Rule::IDENT => {
                            structure.name = struct_element.as_span().as_str().to_owned();
                        },
                        Rule::FIELDS => {
                            for fields_element in struct_element.into_inner() {
                                if let Rule::FIELD = fields_element.as_rule() {
                                    let mut field = Field::default();

                                    for field_element in fields_element.into_inner() {
                                        match field_element.as_rule() {
                                            Rule::DOCS => {
                                                for docs_element in field_element.into_inner() {
                                                    field.docs = docs_element.as_span().as_str().to_owned();
                                                }
                                            },
                                            Rule::IDENT => {
                                                field.name = field_element.as_span().as_str().to_owned();
                                            },
                                            Rule::TYPE => {
                                                for type_element in field_element.into_inner() {
                                                    match type_element.as_rule() {
                                                        Rule::PRIMITIVE => {
                                                            field.ty = Type::Primitive(Primitive::from_pest(type_element)?);
                                                        }, 
                                                        Rule::VECTOR => {
                                                            field.ty = Type::Vector(Vector::default());
                                                        }, 
                                                        Rule::PATH_TYPE => {
                                                            field.ty = Type::Path(PathType::default());
                                                        }, 
                                                        _ => {}
                                                    }
                                                }
                                            },
                                            _ => {},
                                        }
                                    }

                                    structure.fields.push(field);
                                }
                            }
                        }
                        _ => {},
                    }
                }

                structures.push(structure);
            },
            Rule::FUNCTION => {},
            Rule::IMPORT => {},
            _ => {},
        }
    }

    dbg!(&structures);

    Ok(())
}