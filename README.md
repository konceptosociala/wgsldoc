# wgsldoc 📃🧑‍💻

[![GitHub License](https://img.shields.io/github/license/konceptosociala/wgsldoc.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/wgsldoc.svg)](https://crates.io/crates/wgsldoc)

**wgsldoc** is a documentation generator for WGSL (WebGPU Shading Language) shader modules. It parses `.wgsl` files, extracts documentation comments, and generates a structured, searchable HTML site for easy browsing and reference.

## Installation

To install `wgsldoc`, you can use Cargo, the Rust package manager. Run the following command:

```bash
cargo install wgsldoc
```

## Usage
After installation, you can run `wgsldoc` from the command line. 
The following command will generate documentation for all WGSL files (including `README.md` and `favicon.png` files) in the current directory, outputting the result
to the `docs` directory:

```bash
wgsldoc
```

If you want to host your docs as a website, you should specify a base URL with the `-U` option:

```bash
wgsldoc -U https://example.com/docs
```

If you want to only generate the AST (Abstract Syntax Tree) and print it to stdout (or another stream) instead of generating full documentation, you can use the `-A` option:

```bash
wgsldoc -A > ast_output.txt
```
or for `stdout`
```bash
wgsldoc -A
```

### AST example:

Simple shader:
```wgsl
@group(0) @binding(0) var<uniform> color: vec4<f32>;

@fragment
fn main() -> @location(0) vec4<f32> {
    return color;
}
```

```rust
simple => Wgsl {
    module_name: "simple",
    source_code: "...",
    global_docs: None,
    imports: [],
    functions: [
        Function {
            docs: None,
            name: "main",
            args: [],
            return_ty: Some(
                Vector(
                    Vector {
                        dimension: D4,
                        ty: Float32,
                    },
                ),
            ),
        },
    ],
    structures: [],
    constants: [],
    bindings: [
        Binding {
            docs: None,
            attr_group: 0,
            attr_binding: 0,
            name: "color",
            ty: Vector(
                Vector {
                    dimension: D4,
                    ty: Float32,
                },
            ),
        },
    ],
}
```

More advanced usage:

```bash
Usage: wgsldoc [OPTIONS]

Options:
  -N, --name <NAME>              Name of the package to generate documentation for
  -D, --target-dir <TARGET_DIR>  Target directory for the generated documentation 
  -U, --base-url <BASE_URL>      Base URL for future website. If specified, it will be used to generate links in the documentation. Otherwise, the links will use `target_dir` as the base URL
  -A, --ast-only                 Generate AST and print it to stdout instead of generating full documentation
  -I, --input <FILES>            Input files to process. If not specified, the program will look for .wgsl files in the current directory
  -W, --show-undocumented        Show undocumented items in the documentation
  -C, --credits                  Show credits
  -h, --help                     Print help (see more with '--help')
  -V, --version                  Print version
```

## Features

- [x] CLI
    - [x] AST-only mode
    - [x] Documentation generation
    - [x] Show undocumented items
- [x] Parsing
    - [x] Modules
    - [x] Imports
    - [x] Functions
    - [x] Structures
    - [x] Constants
    - [x] Bindings
- [x] HTML Generation
    - [x] Main page
    - [x] Modules
        - [x] Index page
        - [x] Module page
        - [x] Import page
        - [x] Function page
        - [x] Structure page
        - [x] Constants page
        - [x] Bindings page
    - [x] Source code
- [x] Documentation