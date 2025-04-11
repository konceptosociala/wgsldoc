use std::str::FromStr;
use thiserror::Error;

#[derive(Debug)]
pub enum Type {
    Primitive(Primitive),
    Vector(Vector),
    Path(PathType),
}

impl Default for Type {
    fn default() -> Self {
        Type::Primitive(Primitive::default())
    }
}

#[derive(Debug, Default)]
pub enum Primitive {
    Bool,
    Float32,
    Float64,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Sint8,
    Sint16,
    #[default]
    Sint32,
    Sint64,
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

#[derive(Debug, Default)]
pub struct Vector {
    pub dimension: VectorDimension,
    pub ty: Primitive,
}

#[derive(Debug, Default)]
pub enum VectorDimension {
    D2, 
    #[default]
    D3,
    D4,
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

#[derive(Debug, Default)]
pub struct PathType {
    pub module: Option<String>,
    pub name: String,
}