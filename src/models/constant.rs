use serde::Serialize;

use crate::models::import::{Import, RegisterImports};
use crate::models::types::{RenderedType, Type};
use crate::{impl_eq_name, models::ComponentInfo, utils::html::to_html};

#[derive(Debug)]
pub struct Constant {
    docs: Option<String>,
    name: String,
    ty: Option<Type>,
    value: String,
}

#[derive(Serialize, Debug)]
pub struct RenderedConstant {
    pub docs: Option<String>,
    pub name: String,
    pub ty: Option<RenderedType>,
    pub value: String,
}

impl Constant {
    pub fn new(
        docs: Option<String>,
        name: String,
        ty: Option<Type>,
        value: String,
    ) -> Constant {
        Constant { docs, name, ty, value }
    }

    pub fn docs(&self) -> Option<&str> {
        self.docs.as_deref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn constant_type(&self) -> Option<&Type> {
        self.ty.as_ref()
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn rendered(&self, imports: &[Import]) -> RenderedConstant {
        RenderedConstant {
            docs: self.docs.clone(),
            name: self.name.clone(),
            ty: self.constant_type().map(|ty| match ty {
                Type::Primitive(p) => RenderedType {
                    name: p.to_string(),
                    ..Default::default()
                },
                Type::Vector(v) => RenderedType {
                    name: v.to_string(),
                    ..Default::default()
                },
                Type::Path(path) => Type::Path(path.clone()).rendered_type(imports, false),
            }),
            value: self.value.clone(),
        }
    }

    /// Returns a [`ComponentInfo`] containing a summary of the function documentation, 
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
}

impl RegisterImports for Constant {
    fn register_imports(&mut self, imports: &[Import]) {
        if let Some(Type::Path(ref mut ty)) = &mut self.ty { 
            ty.register_imports(imports) 
        }
    }

    fn register_same_module_types(&mut self, type_names: &[String]) {
        if let Some(Type::Path(ref mut ty)) = &mut self.ty { 
            ty.register_same_module_types(type_names)
        }
    }
}

impl_eq_name!(Constant::name);

