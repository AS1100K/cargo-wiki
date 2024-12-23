use clap::{CommandFactory, Parser};
use cargo_wiki::Configuration;
use cargo_wiki::rust_doc::generate_rust_doc;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        Configuration::command().print_help().unwrap();
        return;
    }

    let configuration = Configuration::parse_from(&args[1..]);

    println!("{:#?}", configuration);
    generate_rust_doc(&configuration).expect("Failed to generate rust doc");
}
