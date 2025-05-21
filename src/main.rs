use log::LevelFilter;
use wgsldoc::parser::WgslParser;

const SHADER: &str = include_str!("../test_shader.wgsl");

fn main() -> anyhow::Result<()> {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .init();

    dbg!(WgslParser::parse(SHADER)?);

    Ok(())
}