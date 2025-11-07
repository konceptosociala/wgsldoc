//! Binding model module used for parsing and representing WGSL bindings.
//! Used for generating bindings documentation.

use serde::Serialize;
use crate::{
    impl_eq_name,
    models::{
        import::{Import, RegisterImports},
        types::{RenderedType, Type},
    },
};

/// Represents a binding in a shader module. Example:
/// ```wgsl
/// @group(0) @binding(1) var<uniform> myBinding: MyType;
/// ```
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

/// A serializable representation of a binding for rendering purposes used in Tera.
#[derive(Debug, Serialize)]
pub struct RenderedBinding {
    docs: Option<String>,
    attr_group: u16,
    attr_binding: u16,
    name: String,
    ty: RenderedType,
}

impl Binding {
    /// Creates a new Binding instance (usually from parsed elements).
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

    /// Get field `docs` from instance of `Binding`.
    pub fn docs(&self) -> Option<&str> {
        self.docs.as_deref()
    }

    /// Get field `name` from instance of `Binding`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get field `type` from instance of `Binding`.
    pub fn binding_type(&self) -> &Type {
        &self.ty
    }

    /// Renders the binding into a serializable form for templates.
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
