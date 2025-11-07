//! Structure model module used for parsing and representing WGSL structures.
//! Used for generating structure documentation.

use super::{
    import::{Import, RegisterImports},
    types::Type,
};
use crate::{
    impl_eq_name,
    models::{types::RenderedType, ComponentInfo, RenderedArgField},
    utils::html::to_html,
};

/// Represents a structure in a shader module. Example:
/// ```wgsl
/// struct MyStruct {
///     @location(0) position: vec3<f32>,
///     @location(1) color: vec4<f32>,
/// };
/// ```
#[derive(Debug)]
pub struct Structure {
    docs: Option<String>,
    name: String,
    fields: Vec<Field>,
}

impl Structure {
    /// Creates a new Structure instance (usually from parsed elements).
    pub fn new(docs: Option<String>, name: String, fields: Vec<Field>) -> Structure {
        Structure { docs, name, fields }
    }

    /// Renders the structure's fields into a serializable form for templates.
    pub fn rendered_fields(&self, imports: &[Import]) -> Vec<RenderedArgField> {
        self.fields()
            .iter()
            .map(|field| {
                let ty = match field.field_type() {
                    Type::Primitive(p) => RenderedType {
                        name: p.to_string(),
                        ..Default::default()
                    },
                    Type::Vector(v) => RenderedType {
                        name: v.to_string(),
                        ..Default::default()
                    },
                    Type::Path(path) => Type::Path(path.clone()).rendered_type(imports, false),
                };

                RenderedArgField {
                    docs: field.docs().map(to_html),
                    name: field.name().to_string(),
                    ty,
                }
            })
            .collect()
    }

    /// Returns a [`ComponentInfo`] containing a summary of the structure documentation,
    /// with the summary extracted from the rendered Markdown as HTML.
    pub fn info_rich_text(&self) -> ComponentInfo {
        let summary = self.docs.as_deref().map(to_html);

        ComponentInfo::new(self.name.clone(), summary)
    }

    /// Returns a [`ComponentInfo`] containing a summary of the structure documentation,
    /// with the summary extracted from the rendered Markdown as plain text. The summary is truncated
    /// to `ComponentInfo::SUMMARY_MAX_LENGTH` characters if necessary.
    pub fn info_plain_text(&self) -> ComponentInfo {
        let summary = self.docs.as_deref().map(|docs| {
            let html = to_html(docs);
            let parsed = scraper::Html::parse_fragment(&html);

            let summary = parsed
                .root_element()
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();

            if summary.len() > ComponentInfo::SUMMARY_MAX_LENGTH {
                format!("{}...", &summary[..ComponentInfo::SUMMARY_MAX_LENGTH])
            } else {
                summary
            }
        });

        ComponentInfo::new(self.name.clone(), summary)
    }

    /// Get field `docs` from instance of `Structure`.
    pub fn docs(&self) -> Option<&str> {
        self.docs.as_deref()
    }

    /// Get field `name` from instance of `Structure`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get field `fields` from instance of `Structure`.
    pub fn fields(&self) -> &[Field] {
        &self.fields
    }
}

impl RegisterImports for Structure {
    fn register_imports(&mut self, imports: &[Import]) {
        for field in &mut self.fields {
            field.register_imports(imports);
        }
    }

    fn register_same_module_types(&mut self, type_names: &[String]) {
        for field in &mut self.fields {
            field.register_same_module_types(type_names);
        }
    }
}

impl_eq_name!(Structure::name);

/// Represents a field within a structure. Example:
/// ```wgsl
/// @location(0) output_field: vec3<f32>,
/// ```
/// ```wgsl
/// other_field: Module::Type,
/// ```
#[derive(Debug)]
pub struct Field {
    docs: Option<String>,
    name: String,
    ty: Type,
}

impl Field {
    /// Creates a new Field instance (usually from parsed elements).
    pub fn new(docs: Option<String>, name: String, ty: Type) -> Field {
        Field { docs, name, ty }
    }

    /// Get field `docs` from instance of `Field`.
    pub fn docs(&self) -> Option<&str> {
        self.docs.as_deref()
    }

    /// Get field `name` from instance of `Field`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get field `type` from instance of `Field`.
    pub fn field_type(&self) -> &Type {
        &self.ty
    }
}

impl RegisterImports for Field {
    fn register_imports(&mut self, imports: &[Import]) {
        if let Type::Path(ty) = &mut self.ty {
            ty.register_imports(imports);
        }
    }

    fn register_same_module_types(&mut self, type_names: &[String]) {
        if let Type::Path(ty) = &mut self.ty {
            ty.register_same_module_types(type_names);
        }
    }
}

impl_eq_name!(Field::name);
