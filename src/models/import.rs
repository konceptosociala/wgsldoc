use std::{
    collections::HashSet, 
    hash::Hash, 
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Eq)]
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

    pub fn register(&mut self, file_registry: &HashSet<PathBuf>) -> bool {
        if file_registry.contains(self.path()) {
            self.registered = true;
        }

        self.registered()
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

impl PartialEq for Import {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path && self.name == other.name
    }
}

impl Hash for Import {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.path.hash(state);
        self.name.hash(state);
    }
}