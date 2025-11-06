use wgsldoc::utils::html::to_html;

#[test]
fn test_to_html_simple_text() {
    let result = to_html("Hello, world!");
    assert!(result.contains("Hello, world!"));
}

#[test]
fn test_to_html_bold() {
    let result = to_html("**bold text**");
    assert!(result.contains("<strong>"));
    assert!(result.contains("bold text"));
    assert!(result.contains("</strong>"));
}

#[test]
fn test_to_html_italic() {
    let result = to_html("_italic text_");
    assert!(result.contains("<em>"));
    assert!(result.contains("italic text"));
    assert!(result.contains("</em>"));
}

#[test]
fn test_to_html_code() {
    let result = to_html("`code`");
    assert!(result.contains("<code>"));
    assert!(result.contains("code"));
    assert!(result.contains("</code>"));
}

#[test]
fn test_to_html_heading() {
    let result = to_html("# Heading 1");
    assert!(result.contains("<h1>"));
    assert!(result.contains("Heading 1"));
    assert!(result.contains("</h1>"));
}

#[test]
fn test_to_html_paragraph() {
    let result = to_html("First paragraph\n\nSecond paragraph");
    assert!(result.contains("<p>"));
    assert!(result.contains("First paragraph"));
    assert!(result.contains("Second paragraph"));
    assert!(result.contains("</p>"));
}

#[test]
fn test_to_html_link() {
    let result = to_html("[link text](https://example.com)");
    assert!(result.contains("<a"));
    assert!(result.contains("href=\"https://example.com\""));
    assert!(result.contains("link text"));
    assert!(result.contains("</a>"));
}

#[test]
fn test_to_html_list() {
    let result = to_html("- Item 1\n- Item 2\n- Item 3");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>"));
    assert!(result.contains("Item 1"));
    assert!(result.contains("Item 2"));
    assert!(result.contains("Item 3"));
    assert!(result.contains("</li>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_to_html_with_dangerous_html() {
    let result = to_html("<br>");
    assert!(result.contains("<br>"));
}

#[test]
fn test_to_html_code_block() {
    let result = to_html("```rust\nlet x = 5;\n```");
    assert!(result.contains("<pre>"));
    assert!(result.contains("<code"));
    assert!(result.contains("let x = 5;"));
    assert!(result.contains("</code>"));
    assert!(result.contains("</pre>"));
}

#[test]
fn test_to_html_empty() {
    let result = to_html("");
    assert_eq!(result, "");
}

#[test]
fn test_to_html_multiline() {
    let markdown = r#"# Title

This is a paragraph.

## Subtitle

Another paragraph with **bold** and _italic_."#;
    
    let result = to_html(markdown);
    assert!(result.contains("<h1>"));
    assert!(result.contains("Title"));
    assert!(result.contains("<h2>"));
    assert!(result.contains("Subtitle"));
    assert!(result.contains("<strong>"));
    assert!(result.contains("<em>"));
}
