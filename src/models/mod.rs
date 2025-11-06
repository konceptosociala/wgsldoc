use serde::Serialize;
use crate::{models::{constant::Constant, types::RenderedType}, utils::html::to_html};
use function::Function;
use import::Import;
use structure::Structure;

pub mod structure;
pub mod import;
pub mod types;
pub mod function;
pub mod constant;

#[derive(Debug, Serialize)]
pub struct ComponentInfo {
    pub name: String,
    pub summary: Option<String>,
}

impl ComponentInfo {
    pub const SUMMARY_MAX_LENGTH: usize = 256;

    pub fn new(name: String, summary: Option<String>) -> Self {
        ComponentInfo { name, summary }
    }
}

#[derive(Debug)]
pub struct Wgsl {
    pub module_name: String,
    pub source_code: String,
    pub global_docs: Option<String>,
    pub imports: Vec<Import>,
    pub functions: Vec<Function>,
    pub structures: Vec<Structure>,
    pub constants: Vec<Constant>,
    // TODO: add bindings
    // TODO: add enums
}

impl Wgsl {
    /// Returns a [`ComponentInfo`] containing a summary of the WGSL documentation, 
    /// with the summary extracted from the rendered Markdown as HTML.
    pub fn info_rich_text(&self) -> ComponentInfo {
        let summary = self.global_docs.as_deref().map(to_html);

        ComponentInfo::new(self.module_name.clone(), summary)
    }

    /// Returns a [`ComponentInfo`] containing a summary of the WGSL documentation, 
    /// with the summary extracted from the rendered Markdown as plain text. The summary is truncated 
    /// to `ComponentInfo::SUMMARY_MAX_LENGTH` characters if necessary.
    pub fn info_plain_text(&self) -> ComponentInfo {
        let summary = self.global_docs.as_deref().map(|docs| {
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

        ComponentInfo::new(self.module_name.clone(), summary)
    }
}

#[derive(Serialize, Default, Debug)]
pub struct RenderedArgField {
    docs: Option<String>,
    name: String,
    ty: RenderedType,
}