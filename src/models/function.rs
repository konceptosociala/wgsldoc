use crate::impl_eq_name;

use super::{import::{Import, RegisterImports}, types::{PathType, Primitive, Type, Vector}};

#[derive(Debug)]
pub struct Function {
    docs: Option<String>,
    name: String,
    args: Vec<Arg>,
    return_ty: Option<Type>
}

impl Function {
    pub fn new(
        docs: Option<String>, 
        name: String, 
        args: Vec<Arg>, 
        return_ty: Option<Type>,
    ) -> Function {
        Function { docs, name, args, return_ty }
    }
    
    pub fn docs(&self) -> Option<&str> {
        self.docs.as_deref()
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn args(&self) -> &[Arg] {
        &self.args
    }
    
    pub fn return_type(&self) -> Option<&Type> {
        self.return_ty.as_ref()
    }
}

impl RegisterImports for Function {
    fn register_imports(&mut self, imports: &[Import]) {
        for arg in &mut self.args {
            arg.register_imports(imports)
        }

        if let Some(Type::Path(path_type)) = &mut self.return_ty {
            path_type.register_imports(imports);
        }
    }

    fn register_same_module_types(&mut self, type_names: &[String]) {
        for arg in &mut self.args {
            arg.register_same_module_types(type_names);
        }

        if let Some(Type::Path(path_type)) = &mut self.return_ty {
            path_type.register_same_module_types(type_names);
        }
    }
}

impl_eq_name!(Function::name);

#[derive(Debug)]
pub struct Arg {
    docs: Option<String>,
    name: String,
    ty: FunctionType,
}

impl Arg {
    pub fn new(
        docs: Option<String>, 
        name: String, 
        ty: FunctionType,
    ) -> Arg {
        Arg { docs, name, ty }
    }
    
    pub fn docs(&self) -> Option<&str> {
        self.docs.as_deref()
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn argument_type(&self) -> &FunctionType {
        &self.ty
    }
}

impl RegisterImports for Arg {
    fn register_imports(&mut self, imports: &[Import]) {
        match &mut self.ty {
            FunctionType::FunctionPointer(Type::Path(ref mut path_type)) => {
                path_type.register_imports(imports);
            },
            FunctionType::Path(ref mut ty) => ty.register_imports(imports),
            _ => {},
        }
    }
    
    fn register_same_module_types(&mut self, type_names: &[String]) {
        match &mut self.ty {
            FunctionType::FunctionPointer(Type::Path(ref mut path_type)) => {
                path_type.register_same_module_types(type_names);
            },
            FunctionType::Path(ref mut ty) => ty.register_same_module_types(type_names),
            _ => {},
        }
    }
}

impl_eq_name!(Arg::name);

#[derive(Debug)]
pub enum FunctionType {
    Primitive(Primitive),
    Vector(Vector),
    Path(PathType),
    FunctionPointer(Type),
}

impl Default for FunctionType {
    fn default() -> Self {
        FunctionType::Primitive(Primitive::default())
    }
}