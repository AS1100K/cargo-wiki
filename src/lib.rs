use clap::Parser;

pub mod rust_doc;

#[derive(Parser, Debug)]
pub struct Configuration {
    #[arg(long, default_value_t = false)]
    pub document_private_items: bool,
    #[arg(long, default_value_t = false)]
    pub no_deps: bool,
}