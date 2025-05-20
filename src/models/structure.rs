use std::hash::Hash;

use super::types::Type;

#[derive(Debug)]
pub struct Structure {
    docs: Option<String>,
    name: String,
    fields: Vec<Field>,        
}

impl Structure {
    pub fn new(
        docs: Option<String>,
        name: String,
        fields: Vec<Field>,
    ) -> Structure {
        Structure { docs, name, fields }
    }
    
    pub fn docs(&self) -> Option<&str> {
        self.docs.as_deref()
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn fields(&self) -> &[Field] {
        &self.fields
    }
}

impl PartialEq for Structure {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Structure {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug)]
pub struct Field {
    pub docs: Option<String>,
    pub name: String,
    pub ty: Type,
}

impl Field {
    pub fn new(
        docs: Option<String>, 
        name: String, 
        ty: Type,
    ) -> Field {
        Field { docs, name, ty }
    }
    
    pub fn docs(&self) -> Option<&str> {
        self.docs.as_deref()
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn ty(&self) -> &Type {
        &self.ty
    }
}