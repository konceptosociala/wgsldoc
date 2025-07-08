use std::{
    collections::HashSet, 
    path::{Path, PathBuf},
};

use serde::Serialize;

use crate::{impl_eq_name, models::ComponentInfo, utils::html::to_html};

#[derive(Debug, Clone, Serialize)]
pub struct Import {
    docs: Option<String>,
    path: PathBuf,
    module_name: String,
    name: String,
    registered: bool,
}

impl Import {
    pub fn new(
        docs: Option<String>,
        path: PathBuf,
        name: String
    ) -> Import {
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

    pub fn register(&mut self, file_registry: &HashSet<PathBuf>) -> bool {
        for file in file_registry {
            if file.ends_with(self.path()) {
                self.registered = true;
            }
        }

        self.registered
    }
    
    pub fn docs(&self) -> Option<&str> {
        self.docs.as_deref()
    }
    
    pub fn path(&self) -> &Path {
        &self.path
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn registered(&self) -> bool {
        self.registered
    }
}

impl_eq_name!(Import::name);

pub trait RegisterImports {
    fn register_imports(&mut self, imports: &[Import]);

    fn register_same_module_types(&mut self, type_names: &[String]);
}