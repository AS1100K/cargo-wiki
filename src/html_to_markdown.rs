/// Utility functions for converting HTML formatted documentation to proper markdown
/// 
/// This module handles conversion of HTML entities and tags that are commonly
/// found in rustdoc-generated documentation strings.

/// Converts HTML-formatted documentation string to clean markdown
/// 
/// This function handles:
/// - HTML entities: &lt;, &gt;, &amp;, &quot;, &#39;
/// - HTML tags: <code>...</code> -> `...`
/// - HTML tags: <em>...</em> -> *...*
/// - HTML tags: <strong>...</strong> -> **...**
/// 
/// # Examples
/// 
/// ```
/// use cargo_wiki::html_to_markdown::convert_html_to_markdown;
/// 
/// let html = "That is, this conversion is whatever the implementation of <code>[From]&lt;T&gt; for U</code> chooses to do.";
/// let markdown = convert_html_to_markdown(html);
/// assert_eq!(markdown, "That is, this conversion is whatever the implementation of `[From]<T> for U` chooses to do.");
/// ```
pub fn convert_html_to_markdown(html: &str) -> String {
    let mut result = html.to_string();
    
    // Convert HTML entities first
    result = convert_html_entities(&result);
    
    // Convert HTML tags to markdown
    result = convert_html_tags(&result);
    
    result
}

/// Converts HTML entities to their character equivalents
fn convert_html_entities(input: &str) -> String {
    input
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
}

/// Converts HTML tags to markdown equivalents
fn convert_html_tags(input: &str) -> String {
    let mut result = input.to_string();
    
    // Convert <code>...</code> to `...`
    result = convert_code_tags(&result);
    
    // Convert <em>...</em> to *...*
    result = convert_em_tags(&result);
    
    // Convert <strong>...</strong> to **...**
    result = convert_strong_tags(&result);
    
    result
}

/// Converts <code>...</code> tags to markdown backticks
fn convert_code_tags(input: &str) -> String {
    // Use a simple regex-like approach to handle nested content properly
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '<' {
            // Try to match <code>
            let next_chars: String = chars.clone().take(4).collect();
            if next_chars == "code" {
                // Skip "code"
                for _ in 0..4 {
                    chars.next();
                }
                // Skip ">"
                if chars.peek() == Some(&'>') {
                    chars.next();
                }
                
                // Find the closing </code>
                let mut code_content = String::new();
                let mut in_closing_tag = false;
                let mut closing_tag_chars = String::new();
                
                while let Some(code_ch) = chars.next() {
                    if code_ch == '<' && chars.peek() == Some(&'/') {
                        in_closing_tag = true;
                        closing_tag_chars.clear();
                        chars.next(); // consume '/'
                        continue;
                    }
                    
                    if in_closing_tag {
                        if code_ch == '>' {
                            if closing_tag_chars == "code" {
                                // Found closing </code>
                                break;
                            } else {
                                // Not a code closing tag, add it to content
                                code_content.push('<');
                                code_content.push('/');
                                code_content.push_str(&closing_tag_chars);
                                code_content.push(code_ch);
                                in_closing_tag = false;
                                closing_tag_chars.clear();
                            }
                        } else {
                            closing_tag_chars.push(code_ch);
                        }
                    } else {
                        code_content.push(code_ch);
                    }
                }
                
                // Add the code content with backticks
                result.push('`');
                result.push_str(&code_content);
                result.push('`');
            } else {
                result.push(ch);
            }
        } else {
            result.push(ch);
        }
    }
    
    result
}

/// Converts <em>...</em> tags to markdown italic
fn convert_em_tags(input: &str) -> String {
    simple_tag_replace(input, "em", "*")
}

/// Converts <strong>...</strong> tags to markdown bold
fn convert_strong_tags(input: &str) -> String {
    simple_tag_replace(input, "strong", "**")
}

/// Helper function for simple tag replacement
fn simple_tag_replace(input: &str, tag: &str, markdown: &str) -> String {
    let open_tag = format!("<{}>", tag);
    let close_tag = format!("</{}>", tag);
    
    input
        .replace(&open_tag, markdown)
        .replace(&close_tag, markdown)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_entities() {
        assert_eq!(convert_html_entities("&lt;T&gt;"), "<T>");
        assert_eq!(convert_html_entities("&amp;&quot;&#39;"), "&\"'");
        assert_eq!(convert_html_entities("&apos;"), "'");
    }

    #[test]
    fn test_code_tags() {
        assert_eq!(convert_code_tags("<code>hello</code>"), "`hello`");
        assert_eq!(
            convert_code_tags("<code>[From]&lt;T&gt; for U</code>"),
            "`[From]&lt;T&gt; for U`"
        );
        assert_eq!(convert_code_tags("before <code>code</code> after"), "before `code` after");
    }

    #[test]
    fn test_em_tags() {
        assert_eq!(convert_em_tags("<em>italic</em>"), "*italic*");
        assert_eq!(convert_em_tags("before <em>text</em> after"), "before *text* after");
    }

    #[test]
    fn test_strong_tags() {
        assert_eq!(convert_strong_tags("<strong>bold</strong>"), "**bold**");
        assert_eq!(convert_strong_tags("before <strong>text</strong> after"), "before **text** after");
    }

    #[test]
    fn test_full_conversion() {
        let html = "That is, this conversion is whatever the implementation of <code>[From]&lt;T&gt; for U</code> chooses to do.";
        let expected = "That is, this conversion is whatever the implementation of `[From]<T> for U` chooses to do.";
        assert_eq!(convert_html_to_markdown(html), expected);
    }

    #[test]
    fn test_complex_html() {
        let html = "See <code>&lt;T&gt;</code> and <em>emphasis</em> with <strong>&amp; bold</strong>";
        let expected = "See `<T>` and *emphasis* with **& bold**";
        assert_eq!(convert_html_to_markdown(html), expected);
    }

    #[test]
    fn test_no_html() {
        let plain = "This is plain text with no HTML";
        assert_eq!(convert_html_to_markdown(plain), plain);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(convert_html_to_markdown(""), "");
    }

    #[test]
    fn test_multiple_code_blocks() {
        let html = "Use <code>foo</code> or <code>bar</code> functions";
        let expected = "Use `foo` or `bar` functions";
        assert_eq!(convert_html_to_markdown(html), expected);
    }
}