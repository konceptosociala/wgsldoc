use std::{
    collections::HashSet, 
    path::{Path, PathBuf},
};

use markdown::to_html;

use crate::{impl_eq_name, models::ComponentInfo};

#[derive(Debug, Clone)]
pub struct Import {
    docs: Option<String>,
    path: PathBuf,
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
            docs,
            path,
            name,
            registered: false,
        }
    }

    pub fn info(&self) -> ComponentInfo {
        let summary = self.docs().map(|docs| {
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