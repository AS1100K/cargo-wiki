use cargo_wiki::rust_doc::generate_rust_doc;
use cargo_wiki::{generate_doc_for_entire_dir, generate_wiki_directory, Configuration};
use clap::{CommandFactory, Parser};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        Configuration::command().print_help().unwrap();
        return;
    }

    let configuration = Configuration::parse_from(&args[1..]);
    println!("{:#?}", configuration);

    generate_wiki_directory().unwrap();
    generate_rust_doc(&configuration).expect("Failed to generate rust doc");
    generate_doc_for_entire_dir(configuration).unwrap();
}
