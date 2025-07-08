use crate::{impl_eq_name, models::ComponentInfo, utils::html::to_html};

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