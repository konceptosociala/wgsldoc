//! Module for parsing WGSL bindings using Pest and converting them into [`Binding`] model.

use super::{error::ParsingError, FromPest, Rule};
use crate::models::{binding::Binding, types::Type};
use pest::iterators::Pair;

impl FromPest for Binding {
    fn from_pest(element: Pair<'_, Rule>) -> Result<Self, ParsingError>
    where
        Self: Sized,
    {
        match element.as_rule() {
            Rule::RESOURCE_BINDING => {
                let mut docs = None;
                let mut attr_group = 0;
                let mut attr_binding = 0;
                let mut name = String::new();
                let mut ty = Type::default();

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
                            ty = Type::from_pest(const_element)?;
                        }
                        Rule::BINDING_ATTRS => {
                            for binding_attr_element in const_element.into_inner() {
                                match binding_attr_element.as_rule() {
                                    Rule::ATTR_GROUP => {
                                        let group_str = binding_attr_element
                                            .into_inner()
                                            .next()
                                            .unwrap()
                                            .as_span()
                                            .as_str();
                                        attr_group = group_str.parse::<u16>().unwrap_or(0);
                                    }
                                    Rule::ATTR_BINDING => {
                                        let binding_str = binding_attr_element
                                            .into_inner()
                                            .next()
                                            .unwrap()
                                            .as_span()
                                            .as_str();
                                        attr_binding = binding_str.parse::<u16>().unwrap_or(0);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }

                Ok(Binding::new(docs, attr_group, attr_binding, name, ty))
            }
            _ => Err(ParsingError::InvalidPestRule {
                expected: Rule::RESOURCE_BINDING,
                found: element.as_rule(),
            }),
        }
    }
}
