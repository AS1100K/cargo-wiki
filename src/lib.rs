// Uncomment it when you are generating doc with feature `doc_examples`
// #![feature(more_maybe_bounds)]

use crate::wiki::generator::generate_wiki;
use crate::wiki::CrateExt;
use anyhow::Result;
use clap::{Parser, ValueEnum};
use rustdoc_types::Crate;
use std::env::current_dir;
use std::fs;
use std::fs::create_dir_all;
use std::path::Path;

#[cfg(feature = "doc_examples")]
pub mod _examples;
pub mod blocks;
pub mod generators;
pub mod rust_doc;
pub mod wiki;

#[derive(Parser, Debug)]
#[command(name = "cargo wiki")]
pub struct Configuration {
    /// The package to document. See [cargo-pkgid](https://doc.rust-lang.org/cargo/commands/cargo-pkgid.html)
    /// for the SPEC format.
    #[arg(short, long, default_value_t = String::new())]
    pub package: String,
    #[arg(long, default_value_t = false)]
    pub workspace: bool,
    /// Space or comma separated list of features to activate. Features of workspace members may be
    /// enabled with package-name/feature-name syntax. This flag may be specified multiple times,
    /// which enables all specified features.
    #[arg(short, long, default_value_t = String::new())]
    pub features: String,
    /// Activate all available features of all selected packages.
    #[arg(long, default_value_t = false)]
    pub all_features: bool,
    /// Do not activate the default feature of the selected packages.
    #[arg(long, default_value_t = false)]
    pub no_default_features: bool,
    #[arg(long, default_value_t = false)]
    pub document_private_items: bool,
    #[arg(long, default_value_t = false)]
    pub no_deps: bool,
    #[arg(short, long, value_enum, default_value_t = MarkdownFlavor::GitHub)]
    pub markdown_flavor: MarkdownFlavor,
    #[arg(short, long, value_enum, default_value_t = WikiStructure::Directory)]
    pub structure: WikiStructure,
    // TODO: Add more arguments to support more features
    // refer: https://doc.rust-lang.org/cargo/commands/cargo-doc.html
    // refer: https://doc.rust-lang.org/cargo/commands/cargo-rustdoc.html
    /// Sets the default file extension that will be used for all
    /// links. This doesn't change file type.
    #[arg(long, default_value_t = String::from(".md"))]
    pub default_link_file_extension: String,
    /// Sets the default file name for module files
    #[arg(long, default_value_t = String::from("README"))]
    pub default_module_file_name: String,
    #[arg(short = 'r', long, default_value_t = String::from("/"))]
    pub html_root_url: String,
}

/// Markdown Flavor to be used
#[derive(Debug, ValueEnum, Clone, PartialEq)]
pub enum MarkdownFlavor {
    /// GitHub Markdown Flavor
    GitHub,
    /// GitLab Markdown Flavor
    GitLab,
    // TODO: Add support for other markdown flavors
}

/// Structure of the wiki
#[derive(Debug, ValueEnum, Clone, PartialEq)]
pub enum WikiStructure {
    /// Structure via directory. This will create multiple directories.
    Directory,
    /// A Single file for the entire crate.
    SingleFile,
}

pub const WIKI_OUTPUT_PATH: &str = "target/wiki";
pub const WIKI_CACHE_PATH: &str = "target/wiki_cache";

pub fn gen_path(path: &str) -> Result<()> {
    let wiki_output_path =
        Path::new(&current_dir().expect(
            "Failed to get the current directory where the `cargo wiki` command is running.",
        ))
        .join(path);

    if !wiki_output_path.exists() {
        create_dir_all(wiki_output_path)?
    }
    Ok(())
}

pub fn save_file(path: &str, content: &str) -> Result<()> {
    let new_path = Path::new(path);

    if let Some(parent) = new_path.parent() {
        gen_path(
            parent
                .to_str()
                .expect("Failed to convert Path to string at lib.rs"),
        )?;
    }

    fs::write(path, content).expect("TODO: Panic Message at lib.rs");
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
                generate_wiki(&configuration, crate_type)?;
            }
        } else {
            eprintln!("Failed to know the type of {:?}", entry.path());
        }
    }

    Ok(())
}
