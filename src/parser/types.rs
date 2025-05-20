use std::str::FromStr;
use pest::iterators::Pair;
use thiserror::Error;

use crate::models::types::{PathType, Primitive, Type, Vector, VectorDimension};
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

#[derive(Debug, Error)]
#[error("Invalid primitive type `{0}`; available are bool, f32, f64, u8, u16, u32, u64, i8, i16, i32, i64")]
pub struct InvalidPrimitiveType(String);

impl FromStr for Primitive {
    type Err = InvalidPrimitiveType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Primitive::*;

        match s {
            "bool" => Ok(Bool),
            "f32" => Ok(Float32),
            "f64" => Ok(Float64),
            "u8" => Ok(Uint8),
            "u16" => Ok(Uint16),
            "u32" => Ok(Uint32),
            "u64" => Ok(Uint64),
            "i8" => Ok(Sint8),
            "i16" => Ok(Sint16),
            "i32" => Ok(Sint32),
            "i64" => Ok(Sint64),
            _ => Err(InvalidPrimitiveType(s.to_owned())),
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

#[derive(Debug, Error)]
#[error("Invalid vector dimension `{0}`; available are 2, 3, 4")]
pub struct InvalidVectorDimension(String);

impl FromStr for VectorDimension {
    type Err = InvalidPrimitiveType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use VectorDimension::*;

        match s {
            "2" => Ok(D2),
            "3" => Ok(D3),
            "4" => Ok(D4),
            _ => Err(InvalidPrimitiveType(s.to_owned())),
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