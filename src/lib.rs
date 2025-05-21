use std::{
    collections::HashSet, 
    path::{Path, PathBuf},
};

use models::Wgsl;

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
    pub fn new(_shaders: &[impl AsRef<Path>]) -> Document {

        todo!("Document::new")
    }

    pub fn open(_directory: impl AsRef<Path>) -> Document {

        todo!("Document::open")
    }
    
    pub fn shaders(&self) -> &[Wgsl] {
        &self.shaders
    }
    
    pub fn file_registry(&self) -> &HashSet<PathBuf> {
        &self.file_registry
    }
}