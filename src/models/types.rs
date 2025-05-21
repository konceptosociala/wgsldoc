use super::import::{Import, RegisterImports};

#[derive(Debug)]
pub enum Type {
    Primitive(Primitive),
    Vector(Vector),
    Path(PathType),
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

#[derive(Debug, Default)]
pub struct Vector {
    dimension: VectorDimension,
    ty: Primitive,
}

impl Vector {
    pub fn new(
        dimension: VectorDimension, 
        ty: Primitive,
    ) -> Vector {
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

#[derive(Debug, Default, PartialEq, Eq)]
pub enum ImportModule {
    #[default]
    Undefined,
    Named(String),
    This,
}

#[derive(Debug)]
pub struct PathType {
    module: Option<String>,
    name: String,
    import_module: ImportModule,
}

impl PathType {
    pub fn new(
        module: Option<String>, 
        name: String, 
    ) -> PathType {
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