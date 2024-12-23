use clap::Parser;
use cargo_wiki::Configuration;
use cargo_wiki::rust_doc::generate_rust_doc;

fn main() {
    let args = std::env::args().skip(2).collect::<Vec<String>>();
    let configuration = Configuration::parse_from(args);

    println!("{:#?}", configuration);
    generate_rust_doc(&configuration).expect("Failed to generate rust doc");
}
