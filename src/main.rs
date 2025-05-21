use log::LevelFilter;
use wgsldoc::Document;

fn main() -> anyhow::Result<()> {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .init();

    let document = Document::open("test_shaders")?;
    let registered = document.register();

    println!("{:#?}", registered.shaders());

    Ok(())
}