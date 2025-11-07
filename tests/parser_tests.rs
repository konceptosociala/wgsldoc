use wgsldoc::parser::WgslParser;

#[test]
fn test_parse_empty_shader() {
    let result = WgslParser::parse("empty", "");
    assert!(result.is_ok());
    let shader = result.unwrap();
    assert_eq!(shader.module_name, "empty");
    assert!(shader.functions.is_empty());
    assert!(shader.structures.is_empty());
    assert!(shader.imports.is_empty());
}

#[test]
fn test_parse_simple_function() {
    let shader_code = r#"
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
"#;

    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_ok());
    let shader = result.unwrap();
    assert_eq!(shader.functions.len(), 1);
    assert_eq!(shader.functions[0].name(), "add");
    assert_eq!(shader.functions[0].args().len(), 2);
}

#[test]
fn test_parse_function_with_docs() {
    let shader_code = r#"
/// Adds two numbers together
/// Returns the sum
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
"#;

    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_ok());
    let shader = result.unwrap();
    assert_eq!(shader.functions.len(), 1);
    assert!(shader.functions[0].docs().is_some());
    assert!(shader.functions[0]
        .docs()
        .unwrap()
        .contains("Adds two numbers"));
}

#[test]
fn test_parse_simple_struct() {
    let shader_code = r#"
struct Point {
    x: f32,
    y: f32,
}
"#;

    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_ok());
    let shader = result.unwrap();
    assert_eq!(shader.structures.len(), 1);
    assert_eq!(shader.structures[0].name(), "Point");
    assert_eq!(shader.structures[0].fields().len(), 2);
}

#[test]
fn test_parse_struct_with_docs() {
    let shader_code = r#"
/// A 2D point in space
struct Point {
    /// X coordinate
    x: f32,
    /// Y coordinate
    y: f32,
}
"#;

    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_ok());
    let shader = result.unwrap();
    assert_eq!(shader.structures.len(), 1);
    assert!(shader.structures[0].docs().is_some());
    assert!(shader.structures[0].docs().unwrap().contains("2D point"));
}

#[test]
fn test_parse_import() {
    let shader_code = r#"
#import utils.wgsl as Utils;
"#;

    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_ok());
    let shader = result.unwrap();
    assert_eq!(shader.imports.len(), 1);
    assert_eq!(shader.imports[0].name(), "Utils");
}

#[test]
fn test_parse_import_with_docs() {
    let shader_code = r#"
/// Utils import with some markdown
#import utils.wgsl as Utils;
"#;

    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_ok());
    let shader = result.unwrap();
    assert_eq!(shader.imports.len(), 1);
    assert!(shader.imports[0].docs().is_some());
}

#[test]
fn test_parse_vector_types() {
    let shader_code = r#"
struct VectorData {
    pos: vec3<f32>,
    color: vec4<f32>,
}
"#;

    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_ok());
    let shader = result.unwrap();
    assert_eq!(shader.structures.len(), 1);
    assert_eq!(shader.structures[0].fields().len(), 2);
}

#[test]
fn test_parse_function_with_vector_args() {
    let shader_code = r#"
fn distance(a: vec3<f32>, b: vec3<f32>) -> f32 {
    let diff = a - b;
    return length(diff);
}
"#;

    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_ok());
    let shader = result.unwrap();
    assert_eq!(shader.functions.len(), 1);
    assert_eq!(shader.functions[0].args().len(), 2);
}

#[test]
fn test_parse_function_pointer() {
    let shader_code = r#"
fn set_value(ptr: ptr<function, f32>) {
    *ptr = 42.0;
}
"#;

    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_ok());
    let shader = result.unwrap();
    assert_eq!(shader.functions.len(), 1);
    assert_eq!(shader.functions[0].args().len(), 1);
}

#[test]
fn test_parse_global_docs() {
    let shader_code = r#"
//! This is a global module documentation
//! It spans multiple lines

fn test() {}
"#;

    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_ok());
    let shader = result.unwrap();
    assert!(shader.global_docs.is_some());
    assert!(shader
        .global_docs
        .unwrap()
        .contains("global module documentation"));
}

#[test]
fn test_parse_path_type() {
    let shader_code = r#"
fn use_camera(camera: Utils::Camera) {}
"#;

    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_ok());
    let shader = result.unwrap();
    assert_eq!(shader.functions.len(), 1);
    assert_eq!(shader.functions[0].args().len(), 1);
}

#[test]
fn test_parse_multiple_functions() {
    let shader_code = r#"
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn subtract(a: i32, b: i32) -> i32 {
    return a - b;
}

fn multiply(a: i32, b: i32) -> i32 {
    return a * b;
}
"#;

    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_ok());
    let shader = result.unwrap();
    assert_eq!(shader.functions.len(), 3);
}

#[test]
fn test_parse_multiple_structs() {
    let shader_code = r#"
struct Point {
    x: f32,
    y: f32,
}

struct Color {
    r: f32,
    g: f32,
    b: f32,
}
"#;

    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_ok());
    let shader = result.unwrap();
    assert_eq!(shader.structures.len(), 2);
}

#[test]
fn test_parse_complex_shader() {
    let shader_code = r#"
//! Ray tracing module

/// Ray structure
struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
}

/// Create a ray from coordinates
fn on_coords(pos: vec2<u32>) -> Ray {
    return Ray(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(1.0, 0.0, 0.0));
}
"#;

    let result = WgslParser::parse("ray", shader_code);
    assert!(result.is_ok());
    let shader = result.unwrap();
    assert_eq!(shader.module_name, "ray");
    assert!(shader.global_docs.is_some());
    assert_eq!(shader.structures.len(), 1);
    assert_eq!(shader.functions.len(), 1);
}

#[test]
fn test_parse_all_primitive_types() {
    let shader_code = r#"
struct Primitives {
    a: bool,
    b: f32,
    c: f64,
    d: u8,
    e: u16,
    f: u32,
    g: u64,
    h: i8,
    i: i16,
    j: i32,
    k: i64,
}
"#;

    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_ok());
    let shader = result.unwrap();
    assert_eq!(shader.structures.len(), 1);
    assert_eq!(shader.structures[0].fields().len(), 11);
}

#[test]
fn test_parse_all_vector_dimensions() {
    let shader_code = r#"
struct Vectors {
    v2: vec2<f32>,
    v3: vec3<f32>,
    v4: vec4<f32>,
}
"#;

    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_ok());
    let shader = result.unwrap();
    assert_eq!(shader.structures.len(), 1);
    assert_eq!(shader.structures[0].fields().len(), 3);
}
