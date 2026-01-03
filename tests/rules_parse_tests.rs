use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/wgsldoc.pest"]
struct WgslGrammarTester;

#[test]
fn test_rule_shader_empty() {
    let input = "";
    let result = WgslGrammarTester::parse(Rule::SHADER, input);
    assert!(result.is_ok(), "Empty shader should parse successfully");
}

#[test]
fn test_rule_shader_with_global_docs() {
    let input = "//! This is global documentation\n//! Second line\n";
    let result = WgslGrammarTester::parse(Rule::SHADER, input);
    assert!(result.is_ok(), "Shader with global docs should parse");
}

#[test]
fn test_rule_shader_with_function() {
    let input = "fn test() {}";
    let result = WgslGrammarTester::parse(Rule::SHADER, input);
    assert!(result.is_ok(), "Shader with function should parse");
}

#[test]
fn test_rule_shader_with_structure() {
    let input = "struct Test { x: f32 }";
    let result = WgslGrammarTester::parse(Rule::SHADER, input);
    assert!(result.is_ok(), "Shader with structure should parse");
}

#[test]
fn test_rule_shader_complex() {
    let input = r#"
//! Global docs
#import test.wgsl as Test;
struct Point { x: f32, y: f32 }
fn add(a: f32, b: f32) -> f32 { return a + b; }
const PI: f32 = 3.14159;
@group(0) @binding(0) var<uniform> data: Data;
"#;
    let result = WgslGrammarTester::parse(Rule::SHADER, input);
    assert!(result.is_ok(), "Complex shader should parse");
}

#[test]
fn test_rule_number() {
    let input = "42";
    let result = WgslGrammarTester::parse(Rule::NUMBER, input);
    assert!(result.is_ok(), "Simple number should parse");
}

#[test]
fn test_rule_number_multidigit() {
    let input = "1234567890";
    let result = WgslGrammarTester::parse(Rule::NUMBER, input);
    assert!(result.is_ok(), "Multi-digit number should parse");
}

#[test]
fn test_rule_number_zero() {
    let input = "0";
    let result = WgslGrammarTester::parse(Rule::NUMBER, input);
    assert!(result.is_ok(), "Zero should parse");
}

#[test]
fn test_rule_attr_group() {
    let input = "@group(0)";
    let result = WgslGrammarTester::parse(Rule::ATTR_GROUP, input);
    assert!(result.is_ok(), "@group(0) should parse");
}

#[test]
fn test_rule_attr_group_large() {
    let input = "@group(1234)";
    let result = WgslGrammarTester::parse(Rule::ATTR_GROUP, input);
    assert!(result.is_ok(), "@group with large number should parse");
}

#[test]
fn test_rule_attr_binding() {
    let input = "@binding(0)";
    let result = WgslGrammarTester::parse(Rule::ATTR_BINDING, input);
    assert!(result.is_ok(), "@binding(0) should parse");
}

#[test]
fn test_rule_attr_binding_large() {
    let input = "@binding(999)";
    let result = WgslGrammarTester::parse(Rule::ATTR_BINDING, input);
    assert!(result.is_ok(), "@binding with large number should parse");
}

#[test]
fn test_rule_binding_attrs() {
    let input = "@group(0) @binding(1)";
    let result = WgslGrammarTester::parse(Rule::BINDING_ATTRS, input);
    assert!(result.is_ok(), "Combined binding attributes should parse");
}

#[test]
fn test_rule_storage_class_uniform() {
    let input = "uniform";
    let result = WgslGrammarTester::parse(Rule::STORAGE_CLASS, input);
    assert!(result.is_ok(), "uniform storage class should parse");
}

#[test]
fn test_rule_storage_class_storage() {
    let input = "storage";
    let result = WgslGrammarTester::parse(Rule::STORAGE_CLASS, input);
    assert!(result.is_ok(), "storage storage class should parse");
}

#[test]
fn test_rule_storage_class_private() {
    let input = "private";
    let result = WgslGrammarTester::parse(Rule::STORAGE_CLASS, input);
    assert!(result.is_ok(), "private storage class should parse");
}

#[test]
fn test_rule_storage_class_workgroup() {
    let input = "workgroup";
    let result = WgslGrammarTester::parse(Rule::STORAGE_CLASS, input);
    assert!(result.is_ok(), "workgroup storage class should parse");
}

#[test]
fn test_rule_var_template() {
    let input = "<uniform>";
    let result = WgslGrammarTester::parse(Rule::VAR_TEMPLATE, input);
    assert!(result.is_ok(), "<uniform> template should parse");
}

#[test]
fn test_rule_var_template_storage() {
    let input = "<storage>";
    let result = WgslGrammarTester::parse(Rule::VAR_TEMPLATE, input);
    assert!(result.is_ok(), "<storage> template should parse");
}

#[test]
fn test_rule_resource_binding_simple() {
    let input = "@group(0) @binding(0) var<uniform> myBuffer: f32;";
    let result = WgslGrammarTester::parse(Rule::RESOURCE_BINDING, input);
    assert!(result.is_ok(), "Simple resource binding should parse");
}

#[test]
fn test_rule_resource_binding_no_template() {
    let input = "@group(0) @binding(0) var myBuffer: f32;";
    let result = WgslGrammarTester::parse(Rule::RESOURCE_BINDING, input);
    assert!(result.is_ok(), "Resource binding without template should parse");
}

#[test]
fn test_rule_resource_binding_with_docs() {
    let input = "/// My buffer\n@group(0) @binding(0) var<uniform> myBuffer: f32;";
    let result = WgslGrammarTester::parse(Rule::RESOURCE_BINDING, input);
    assert!(result.is_ok(), "Resource binding with docs should parse");
}

#[test]
fn test_rule_resource_binding_no_semicolon() {
    let input = "@group(0) @binding(0) var<uniform> myBuffer: f32";
    let result = WgslGrammarTester::parse(Rule::RESOURCE_BINDING, input);
    assert!(result.is_ok(), "Resource binding without semicolon should parse");
}

#[test]
fn test_rule_const_simple() {
    let input = "const PI: f32 = 3.14159;";
    let result = WgslGrammarTester::parse(Rule::CONST, input);
    assert!(result.is_ok(), "Simple constant should parse");
}

#[test]
fn test_rule_const_no_type() {
    let input = "const VALUE = 42;";
    let result = WgslGrammarTester::parse(Rule::CONST, input);
    assert!(result.is_ok(), "Constant without type should parse");
}

#[test]
fn test_rule_const_with_docs() {
    let input = "/// Pi constant\nconst PI: f32 = 3.14159;";
    let result = WgslGrammarTester::parse(Rule::CONST, input);
    assert!(result.is_ok(), "Constant with docs should parse");
}

#[test]
fn test_rule_const_value_simple() {
    let input = "3.14159";
    let result = WgslGrammarTester::parse(Rule::CONST_VALUE, input);
    assert!(result.is_ok(), "Simple constant value should parse");
}

#[test]
fn test_rule_const_value_complex() {
    let input = "vec3<f32>(1.0, 2.0, 3.0)";
    let result = WgslGrammarTester::parse(Rule::CONST_VALUE, input);
    assert!(result.is_ok(), "Complex constant value should parse");
}

#[test]
fn test_rule_const_value_expression() {
    let input = "2.0 * 3.14159";
    let result = WgslGrammarTester::parse(Rule::CONST_VALUE, input);
    assert!(result.is_ok(), "Expression constant value should parse");
}

#[test]
fn test_rule_import_path_simple() {
    let input = "utils.wgsl";
    let result = WgslGrammarTester::parse(Rule::IMPORT_PATH, input);
    assert!(result.is_ok(), "Simple import path should parse");
}

#[test]
fn test_rule_import_path_nested() {
    let input = "lib/utils/math.wgsl";
    let result = WgslGrammarTester::parse(Rule::IMPORT_PATH, input);
    assert!(result.is_ok(), "Nested import path should parse");
}

#[test]
fn test_rule_import_path_with_underscores() {
    let input = "my_custom_utils.wgsl";
    let result = WgslGrammarTester::parse(Rule::IMPORT_PATH, input);
    assert!(result.is_ok(), "Import path with underscores should parse");
}

#[test]
fn test_rule_module_name() {
    let input = "Utils";
    let result = WgslGrammarTester::parse(Rule::MODULE_NAME, input);
    assert!(result.is_ok(), "Module name should parse");
}

#[test]
fn test_rule_module_name_lowercase() {
    let input = "utils";
    let result = WgslGrammarTester::parse(Rule::MODULE_NAME, input);
    assert!(result.is_ok(), "Lowercase module name should parse");
}

#[test]
fn test_rule_import_simple() {
    let input = "#import utils.wgsl as Utils;";
    let result = WgslGrammarTester::parse(Rule::IMPORT, input);
    assert!(result.is_ok(), "Simple import should parse");
}

#[test]
fn test_rule_import_no_semicolon() {
    let input = "#import utils.wgsl as Utils";
    let result = WgslGrammarTester::parse(Rule::IMPORT, input);
    assert!(result.is_ok(), "Import without semicolon should parse");
}

#[test]
fn test_rule_import_with_docs() {
    let input = "/// Utils module\n#import utils.wgsl as Utils;";
    let result = WgslGrammarTester::parse(Rule::IMPORT, input);
    assert!(result.is_ok(), "Import with docs should parse");
}

#[test]
fn test_rule_import_nested_path() {
    let input = "#import lib/utils/math.wgsl as Math;";
    let result = WgslGrammarTester::parse(Rule::IMPORT, input);
    assert!(result.is_ok(), "Import with nested path should parse");
}

#[test]
fn test_rule_builtin_import_simple() {
    let input = "#import utils;";
    let result = WgslGrammarTester::parse(Rule::BUILTIN_IMPORT, input);
    assert!(result.is_ok(), "Simple builtin import should parse");
}

#[test]
fn test_rule_builtin_import_path() {
    let input = "#import utils::math;";
    let result = WgslGrammarTester::parse(Rule::BUILTIN_IMPORT, input);
    assert!(result.is_ok(), "Builtin import with path should parse");
}

#[test]
fn test_rule_builtin_import_nested() {
    let input = "#import utils::math::vec;";
    let result = WgslGrammarTester::parse(Rule::BUILTIN_IMPORT, input);
    assert!(result.is_ok(), "Nested builtin import should parse");
}

#[test]
fn test_rule_builtin_import_with_docs() {
    let input = "/// Math utilities\n#import utils::math;";
    let result = WgslGrammarTester::parse(Rule::BUILTIN_IMPORT, input);
    assert!(result.is_ok(), "Builtin import with docs should parse");
}

#[test]
fn test_rule_builtin_import_no_semicolon() {
    let input = "#import utils::math";
    let result = WgslGrammarTester::parse(Rule::BUILTIN_IMPORT, input);
    assert!(result.is_ok(), "Builtin import without semicolon should parse");
}

#[test]
fn test_rule_builtin_import_content_simple() {
    let input = "utils";
    let result = WgslGrammarTester::parse(Rule::BUILTIN_IMPORT_CONTENT, input);
    assert!(result.is_ok(), "Simple builtin import content should parse");
}

#[test]
fn test_rule_builtin_import_content_with_module() {
    let input = "utils::Math";
    let result = WgslGrammarTester::parse(Rule::BUILTIN_IMPORT_CONTENT, input);
    assert!(result.is_ok(), "Builtin import content with module should parse");
}

#[test]
fn test_rule_builtin_import_content_with_list() {
    let input = "utils::{foo, bar}";
    let result = WgslGrammarTester::parse(Rule::BUILTIN_IMPORT_CONTENT, input);
    assert!(result.is_ok(), "Builtin import content with list should parse");
}

#[test]
fn test_rule_import_list_single() {
    let input = "{foo}";
    let result = WgslGrammarTester::parse(Rule::IMPORT_LIST, input);
    assert!(result.is_ok(), "Import list with single item should parse");
}

#[test]
fn test_rule_import_list_multiple() {
    let input = "{foo, bar, baz}";
    let result = WgslGrammarTester::parse(Rule::IMPORT_LIST, input);
    assert!(result.is_ok(), "Import list with multiple items should parse");
}

#[test]
fn test_rule_import_list_trailing_comma() {
    let input = "{foo, bar,}";
    let result = WgslGrammarTester::parse(Rule::IMPORT_LIST, input);
    assert!(result.is_ok(), "Import list with trailing comma should parse");
}

#[test]
fn test_rule_structure_simple() {
    let input = "struct Point { x: f32 }";
    let result = WgslGrammarTester::parse(Rule::STRUCTURE, input);
    assert!(result.is_ok(), "Simple structure should parse");
}

#[test]
fn test_rule_structure_multiple_fields() {
    let input = "struct Point { x: f32, y: f32, z: f32 }";
    let result = WgslGrammarTester::parse(Rule::STRUCTURE, input);
    assert!(result.is_ok(), "Structure with multiple fields should parse");
}

#[test]
fn test_rule_structure_empty() {
    let input = "struct Empty {}";
    let result = WgslGrammarTester::parse(Rule::STRUCTURE, input);
    assert!(result.is_ok(), "Empty structure should parse");
}

#[test]
fn test_rule_structure_with_docs() {
    let input = "/// A 3D point\nstruct Point { x: f32, y: f32 }";
    let result = WgslGrammarTester::parse(Rule::STRUCTURE, input);
    assert!(result.is_ok(), "Structure with docs should parse");
}

#[test]
fn test_rule_fields_single() {
    let input = "x: f32";
    let result = WgslGrammarTester::parse(Rule::FIELDS, input);
    assert!(result.is_ok(), "Single field should parse");
}

#[test]
fn test_rule_fields_multiple() {
    let input = "x: f32, y: f32, z: f32";
    let result = WgslGrammarTester::parse(Rule::FIELDS, input);
    assert!(result.is_ok(), "Multiple fields should parse");
}

#[test]
fn test_rule_fields_trailing_comma() {
    let input = "x: f32, y: f32,";
    let result = WgslGrammarTester::parse(Rule::FIELDS, input);
    assert!(result.is_ok(), "Fields with trailing comma should parse");
}

#[test]
fn test_rule_fields_empty() {
    let input = "";
    let result = WgslGrammarTester::parse(Rule::FIELDS, input);
    assert!(result.is_ok(), "Empty fields should parse");
}

#[test]
fn test_rule_field_simple() {
    let input = "x: f32";
    let result = WgslGrammarTester::parse(Rule::FIELD, input);
    assert!(result.is_ok(), "Simple field should parse");
}

#[test]
fn test_rule_field_with_location() {
    let input = "@location(0) position: vec3<f32>";
    let result = WgslGrammarTester::parse(Rule::FIELD, input);
    assert!(result.is_ok(), "Field with location should parse");
}

#[test]
fn test_rule_field_with_docs() {
    let input = "/// X coordinate\nx: f32";
    let result = WgslGrammarTester::parse(Rule::FIELD, input);
    assert!(result.is_ok(), "Field with docs should parse");
}

#[test]
fn test_rule_field_vector_type() {
    let input = "position: vec3<f32>";
    let result = WgslGrammarTester::parse(Rule::FIELD, input);
    assert!(result.is_ok(), "Field with vector type should parse");
}

#[test]
fn test_rule_function_simple() {
    let input = "fn test() {}";
    let result = WgslGrammarTester::parse(Rule::FUNCTION, input);
    assert!(result.is_ok(), "Simple function should parse");
}

#[test]
fn test_rule_function_with_args() {
    let input = "fn add(a: f32, b: f32) {}";
    let result = WgslGrammarTester::parse(Rule::FUNCTION, input);
    assert!(result.is_ok(), "Function with args should parse");
}

#[test]
fn test_rule_function_with_return() {
    let input = "fn get_value() -> f32 {}";
    let result = WgslGrammarTester::parse(Rule::FUNCTION, input);
    assert!(result.is_ok(), "Function with return type should parse");
}

#[test]
fn test_rule_function_with_body() {
    let input = "fn add(a: f32, b: f32) -> f32 { return a + b; }";
    let result = WgslGrammarTester::parse(Rule::FUNCTION, input);
    assert!(result.is_ok(), "Function with body should parse");
}

#[test]
fn test_rule_function_with_docs() {
    let input = "/// Adds two numbers\nfn add(a: f32, b: f32) -> f32 {}";
    let result = WgslGrammarTester::parse(Rule::FUNCTION, input);
    assert!(result.is_ok(), "Function with docs should parse");
}

#[test]
fn test_rule_function_vertex_entry() {
    let input = "@vertex fn vs_main() -> @location(0) vec4<f32> {}";
    let result = WgslGrammarTester::parse(Rule::FUNCTION, input);
    assert!(result.is_ok(), "Vertex entry function should parse");
}

#[test]
fn test_rule_function_fragment_entry() {
    let input = "@fragment fn fs_main() -> @location(0) vec4<f32> {}";
    let result = WgslGrammarTester::parse(Rule::FUNCTION, input);
    assert!(result.is_ok(), "Fragment entry function should parse");
}

#[test]
fn test_rule_function_compute_entry() {
    let input = "@compute fn cs_main() {}";
    let result = WgslGrammarTester::parse(Rule::FUNCTION, input);
    assert!(result.is_ok(), "Compute entry function should parse");
}

#[test]
fn test_rule_function_trailing_semicolon() {
    let input = "fn test() {};";
    let result = WgslGrammarTester::parse(Rule::FUNCTION, input);
    assert!(result.is_ok(), "Function with trailing semicolon should parse");
}

#[test]
fn test_rule_function_nested_braces() {
    let input = "fn test() { if (true) { let x = 1; } }";
    let result = WgslGrammarTester::parse(Rule::FUNCTION, input);
    assert!(result.is_ok(), "Function with nested braces should parse");
}

#[test]
fn test_rule_args_single() {
    let input = "a: f32";
    let result = WgslGrammarTester::parse(Rule::ARGS, input);
    assert!(result.is_ok(), "Single argument should parse");
}

#[test]
fn test_rule_args_multiple() {
    let input = "a: f32, b: i32, c: vec3<f32>";
    let result = WgslGrammarTester::parse(Rule::ARGS, input);
    assert!(result.is_ok(), "Multiple arguments should parse");
}

#[test]
fn test_rule_args_trailing_comma() {
    let input = "a: f32, b: i32,";
    let result = WgslGrammarTester::parse(Rule::ARGS, input);
    assert!(result.is_ok(), "Arguments with trailing comma should parse");
}

#[test]
fn test_rule_args_empty() {
    let input = "";
    let result = WgslGrammarTester::parse(Rule::ARGS, input);
    assert!(result.is_ok(), "Empty arguments should parse");
}

#[test]
fn test_rule_arg_simple() {
    let input = "value: f32";
    let result = WgslGrammarTester::parse(Rule::ARG, input);
    assert!(result.is_ok(), "Simple argument should parse");
}

#[test]
fn test_rule_arg_with_docs() {
    let input = "/// The input value\nvalue: f32";
    let result = WgslGrammarTester::parse(Rule::ARG, input);
    assert!(result.is_ok(), "Argument with docs should parse");
}

#[test]
fn test_rule_arg_vector() {
    let input = "position: vec3<f32>";
    let result = WgslGrammarTester::parse(Rule::ARG, input);
    assert!(result.is_ok(), "Argument with vector type should parse");
}

#[test]
fn test_rule_arg_function_pointer() {
    let input = "ptr: ptr<function, f32>";
    let result = WgslGrammarTester::parse(Rule::ARG, input);
    assert!(result.is_ok(), "Argument with function pointer should parse");
}

#[test]
fn test_rule_return_simple() {
    let input = "-> f32";
    let result = WgslGrammarTester::parse(Rule::RETURN, input);
    assert!(result.is_ok(), "Simple return type should parse");
}

#[test]
fn test_rule_return_with_location() {
    let input = "-> @location(0) vec4<f32>";
    let result = WgslGrammarTester::parse(Rule::RETURN, input);
    assert!(result.is_ok(), "Return type with location should parse");
}

#[test]
fn test_rule_return_vector() {
    let input = "-> vec3<f32>";
    let result = WgslGrammarTester::parse(Rule::RETURN, input);
    assert!(result.is_ok(), "Vector return type should parse");
}

#[test]
fn test_rule_type_primitive() {
    let input = "f32";
    let result = WgslGrammarTester::parse(Rule::TYPE, input);
    assert!(result.is_ok(), "Primitive type should parse");
}

#[test]
fn test_rule_type_vector() {
    let input = "vec3<f32>";
    let result = WgslGrammarTester::parse(Rule::TYPE, input);
    assert!(result.is_ok(), "Vector type should parse");
}

#[test]
fn test_rule_type_path() {
    let input = "MyCustomType";
    let result = WgslGrammarTester::parse(Rule::TYPE, input);
    assert!(result.is_ok(), "Path type should parse");
}

#[test]
fn test_rule_function_type_primitive() {
    let input = "f32";
    let result = WgslGrammarTester::parse(Rule::FUNCTION_TYPE, input);
    assert!(result.is_ok(), "Primitive function type should parse");
}

#[test]
fn test_rule_function_type_pointer() {
    let input = "ptr<function, f32>";
    let result = WgslGrammarTester::parse(Rule::FUNCTION_TYPE, input);
    assert!(result.is_ok(), "Function pointer type should parse");
}

#[test]
fn test_rule_function_pointer_simple() {
    let input = "ptr<function, f32>";
    let result = WgslGrammarTester::parse(Rule::FUNCTION_POINTER, input);
    assert!(result.is_ok(), "Simple function pointer should parse");
}

#[test]
fn test_rule_function_pointer_vector() {
    let input = "ptr<function, vec3<f32>>";
    let result = WgslGrammarTester::parse(Rule::FUNCTION_POINTER, input);
    assert!(result.is_ok(), "Function pointer to vector should parse");
}

#[test]
fn test_rule_generic_args_single() {
    let input = "<f32>";
    let result = WgslGrammarTester::parse(Rule::GENERIC_ARGS, input);
    assert!(result.is_ok(), "Single generic arg should parse");
}

#[test]
fn test_rule_generic_args_multiple() {
    let input = "<f32, u32>";
    let result = WgslGrammarTester::parse(Rule::GENERIC_ARGS, input);
    assert!(result.is_ok(), "Multiple generic args should parse");
}

#[test]
fn test_rule_generic_args_nested() {
    let input = "<vec3<f32>>";
    let result = WgslGrammarTester::parse(Rule::GENERIC_ARGS, input);
    assert!(result.is_ok(), "Nested generic args should parse");
}

#[test]
fn test_rule_path_type_simple() {
    let input = "Camera";
    let result = WgslGrammarTester::parse(Rule::PATH_TYPE, input);
    assert!(result.is_ok(), "Simple path type should parse");
}

#[test]
fn test_rule_path_type_with_module() {
    let input = "Utils::Camera";
    let result = WgslGrammarTester::parse(Rule::PATH_TYPE, input);
    assert!(result.is_ok(), "Path type with module should parse");
}

#[test]
fn test_rule_path_type_with_generics() {
    let input = "Array<f32>";
    let result = WgslGrammarTester::parse(Rule::PATH_TYPE, input);
    assert!(result.is_ok(), "Path type with generics should parse");
}

#[test]
fn test_rule_path_type_module_and_generics() {
    let input = "Utils::Array<f32>";
    let result = WgslGrammarTester::parse(Rule::PATH_TYPE, input);
    assert!(result.is_ok(), "Path type with module and generics should parse");
}

#[test]
fn test_rule_module() {
    let input = "Utils";
    let result = WgslGrammarTester::parse(Rule::MODULE, input);
    assert!(result.is_ok(), "Module name should parse");
}

#[test]
fn test_rule_module_lowercase() {
    let input = "utils";
    let result = WgslGrammarTester::parse(Rule::MODULE, input);
    assert!(result.is_ok(), "Lowercase module name should parse");
}

#[test]
fn test_rule_primitive_bool() {
    let input = "bool";
    let result = WgslGrammarTester::parse(Rule::PRIMITIVE, input);
    assert!(result.is_ok(), "bool primitive should parse");
}

#[test]
fn test_rule_primitive_f32() {
    let input = "f32";
    let result = WgslGrammarTester::parse(Rule::PRIMITIVE, input);
    assert!(result.is_ok(), "f32 primitive should parse");
}

#[test]
fn test_rule_primitive_f64() {
    let input = "f64";
    let result = WgslGrammarTester::parse(Rule::PRIMITIVE, input);
    assert!(result.is_ok(), "f64 primitive should parse");
}

#[test]
fn test_rule_primitive_i8() {
    let input = "i8";
    let result = WgslGrammarTester::parse(Rule::PRIMITIVE, input);
    assert!(result.is_ok(), "i8 primitive should parse");
}

#[test]
fn test_rule_primitive_i16() {
    let input = "i16";
    let result = WgslGrammarTester::parse(Rule::PRIMITIVE, input);
    assert!(result.is_ok(), "i16 primitive should parse");
}

#[test]
fn test_rule_primitive_i32() {
    let input = "i32";
    let result = WgslGrammarTester::parse(Rule::PRIMITIVE, input);
    assert!(result.is_ok(), "i32 primitive should parse");
}

#[test]
fn test_rule_primitive_i64() {
    let input = "i64";
    let result = WgslGrammarTester::parse(Rule::PRIMITIVE, input);
    assert!(result.is_ok(), "i64 primitive should parse");
}

#[test]
fn test_rule_primitive_u8() {
    let input = "u8";
    let result = WgslGrammarTester::parse(Rule::PRIMITIVE, input);
    assert!(result.is_ok(), "u8 primitive should parse");
}

#[test]
fn test_rule_primitive_u16() {
    let input = "u16";
    let result = WgslGrammarTester::parse(Rule::PRIMITIVE, input);
    assert!(result.is_ok(), "u16 primitive should parse");
}

#[test]
fn test_rule_primitive_u32() {
    let input = "u32";
    let result = WgslGrammarTester::parse(Rule::PRIMITIVE, input);
    assert!(result.is_ok(), "u32 primitive should parse");
}

#[test]
fn test_rule_primitive_u64() {
    let input = "u64";
    let result = WgslGrammarTester::parse(Rule::PRIMITIVE, input);
    assert!(result.is_ok(), "u64 primitive should parse");
}

#[test]
fn test_rule_vector_vec2() {
    let input = "vec2<f32>";
    let result = WgslGrammarTester::parse(Rule::VECTOR, input);
    assert!(result.is_ok(), "vec2 should parse");
}

#[test]
fn test_rule_vector_vec3() {
    let input = "vec3<f32>";
    let result = WgslGrammarTester::parse(Rule::VECTOR, input);
    assert!(result.is_ok(), "vec3 should parse");
}

#[test]
fn test_rule_vector_vec4() {
    let input = "vec4<f32>";
    let result = WgslGrammarTester::parse(Rule::VECTOR, input);
    assert!(result.is_ok(), "vec4 should parse");
}

#[test]
fn test_rule_vector_i32() {
    let input = "vec3<i32>";
    let result = WgslGrammarTester::parse(Rule::VECTOR, input);
    assert!(result.is_ok(), "vec3<i32> should parse");
}

#[test]
fn test_rule_vector_u32() {
    let input = "vec4<u32>";
    let result = WgslGrammarTester::parse(Rule::VECTOR, input);
    assert!(result.is_ok(), "vec4<u32> should parse");
}

#[test]
fn test_rule_vector_dimension_2() {
    let input = "2";
    let result = WgslGrammarTester::parse(Rule::VECTOR_DIMENSION, input);
    assert!(result.is_ok(), "Vector dimension 2 should parse");
}

#[test]
fn test_rule_vector_dimension_3() {
    let input = "3";
    let result = WgslGrammarTester::parse(Rule::VECTOR_DIMENSION, input);
    assert!(result.is_ok(), "Vector dimension 3 should parse");
}

#[test]
fn test_rule_vector_dimension_4() {
    let input = "4";
    let result = WgslGrammarTester::parse(Rule::VECTOR_DIMENSION, input);
    assert!(result.is_ok(), "Vector dimension 4 should parse");
}

#[test]
fn test_rule_ident_simple() {
    let input = "test";
    let result = WgslGrammarTester::parse(Rule::IDENT, input);
    assert!(result.is_ok(), "Simple identifier should parse");
}

#[test]
fn test_rule_ident_uppercase() {
    let input = "TEST";
    let result = WgslGrammarTester::parse(Rule::IDENT, input);
    assert!(result.is_ok(), "Uppercase identifier should parse");
}

#[test]
fn test_rule_ident_mixed_case() {
    let input = "myVariable";
    let result = WgslGrammarTester::parse(Rule::IDENT, input);
    assert!(result.is_ok(), "Mixed case identifier should parse");
}

#[test]
fn test_rule_ident_with_numbers() {
    let input = "var123";
    let result = WgslGrammarTester::parse(Rule::IDENT, input);
    assert!(result.is_ok(), "Identifier with numbers should parse");
}

#[test]
fn test_rule_ident_with_underscores() {
    let input = "my_variable_name";
    let result = WgslGrammarTester::parse(Rule::IDENT, input);
    assert!(result.is_ok(), "Identifier with underscores should parse");
}

#[test]
fn test_rule_ident_snake_case() {
    let input = "this_is_snake_case";
    let result = WgslGrammarTester::parse(Rule::IDENT, input);
    assert!(result.is_ok(), "Snake case identifier should parse");
}

#[test]
fn test_rule_ident_camel_case() {
    let input = "thisIsCamelCase";
    let result = WgslGrammarTester::parse(Rule::IDENT, input);
    assert!(result.is_ok(), "Camel case identifier should parse");
}

#[test]
fn test_rule_path_simple() {
    let input = "file.wgsl";
    let result = WgslGrammarTester::parse(Rule::PATH, input);
    assert!(result.is_ok(), "Simple path should parse");
}

#[test]
fn test_rule_path_with_directory() {
    let input = "lib/utils.wgsl";
    let result = WgslGrammarTester::parse(Rule::PATH, input);
    assert!(result.is_ok(), "Path with directory should parse");
}

#[test]
fn test_rule_path_nested() {
    let input = "lib/utils/math/vector.wgsl";
    let result = WgslGrammarTester::parse(Rule::PATH, input);
    assert!(result.is_ok(), "Nested path should parse");
}

#[test]
fn test_rule_path_with_underscores() {
    let input = "my_custom_file.wgsl";
    let result = WgslGrammarTester::parse(Rule::PATH, input);
    assert!(result.is_ok(), "Path with underscores should parse");
}

#[test]
fn test_rule_path_with_dots() {
    let input = "file.min.wgsl";
    let result = WgslGrammarTester::parse(Rule::PATH, input);
    assert!(result.is_ok(), "Path with dots should parse");
}

#[test]
fn test_rule_global_docs_single_line() {
    let input = "//! This is global documentation\n";
    let result = WgslGrammarTester::parse(Rule::GLOBAL_DOCS, input);
    assert!(result.is_ok(), "Single line global docs should parse");
}

#[test]
fn test_rule_global_docs_multiple_lines() {
    let input = "//! Line 1\n//! Line 2\n//! Line 3\n";
    let result = WgslGrammarTester::parse(Rule::GLOBAL_DOCS, input);
    assert!(result.is_ok(), "Multi-line global docs should parse");
}

#[test]
fn test_rule_global_docs_with_content() {
    let input = "//! This module provides ray tracing utilities\n";
    let result = WgslGrammarTester::parse(Rule::GLOBAL_DOCS, input);
    assert!(result.is_ok(), "Global docs with content should parse");
}

#[test]
fn test_rule_docs_single_line() {
    let input = "/// This is documentation\n";
    let result = WgslGrammarTester::parse(Rule::DOCS, input);
    assert!(result.is_ok(), "Single line docs should parse");
}

#[test]
fn test_rule_docs_multiple_lines() {
    let input = "/// Line 1\n/// Line 2\n/// Line 3\n";
    let result = WgslGrammarTester::parse(Rule::DOCS, input);
    assert!(result.is_ok(), "Multi-line docs should parse");
}

#[test]
fn test_rule_docs_with_markdown() {
    let input = "/// # Header\n/// Some **bold** text\n";
    let result = WgslGrammarTester::parse(Rule::DOCS, input);
    assert!(result.is_ok(), "Docs with markdown should parse");
}

#[test]
fn test_rule_docs_content_simple() {
    let input = "Simple documentation text";
    let result = WgslGrammarTester::parse(Rule::DOCS_CONTENT, input);
    assert!(result.is_ok(), "Simple docs content should parse");
}

#[test]
fn test_rule_docs_content_with_special_chars() {
    let input = "Text with @special #chars and $symbols!";
    let result = WgslGrammarTester::parse(Rule::DOCS_CONTENT, input);
    assert!(result.is_ok(), "Docs content with special chars should parse");
}

#[test]
fn test_rule_docs_content_empty() {
    let input = "";
    let result = WgslGrammarTester::parse(Rule::DOCS_CONTENT, input);
    assert!(result.is_ok(), "Empty docs content should parse");
}

#[test]
fn test_rule_docs_content_with_spaces() {
    let input = "   Text with leading and trailing spaces   ";
    let result = WgslGrammarTester::parse(Rule::DOCS_CONTENT, input);
    assert!(result.is_ok(), "Docs content with spaces should parse");
}

#[test]
fn test_complex_struct_with_all_features() {
    let input = r#"/// A complex vertex structure
/// With multiple lines of documentation
struct Vertex {
    /// Position in world space
    position: vec3<f32>,
    /// Normal vector
    normal: vec3<f32>,
    /// UV coordinates
    uv: vec2<f32>,
    /// Vertex color
    color: vec4<f32>,
}
"#;
    let result = WgslGrammarTester::parse(Rule::STRUCTURE, input);
    assert!(result.is_ok(), "Complex struct should parse");
}

#[test]
fn test_complex_function_with_all_features() {
    let input = 
r#"/// Computes the lighting for a fragment
/// Returns the final color
@fragment
fn compute_lighting(
    /// Fragment position
    position: vec3<f32>,
    /// Surface normal
    normal: vec3<f32>,
    /// View direction
    view_dir: vec3<f32>
) -> @location(0) vec4<f32> {
    let diffuse = max(dot(normal, vec3<f32>(0.0, 1.0, 0.0)), 0.0);
    return vec4<f32>(diffuse, diffuse, diffuse, 1.0);
}
"#;
    let result = WgslGrammarTester::parse(Rule::FUNCTION, input);
    println!("{:?}", result);
    assert!(result.is_ok(), "Complex function should parse");
}

#[test]
fn test_complete_shader_example() {
    let input = r#"
//! A complete shader module
//! Demonstrates all features

/// Import utilities
#import utils.wgsl as Utils;

/// Camera data binding
@group(0) @binding(0)
var<uniform> camera: Utils::Camera;

/// PI constant
const PI: f32 = 3.14159265359;

/// Vertex input structure
struct VertexInput {
    /// Position attribute
    position: vec3<f32>,
    /// Normal attribute
    normal: vec3<f32>,
}

/// Main vertex shader
@vertex
fn vs_main(input: VertexInput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.position, 1.0);
}
"#;
    let result = WgslGrammarTester::parse(Rule::SHADER, input);
    assert!(result.is_ok(), "Complete shader example should parse");
}

#[test]
fn test_multiple_imports_various_styles() {
    let input = r#"
#import utils.wgsl as Utils;
#import math.wgsl as Math;
#import geometry::shapes;
#import rendering::{lighting, shadows};
"#;
    let result = WgslGrammarTester::parse(Rule::SHADER, input);
    assert!(result.is_ok(), "Multiple import styles should parse");
}

#[test]
fn test_multiple_bindings() {
    let input = r#"
@group(0) @binding(0) var<uniform> camera: Camera;
@group(0) @binding(1) var<uniform> lights: Lights;
@group(1) @binding(0) var<storage> vertices: array<Vertex>;
@group(1) @binding(1) var texture: texture_2d<f32>;
"#;
    let result = WgslGrammarTester::parse(Rule::SHADER, input);
    assert!(result.is_ok(), "Multiple bindings should parse");
}

#[test]
fn test_all_entry_points() {
    let input = r#"
@vertex
fn vs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0);
}

@compute
fn cs_main() {
    // Compute work
}
"#;
    let result = WgslGrammarTester::parse(Rule::SHADER, input);
    assert!(result.is_ok(), "All entry point types should parse");
}

#[test]
fn test_nested_generic_types() {
    let input = r#"
struct Container {
    data: Array<Vec<Point<f32>>>,
}
"#;
    let result = WgslGrammarTester::parse(Rule::STRUCTURE, input);
    assert!(result.is_ok(), "Nested generic types should parse");
}
