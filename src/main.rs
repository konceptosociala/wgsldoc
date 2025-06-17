use log::LevelFilter;
use wgsldoc::{
    generator::TeraGenerator, 
    Document
};

fn main() -> anyhow::Result<()> {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .init();


    let document = Document::open("test_shaders", "test_shaders")?;
    let registered = document.register();
    registered.generate(&mut TeraGenerator::new(None), "/home/nutov2/doc/wgsldoc-test")?;
    
    Ok(())
}