use crate::impl_eq_name;

use super::{import::{Import, RegisterImports}, types::Type};

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

impl RegisterImports for Structure {
    fn register_imports(&mut self, imports: &[Import]) -> bool {
        let mut registered = false;

        for field in &mut self.fields {
            if field.register_imports(imports) {
                registered = true;
            }
        }

        registered
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

impl RegisterImports for Field {
    fn register_imports(&mut self, imports: &[Import]) -> bool {
        if let Type::Path(ty) = &mut self.ty {
            return ty.register_imports(imports);
        }

        false
    }
}

impl_eq_name!(Field::name);