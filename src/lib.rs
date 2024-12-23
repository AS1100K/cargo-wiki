use clap::{Parser, ValueEnum};

pub mod rust_doc;

#[derive(Parser, Debug)]
#[command(name = "cargo wiki")]
pub struct Configuration {
    #[arg(short, long, default_value_t = String::from(""))]
    pub package: String,
    #[arg(long, default_value_t = false)]
    pub workspace: bool,
    #[arg(long, default_value_t = false)]
    pub document_private_items: bool,
    #[arg(long, default_value_t = false)]
    pub no_deps: bool,
    #[arg(short, long, value_enum, default_value_t = MarkdownFlavor::GitHub)]
    pub markdown_flavor: MarkdownFlavor,
    // TODO: Add more arguments to support more feautures
    // refer: https://doc.rust-lang.org/cargo/commands/cargo-doc.html
    // refer: https://doc.rust-lang.org/cargo/commands/cargo-rustdoc.html
}

/// Markdown Flavor to be used
#[derive(Debug, ValueEnum, Clone)]
pub enum MarkdownFlavor {
    /// GitHub Markdown Flavor
    GitHub,
    /// GitLab Markdown Flavor
    GitLab,
    // TODO: Add support for other markdown flavors
}