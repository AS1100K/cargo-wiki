use dyn_clone::DynClone;

pub mod inline;

pub trait ToMarkdown: std::fmt::Debug + DynClone {
    fn expects_new_line(&self) -> bool;

    fn to_markdown(&self) -> String;
}
dyn_clone::clone_trait_object!(ToMarkdown);

pub type Block = Box<dyn ToMarkdown>;
pub type Document = Vec<Box<dyn ToMarkdown>>;

impl ToMarkdown for Document {
    fn expects_new_line(&self) -> bool {
        false
    }

    fn to_markdown(&self) -> String {
        let mut block = String::new();

        for block_item in self {
            if block_item.expects_new_line() {
                block.push_str("\n\n");
            }

            block.push_str(&block_item.to_markdown());
        }

        block
    }
}

#[derive(Debug, Clone)]
pub struct GroupBlock {
    group: Document,
}

impl GroupBlock {
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

impl ToMarkdown for GroupBlock {
    fn expects_new_line(&self) -> bool {
        true
    }

    fn to_markdown(&self) -> String {
        self.group.to_markdown()
    }
}

#[derive(Debug, Clone)]
pub struct RawBlock(String);

impl From<String> for RawBlock {
    fn from(value: String) -> Self {
        RawBlock(value)
    }
}

impl From<&String> for RawBlock {
    fn from(value: &String) -> Self {
        RawBlock(value.to_owned())
    }
}

impl From<&str> for RawBlock {
    fn from(value: &str) -> Self {
        RawBlock(String::from(value))
    }
}

impl ToMarkdown for RawBlock {
    fn expects_new_line(&self) -> bool {
        true
    }

    fn to_markdown(&self) -> String {
        self.0.to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct EmptyElement;

impl ToMarkdown for EmptyElement {
    fn expects_new_line(&self) -> bool {
        false
    }

    fn to_markdown(&self) -> String {
        String::new()
    }
}

#[derive(Debug, Clone)]
pub struct NLines(u8);

impl NLines {
    pub fn new(n_lines: u8) -> Self {
        Self(n_lines)
    }
}

impl ToMarkdown for NLines {
    fn expects_new_line(&self) -> bool {
        false
    }

    fn to_markdown(&self) -> String {
        let mut lines = String::new();

        for _ in 0..self.0 {
            lines.push('\n');
        }

        lines
    }
}

#[derive(Debug, Clone)]
pub struct Title(u8, Box<dyn ToMarkdown>);

impl Title {
    pub fn new<E>(level: u8, element: E) -> Self
    where
        E: ToMarkdown + 'static,
    {
        if level > 6 || level == 0 {
            panic!("The Level for title must be between 1 to 6 (inclusive).");
        }

        Self(level, Box::new(element))
    }
}

impl ToMarkdown for Title {
    fn expects_new_line(&self) -> bool {
        true
    }

    fn to_markdown(&self) -> String {
        let mut markdown = String::new();

        for _ in 0..self.0 {
            markdown.push('#');
        }

        markdown.push_str(" ");
        markdown.push_str(&self.1.to_markdown());

        markdown
    }
}

#[derive(Debug, Clone)]
pub struct ListBlock {
    list_type: ListType,
    blocks: Vec<(Box<dyn ToMarkdown>, Box<dyn ToMarkdown>)>,
}

impl ListBlock {
    pub fn new_ordered_list() -> Self {
        Self {
            list_type: ListType::Ordered,
            blocks: Vec::new(),
        }
    }

    pub fn new_unordered_list() -> Self {
        Self {
            list_type: ListType::Unordered,
            blocks: Vec::new(),
        }
    }

    pub fn push<N, D>(&mut self, item_name: N, item_description: D)
    where
        N: ToMarkdown + 'static,
        D: ToMarkdown + 'static,
    {
        self.blocks
            .push((Box::new(item_name), Box::new(item_description)));
    }

    pub fn push_c<N, D>(mut self, item_name: N, item_description: D) -> Self
    where
        N: ToMarkdown + 'static,
        D: ToMarkdown + 'static,
    {
        self.blocks
            .push((Box::new(item_name), Box::new(item_description)));
        self
    }
}

impl ToMarkdown for ListBlock {
    fn expects_new_line(&self) -> bool {
        true
    }

    fn to_markdown(&self) -> String {
        let mut markdown = String::new();

        for (i, (item_name, item_description)) in self.blocks.iter().enumerate() {
            match &self.list_type {
                ListType::Unordered => {
                    markdown.push_str("- ");
                }
                ListType::Ordered => {
                    let current_number = (i + 1).to_string();
                    let item_number = format!("{}. ", current_number);
                    markdown.push_str(&item_number);
                }
            }

            markdown.push_str(&item_name.to_markdown());

            let item_description = item_description.to_markdown();

            if !item_description.is_empty() {
                markdown.push_str("\n\n\t");
                markdown.push_str(&item_description);
            }

            markdown.push_str("\n");
        }

        markdown
    }
}

#[derive(Debug, Clone)]
enum ListType {
    Ordered,
    Unordered,
}

#[derive(Debug, Clone)]
pub struct DropDown {
    state: DropDownState,
    summary: Box<dyn ToMarkdown>,
    inner: Vec<Box<dyn ToMarkdown>>,
}

impl DropDown {
    pub fn new_closed<S, I>(summary: S, inner: I) -> Self
    where
        S: ToMarkdown + 'static,
        I: ToMarkdown + 'static,
    {
        Self {
            state: DropDownState::Closed,
            summary: Box::new(summary),
            inner: vec![Box::new(inner)],
        }
    }

    pub fn new_opened<S, I>(summary: S, inner: I) -> Self
    where
        S: ToMarkdown + 'static,
        I: ToMarkdown + 'static,
    {
        Self {
            state: DropDownState::Opened,
            summary: Box::new(summary),
            inner: vec![Box::new(inner)],
        }
    }

    pub fn push<S, I>(&mut self, inner: I)
    where
        S: ToMarkdown + 'static,

        I: ToMarkdown + 'static,
    {
        self.inner.push(Box::new(inner));
    }

    pub fn push_c<I>(mut self, inner: I) -> Self
    where
        I: ToMarkdown + 'static,
    {
        self.inner.push(Box::new(inner));
        self
    }
}

impl ToMarkdown for DropDown {
    fn expects_new_line(&self) -> bool {
        true
    }

    fn to_markdown(&self) -> String {
        let mut markdown = String::from("<details ");
        markdown.push_str(self.state.to_str());
        markdown.push_str(">\n");
        markdown.push_str("<summary>");
        markdown.push_str(&self.summary.to_markdown());
        markdown.push_str("</summary>\n");
        markdown.push_str(&self.inner.to_markdown());
        markdown.push_str("\n</details>");

        markdown
    }
}

#[derive(Debug, Clone)]
enum DropDownState {
    Opened,
    Closed,
}

impl DropDownState {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Opened => "open",
            Self::Closed => "close",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inline::*;

    #[test]
    fn test_title() {
        let actual = Title::new(1, Text::from("H1"));
        let expected = String::from("# H1");
        assert_eq!(actual.to_markdown(), expected);

        let actual = Title::new(2, Text::from("H2"));
        let expected = String::from("## H2");
        assert_eq!(actual.to_markdown(), expected);

        let actual = Title::new(6, Text::from("H6"));
        let expected = String::from("###### H6");
        assert_eq!(actual.to_markdown(), expected);
    }

    #[test]
    #[should_panic]
    fn test_title_zero_level() {
        Title::new(0, EmptyElement);
    }

    #[test]
    #[should_panic]
    fn test_wrong_seven_level() {
        Title::new(7, EmptyElement);
    }

    #[test]
    fn test_n_lines() {
        let actual = NLines::new(5);
        let expected = String::from("\n\n\n\n\n");

        assert_eq!(actual.to_markdown(), expected);
    }

    #[test]
    fn test_unordered_list_block() {
        let mut actual = ListBlock::new_unordered_list();

        actual.push(Text::new("Text Inline Element".into()), EmptyElement);

        actual.push(
            Bold::new(Text::new("Bold Text".into())),
            Text::new("This is a basic description".into()),
        );

        let expected = String::from(
            "\
            - Text Inline Element\n\
            - <b>Bold Text</b>\
            \n\n\tThis is a basic description\n\
            ",
        );

        assert_eq!(actual.to_markdown(), expected);
    }

    #[test]
    fn test_ordered_list_block() {
        let mut actual = ListBlock::new_ordered_list();

        actual.push(Text::new("Text Inline Element".into()), EmptyElement);

        actual.push(
            Bold::new(Text::new("Bold Text".into())),
            Text::new("This is a basic description".into()),
        );

        let expected = String::from(
            "\
            1. Text Inline Element\n\
            2. <b>Bold Text</b>\
            \n\n\tThis is a basic description\n\
            ",
        );

        assert_eq!(actual.to_markdown(), expected);
    }

    #[test]
    fn test_dropdown() {
        let actual = DropDown::new_closed(
            Bold::new(Text::from("Fields")),
            ListBlock::new_ordered_list()
                .push_c(CodeSpan::from("value"), Text::from("This is a value"))
                .push_c(CodeSpan::from("field"), Text::from("This is a field")),
        );

        let expected = String::from(
            "\
        <details close>\n\
        <summary><b>Fields</b></summary>\n\n\n\
        1. `value`\n\n\
        \tThis is a value\n\
        2. `field`\n\n\
        \tThis is a field\
        \n\n\
        </details>\
        ",
        );

        assert_eq!(actual.to_markdown(), expected);
    }
}
