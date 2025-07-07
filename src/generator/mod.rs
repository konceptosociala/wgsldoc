use std::path::Path;

use tera::Tera;

use crate::{models::{function::Function, import::Import, structure::Structure, ComponentInfo, Wgsl}, utils::html::to_html};

pub mod assets {
    pub const PICO_CSS: &str = include_str!("static/pico.classless.min.css");
    pub const HIGHLIGHT_CSS: &str = include_str!("static/highlight.min.css");
    pub const DEFAULT_FAVICON: &[u8] = include_bytes!("static/default_favicon.png");
    pub const HIGHLIGHT_JS: &str = include_str!("static/highlight.min.js");
    pub const WGSL_JS: &str = include_str!("static/wgsl.min.js");
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
        modules: &[ComponentInfo],
    ) -> String;

    fn generate_module(
        &mut self, 
        pkg_name: &str,
        assets_subpath: impl AsRef<Path>,
        shader: &Wgsl,
    ) -> String;

    fn generate_source(
        &mut self,
        pkg_name: &str,
        assets_subpath: impl AsRef<Path>,
        shader: &Wgsl,
    ) -> String;
}

pub struct TeraGenerator {
    tera: Tera,
    base_url: Option<String>,
}

impl TeraGenerator {
    pub const BASE_TEMPLATE: &str = include_str!("templates/base.html.tera");
    pub const INDEX_TEMPLATE: &str = include_str!("templates/index.html.tera");
    pub const MODULES_TEMPLATE: &str = include_str!("templates/modules.html.tera");
    pub const MODULE_TEMPLATE: &str = include_str!("templates/module.html.tera");
    pub const SOURCE_TEMPLATE: &str = include_str!("templates/source.html.tera");

    pub const TEMPLATES: [(&str, &str); 5] = [
        ("base.html.tera", Self::BASE_TEMPLATE),
        ("index.html.tera", Self::INDEX_TEMPLATE),
        ("modules.html.tera", Self::MODULES_TEMPLATE),
        ("module.html.tera", Self::MODULE_TEMPLATE),
        ("source.html.tera", Self::SOURCE_TEMPLATE),
    ];

    pub fn new(base_url: Option<String>) -> Self {
        let mut tera = Tera::default();
        tera.add_raw_templates(Self::TEMPLATES).unwrap();

        TeraGenerator { tera, base_url }
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
        
        if let Some(base_url) = &self.base_url {
            ctx.insert("assets_subpath", 
                base_url
                    .trim_end_matches('/')
            );
        } else {
            ctx.insert("assets_subpath",
                assets_subpath
                    .as_ref()
                    .to_str()
                    .unwrap_or("")
                    .trim_end_matches('/')
            );
        }

        if let Some(readme) = readme {
            ctx.insert("readme", &to_html(readme));
        }

        self.tera.render("index.html.tera", &ctx).unwrap()
    }

    fn generate_modules_index(
        &mut self, 
        pkg_name: &str,
        assets_subpath: impl AsRef<Path>,
        modules: &[ComponentInfo],
    ) -> String {
        let mut ctx = tera::Context::new();
        ctx.insert("pkg_name", pkg_name);

        if let Some(base_url) = &self.base_url {
            ctx.insert("assets_subpath", 
                base_url
                    .trim_end_matches('/')
            );
        } else {
            ctx.insert("assets_subpath",
                assets_subpath
                    .as_ref()
                    .to_str()
                    .unwrap_or("")
                    .trim_end_matches('/')
            );
        }

        ctx.insert("modules", modules);

        self.tera.render("modules.html.tera", &ctx).unwrap()
    }

    fn generate_module(
        &mut self, 
        pkg_name: &str,
        assets_subpath: impl AsRef<Path>,
        shader: &Wgsl,
    ) -> String {
        let mut ctx = tera::Context::new();
        ctx.insert("pkg_name", pkg_name);

        if let Some(base_url) = &self.base_url {
            ctx.insert("assets_subpath", 
                base_url
                    .trim_end_matches('/')
            );
        } else {
            ctx.insert("assets_subpath",
                assets_subpath
                    .as_ref()
                    .to_str()
                    .unwrap_or("")
                    .trim_end_matches('/')
            );
        }

        ctx.insert("source", &shader.module_name);

        ctx.insert("module", &shader.info_rich_text());

        let functions = shader.functions.iter()
            .map(|f| f.info())
            .collect::<Vec<_>>();
        ctx.insert("functions", &functions);

        let structures = shader.structures.iter()
            .map(|s| s.info())
            .collect::<Vec<_>>();
        ctx.insert("structures", &structures);

        self.tera.render("module.html.tera", &ctx).unwrap()
    }

    fn generate_source(
        &mut self, 
        pkg_name: &str,
        assets_subpath: impl AsRef<Path>,
        shader: &Wgsl,
    ) -> String {
        let mut ctx = tera::Context::new();
        ctx.insert("pkg_name", pkg_name);

        if let Some(base_url) = &self.base_url {
            ctx.insert("assets_subpath", 
                base_url
                    .trim_end_matches('/')
            );
        } else {
            ctx.insert("assets_subpath",
                assets_subpath
                    .as_ref()
                    .to_str()
                    .unwrap_or("")
                    .trim_end_matches('/')
            );
        }

        ctx.insert("module_name", &shader.module_name);
        ctx.insert("source_code", &shader.source_code);

        self.tera.render("source.html.tera", &ctx).unwrap()
    }
}