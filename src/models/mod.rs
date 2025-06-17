use function::Function;
use import::Import;
use serde::Serialize;
use structure::Structure;

pub mod structure;
pub mod import;
pub mod types;
pub mod function;

#[derive(Debug)]
pub struct Wgsl {
    pub module_name: String,
    pub global_docs: Option<String>,
    pub imports: Vec<Import>,
    pub functions: Vec<Function>,
    pub structures: Vec<Structure>,
    // TODO: add entry points
    // TODO: add builtin imports
    // TODO: add constants
    // TODO: add bindings
    // TODO: add enums
}

#[derive(Debug, Serialize)]
pub struct ModuleInfo {
    pub name: String,
    pub summary: Option<String>,
}

impl ModuleInfo {
    pub const SUMMARY_MAX_LENGTH: usize = 256;

    pub fn new(name: String, summary: Option<String>) -> Self {
        ModuleInfo { name, summary }
    }
}

impl Wgsl {
    pub fn module_info(&self) -> ModuleInfo {
        let summary = self.global_docs.as_deref().map(|docs| {
            let html = markdown::to_html(docs);
            let parsed = scraper::Html::parse_fragment(&html);

            let summary = parsed
                .root_element()
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();

            if summary.len() > ModuleInfo::SUMMARY_MAX_LENGTH {
                format!("{}...", &summary[..ModuleInfo::SUMMARY_MAX_LENGTH])
            } else {
                summary
            }
        });

        ModuleInfo::new(self.module_name.clone(), summary)
    }
}