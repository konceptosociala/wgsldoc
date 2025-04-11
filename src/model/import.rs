use std::{
    hash::Hash, 
    path::PathBuf,
};

#[derive(Debug, Clone)]
pub struct Import {
    pub docs: String,
    pub path: PathBuf,
    pub module_name: String,
}

impl PartialEq for Import {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path && self.module_name == other.module_name
    }
}

impl Hash for Import {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.path.hash(state);
        self.module_name.hash(state);
    }
}