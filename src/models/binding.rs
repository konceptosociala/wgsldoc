use serde::Serialize;

use crate::{
    impl_eq_name,
    models::{
        import::{Import, RegisterImports},
        types::{RenderedType, Type},
    },
};

#[derive(Debug)]
pub struct Binding {
    docs: Option<String>,
    attr_group: u16,
    attr_binding: u16,
    name: String,
    ty: Type,
}

impl RegisterImports for Binding {
    fn register_imports(&mut self, imports: &[Import]) {
        if let Type::Path(ref mut ty) = &mut self.ty {
            ty.register_imports(imports)
        }
    }

    fn register_same_module_types(&mut self, type_names: &[String]) {
        if let Type::Path(ref mut ty) = &mut self.ty {
            ty.register_same_module_types(type_names)
        }
    }
}

impl_eq_name!(Binding::name);

#[derive(Debug, Serialize)]
pub struct RenderedBinding {
    pub docs: Option<String>,
    pub attr_group: u16,
    pub attr_binding: u16,
    pub name: String,
    pub ty: RenderedType,
}

impl Binding {
    pub fn new(
        docs: Option<String>,
        attr_group: u16,
        attr_binding: u16,
        name: String,
        ty: Type,
    ) -> Binding {
        Binding {
            docs,
            attr_group,
            attr_binding,
            name,
            ty,
        }
    }

    pub fn docs(&self) -> Option<&str> {
        self.docs.as_deref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn binding_type(&self) -> &Type {
        &self.ty
    }

    pub fn rendered(&self, imports: &[Import]) -> RenderedBinding {
        RenderedBinding {
            docs: self.docs.clone(),
            attr_group: self.attr_group,
            attr_binding: self.attr_binding,
            name: self.name.clone(),
            ty: self.binding_type().rendered_type(imports, false),
        }
    }
}
