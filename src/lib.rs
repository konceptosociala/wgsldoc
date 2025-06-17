use std::{
    collections::HashSet, fs, path::{Path, PathBuf}
};

use generator::Generator;
use models::{
    import::RegisterImports, 
    Wgsl,
};
use parser::WgslParser;
use utils::error::Error;

use crate::generator::assets;

pub mod cli;
pub mod generator;
pub mod models;
pub mod parser;
pub mod utils;

pub struct Document {
    pkg_name: String,
    readme: Option<String>,
    file_registry: HashSet<PathBuf>,
    shaders: Vec<Wgsl>
}

impl Document {
    pub fn new(
        pkg_name: impl Into<String>, 
        paths: &[impl AsRef<Path>],
    ) -> Result<Document, Error> {
        log::info!("Loading shaders...");

        let mut readme = None;
        let mut file_registry = HashSet::new();
        let mut shaders = vec![];

        for path in paths {
            if path.as_ref().extension().is_some_and(|ext| ext == "wgsl") {
                file_registry.insert(path.as_ref().to_owned());

                let shader = fs::read_to_string(path)?;
                shaders.push(WgslParser::parse(&shader)?);
            } else if path.as_ref().file_name().is_some_and(|name| name == "README.md") {
                readme = Some(fs::read_to_string(path)?);
            }
        }

        Ok(Document {
            pkg_name: pkg_name.into(),
            readme,
            file_registry,
            shaders,
        })
    }

    pub fn open(
        pkg_name: impl Into<String>,
        directory: impl AsRef<Path>,
    ) -> Result<Document, Error> {
        let paths = fs::read_dir(directory)?
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
        }

        RegisteredDocument {
            pkg_name: self.pkg_name,
            readme: self.readme,
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
    
    pub fn readme(&self) -> Option<&str> {
        self.readme.as_deref()
    }
}

pub struct RegisteredDocument {
    pkg_name: String,
    readme: Option<String>,
    file_registry: HashSet<PathBuf>,
    shaders: Vec<Wgsl>
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

        // @/index.html
        let index_path = concat_path(&path, "index.html");
        let index_content = generator.generate_index(self.pkg_name(), path.as_ref(), self.readme());

        // @/modules
        let modules_path = concat_path(&path, "modules");
        fs::create_dir_all(&modules_path)?;

        // @/modules/index.html
        let modules_index_path = concat_path(&modules_path, "index.html");
        let modules_index_content = generator.generate_modules_index(self.pkg_name(), path.as_ref(), self.readme());

        // Write to disk
        fs::write(pico_css_path, assets::PICO_CSS)?;
        fs::write(index_path, index_content)?;
        fs::write(modules_index_path, modules_index_content)?;

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
}

fn concat_path(
    path: impl AsRef<Path>, 
    filename: &str,
) -> PathBuf {
    let mut buf = path.as_ref().to_path_buf();
    buf.push(filename);

    buf
}