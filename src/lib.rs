use std::{collections::HashSet, path::{Path, PathBuf}};

use models::Wgsl;

pub mod models;
pub mod parser;
pub mod generator;

pub struct Document {
    file_registry: HashSet<PathBuf>,
    shaders: Vec<Wgsl>
}

impl Document {
    pub fn new(shaders: &[impl AsRef<Path>]) -> Document {

        todo!("Document::new")
    }

    pub fn open(directory: impl AsRef<Path>) -> Document {

        todo!("Document::open")
    }
    
    pub fn shaders(&self) -> &[Wgsl] {
        &self.shaders
    }
    
    pub fn file_registry(&self) -> &HashSet<PathBuf> {
        &self.file_registry
    }
}