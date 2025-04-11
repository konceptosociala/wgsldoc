use pest::iterators::Pair;

use crate::model::{structure::{Field, Structure}, types::Type};
use super::{FromPest, InvalidPestRule, Rule};

impl FromPest for Structure {
    fn from_pest(element: Pair<'_, Rule>) -> Result<Self, InvalidPestRule> 
            where 
                Self: Sized 
    {
        match element.as_rule() {
            Rule::STRUCTURE => {
                let mut structure = Structure::default();

                for struct_element in element.into_inner() {
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
                                    structure.fields.push(Field::from_pest(fields_element)?);
                                }
                            }
                        }
                        _ => {},
                    }
                }
                
                Ok(structure)
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
                let mut field = Field::default();

                for field_element in element.into_inner() {
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
                            field.ty = Type::from_pest(field_element)?;
                        },
                        _ => {},
                    }
                }
                
                Ok(field)
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