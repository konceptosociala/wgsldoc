# wgsldoc 📃🧑‍💻

![![GitHub License](https://img.shields.io/github/license/konceptosociala/wgsldoc.svg)](LICENSE)
![![Crates.io](https://img.shields.io/crates/v/wgsldoc.svg)](https://crates.io/crates/wgsldoc)

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
  -h, --help                     Print help (see more with '--help')
  -V, --version                  Print version
```

## Tasklist

- [x] CLI
    - [x] AST-only mode
    - [x] Documentation generation
    - [x] Show undocumented items
- [ ] Parsing
    - [x] Modules
    - [x] Imports
    - [x] Functions
    - [x] Structures
    - [ ] Entry points
    - [ ] Built-in imports
    - [ ] Constants
    - [ ] Bindings
    - [ ] Enums
- [ ] Documentation
- [ ] Search functionality
    - [ ] Search index generation
    - [ ] Search page
- [ ] HTML Generation
    - [x] Main page
    - [ ] Modules
        - [x] Index page
        - [x] Module page
        - [ ] Import page
        - [ ] Function page
        - [ ] Structure page
        - [ ] Entry point page
        - [ ] Built-in import page
        - [ ] Constants page
        - [ ] Bindings page
        - [ ] Enums page
    - [ ] Source code
        - [ ] Index page
        - [ ] Source page