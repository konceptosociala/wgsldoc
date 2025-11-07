//! The most important module, containing data structures for representing WGSL types.
//! Types used in functions, structures, bindings, and constants.
//! Used for parsing, processing, and generating documentation of types.

use super::import::{Import, RegisterImports};
use serde::Serialize;
use std::fmt::Display;

/// Represents a type in WGSL. Can be a primitive, vector, or path type.
#[derive(Debug)]
pub enum Type {
    /// A primitive type (e.g., `f32`, `i32`).
    Primitive(Primitive),
    /// A vector type (e.g., `vec2<T>`, `vec3<T>`, `vec4<T>`).
    Vector(Vector),
    /// A path type (e.g., `MyType`, `Module::MyType`).
    Path(PathType),
}

impl Type {
    /// Converts the [`Type`] into a [`RenderedType`] for documentation rendering.
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

/// Represents primitive WGSL types.
#[derive(Debug, Default)]
pub enum Primitive {
    /// A boolean type.
    Bool,
    /// A 32-bit floating-point type.
    Float32,
    /// A 64-bit floating-point type.
    Float64,
    /// An 8-bit unsigned integer type.
    Uint8,
    /// A 16-bit unsigned integer type.
    Uint16,
    /// A 32-bit unsigned integer type.
    Uint32,
    /// A 64-bit unsigned integer type.
    Uint64,
    /// An 8-bit signed integer type.
    Sint8,
    /// A 16-bit signed integer type.
    Sint16,
    /// A 32-bit signed integer type.
    #[default]
    Sint32,
    /// A 64-bit signed integer type.
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

/// Represents vector WGSL types.
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
    /// Creates a new Vector instance (usually from parsed elements).
    pub fn new(dimension: VectorDimension, ty: Primitive) -> Vector {
        Vector { dimension, ty }
    }

    /// Get field `type` from instance of `Vector`.
    pub fn vector_type(&self) -> &Primitive {
        &self.ty
    }

    /// Get field `dimension` from instance of `Vector`.
    pub fn dimension(&self) -> &VectorDimension {
        &self.dimension
    }
}

/// Represents the dimension of a vector type.
#[derive(Debug, Default)]
pub enum VectorDimension {
    /// A 2-dimensional vector.
    D2,
    /// A 3-dimensional vector.
    #[default]
    D3,
    /// A 4-dimensional vector.
    D4,
}

/// Structure inside a path type, indicating its import status, whether
/// it's imported from another module, defined in the same module or 
/// its origin is undefined.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum ImportModule {
    /// The import is undefined.
    #[default]
    Undefined,
    /// The import is named.
    Named(String),
    /// The import is from the current module.
    This,
}

/// Represents a path type in WGSL, which may include module information and import status.
#[derive(Debug, Clone)]
pub struct PathType {
    module: Option<String>,
    name: String,
    import_module: ImportModule,
}

impl PathType {
    /// Creates a new PathType instance (usually from parsed elements).
    pub fn new(module: Option<String>, name: String) -> PathType {
        PathType {
            module,
            name,
            import_module: ImportModule::Undefined,
        }
    }

    /// Get field `module` from instance of `PathType`.
    pub fn module(&self) -> Option<&str> {
        self.module.as_deref()
    }

    /// Get field `name` from instance of `PathType`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get field `import_module` from instance of `PathType`.
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

/// A serializable representation of a type for rendering purposes used in Tera.
#[derive(Serialize, Default, Debug)]
pub struct RenderedType {
    /// Indicates if the type is from the same module.
    pub is_this: bool,
    /// Indicates if the type is a function pointer (like ptr<function, T>).
    pub is_function_pointer: bool,
    /// The name of the type.
    pub name: String,
    /// The module from which the type is imported, 
    /// if any (used, if type has module, but import origin is undefined).
    pub module: Option<String>,
    /// The import module name, if any.
    pub import: Option<String>,
}
