pub fn to_html(value: &str) -> String {
    use markdown as md;

    md::to_html_with_options(
        value, 
        &md::Options {
            compile: md::CompileOptions {
                allow_dangerous_html: true,
                ..md::CompileOptions::default()
            },
            ..md::Options::default()
        }
    ).unwrap()
}