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

#[derive(Debug, Default)]
pub struct PathType {
    pub module: Option<String>,
    pub name: String,
    pub imported: bool,
}