//! Import model module used for parsing and representing WGSL imports.
//! Used for generating imports documentation.

use crate::{impl_eq_name, models::ComponentInfo, utils::html::to_html};
use serde::Serialize;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Represents an import in a shader module. Example:
/// ```wgsl
/// import "module.wgsl" as MyModule;
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Import {
    docs: Option<String>,
    path: PathBuf,
    module_name: String,
    name: String,
    registered: bool,
}

impl Import {
    /// Creates a new Import instance (usually from parsed elements).
    pub fn new(docs: Option<String>, path: PathBuf, name: String) -> Import {
        Import {
            docs: docs.map(|s| to_html(&s)),
            module_name: path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default()
                .to_string(),
            path,
            name,
            registered: false,
        }
    }

    /// Get field `module_name` from instance of `Import`.
    pub fn module_name(&self) -> &str {
        &self.module_name
    }

    /// Returns a [`ComponentInfo`] containing a summary of the import documentation,
    /// with the summary extracted from the rendered Markdown as HTML.
    pub fn info_rich_text(&self) -> ComponentInfo {
        let summary = self.docs.as_deref().map(to_html);

        ComponentInfo::new(self.name.clone(), summary)
    }

    /// Returns a [`ComponentInfo`] containing a summary of the import documentation,
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

    /// Registers the import if its path is found in the provided file registry.
    pub fn register(&mut self, file_registry: &HashSet<PathBuf>) -> bool {
        for file in file_registry {
            if file.ends_with(self.path()) {
                self.registered = true;
            }
        }

        self.registered
    }

    /// Get field `docs` from instance of `Import`.
    pub fn docs(&self) -> Option<&str> {
        self.docs.as_deref()
    }

    /// Get field `path` from instance of `Import`.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get field `name` from instance of `Import`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get field `registered` from instance of `Import`.
    pub fn registered(&self) -> bool {
        self.registered
    }
}

impl_eq_name!(Import::name);

/// Trait for registering imports in types that may reference other types.
/// This helps creating reference links in documentation by tracking which types are imported.
pub trait RegisterImports {
    /// Registers the imports used by the type.
    fn register_imports(&mut self, imports: &[Import]);

    /// Registers the types from the same module as the type.
    fn register_same_module_types(&mut self, type_names: &[String]);
}
