use clap::Parser;
use fs_err as fs;
use log::LevelFilter;
use std::env;
use wgsldoc::{cli::Args, generator::TeraGenerator, Document};

fn main() -> anyhow::Result<()> {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .init();

    let mut args = Args::parse();

    if args.credits {
        println!(r#"                    .__       .___             "#);
        println!(r#"__  _  ______  _____|  |    __| _/____   ____  "#);
        println!(r#"\ \/ \/ / ___\/  ___/  |   / __ |/  _ \_/ ___\ "#);
        println!(r#" \     / /_/  >___ \|  |__/ /_/ (  <_> )  \___ "#);
        println!(r#"  \/\_/\___  /____  >____/\____ |\____/ \___  >"#);
        println!(r#"      /_____/     \/           \/           \/ "#);
        println!("\x1b[1mwgsldoc\x1b[0m - WGSL Documentation Generator");
        println!("Developed by Oleksandr Hnutov (konceptosociala).");
        println!("Visit \x1b[34mhttps://github.com/konceptosociala/wgsldoc\x1b[0m for more information.");
        return Ok(());
    }

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

    if args.show_undocumented {
        show_undocumented_mode(&registered);
        return Ok(());
    }

    if args.ast_only {
        ast_only_mode(&registered);
        return Ok(());
    }

    registered.generate(&mut TeraGenerator::new(args.base_url), args.target_dir)?;

    Ok(())
}

fn ast_only_mode(registered: &wgsldoc::RegisteredDocument) {
    log::info!("AST-only mode enabled. Printing the AST to stdout.");

    for shader in registered.shaders() {
        println!("{} => {:#?}", shader.module_name, shader);
    }
}

fn show_undocumented_mode(registered: &wgsldoc::RegisteredDocument) {
    log::info!("Entering undocumented mode. This will log warnings for any undocumented items in the shaders.");

    for shader in registered.shaders() {
        if shader.global_docs.is_none() {
            log::warn!("Shader '{}' has no documentation.", shader.module_name);
        }

        for function in &shader.functions {
            if function.docs().is_none() {
                log::warn!(
                    "Function '{}' in shader '{}' has no documentation.",
                    function.name(),
                    shader.module_name
                );
            }

            for arg in function.args() {
                if arg.docs().is_none() {
                    log::warn!(
                        "Argument '{}' in function '{}' of shader '{}' has no documentation.",
                        arg.name(),
                        function.name(),
                        shader.module_name
                    );
                }
            }
        }

        for structure in &shader.structures {
            if structure.docs().is_none() {
                log::warn!(
                    "Struct '{}' in shader '{}' has no documentation.",
                    structure.name(),
                    shader.module_name
                );
            }

            for field in structure.fields() {
                if field.docs().is_none() {
                    log::warn!(
                        "Field '{}' in struct '{}' of shader '{}' has no documentation.",
                        field.name(),
                        structure.name(),
                        shader.module_name
                    );
                }
            }
        }

        for import in &shader.imports {
            if import.docs().is_none() {
                log::warn!(
                    "Import '{}' in shader '{}' has no documentation.",
                    import.name(),
                    shader.module_name
                );
            }
        }
    }
}
