use super::types::Type;

#[derive(Default, Debug)]
pub struct Structure {
    pub docs: String,
    pub name: String,
    pub fields: Vec<Field>,        
}

#[derive(Debug, Default)]
pub struct Field {
    pub docs: String,
    pub name: String,
    pub ty: Type,
}