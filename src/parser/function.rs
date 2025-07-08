use pest::iterators::Pair;
use crate::models::{
    function::{Arg, Function, FunctionType},
    types::{PathType, Primitive, Type, Vector},
};
use super::{error::ParsingError, FromPest, Rule};

impl FromPest for Function {
    fn from_pest(element: Pair<'_, Rule>) -> Result<Self, ParsingError> 
    where 
        Self: Sized 
    {
        match element.as_rule() {
            Rule::FUNCTION => {
                let mut docs = None;
                let mut name = String::new();
                let mut args = vec![];
                let mut return_ty = None;

                for function_element in element.into_inner() {
                    match function_element.as_rule() {
                        Rule::DOCS => {
                            for docs_element in function_element.into_inner() {
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
                        },
                        Rule::IDENT => {
                            name = function_element.as_span().as_str().to_owned();
                        },
                        Rule::ARGS => {
                            for args_element in function_element.into_inner() {
                                if let Rule::ARG = args_element.as_rule() {
                                    let arg = Arg::from_pest(args_element)?;
                                    
                                    if args.contains(&arg) {
                                        log::warn!("Argument with name `{}` already exists in function `{name}`!", arg.name())
                                    } else {
                                        args.push(arg);
                                    }
                                }
                            }
                        },
                        Rule::RETURN => {
                            return_ty = Some(Type::from_pest(function_element.into_inner().next().unwrap())?)
                        }
                        _ => {},
                    }
                }

                Ok(Function::new(docs, name, args, return_ty))
            },
            _ => Err(
                ParsingError::InvalidPestRule { 
                    expected: Rule::FUNCTION, 
                    found: element.as_rule(), 
                }
            )
        }
    }
}

impl FromPest for Arg {
    fn from_pest(element: Pair<'_, Rule>) -> Result<Self, ParsingError> 
    where 
        Self: Sized 
    {
        match element.as_rule() {
            Rule::ARG => {
                let mut docs = None;
                let mut name = String::new();
                let mut ty = FunctionType::default();

                for arg_element in element.into_inner() {
                    match arg_element.as_rule() {
                        Rule::DOCS => {
                            for docs_element in arg_element.into_inner() {
                                docs = Some(docs_element.as_span().as_str().to_owned())
                                    .filter(|s| !s.is_empty());
                            }
                        },
                        Rule::IDENT => {
                            name = arg_element.as_span().as_str().to_owned();
                        },
                        Rule::FUNCTION_TYPE => {
                            ty = FunctionType::from_pest(arg_element)?;
                        }
                        _ => {},
                    }
                }

                Ok(Arg::new(docs, name, ty))
            },
            _ => Err(
                ParsingError::InvalidPestRule { 
                    expected: Rule::ARG, 
                    found: element.as_rule(),
                }
            )
        }
    }
}

impl FromPest for FunctionType {
    fn from_pest(element: Pair<'_, Rule>) -> Result<Self, ParsingError> 
    where 
        Self: Sized 
    {
        match element.as_rule() {
            Rule::FUNCTION_TYPE => {
                let mut ty = FunctionType::default();

                for type_element in element.into_inner() {
                    match type_element.as_rule() {
                        Rule::PRIMITIVE => {
                            ty = FunctionType::Primitive(Primitive::from_pest(type_element)?);
                        }, 
                        Rule::VECTOR => {
                            ty = FunctionType::Vector(Vector::from_pest(type_element)?);
                        }, 
                        Rule::PATH_TYPE => {
                            ty = FunctionType::Path(PathType::from_pest(type_element)?);
                        }, 
                        Rule::FUNCTION_POINTER => {
                            ty = FunctionType::FunctionPointer(
                                Type::from_pest(type_element.into_inner().next().unwrap())?
                            );
                        }
                        _ => {}
                    }
                }

                Ok(ty)
            },
            _ => Err(
                ParsingError::InvalidPestRule { 
                    expected: Rule::FUNCTION_TYPE, 
                    found: element.as_rule(),
                }
            )
        }
    }
}