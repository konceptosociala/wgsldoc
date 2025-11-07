//! Module for parsing WGSL constants using Pest and converting them into [`Constant`] model.

use super::{error::ParsingError, FromPest, Rule};
use crate::models::{constant::Constant, types::Type};
use pest::iterators::Pair;

impl FromPest for Constant {
    fn from_pest(element: Pair<'_, Rule>) -> Result<Self, ParsingError>
    where
        Self: Sized,
    {
        match element.as_rule() {
            Rule::CONST => {
                let mut docs = None;
                let mut name = String::new();
                let mut ty = None;
                let mut value = String::new();

                for const_element in element.into_inner() {
                    match const_element.as_rule() {
                        Rule::DOCS => {
                            for docs_element in const_element.into_inner() {
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
                            name = const_element.as_span().as_str().to_owned();
                        }
                        Rule::TYPE => {
                            ty = Some(Type::from_pest(const_element)?);
                        }
                        Rule::CONST_VALUE => {
                            value = const_element.as_span().as_str().to_owned();
                        }
                        _ => {}
                    }
                }

                Ok(Constant::new(docs, name, ty, value))
            }
            _ => Err(ParsingError::InvalidPestRule {
                expected: Rule::CONST,
                found: element.as_rule(),
            }),
        }
    }
}
