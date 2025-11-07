use super::import::{Import, RegisterImports};
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug)]
pub enum Type {
    Primitive(Primitive),
    Vector(Vector),
    Path(PathType),
}

impl Type {
    pub fn rendered_type(&self, imports: &[Import], is_fn_ptr: bool) -> RenderedType {
        match self {
            Type::Primitive(p) => RenderedType {
                name: p.to_string(),
                is_function_pointer: is_fn_ptr,
                ..Default::default()
            },
            Type::Vector(v) => RenderedType {
                name: v.to_string(),
                is_function_pointer: is_fn_ptr,
                ..Default::default()
            },
            Type::Path(path) => {
                let mut rty = RenderedType {
                    name: path.name().to_string(),
                    module: path.module().map(|s| s.to_string()),
                    is_function_pointer: is_fn_ptr,
                    ..Default::default()
                };

                match path.import_module() {
                    ImportModule::Named(name) => {
                        rty.import = imports
                            .iter()
                            .find(|i| i.name() == name)
                            .map(|i| i.module_name().to_string());
                    }
                    ImportModule::This => {
                        rty.is_this = true;
                    }
                    _ => {}
                }

                rty
            }
        }
    }
}

impl Default for Type {
    fn default() -> Self {
        Type::Primitive(Primitive::default())
    }
}

#[derive(Debug, Default)]
pub enum Primitive {
    Bool,
    Float32,
    Float64,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Sint8,
    Sint16,
    #[default]
    Sint32,
    Sint64,
}

impl Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::Bool => write!(f, "bool"),
            Primitive::Float32 => write!(f, "f32"),
            Primitive::Float64 => write!(f, "f64"),
            Primitive::Uint8 => write!(f, "u8"),
            Primitive::Uint16 => write!(f, "u16"),
            Primitive::Uint32 => write!(f, "u32"),
            Primitive::Uint64 => write!(f, "u64"),
            Primitive::Sint8 => write!(f, "i8"),
            Primitive::Sint16 => write!(f, "i16"),
            Primitive::Sint32 => write!(f, "i32"),
            Primitive::Sint64 => write!(f, "i64"),
        }
    }
}

#[derive(Debug, Default)]
pub struct Vector {
    dimension: VectorDimension,
    ty: Primitive,
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.dimension {
            VectorDimension::D2 => write!(f, "vec2&lt;{}&gt;", self.ty),
            VectorDimension::D3 => write!(f, "vec3&lt;{}&gt;", self.ty),
            VectorDimension::D4 => write!(f, "vec4&lt;{}&gt;", self.ty),
        }
    }
}

impl Vector {
    pub fn new(dimension: VectorDimension, ty: Primitive) -> Vector {
        Vector { dimension, ty }
    }

    pub fn vector_type(&self) -> &Primitive {
        &self.ty
    }

    pub fn dimension(&self) -> &VectorDimension {
        &self.dimension
    }
}

#[derive(Debug, Default)]
pub enum VectorDimension {
    D2,
    #[default]
    D3,
    D4,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum ImportModule {
    #[default]
    Undefined,
    Named(String),
    This,
}

#[derive(Debug, Clone)]
pub struct PathType {
    module: Option<String>,
    name: String,
    import_module: ImportModule,
}

impl PathType {
    pub fn new(module: Option<String>, name: String) -> PathType {
        PathType {
            module,
            name,
            import_module: ImportModule::Undefined,
        }
    }

    pub fn module(&self) -> Option<&str> {
        self.module.as_deref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn import_module(&self) -> &ImportModule {
        &self.import_module
    }
}

impl RegisterImports for PathType {
    fn register_imports(&mut self, imports: &[Import]) {
        if self.import_module != ImportModule::Undefined {
            return;
        }

        if let Some(module_name) = &self.module {
            for import in imports {
                if import.registered() && import.name() == module_name {
                    self.import_module = ImportModule::Named(import.name().to_owned());
                }
            }
        }
    }

    fn register_same_module_types(&mut self, type_names: &[String]) {
        if self.import_module != ImportModule::Undefined {
            return;
        }

        for type_name in type_names {
            if *type_name == self.name {
                self.import_module = ImportModule::This;
            }
        }
    }
}

#[derive(Serialize, Default, Debug)]
pub struct RenderedType {
    pub is_this: bool,
    pub is_function_pointer: bool,
    pub name: String,
    pub module: Option<String>,
    pub import: Option<String>,
}
