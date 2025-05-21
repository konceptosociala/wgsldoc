use linked_hash_set::LinkedHashSet;

use super::import::Import;

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
    dimension: VectorDimension,
    ty: Primitive,
}

impl Vector {
    pub fn new(
        dimension: VectorDimension, 
        ty: Primitive,
    ) -> Vector {
        Vector { dimension, ty }
    }
    
    pub fn vector_type(&self) -> &Primitive {
        &self.ty
    }
    
    pub fn dimension(&self) -> &VectorDimension {
        &self.dimension
    }
}

#[derive(Debug, Default)]
pub enum VectorDimension {
    D2, 
    #[default]
    D3,
    D4,
}

#[derive(Debug)]
pub struct PathType {
    module: Option<String>,
    name: String,
    imported: bool,
}

impl PathType {
    pub fn new(
        module: Option<String>, 
        name: String, 
    ) -> PathType {
        PathType { 
            module, 
            name, 
            imported: false,
        }
    }

    pub fn import(&mut self, imports: &LinkedHashSet<Import>) -> bool {
        if let Some(module_name) = &self.module {
            for import in imports {
                if import.name() == module_name {
                    self.imported = true;
                }
            }
        }

        self.imported
    }
    
    pub fn module(&self) -> Option<&str> {
        self.module.as_deref()
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn imported(&self) -> bool {
        self.imported
    }
}