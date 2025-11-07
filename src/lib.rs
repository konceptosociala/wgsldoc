#![warn(missing_docs)]

use crate::generator::assets;
use fs_err as fs;
use generator::Generator;
use models::{import::RegisterImports, Wgsl};
use parser::WgslParser;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};
use utils::error::Error;

pub mod cli;
pub mod generator;
pub mod models;
pub mod parser;
pub mod utils;

pub type IconData = Vec<u8>;

pub struct Document {
    pkg_name: String,
    favicon: IconData,
    readme: Option<String>,
    file_registry: HashSet<PathBuf>,
    shaders: Vec<Wgsl>,
}

impl Document {
    pub fn new(pkg_name: impl Into<String>, paths: &[impl AsRef<Path>]) -> Result<Document, Error> {
        log::info!("Loading shaders...");

        let mut readme = None;
        let mut favicon = None;
        let mut file_registry = HashSet::new();
        let mut shaders = vec![];

        for path in paths {
            if path.as_ref().extension().is_some_and(|ext| ext == "wgsl") {
                file_registry.insert(path.as_ref().to_owned());

                if let Some(module_name) = path.as_ref().file_stem().and_then(|name| name.to_str())
                {
                    if module_name.starts_with('.') {
                        continue;
                    }

                    let shader = fs::read_to_string(path)?;
                    shaders.push(WgslParser::parse(module_name, &shader)?);
                }
            } else {
                match path.as_ref().file_name().and_then(|name| name.to_str()) {
                    Some("README.md") => readme = Some(fs::read_to_string(path)?),
                    Some("favicon.png") => favicon = Some(fs::read(path)?),
                    _ => {}
                }
            }
        }

        Ok(Document {
            pkg_name: pkg_name.into(),
            favicon: favicon.unwrap_or(assets::DEFAULT_FAVICON.to_vec()),
            readme,
            file_registry,
            shaders,
        })
    }

    pub fn open(
        pkg_name: impl Into<String>,
        directory: impl AsRef<Path>,
    ) -> Result<Document, Error> {
        let paths = fs::read_dir(directory.as_ref())?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .collect::<Vec<_>>();

        Document::new(pkg_name, &paths)
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

            for binding in &mut shader.bindings {
                binding.register_imports(&shader.imports);
                binding.register_same_module_types(&same_module_types);
            }

            for constant in &mut shader.constants {
                constant.register_imports(&shader.imports);
                constant.register_same_module_types(&same_module_types);
            }
        }

        RegisteredDocument {
            pkg_name: self.pkg_name,
            favicon: self.favicon,
            readme: self.readme,
            file_registry: self.file_registry,
            shaders: self.shaders,
        }
    }

    pub fn pkg_name(&self) -> &str {
        &self.pkg_name
    }

    pub fn favicon(&self) -> &IconData {
        self.favicon.as_ref()
    }

    pub fn shaders(&self) -> &[Wgsl] {
        &self.shaders
    }

    pub fn file_registry(&self) -> &HashSet<PathBuf> {
        &self.file_registry
    }

    pub fn readme(&self) -> Option<&str> {
        self.readme.as_deref()
    }
}

pub struct RegisteredDocument {
    pkg_name: String,
    favicon: IconData,
    readme: Option<String>,
    file_registry: HashSet<PathBuf>,
    shaders: Vec<Wgsl>,
}

impl RegisteredDocument {
    pub fn generate(
        &self,
        generator: &mut impl Generator,
        path: impl AsRef<Path>,
    ) -> Result<(), Error> {
        log::info!("Generating documentation...");

        // @/
        fs::create_dir_all(path.as_ref())?;

        // @/css
        let css_path = concat_path(&path, "css");
        fs::create_dir_all(&css_path)?;

        // @/css/pico.classless.min.css
        let pico_css_path = concat_path(&css_path, "pico.classless.min.css");
        fs::write(pico_css_path, assets::PICO_CSS)?;

        // @/css/highlight.min.css
        let highlight_css_path = concat_path(&css_path, "highlight.min.css");
        fs::write(highlight_css_path, assets::HIGHLIGHT_CSS)?;

        // @/js
        let js_path = concat_path(&path, "js");
        fs::create_dir_all(&js_path)?;

        // @/js/highlight.min.js
        let highlight_js_path = concat_path(&js_path, "highlight.min.js");
        fs::write(highlight_js_path, assets::HIGHLIGHT_JS)?;

        // @/js/wgsl.min.js
        let wgsl_js_path = concat_path(&js_path, "wgsl.min.js");
        fs::write(wgsl_js_path, assets::WGSL_JS)?;

        // @/favicon.png
        let favicon_path = concat_path(&path, "favicon.png");
        fs::write(favicon_path, self.favicon())?;

        // @/index.html
        let index_path = concat_path(&path, "index.html");
        let index_content = generator.generate_index(self.pkg_name(), path.as_ref(), self.readme());
        fs::write(index_path, index_content)?;

        // @/modules
        let modules_path = concat_path(&path, "modules");
        fs::create_dir_all(&modules_path)?;

        // @/modules/index.html
        let modules = self
            .shaders
            .iter()
            .map(|shader| shader.info_plain_text())
            .collect::<Vec<_>>();

        let modules_index_path = concat_path(&modules_path, "index.html");
        let modules_index_content =
            generator.generate_modules_index(self.pkg_name(), path.as_ref(), &modules);
        fs::write(modules_index_path, modules_index_content)?;

        // @/modules/<module_name>/index.html
        for shader in &self.shaders {
            let module_path = concat_path(&modules_path, &shader.module_name);
            fs::create_dir_all(&module_path)?;

            let module_index_path = concat_path(&module_path, "index.html");
            let module_content = generator.generate_module(self.pkg_name(), path.as_ref(), shader);

            fs::write(module_index_path, module_content)?;

            // @/modules/<module_name>/fn.<function_name>.html
            for function in &shader.functions {
                let function_path =
                    concat_path(&module_path, &format!("fn.{}.html", function.name()));
                let function_content = generator.generate_fn(
                    self.pkg_name(),
                    path.as_ref(),
                    function,
                    &shader.imports,
                );

                fs::write(function_path, function_content)?;
            }

            // @/modules/<module_name>/struct.<structure_name>.html
            for structure in &shader.structures {
                let structure_path =
                    concat_path(&module_path, &format!("struct.{}.html", structure.name()));
                let structure_content = generator.generate_struct(
                    self.pkg_name(),
                    path.as_ref(),
                    structure,
                    &shader.imports,
                );

                fs::write(structure_path, structure_content)?;
            }
        }

        // @/source/<module_name>.html
        let source_path = concat_path(&path, "source");
        fs::create_dir_all(&source_path)?;

        for shader in &self.shaders {
            let source_file_path =
                concat_path(&source_path, &format!("{}.html", shader.module_name));
            let source_content = generator.generate_source(self.pkg_name(), path.as_ref(), shader);

            fs::write(source_file_path, source_content)?;
        }

        Ok(())
    }

    pub fn pkg_name(&self) -> &str {
        &self.pkg_name
    }

    pub fn shaders(&self) -> &[Wgsl] {
        &self.shaders
    }

    pub fn file_registry(&self) -> &HashSet<PathBuf> {
        &self.file_registry
    }

    pub fn readme(&self) -> Option<&str> {
        self.readme.as_deref()
    }

    pub fn favicon(&self) -> &IconData {
        self.favicon.as_ref()
    }
}

fn concat_path(path: impl AsRef<Path>, filename: &str) -> PathBuf {
    let mut buf = path.as_ref().to_path_buf();
    buf.push(filename);

    buf
}
