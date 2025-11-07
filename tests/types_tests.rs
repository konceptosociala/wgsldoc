use std::str::FromStr;
use wgsldoc::models::types::{ImportModule, PathType, Primitive, Type, Vector, VectorDimension};

#[test]
fn test_primitive_display() {
    assert_eq!(format!("{}", Primitive::Bool), "bool");
    assert_eq!(format!("{}", Primitive::Float32), "f32");
    assert_eq!(format!("{}", Primitive::Float64), "f64");
    assert_eq!(format!("{}", Primitive::Uint8), "u8");
    assert_eq!(format!("{}", Primitive::Uint16), "u16");
    assert_eq!(format!("{}", Primitive::Uint32), "u32");
    assert_eq!(format!("{}", Primitive::Uint64), "u64");
    assert_eq!(format!("{}", Primitive::Sint8), "i8");
    assert_eq!(format!("{}", Primitive::Sint16), "i16");
    assert_eq!(format!("{}", Primitive::Sint32), "i32");
    assert_eq!(format!("{}", Primitive::Sint64), "i64");
}

#[test]
fn test_primitive_from_str() {
    assert!(matches!(Primitive::from_str("bool"), Ok(Primitive::Bool)));
    assert!(matches!(Primitive::from_str("f32"), Ok(Primitive::Float32)));
    assert!(matches!(Primitive::from_str("f64"), Ok(Primitive::Float64)));
    assert!(matches!(Primitive::from_str("u32"), Ok(Primitive::Uint32)));
    assert!(matches!(Primitive::from_str("i32"), Ok(Primitive::Sint32)));
    assert!(Primitive::from_str("invalid").is_err());
}

#[test]
fn test_vector_display() {
    let vec2 = Vector::new(VectorDimension::D2, Primitive::Float32);
    assert_eq!(format!("{}", vec2), "vec2&lt;f32&gt;");

    let vec3 = Vector::new(VectorDimension::D3, Primitive::Float32);
    assert_eq!(format!("{}", vec3), "vec3&lt;f32&gt;");

    let vec4 = Vector::new(VectorDimension::D4, Primitive::Float32);
    assert_eq!(format!("{}", vec4), "vec4&lt;f32&gt;");
}

#[test]
fn test_vector_dimension_from_str() {
    assert!(matches!(
        VectorDimension::from_str("2"),
        Ok(VectorDimension::D2)
    ));
    assert!(matches!(
        VectorDimension::from_str("3"),
        Ok(VectorDimension::D3)
    ));
    assert!(matches!(
        VectorDimension::from_str("4"),
        Ok(VectorDimension::D4)
    ));
    assert!(VectorDimension::from_str("5").is_err());
}

#[test]
fn test_path_type_new() {
    let path = PathType::new(None, "Camera".to_string());
    assert_eq!(path.name(), "Camera");
    assert!(path.module().is_none());
    assert_eq!(*path.import_module(), ImportModule::Undefined);
}

#[test]
fn test_path_type_with_module() {
    let path = PathType::new(Some("Utils".to_string()), "Camera".to_string());
    assert_eq!(path.name(), "Camera");
    assert_eq!(path.module(), Some("Utils"));
    assert_eq!(*path.import_module(), ImportModule::Undefined);
}

#[test]
fn test_rendered_type_primitive() {
    let ty = Type::Primitive(Primitive::Float32);
    let rendered = ty.rendered_type(&[], false);
    assert_eq!(rendered.name, "f32");
    assert!(!rendered.is_function_pointer);
    assert!(rendered.import.is_none());
}

#[test]
fn test_rendered_type_vector() {
    let ty = Type::Vector(Vector::new(VectorDimension::D3, Primitive::Float32));
    let rendered = ty.rendered_type(&[], false);
    assert_eq!(rendered.name, "vec3&lt;f32&gt;");
    assert!(!rendered.is_function_pointer);
}

#[test]
fn test_rendered_type_path() {
    let ty = Type::Path(PathType::new(None, "Camera".to_string()));
    let rendered = ty.rendered_type(&[], false);
    assert_eq!(rendered.name, "Camera");
    assert!(rendered.module.is_none());
    assert!(!rendered.is_this);
}

#[test]
fn test_rendered_type_function_pointer() {
    let ty = Type::Primitive(Primitive::Float32);
    let rendered = ty.rendered_type(&[], true);
    assert!(rendered.is_function_pointer);
}

#[test]
fn test_import_module_equality() {
    assert_eq!(ImportModule::Undefined, ImportModule::Undefined);
    assert_eq!(
        ImportModule::Named("Test".to_string()),
        ImportModule::Named("Test".to_string())
    );
    assert_eq!(ImportModule::This, ImportModule::This);
    assert_ne!(ImportModule::Undefined, ImportModule::This);
}

#[test]
fn test_type_default() {
    let ty = Type::default();
    match ty {
        Type::Primitive(p) => match p {
            Primitive::Sint32 => {}
            _ => panic!("Expected default to be Sint32"),
        },
        _ => panic!("Expected default type to be Primitive"),
    }
}
