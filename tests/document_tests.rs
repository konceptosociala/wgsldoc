use wgsldoc::Document;
use std::path::PathBuf;
use fs_err as fs;
use tempfile::TempDir;

#[test]
fn test_document_new_empty() {
    let result = Document::new("test_project", &[] as &[PathBuf]);
    assert!(result.is_ok());
    let doc = result.unwrap();
    assert_eq!(doc.pkg_name(), "test_project");
    assert_eq!(doc.shaders().len(), 0);
}

#[test]
fn test_document_with_test_shaders() {
    let paths = vec![
        PathBuf::from("test_shaders/ray.wgsl"),
    ];
    
    let result = Document::new("test_project", &paths);
    if let Ok(doc) = result {
        assert_eq!(doc.pkg_name(), "test_project");
        assert!(!doc.shaders().is_empty());
    }
}

#[test]
fn test_document_open_directory() {
    let temp_dir = TempDir::new().unwrap();
    let shader_path = temp_dir.path().join("test.wgsl");
    
    fs::write(&shader_path, "fn test() {}").unwrap();
    
    let result = Document::open("test_project", temp_dir.path());
    assert!(result.is_ok());
    
    let doc = result.unwrap();
    assert_eq!(doc.pkg_name(), "test_project");
    assert_eq!(doc.shaders().len(), 1);
    assert_eq!(doc.shaders()[0].module_name, "test");
}

#[test]
fn test_document_ignores_non_wgsl_files() {
    let temp_dir = TempDir::new().unwrap();
    
    fs::write(temp_dir.path().join("shader.wgsl"), "fn test() {}").unwrap();
    fs::write(temp_dir.path().join("readme.txt"), "readme").unwrap();
    fs::write(temp_dir.path().join("data.json"), "{}").unwrap();
    
    let result = Document::open("test_project", temp_dir.path());
    assert!(result.is_ok());
    
    let doc = result.unwrap();
    assert_eq!(doc.shaders().len(), 1);
}

#[test]
fn test_document_with_readme() {
    let temp_dir = TempDir::new().unwrap();
    let readme_path = temp_dir.path().join("README.md");
    let shader_path = temp_dir.path().join("test.wgsl");
    
    fs::write(&readme_path, "# Test Project\n\nThis is a test.").unwrap();
    fs::write(&shader_path, "fn test() {}").unwrap();
    
    let result = Document::open("test_project", temp_dir.path());
    assert!(result.is_ok());
    
    let doc = result.unwrap();
    assert!(doc.readme().is_some());
    assert!(doc.readme().unwrap().contains("Test Project"));
}

#[test]
fn test_document_with_favicon() {
    let temp_dir = TempDir::new().unwrap();
    let favicon_path = temp_dir.path().join("favicon.png");
    let shader_path = temp_dir.path().join("test.wgsl");
    
    let png_data = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A,
    ];
    fs::write(&favicon_path, png_data).unwrap();
    fs::write(&shader_path, "fn test() {}").unwrap();
    
    let result = Document::open("test_project", temp_dir.path());
    assert!(result.is_ok());
    
    let doc = result.unwrap();
    assert!(!doc.favicon().is_empty());
}

#[test]
fn test_document_ignores_hidden_files() {
    let temp_dir = TempDir::new().unwrap();
    
    fs::write(temp_dir.path().join(".hidden.wgsl"), "fn hidden() {}").unwrap();
    fs::write(temp_dir.path().join("visible.wgsl"), "fn visible() {}").unwrap();
    
    let result = Document::open("test_project", temp_dir.path());
    assert!(result.is_ok());
    
    let doc = result.unwrap();
    assert_eq!(doc.shaders().len(), 1);
    assert_eq!(doc.shaders()[0].module_name, "visible");
}

#[test]
fn test_document_register() {
    let temp_dir = TempDir::new().unwrap();
    let shader_path = temp_dir.path().join("test.wgsl");
    
    fs::write(&shader_path, "fn test() {}").unwrap();
    
    let doc = Document::open("test_project", temp_dir.path()).unwrap();
    let registered = doc.register();
    
    assert_eq!(registered.pkg_name(), "test_project");
    assert_eq!(registered.shaders().len(), 1);
}

#[test]
fn test_document_multiple_shaders() {
    let temp_dir = TempDir::new().unwrap();
    
    fs::write(temp_dir.path().join("shader1.wgsl"), "fn func1() {}").unwrap();
    fs::write(temp_dir.path().join("shader2.wgsl"), "fn func2() {}").unwrap();
    fs::write(temp_dir.path().join("shader3.wgsl"), "fn func3() {}").unwrap();
    
    let result = Document::open("test_project", temp_dir.path());
    assert!(result.is_ok());
    
    let doc = result.unwrap();
    assert_eq!(doc.shaders().len(), 3);
}

#[test]
fn test_document_file_registry() {
    let temp_dir = TempDir::new().unwrap();
    let shader_path = temp_dir.path().join("test.wgsl");
    
    fs::write(&shader_path, "fn test() {}").unwrap();
    
    let doc = Document::open("test_project", temp_dir.path()).unwrap();
    
    assert_eq!(doc.file_registry().len(), 1);
    assert!(doc.file_registry().iter().any(|p| p.ends_with("test.wgsl")));
}

#[test]
fn test_document_with_complex_shader() {
    let temp_dir = TempDir::new().unwrap();
    let shader_path = temp_dir.path().join("complex.wgsl");
    
    let shader_code = r#"
//! Module documentation

struct Point {
    x: f32,
    y: f32,
}

fn distance(a: Point, b: Point) -> f32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    return sqrt(dx * dx + dy * dy);
}
"#;
    
    fs::write(&shader_path, shader_code).unwrap();
    
    let result = Document::open("test_project", temp_dir.path());
    assert!(result.is_ok());
    
    let doc = result.unwrap();
    assert_eq!(doc.shaders().len(), 1);
    
    let shader = &doc.shaders()[0];
    assert!(shader.global_docs.is_some());
    assert_eq!(shader.structures.len(), 1);
    assert_eq!(shader.functions.len(), 1);
}

#[test]
fn test_registered_document_accessors() {
    let temp_dir = TempDir::new().unwrap();
    let shader_path = temp_dir.path().join("test.wgsl");
    
    fs::write(&shader_path, "fn test() {}").unwrap();
    
    let doc = Document::open("test_project", temp_dir.path()).unwrap();
    let registered = doc.register();
    
    assert_eq!(registered.pkg_name(), "test_project");
    assert!(!registered.shaders().is_empty());
    assert!(!registered.file_registry().is_empty());
    assert!(registered.readme().is_none());
    assert!(!registered.favicon().is_empty());
}
