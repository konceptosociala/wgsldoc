use std::path::Path;

use tera::Tera;

use crate::models::{function::Function, import::Import, structure::Structure};

pub mod assets {
    pub const PICO_CSS: &str = include_str!("static/pico.classless.min.css");
}

pub trait Generator {
    fn generate_fn(
        &mut self, 
        pkg_name: &str,
        assets_subpath: impl AsRef<Path>,
        function: &Function, 
        imports: &[Import],
    ) -> String;

    fn generate_struct(
        &mut self, 
        pkg_name: &str,
        assets_subpath: impl AsRef<Path>,
        structure: &Structure, 
        imports: &[Import],
    ) -> String;

    fn generate_index(
        &mut self, 
        pkg_name: &str,
        assets_subpath: impl AsRef<Path>,
        readme: Option<&str>,
    ) -> String;

    fn generate_modules_index(
        &mut self, 
        pkg_name: &str,
        assets_subpath: impl AsRef<Path>,
        readme: Option<&str>,
    ) -> String;
}

pub struct TeraGenerator {
    tera: Tera,
}

impl TeraGenerator {
    pub const BASE_TEMPLATE: &str = include_str!("templates/base.html.tera");
    pub const INDEX_TEMPLATE: &str = include_str!("templates/index.html.tera");

    pub const TEMPLATES: [(&str, &str); 2] = [
        ("base.html.tera", Self::BASE_TEMPLATE),
        ("index.html.tera", Self::INDEX_TEMPLATE),
    ];

    pub fn new() -> Self {
        let mut tera = Tera::default();
        tera.add_raw_templates(Self::TEMPLATES).unwrap();

        TeraGenerator { tera }
    }
}

impl Default for TeraGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl Generator for TeraGenerator {
    fn generate_fn(
        &mut self, 
        _pkg_name: &str,
        _assets_subpath: impl AsRef<Path>,
        _function: &Function, 
        _imports: &[Import],
    ) -> String {
        // TODO: Implement function generation logic
        String::from("generate_fn is not implemented yet")
    }

    fn generate_struct(
        &mut self, 
        _pkg_name: &str,
        _assets_subpath: impl AsRef<Path>,
        _structure: &Structure, 
        _imports: &[Import],
    ) -> String {
        // TODO: Implement structure generation logic
        String::from("generate_struct is not implemented yet")
    }

    fn generate_index(
        &mut self, 
        pkg_name: &str,
        assets_subpath: impl AsRef<Path>,
        readme: Option<&str>,
    ) -> String {
        let mut ctx = tera::Context::new();
        ctx.insert("pkg_name", pkg_name);
        ctx.insert("assets_subpath", 
            assets_subpath
                .as_ref()
                .to_str()
                .unwrap_or("")
                .trim_end_matches('/')
        );

        if let Some(readme) = readme {
            ctx.insert("readme", &markdown::to_html(readme));
        }

        self.tera.render("index.html.tera", &ctx).unwrap()
    }

    fn generate_modules_index(
        &mut self, 
        pkg_name: &str,
        assets_subpath: impl AsRef<Path>,
        readme: Option<&str>,
    ) -> String {
        // TODO: Implement modules index generation logic
        String::from("generate_modules_index is not implemented yet")
    }
}