use linked_hash_set::LinkedHashSet;

use crate::impl_eq_name;

use super::types::Type;

#[derive(Debug)]
pub struct Structure {
    docs: Option<String>,
    name: String,
    fields: LinkedHashSet<Field>,
}

impl Structure {
    pub fn new(
        docs: Option<String>,
        name: String,
        fields: LinkedHashSet<Field>,
    ) -> Structure {
        Structure { docs, name, fields }
    }
    
    pub fn docs(&self) -> Option<&str> {
        self.docs.as_deref()
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn fields(&self) -> &LinkedHashSet<Field> {
        &self.fields
    }
}

impl_eq_name!(Structure::name);

#[derive(Debug)]
pub struct Field {
    docs: Option<String>,
    name: String,
    ty: Type,
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
    
    pub fn field_type(&self) -> &Type {
        &self.ty
    }
}

impl_eq_name!(Field::name);