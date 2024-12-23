use crate::wiki::generator::generate;
use crate::wiki::CrateExt;
use anyhow::Result;
use clap::{Parser, ValueEnum};
use rustdoc_types::Crate;
use std::env::current_dir;
use std::fs;
use std::fs::create_dir_all;
use std::path::Path;

pub mod rust_doc;
pub mod wiki;

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
    // TODO: Add more arguments to support more features
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

pub const WIKI_OUTPUT_PATH: &str = "target/wiki";

pub fn generate_wiki_directory() -> Result<()> {
    let wiki_output_path =
        Path::new(&current_dir().expect(
            "Failed to get the current directory where the `cargo wiki` command is running.",
        ))
        .join(WIKI_OUTPUT_PATH);

    if !wiki_output_path.exists() {
        create_dir_all(wiki_output_path)?
    }
    Ok(())
}

pub const RUSTDOC_OUTPUT: &str = "target/doc";

pub fn generate_doc_for_entire_dir(configuration: Configuration) -> Result<()> {
    let rustdoc_path =
        Path::new(&current_dir().expect(
            "Failed to get the current directory where the `cargo wiki` command is running.",
        ))
        .join(RUSTDOC_OUTPUT);

    let dir_iterator = fs::read_dir(&rustdoc_path).map_err(|_| {
        anyhow::Error::msg(format!("Failed to read contents of '{:?}'", &rustdoc_path))
    })?;

    for entry in dir_iterator {
        let entry = entry?;
        let file_type = entry.file_type();

        if let Ok(file_type) = file_type {
            if file_type.is_file() && entry.path().extension() == Some("json".as_ref()) {
                let crate_type = Crate::from_file(&entry.path())?;
                generate(&configuration, crate_type)?;
            }
        } else {
            eprintln!("Failed to know the type of {:?}", entry.path());
        }
    }

    Ok(())
}
