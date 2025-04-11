use std::str::FromStr;
use pest::iterators::Pair;

use crate::model::types::{PathType, Primitive, Type, Vector, VectorDimension};
use super::{FromPest, InvalidPestRule, Rule};

impl FromPest for Type {
    fn from_pest(element: Pair<'_, Rule>) -> Result<Self, InvalidPestRule> 
            where 
                Self: Sized 
    {
        match element.as_rule() {
            Rule::TYPE => {
                let mut ty = Type::default();

                for type_element in element.into_inner() {
                    match type_element.as_rule() {
                        Rule::PRIMITIVE => {
                            ty = Type::Primitive(Primitive::from_pest(type_element)?);
                        }, 
                        Rule::VECTOR => {
                            ty = Type::Vector(Vector::from_pest(type_element)?);
                        }, 
                        Rule::PATH_TYPE => {
                            ty = Type::Path(PathType::from_pest(type_element)?);
                        }, 
                        _ => {}
                    }
                }

                Ok(ty)
            },
            _ => Err(
                InvalidPestRule {
                    expected: Rule::TYPE,
                    found: element.as_rule(),
                }
            )
        }
    }
}

impl FromPest for Primitive {
    fn from_pest(element: Pair<'_, Rule>) -> Result<Self, InvalidPestRule> 
            where 
                Self: Sized 
    {
        match element.as_rule() {
            Rule::PRIMITIVE => Ok(
                Primitive::from_str(element.as_span().as_str()).unwrap()
            ),
            _ => Err(
                InvalidPestRule {
                    expected: Rule::PRIMITIVE,
                    found: element.as_rule(),
                }
            )
        }
    }
}

impl FromPest for Vector {
    fn from_pest(element: Pair<'_, Rule>) -> Result<Self, InvalidPestRule> 
            where 
                Self: Sized 
    {
        match element.as_rule() {
            Rule::VECTOR => {
                let mut vector = Vector::default();

                for vector_element in element.into_inner() {
                    match vector_element.as_rule() {
                        Rule::VECTOR_DIMENSION => {
                            vector.dimension = VectorDimension::from_pest(vector_element)?;
                        },
                        Rule::PRIMITIVE => {
                            vector.ty = Primitive::from_pest(vector_element)?;
                        },
                        _ => {},
                    }
                }
                
                Ok(vector)
            },
            _ => Err(
                InvalidPestRule {
                    expected: Rule::VECTOR,
                    found: element.as_rule(),
                }
            )
        }
    }
}

impl FromPest for VectorDimension {
    fn from_pest(element: Pair<'_, Rule>) -> Result<Self, InvalidPestRule> 
            where 
                Self: Sized 
    {
        match element.as_rule() {
            Rule::VECTOR_DIMENSION => Ok(
                VectorDimension::from_str(element.as_span().as_str()).unwrap()
            ),
            _ => Err(
                InvalidPestRule {
                    expected: Rule::VECTOR_DIMENSION,
                    found: element.as_rule(),
                }
            )
        }
    }
}

impl FromPest for PathType {
    fn from_pest(element: Pair<'_, Rule>) -> Result<Self, InvalidPestRule> 
            where 
                Self: Sized 
    {
        match element.as_rule() {
            Rule::PATH_TYPE => {
                let mut path_type = PathType::default();

                for path_type_element in element.into_inner() {
                    match path_type_element.as_rule() {
                        Rule::MODULE => {
                            path_type.module = Some(path_type_element.as_span().as_str().to_owned());
                        },
                        Rule::IDENT => {
                            path_type.name = path_type_element.as_span().as_str().to_owned();
                        }
                        _ => {},
                    }
                }

                Ok(path_type)
            },
            _ => Err(
                InvalidPestRule {
                    expected: Rule::PATH_TYPE,
                    found: element.as_rule(),
                }
            )
        }
    }
}