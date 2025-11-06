use wgsldoc::models::{
    structure::{Structure, Field},
    function::{Function, Arg, FunctionType},
    import::Import,
    types::{Type, Primitive, PathType, Vector, VectorDimension},
    ComponentInfo,
};
use std::path::PathBuf;

#[test]
fn test_structure_new() {
    let structure = Structure::new(
        Some("Test struct".to_string()),
        "Point".to_string(),
        vec![],
    );
    assert_eq!(structure.name(), "Point");
    assert_eq!(structure.docs(), Some("Test struct"));
    assert_eq!(structure.fields().len(), 0);
}

#[test]
fn test_structure_with_fields() {
    let fields = vec![
        Field::new(None, "x".to_string(), Type::Primitive(Primitive::Float32)),
        Field::new(None, "y".to_string(), Type::Primitive(Primitive::Float32)),
    ];
    
    let structure = Structure::new(None, "Point".to_string(), fields);
    assert_eq!(structure.fields().len(), 2);
    assert_eq!(structure.fields()[0].name(), "x");
    assert_eq!(structure.fields()[1].name(), "y");
}

#[test]
fn test_structure_info_plain_text() {
    let structure = Structure::new(
        Some("A simple point structure".to_string()),
        "Point".to_string(),
        vec![],
    );
    
    let info = structure.info_plain_text();
    assert_eq!(info.name, "Point");
    assert!(info.summary.is_some());
}

#[test]
fn test_structure_info_rich_text() {
    let structure = Structure::new(
        Some("A **bold** structure".to_string()),
        "Point".to_string(),
        vec![],
    );
    
    let info = structure.info_rich_text();
    assert_eq!(info.name, "Point");
    assert!(info.summary.is_some());
}

#[test]
fn test_field_new() {
    let field = Field::new(
        Some("X coordinate".to_string()),
        "x".to_string(),
        Type::Primitive(Primitive::Float32),
    );
    
    assert_eq!(field.name(), "x");
    assert_eq!(field.docs(), Some("X coordinate"));
}

#[test]
fn test_function_new() {
    let function = Function::new(
        Some("Adds two numbers".to_string()),
        "add".to_string(),
        vec![],
        Some(Type::Primitive(Primitive::Sint32)),
    );
    
    assert_eq!(function.name(), "add");
    assert_eq!(function.docs(), Some("Adds two numbers"));
    assert!(function.return_type().is_some());
}

#[test]
fn test_function_with_args() {
    let args = vec![
        Arg::new(
            None,
            "a".to_string(),
            FunctionType::Primitive(Primitive::Sint32),
        ),
        Arg::new(
            None,
            "b".to_string(),
            FunctionType::Primitive(Primitive::Sint32),
        ),
    ];
    
    let function = Function::new(
        None,
        "add".to_string(),
        args,
        Some(Type::Primitive(Primitive::Sint32)),
    );
    
    assert_eq!(function.args().len(), 2);
    assert_eq!(function.args()[0].name(), "a");
    assert_eq!(function.args()[1].name(), "b");
}

#[test]
fn test_function_info_plain_text() {
    let function = Function::new(
        Some("Calculates distance".to_string()),
        "distance".to_string(),
        vec![],
        Some(Type::Primitive(Primitive::Float32)),
    );
    
    let info = function.info_plain_text();
    assert_eq!(info.name, "distance");
    assert!(info.summary.is_some());
}

#[test]
fn test_function_info_rich_text() {
    let function = Function::new(
        Some("A **fast** function".to_string()),
        "compute".to_string(),
        vec![],
        None,
    );
    
    let info = function.info_rich_text();
    assert_eq!(info.name, "compute");
    assert!(info.summary.is_some());
}

#[test]
fn test_arg_new() {
    let arg = Arg::new(
        Some("Input value".to_string()),
        "x".to_string(),
        FunctionType::Primitive(Primitive::Float32),
    );
    
    assert_eq!(arg.name(), "x");
    assert_eq!(arg.docs(), Some("Input value"));
}

#[test]
fn test_arg_with_vector_type() {
    let arg = Arg::new(
        None,
        "position".to_string(),
        FunctionType::Vector(Vector::new(VectorDimension::D3, Primitive::Float32)),
    );
    
    assert_eq!(arg.name(), "position");
}

#[test]
fn test_arg_with_path_type() {
    let arg = Arg::new(
        None,
        "camera".to_string(),
        FunctionType::Path(PathType::new(Some("Utils".to_string()), "Camera".to_string())),
    );
    
    assert_eq!(arg.name(), "camera");
}

#[test]
fn test_arg_with_function_pointer() {
    let arg = Arg::new(
        None,
        "ptr".to_string(),
        FunctionType::FunctionPointer(Type::Primitive(Primitive::Float32)),
    );
    
    assert_eq!(arg.name(), "ptr");
}

#[test]
fn test_import_new() {
    let import = Import::new(
        Some("Utility functions".to_string()),
        PathBuf::from("utils.wgsl"),
        "Utils".to_string(),
    );
    
    assert_eq!(import.name(), "Utils");
    assert_eq!(import.module_name(), "utils");
    assert!(!import.registered());
}

#[test]
fn test_import_info_plain_text() {
    let import = Import::new(
        Some("Helper module".to_string()),
        PathBuf::from("helpers.wgsl"),
        "Helpers".to_string(),
    );
    
    let info = import.info_plain_text();
    assert_eq!(info.name, "Helpers");
}

#[test]
fn test_import_info_rich_text() {
    let import = Import::new(
        Some("**Important** module".to_string()),
        PathBuf::from("module.wgsl"),
        "Module".to_string(),
    );
    
    let info = import.info_rich_text();
    assert_eq!(info.name, "Module");
}

#[test]
fn test_component_info_new() {
    let info = ComponentInfo::new("TestComponent".to_string(), Some("Summary".to_string()));
    assert_eq!(info.name, "TestComponent");
    assert_eq!(info.summary, Some("Summary".to_string()));
}

#[test]
fn test_component_info_without_summary() {
    let info = ComponentInfo::new("TestComponent".to_string(), None);
    assert_eq!(info.name, "TestComponent");
    assert!(info.summary.is_none());
}
