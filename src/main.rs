use std::env;
use clap::Parser;
use log::LevelFilter;
use wgsldoc::{cli::Args, generator::TeraGenerator, Document};
use fs_err as fs;

fn main() -> anyhow::Result<()> {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .init();

    let mut args = Args::parse();

    if !args.target_dir.is_absolute() {
        args.target_dir = env::current_dir()?.join(&args.target_dir);
        args.target_dir = fs::canonicalize(&args.target_dir)?;
    }

    let document = if !args.files.is_empty() {
        Document::new(args.name, &args.files)?
    } else {
        Document::open(args.name, env::current_dir()?)?
    };

    if document.shaders().is_empty() {
        log::warn!("No WGSL shaders found in the specified files or directory.");
        return Ok(());
    }

    let registered = document.register();
    
    if args.ast_only {
        for shader in registered.shaders() {
            println!("{} => {:#?}", shader.module_name, shader);
        }
        return Ok(());
    }

    registered.generate(&mut TeraGenerator::new(args.base_url), args.target_dir)?;

    Ok(())
}