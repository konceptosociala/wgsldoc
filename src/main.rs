use pest::Parser;
use wgsldoc::{
    model::structure::Structure, 
    parser::{FromPest, Rule, WgslParser}, view::WgslDocApp,
};

const SHADER: &str = include_str!("../test_shader.wgsl");

fn main() -> anyhow::Result<()> {
    let mut structures = vec![];
    let wgsl = WgslParser::parse(Rule::SHADER, SHADER)?;

    for shader_element in wgsl {
        match shader_element.as_rule() {
            Rule::STRUCTURE => structures.push(Structure::from_pest(shader_element)?),
            Rule::FUNCTION => {},
            Rule::IMPORT => {},
            _ => {},
        }
    }

    dbg!(&structures);

    yew::Renderer::<WgslDocApp>::new().render();

    Ok(())
}