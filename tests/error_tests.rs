use fs_err as fs;
use std::path::PathBuf;
use wgsldoc::parser::WgslParser;
use wgsldoc::Document;

#[test]
fn test_parse_invalid_syntax() {
    let shader_code = "fn invalid syntax here";
    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_err());
}

#[test]
fn test_parse_unclosed_struct() {
    let shader_code = r#"
struct Point {
    x: f32,
    y: f32
"#;
    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_type() {
    let shader_code = r#"
struct Point {
    x: invalid_type,
}
"#;
    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_ok());
}

#[test]
fn test_parse_empty_function_no_body() {
    let shader_code = "fn test()";
    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_err());
}

#[test]
fn test_parse_duplicate_structure_names() {
    let shader_code = r#"
struct Point {
    x: f32,
}

struct Point {
    y: f32,
}
"#;
    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_ok());

    let shader = result.unwrap();
    assert_eq!(shader.structures.len(), 1);
}

#[test]
fn test_parse_duplicate_function_names() {
    let shader_code = r#"
fn test() {
    return;
}

fn test() {
    return;
}
"#;
    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_ok());

    let shader = result.unwrap();
    assert_eq!(shader.functions.len(), 1);
}

#[test]
fn test_document_nonexistent_directory() {
    let result = Document::open("test", "/nonexistent/directory/path");
    assert!(result.is_err());
}

#[test]
fn test_document_invalid_wgsl_file() {
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();
    let shader_path = temp_dir.path().join("invalid.wgsl");

    fs::write(&shader_path, "this is not valid WGSL syntax {{{{").unwrap();

    let result = Document::open("test_project", temp_dir.path());
    assert!(result.is_err());
}

#[test]
fn test_parse_function_missing_return_type_arrow() {
    let shader_code = r#"
fn test() f32 {
    return 0.0;
}
"#;
    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_err());
}

#[test]
fn test_parse_struct_missing_field_type() {
    let shader_code = r#"
struct Point {
    x,
}
"#;
    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_err());
}

#[test]
fn test_parse_nested_braces() {
    let shader_code = r#"
fn complex() {
    if (true) {
        if (false) {
            let x = 5;
        }
    }
}
"#;
    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_ok());
}

#[test]
fn test_parse_comments_not_docs() {
    let shader_code = r#"
// This is a regular comment, not documentation
fn test() {}
"#;
    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_ok());

    let shader = result.unwrap();
    assert!(shader.functions[0].docs().is_none());
}

#[test]
fn test_parse_mixed_documentation() {
    let shader_code = r#"
//! Global docs
// Regular comment
/// Function docs
fn test() {}
"#;
    let result = WgslParser::parse("test", shader_code);
    assert!(result.is_ok());

    let shader = result.unwrap();
    assert!(shader.global_docs.is_some());
    assert!(shader.functions[0].docs().is_some());
}

#[test]
fn test_document_with_empty_paths() {
    let result = Document::new("test", &[] as &[PathBuf]);
    assert!(result.is_ok());
}
