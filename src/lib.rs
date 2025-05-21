use std::{
    collections::HashSet, 
    fs, 
    path::{Path, PathBuf},
};

use models::{
    import::RegisterImports, 
    Wgsl,
};
use parser::WgslParser;
use utils::error::Error;

pub mod cli;
pub mod generator;
pub mod models;
pub mod parser;
pub mod utils;

pub struct Document {
    file_registry: HashSet<PathBuf>,
    shaders: Vec<Wgsl>
}

impl Document {
    pub fn new(paths: &[impl AsRef<Path>]) -> Result<Document, Error> {
        log::info!("Loading shaders...");

        let mut file_registry = HashSet::new();
        let mut shaders = vec![];

        for path in paths {
            file_registry.insert(path.as_ref().to_owned());

            let shader = fs::read_to_string(path)?;
            shaders.push(WgslParser::parse(&shader)?);
        }

        Ok(Document { file_registry, shaders })
    }

    pub fn open(directory: impl AsRef<Path>) -> Result<Document, Error> {
        let paths = fs::read_dir(directory)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .collect::<Vec<_>>();

        Document::new(&paths)
    }

    pub fn register(mut self) -> RegisteredDocument {
        log::info!("Registering document...");

        for shader in &mut self.shaders {
            for import in &mut shader.imports {
                import.register(&self.file_registry);
            }

            let same_module_types = shader
                .structures
                .iter()
                .map(|s| s.name().to_owned())
                .collect::<Vec<_>>();

            for structure in &mut shader.structures {
                structure.register_imports(&shader.imports);
                structure.register_same_module_types(&same_module_types);
            }

            for function in &mut shader.functions {
                function.register_imports(&shader.imports);
                function.register_same_module_types(&same_module_types);
            }
        }

        RegisteredDocument {
            file_registry: self.file_registry,
            shaders: self.shaders,
        }
    }
    
    pub fn shaders(&self) -> &[Wgsl] {
        &self.shaders
    }
    
    pub fn file_registry(&self) -> &HashSet<PathBuf> {
        &self.file_registry
    }
}

pub struct RegisteredDocument {
    file_registry: HashSet<PathBuf>,
    shaders: Vec<Wgsl>
}

impl RegisteredDocument {
    pub fn shaders(&self) -> &[Wgsl] {
        &self.shaders
    }
    
    pub fn file_registry(&self) -> &HashSet<PathBuf> {
        &self.file_registry
    }
}