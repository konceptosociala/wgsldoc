use pest::iterators::Pair;

use crate::models::{structure::{Field, Structure}, types::Type};
use super::{FromPest, InvalidPestRule, Rule};

impl FromPest for Structure {
    fn from_pest(element: Pair<'_, Rule>) -> Result<Self, InvalidPestRule> 
            where 
                Self: Sized 
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
                                docs = Some(docs_element.as_span().as_str().to_owned())
                                    .filter(|s| !s.is_empty());
                            }
                        },
                        Rule::IDENT => {
                            name = struct_element.as_span().as_str().to_owned();
                        },
                        Rule::FIELDS => {
                            for fields_element in struct_element.into_inner() {
                                if let Rule::FIELD = fields_element.as_rule() {
                                    fields.push(Field::from_pest(fields_element)?);
                                }
                            }
                        }
                        _ => {},
                    }
                }
                
                Ok(Structure::new(docs, name, fields))
            },
            _ => Err(
                InvalidPestRule {
                    expected: Rule::STRUCTURE,
                    found: element.as_rule(),
                }
            )
        }
    }
}

impl FromPest for Field {
    fn from_pest(element: Pair<'_, Rule>) -> Result<Self, InvalidPestRule> 
            where 
                Self: Sized 
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
                        },
                        Rule::IDENT => {
                            name = field_element.as_span().as_str().to_owned();
                        },
                        Rule::TYPE => {
                            ty = Type::from_pest(field_element)?;
                        },
                        _ => {},
                    }
                }
                
                Ok(Field::new(docs, name, ty))
            },
            _ => Err(
                InvalidPestRule {
                    expected: Rule::FIELD,
                    found: element.as_rule(),
                }
            )
        }
    }
}