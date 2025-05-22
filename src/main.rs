use log::LevelFilter;
use wgsldoc::{
    generator::TeraGenerator, 
    Document,
};

fn main() -> anyhow::Result<()> {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .init();

    let document = Document::open("test_shaders")?;
    let registered = document.register();

    if !std::fs::exists("/tmp/wgsldoc")? {
        std::fs::create_dir("/tmp/wgsldoc")?;
    }

    registered.generate(&mut TeraGenerator, "/tmp/wgsldoc");
    
    Ok(())
}