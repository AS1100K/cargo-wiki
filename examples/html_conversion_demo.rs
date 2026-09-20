/// Demonstration of HTML to markdown conversion functionality
/// 
/// This test creates documentation with HTML content and verifies that it gets
/// properly converted to markdown format when processed by cargo-wiki.

use cargo_wiki::html_to_markdown::convert_html_to_markdown;

fn main() {
    println!("=== HTML to Markdown Conversion Demonstration ===\n");
    
    // Example from the original issue
    let original_html = "Calls `U::from(self)`.\n\nThat is, this conversion is whatever the implementation of\n<code>[From]&lt;T&gt; for U</code> chooses to do.";
    let converted = convert_html_to_markdown(original_html);
    
    println!("Original HTML documentation:");
    println!("{}\n", original_html);
    
    println!("Converted to Markdown:");
    println!("{}\n", converted);
    
    println!("---\n");
    
    // More complex example
    let complex_html = "This function works with <code>&lt;T: Clone + Send&gt;</code> types.\n\nIt uses <strong>advanced</strong> techniques and <em>special</em> handling for <code>&amp;mut references</code>.";
    let complex_converted = convert_html_to_markdown(complex_html);
    
    println!("Complex HTML documentation:");
    println!("{}\n", complex_html);
    
    println!("Converted to Markdown:");
    println!("{}\n", complex_converted);
    
    println!("=== Conversion successful! ===");
}