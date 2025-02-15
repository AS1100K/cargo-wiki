use super::*;

#[derive(Debug, Clone)]
pub struct InlineGroup {
    group: Document,
}

impl InlineGroup {
    pub fn new() -> Self {
        Self { group: Vec::new() }
    }

    pub fn push<E>(&mut self, element: E)
    where
        E: ToMarkdown + 'static,
    {
        self.group.push(Box::new(element));
    }

    pub fn push_c<E>(mut self, element: E) -> Self
    where
        E: ToMarkdown + 'static,
    {
        self.group.push(Box::new(element));
        self
    }
}

impl ToMarkdown for InlineGroup {
    fn expects_new_line(&self) -> bool {
        false
    }

    fn to_markdown(&self) -> String {
        self.group.to_markdown()
    }
}

#[derive(Debug, Clone)]
pub struct Space(u8);

impl Space {
    pub fn new(n_times: u8) -> Self {
        Self(n_times)
    }
}

impl ToMarkdown for Space {
    fn expects_new_line(&self) -> bool {
        false
    }

    fn to_markdown(&self) -> String {
        let mut spaces = String::new();

        for _ in 0..self.0 {
            spaces.push(' ');
        }

        spaces
    }
}

#[derive(Debug, Clone)]
pub struct Text(String);

impl Text {
    pub fn new(text: String) -> Self {
        Self(text)
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&String> for Text {
    fn from(value: &String) -> Self {
        Self(value.to_owned())
    }
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

impl ToMarkdown for Text {
    fn expects_new_line(&self) -> bool {
        false
    }

    fn to_markdown(&self) -> String {
        self.0.to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct Bold(Vec<Box<dyn ToMarkdown>>);

impl Bold {
    pub fn new<E>(element: E) -> Self
    where
        E: ToMarkdown + 'static,
    {
        Self(vec![Box::new(element)])
    }

    pub fn push<E>(&mut self, element: E)
    where
        E: ToMarkdown + 'static,
    {
        self.0.push(Box::new(element));
    }

    pub fn push_c<E>(mut self, element: E) -> Self
    where
        E: ToMarkdown + 'static,
    {
        self.push(element);
        self
    }
}

impl ToMarkdown for Bold {
    fn expects_new_line(&self) -> bool {
        false
    }

    fn to_markdown(&self) -> String {
        format!("<b>{}</b>", self.0.to_markdown())
    }
}

#[derive(Debug, Clone)]
pub struct Italic(Vec<Box<dyn ToMarkdown>>);

impl Italic {
    pub fn new<E>(element: E) -> Self
    where
        E: ToMarkdown + 'static,
    {
        Self(vec![Box::new(element)])
    }

    pub fn push<E>(&mut self, element: E)
    where
        E: ToMarkdown + 'static,
    {
        self.0.push(Box::new(element));
    }

    pub fn push_c<E>(mut self, element: E) -> Self
    where
        E: ToMarkdown + 'static,
    {
        self.push(element);
        self
    }
}

impl ToMarkdown for Italic {
    fn expects_new_line(&self) -> bool {
        false
    }

    fn to_markdown(&self) -> String {
        format!("<i>{}</i>", self.0.to_markdown())
    }
}

#[derive(Debug, Clone)]
pub struct Underline(Vec<Box<dyn ToMarkdown>>);

impl Underline {
    pub fn new<E>(element: E) -> Self
    where
        E: ToMarkdown + 'static,
    {
        Self(vec![Box::new(element)])
    }

    pub fn push<E>(&mut self, element: E)
    where
        E: ToMarkdown + 'static,
    {
        self.0.push(Box::new(element));
    }

    pub fn push_c<E>(mut self, element: E) -> Self
    where
        E: ToMarkdown + 'static,
    {
        self.push(element);
        self
    }
}

impl ToMarkdown for Underline {
    fn expects_new_line(&self) -> bool {
        false
    }

    fn to_markdown(&self) -> String {
        format!("<u>{}</u>", self.0.to_markdown())
    }
}

#[derive(Debug, Clone)]
pub struct CodeSpan(String);

impl CodeSpan {
    pub fn new(code: String) -> Self {
        Self(code)
    }
}

impl From<String> for CodeSpan {
    fn from(value: String) -> Self {
        CodeSpan(value)
    }
}

impl From<&String> for CodeSpan {
    fn from(value: &String) -> Self {
        CodeSpan(value.to_owned())
    }
}

impl From<&str> for CodeSpan {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

impl ToMarkdown for CodeSpan {
    fn expects_new_line(&self) -> bool {
        false
    }

    fn to_markdown(&self) -> String {
        format!("`{}`", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Link(String, String);

impl Link {
    pub fn new(text: String, url: String) -> Self {
        Self(text, url)
    }

    pub fn empty() -> Self {
        Self(String::new(), String::new())
    }
}

impl ToMarkdown for Link {
    fn expects_new_line(&self) -> bool {
        false
    }

    fn to_markdown(&self) -> String {
        format!("[{}]({})", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text() {
        let actual = Text::from("Hello World");
        let expected = String::from("Hello World");

        assert_eq!(actual.to_markdown(), expected);
    }

    #[test]
    fn test_bold() {
        let actual = Bold::new(Text::from("Hello World"));
        let expected = String::from("<b>Hello World</b>");

        assert_eq!(actual.to_markdown(), expected);
    }

    #[test]
    fn test_underline() {
        let actual = Underline::new(Text::from("Hello World"));
        let expected = String::from("<u>Hello World</u>");

        assert_eq!(actual.to_markdown(), expected);
    }

    #[test]
    fn test_italic() {
        let actual = Italic::new(Text::from("Hello World"));
        let expected = String::from("<i>Hello World</i>");

        assert_eq!(actual.to_markdown(), expected);
    }

    #[test]
    fn test_code_span() {
        let actual = CodeSpan::new("fn hello_world();".into());
        let expected = String::from("`fn hello_world();`");

        assert_eq!(actual.to_markdown(), expected);
    }

    #[test]
    fn test_complex_inline_expression() {
        let actual = Bold::new(Text::new("Basic Text".into()))
            .push_c(Space::new(1))
            .push_c(Underline::new(Text::new("Underline".into())))
            .push_c(Space::new(1))
            .push_c(Italic::new(Text::new("Italic".into())));

        let expected = String::from("<b>Basic Text <u>Underline</u> <i>Italic</i></b>");

        assert_eq!(actual.to_markdown(), expected);
    }

    #[test]
    fn test_link() {
        let actual = Link::new(String::from("Text"), String::from("https://example.com"));
        let expected = String::from("[Text](https://example.com)");

        assert_eq!(actual.to_markdown(), expected);
    }
}
