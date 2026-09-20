/// Integration test for HTML to markdown conversion in documentation processing
use cargo_wiki::html_to_markdown::convert_html_to_markdown;

#[test]
fn test_issue_example() {
    // Test the specific example from the issue
    let html_input = "Calls `U::from(self)`.\n\nThat is, this conversion is whatever the implementation of\n<code>[From]&lt;T&gt; for U</code> chooses to do.";
    let expected_output = "Calls `U::from(self)`.\n\nThat is, this conversion is whatever the implementation of\n`[From]<T> for U` chooses to do.";
    
    let actual_output = convert_html_to_markdown(html_input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn test_multiline_documentation() {
    let html_input = "/// First line with <code>some_code</code>\n/// Second line with &lt;Type&gt;\n/// Third line with <em>emphasis</em>";
    let expected_output = "/// First line with `some_code`\n/// Second line with <Type>\n/// Third line with *emphasis*";
    
    let actual_output = convert_html_to_markdown(html_input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn test_complex_rust_generic_types() {
    let html_input = "Implementation for <code>&lt;T: Clone + Send&gt;</code> where <code>T: &lt;'a&gt; Send + Sync</code>";
    let expected_output = "Implementation for `<T: Clone + Send>` where `T: <'a> Send + Sync`";
    
    let actual_output = convert_html_to_markdown(html_input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn test_mixed_html_formatting() {
    let html_input = "Use <strong>bold</strong> text with <code>code_snippet</code> and <em>italic &amp; more</em>";
    let expected_output = "Use **bold** text with `code_snippet` and *italic & more*";
    
    let actual_output = convert_html_to_markdown(html_input);
    assert_eq!(actual_output, expected_output);
}