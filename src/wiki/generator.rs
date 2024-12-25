use crate::generators::struct_gen::StructGenerator;
use crate::generators::Generator;
use crate::{gen_path, Configuration, WIKI_CACHE_PATH};
use anyhow::Result;
use rustdoc_types::{Crate, ItemEnum, Struct};
use std::fs;

pub fn generate_wiki(configuration: &Configuration, crate_type: Crate) -> Result<()> {
    let crate_id = &crate_type.root;
    let path = format!("{}/{}", WIKI_CACHE_PATH, crate_id.0);
    gen_path(&path).expect("Failed to create file");

    for (id, item) in &crate_type.index {
        if let Some(name) = item.name.as_ref() {
            // TODO: Nest files under module if available
            let mut path = format!("{}/", path);

            let item_name = name;
            let mut file_content = String::from("# ");
            file_content.push_str(item_name);
            file_content.push_str("\n");

            match &item.inner {
                ItemEnum::Struct(Struct {
                    kind,
                    generics,
                    impls,
                }) => {
                    path.push_str("struct.");
                    path.push_str(item_name);
                    let syntax = StructGenerator::generate_syntax(
                        &item,
                        &crate_type.index,
                        &crate_type.paths,
                        &crate_type.external_crates,
                    )?;
                    file_content.push_str(&syntax);
                }
                _ => continue,
            }
            path.push_str(".md");
            fs::write(path, file_content).expect("TODO: panic message");
        }
    }
    Ok(())
}
