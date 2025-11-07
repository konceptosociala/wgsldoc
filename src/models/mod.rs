//! Models module containing data structures for representing WGSL components.
//! This components are used throughout the application for parsing, processing, and generating documentation.
//! This includes:
//! - Bindings
//! - Constants
//! - Functions
//! - Imports
//! - Structures
//! - Types

use crate::{
    models::{binding::Binding, constant::Constant, types::RenderedType},
    utils::html::to_html,
};
use function::Function;
use import::Import;
use serde::Serialize;
use structure::Structure;

pub mod binding;
pub mod constant;
pub mod function;
pub mod import;
pub mod structure;
pub mod types;

/// Represents summary information about a WGSL component, such as a module or function.
/// Used for generating documentation overviews based on plain text or Markdown.
#[derive(Debug, Serialize)]
pub struct ComponentInfo {
    /// The name of the component.
    pub name: String,
    /// A brief summary of the component's purpose or functionality.
    pub summary: Option<String>,
}

impl ComponentInfo {
    /// The maximum length for summaries in plain text representation.
    pub const SUMMARY_MAX_LENGTH: usize = 256;

    /// Creates a new ComponentInfo instance.
    pub fn new(name: String, summary: Option<String>) -> Self {
        ComponentInfo { name, summary }
    }
}

/// Main WGSL model representing a shader module with all its components.
#[derive(Debug)]
pub struct Wgsl {
    /// The name of the module.
    pub module_name: String,
    /// The original WGSL source code.
    pub source_code: String,
    /// The global documentation comments for the module.
    pub global_docs: Option<String>,
    /// The list of imports for the module.
    pub imports: Vec<Import>,
    /// The list of functions in the module.
    pub functions: Vec<Function>,
    /// The list of structures in the module.
    pub structures: Vec<Structure>,
    /// The list of constants in the module.
    pub constants: Vec<Constant>,
    /// The list of bindings in the module.
    pub bindings: Vec<Binding>,
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

/// A serializable representation of a function argument 
/// or a structure field, used for rendering purposes used in Tera.
#[derive(Serialize, Default, Debug)]
pub struct RenderedArgField {
    docs: Option<String>,
    name: String,
    ty: RenderedType,
}
