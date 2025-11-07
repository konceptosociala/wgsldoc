//! Module for parsing WGSL structures using Pest and converting them into [`Structure`] model.

use super::{error::ParsingError, FromPest, Rule};
use crate::models::{
    structure::{Field, Structure},
    types::Type,
};
use pest::iterators::Pair;

impl FromPest for Structure {
    fn from_pest(element: Pair<'_, Rule>) -> Result<Self, ParsingError>
    where
        Self: Sized,
    {
        match element.as_rule() {
            Rule::STRUCTURE => {
                let mut docs = None;
                let mut name = String::new();
                let mut fields = vec![];

                for struct_element in element.into_inner() {
                    match struct_element.as_rule() {
                        Rule::DOCS => {
                            for docs_element in struct_element.into_inner() {
                                if docs.is_none() {
                                    docs = Some(String::new());
                                }

                                if let Some(docs) = &mut docs {
                                    if !docs.is_empty() {
                                        docs.push('\n');
                                    }

                                    docs.push_str(docs_element.as_span().as_str());
                                }

                                docs = docs.filter(|s| !s.is_empty());
                            }
                        }
                        Rule::IDENT => {
                            name = struct_element.as_span().as_str().to_owned();
                        }
                        Rule::FIELDS => {
                            for fields_element in struct_element.into_inner() {
                                if let Rule::FIELD = fields_element.as_rule() {
                                    let field = Field::from_pest(fields_element)?;

                                    if fields.contains(&field) {
                                        log::warn!("Field with name `{}` already exists in structure `{name}`!", field.name())
                                    } else {
                                        fields.push(field);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }

                Ok(Structure::new(docs, name, fields))
            }
            _ => Err(ParsingError::InvalidPestRule {
                expected: Rule::STRUCTURE,
                found: element.as_rule(),
            }),
        }
    }
}

impl FromPest for Field {
    fn from_pest(element: Pair<'_, Rule>) -> Result<Self, ParsingError>
    where
        Self: Sized,
    {
        match element.as_rule() {
            Rule::FIELD => {
                let mut docs = None;
                let mut name = String::new();
                let mut ty = Type::default();

                for field_element in element.into_inner() {
                    match field_element.as_rule() {
                        Rule::DOCS => {
                            for docs_element in field_element.into_inner() {
                                docs = Some(docs_element.as_span().as_str().to_owned())
                                    .filter(|s| !s.is_empty());
                            }
                        }
                        Rule::IDENT => {
                            name = field_element.as_span().as_str().to_owned();
                        }
                        Rule::TYPE => {
                            ty = Type::from_pest(field_element)?;
                        }
                        _ => {}
                    }
                }

                Ok(Field::new(docs, name, ty))
            }
            _ => Err(ParsingError::InvalidPestRule {
                expected: Rule::FIELD,
                found: element.as_rule(),
            }),
        }
    }
}
