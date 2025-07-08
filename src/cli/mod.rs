use std::{env, path::PathBuf};
use clap::Parser;

fn default_name() -> String {
    env::current_dir().ok()
        .and_then(|cwd| cwd.file_name().map(|n| n.to_owned()))
        .and_then(|n| n.to_str().map(|s| s.to_string()))
        .unwrap_or_else(|| "default".to_string())
}

fn default_target() -> PathBuf {
    let mut path = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    path.push("docs");
    path
}

#[derive(Parser, Debug, Clone)]
#[command(
    version, 
    about = "Documentation generator for WGSL shaders", 
    long_about = "wgsldoc is a documentation generator for WGSL (WebGPU Shading Language) shader modules. It parses .wgsl files, extracts documentation comments, and generates a structured, searchable HTML site for easy browsing and reference.",
)]
/// Command-line arguments for the documentation generator.
pub struct Args {
    /// Name of the package to generate documentation for.
    #[arg(short = 'N', long, default_value_t = default_name())]
    pub name: String,

    /// Target directory for the generated documentation.
    #[arg(short = 'D', long, default_value_os_t = default_target())]
    pub target_dir: PathBuf,

    /// Base URL for future website. If specified, it will be used to generate links in the documentation.
    /// Otherwise, the links will use `target_dir` as the base URL.
    #[arg(short = 'U', long)]
    pub base_url: Option<String>,

    /// Generate AST and print it to stdout instead of generating full documentation.
    #[arg(short = 'A', long)]
    pub ast_only: bool,

    /// Input files to process. 
    /// If not specified, the program will look for .wgsl files in the current directory.
    #[arg(short = 'I', long = "input")]
    pub files: Vec<PathBuf>,

    /// Show undocumented items in the documentation.
    #[arg(short = 'W', long)]
    pub show_undocumented: bool,
}