use crate::models::{function::Function, import::Import, structure::Structure};

pub trait Generator {
    fn generate_fn(&mut self, function: &Function, imports: &[Import]) -> String;

    fn generate_struct(&mut self, structure: &Structure, imports: &[Import]) -> String;

    fn generate_readme(&mut self, input: &str) -> String;
}

// TODO: implement tera generator
pub struct TeraGenerator;

impl Generator for TeraGenerator {
    fn generate_fn(&mut self, function: &Function, imports: &[Import]) -> String {
        String::new()
    }

    fn generate_struct(&mut self, structure: &Structure, imports: &[Import]) -> String {
        String::new()
    }

    fn generate_readme(&mut self, input: &str) -> String {
        String::new()
    }
}