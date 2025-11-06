use std::path::Path;
use tera::Tera;
use crate::{
    models::{
        function::Function, 
        import::Import, 
        structure::Structure, 
        ComponentInfo, Wgsl,
    }, 
    utils::html::to_html,
};

/// Assets module containing static files as constants.
pub mod assets {
    /// Pico.css stylesheet for minimal styling.
    pub const PICO_CSS: &str = include_str!("static/pico.classless.min.css");
    /// Highlight.js CSS for code highlighting.
    pub const HIGHLIGHT_CSS: &str = include_str!("static/highlight.min.css");
    /// Default favicon image in PNG format.
    pub const DEFAULT_FAVICON: &[u8] = include_bytes!("static/default_favicon.png");
    /// Highlight.js JavaScript for code highlighting.
    pub const HIGHLIGHT_JS: &str = include_str!("static/highlight.min.js");
    /// WGSL syntax highlighting JavaScript.
    pub const WGSL_JS: &str = include_str!("static/wgsl.min.js");
}

/// Trait for documentation generators.
pub trait Generator {
    /// Generates documentation for a function.
    fn generate_fn(
        &mut self, 
        pkg_name: &str,
        assets_subpath: impl AsRef<Path>,
        function: &Function, 
        imports: &[Import],
    ) -> String;

    /// Generates documentation for a structure.
    fn generate_struct(
        &mut self, 
        pkg_name: &str,
        assets_subpath: impl AsRef<Path>,
        structure: &Structure, 
        imports: &[Import],
    ) -> String;

    /// Generates the index page documentation.
    fn generate_index(
        &mut self, 
        pkg_name: &str,
        assets_subpath: impl AsRef<Path>,
        readme: Option<&str>,
    ) -> String;

    /// Generates the modules index documentation.
    fn generate_modules_index(
        &mut self, 
        pkg_name: &str,
        assets_subpath: impl AsRef<Path>,
        modules: &[ComponentInfo],
    ) -> String;

    /// Generates documentation for a module.
    fn generate_module(
        &mut self, 
        pkg_name: &str,
        assets_subpath: impl AsRef<Path>,
        shader: &Wgsl,
    ) -> String;

    /// Generates the source code documentation for a module.
    fn generate_source(
        &mut self,
        pkg_name: &str,
        assets_subpath: impl AsRef<Path>,
        shader: &Wgsl,
    ) -> String;
}

/// Generator implementation using Tera templates.
pub struct TeraGenerator {
    tera: Tera,
    base_url: Option<String>,
}

impl TeraGenerator {
    /// Macros template source.
    pub const MACROS: &str = include_str!("templates/macros.tera");
    /// Base HTML template source.
    pub const BASE_TEMPLATE: &str = include_str!("templates/base.html.tera");
    /// Index HTML template source.
    pub const INDEX_TEMPLATE: &str = include_str!("templates/index.html.tera");
    /// Modules HTML template source.
    pub const MODULES_TEMPLATE: &str = include_str!("templates/modules.html.tera");
    /// Module HTML template source.
    pub const MODULE_TEMPLATE: &str = include_str!("templates/module.html.tera");
    /// Source HTML template source.
    pub const SOURCE_TEMPLATE: &str = include_str!("templates/source.html.tera");
    /// Function HTML template source.
    pub const FN_TEMPLATE: &str = include_str!("templates/fn.html.tera");
    /// Structure HTML template source.
    pub const STRUCT_TEMPLATE: &str = include_str!("templates/struct.html.tera");

    /// Array of all template names and their sources.
    pub const TEMPLATES: [(&str, &str); 8] = [
        ("macros.tera", Self::MACROS),
        ("base.html.tera", Self::BASE_TEMPLATE),
        ("index.html.tera", Self::INDEX_TEMPLATE),
        ("modules.html.tera", Self::MODULES_TEMPLATE),
        ("module.html.tera", Self::MODULE_TEMPLATE),
        ("source.html.tera", Self::SOURCE_TEMPLATE),
        ("fn.html.tera", Self::FN_TEMPLATE),
        ("struct.html.tera", Self::STRUCT_TEMPLATE),
    ];

    /// Creates a new TeraGenerator with an optional base URL.
    pub fn new(base_url: Option<String>) -> Self {
        let mut tera = Tera::default();
        tera.add_raw_templates(Self::TEMPLATES).unwrap();

        TeraGenerator { tera, base_url }
    }
}

impl Generator for TeraGenerator {
    fn generate_fn(
        &mut self, 
        pkg_name: &str,
        assets_subpath: impl AsRef<Path>,
        function: &Function, 
        imports: &[Import],
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

        ctx.insert("function_info", &function.info_rich_text());
        ctx.insert("args", &function.rendered_args(imports));
        ctx.insert("return_type", &function.return_type()
            .map(|ty| ty.rendered_type(imports, false))
        );

        self.tera.render("fn.html.tera", &ctx).unwrap()
    }

    fn generate_struct(
        &mut self, 
        pkg_name: &str,
        assets_subpath: impl AsRef<Path>,
        structure: &Structure, 
        imports: &[Import],
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

        ctx.insert("structure_info", &structure.info_rich_text());
        ctx.insert("fields", &structure.rendered_fields(imports));

        self.tera.render("struct.html.tera", &ctx).unwrap()
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
        ctx.insert("imports", &shader.imports);

        let bindings = shader.bindings.iter()
            .map(|b| b.rendered(&shader.imports))
            .collect::<Vec<_>>();
        ctx.insert("bindings", &bindings);

        let constants = shader.constants.iter()
            .map(|c| c.rendered(&shader.imports))
            .collect::<Vec<_>>();
        ctx.insert("constants", &constants);

        let functions = shader.functions.iter()
            .map(|f| f.info_plain_text())
            .collect::<Vec<_>>();
        ctx.insert("functions", &functions);

        let structures = shader.structures.iter()
            .map(|s| s.info_plain_text())
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