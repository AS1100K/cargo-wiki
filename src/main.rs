use cargo_wiki::rust_doc::generate_rust_doc;
use cargo_wiki::{generate_doc_for_entire_dir, gen_path, Configuration, WIKI_OUTPUT_PATH, WIKI_CACHE_PATH};
use clap::{CommandFactory, Parser};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        Configuration::command().print_help().unwrap();
        return;
    }

    let configuration = Configuration::parse_from(&args[1..]);
    println!("{:#?}", configuration);

    gen_path(WIKI_OUTPUT_PATH).unwrap();
    gen_path(WIKI_CACHE_PATH).unwrap();
    generate_rust_doc(&configuration).expect("Failed to generate rust doc");
    generate_doc_for_entire_dir(configuration).unwrap();
}
