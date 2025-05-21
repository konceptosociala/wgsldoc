use linked_hash_set::LinkedHashSet;

use crate::impl_eq_name;

use super::types::{PathType, Primitive, Type, Vector};

#[derive(Debug)]
pub struct Function {
    docs: Option<String>,
    name: String,
    args: LinkedHashSet<Arg>,
    return_ty: Option<Type>
}

impl Function {
    pub fn new(
        docs: Option<String>, 
        name: String, 
        args: LinkedHashSet<Arg>, 
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
    
    pub fn args(&self) -> &LinkedHashSet<Arg> {
        &self.args
    }
    
    pub fn return_type(&self) -> Option<&Type> {
        self.return_ty.as_ref()
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