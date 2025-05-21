use std::path::PathBuf;

use pest::iterators::Pair;

use crate::models::import::Import;

use super::{error::ParsingError, FromPest, Rule};

impl FromPest for Import {
    fn from_pest(element: Pair<'_, Rule>) -> Result<Self, ParsingError> 
    where 
        Self: Sized 
    {
        match element.as_rule() {
            Rule::IMPORT => {
                let mut docs = None;
                let mut path = PathBuf::new();
                let mut name = String::new();

                for import_element in element.into_inner() {
                    match import_element.as_rule() {
                        Rule::DOCS => {
                            for docs_element in import_element.into_inner() {
                                docs = Some(docs_element.as_span().as_str().to_owned())
                                    .filter(|s| !s.is_empty());
                            }
                        },
                        Rule::IMPORT_PATH => {
                            path = PathBuf::from(import_element.as_span().as_str());
                        },
                        Rule::MODULE_NAME => {
                            name = import_element.as_span().as_str().to_owned();
                        }
                        _ => {},
                    }
                }

                Ok(Import::new(docs, path, name))
            },
            _ => Err(
                ParsingError::InvalidPestRule { 
                    expected: Rule::IMPORT, 
                    found: element.as_rule(),
                }
            )
        }
    }
}