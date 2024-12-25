use crate::generators::struct_gen::StructGenerator;
use crate::generators::Generator;
use crate::{gen_path, Configuration, WIKI_CACHE_PATH};
use anyhow::Result;
use rustdoc_types::{Crate, ItemEnum, Module};
use std::fs;

pub fn generate_wiki(configuration: &Configuration, crate_type: Crate) -> Result<()> {
    let crate_id = &crate_type.root;
    let mut path = format!("{}/", WIKI_CACHE_PATH);
    gen_path(&path).expect("Failed to create file");

    let Some(root_module) = crate_type.index.get(&crate_type.root) else {
        return Err(anyhow::Error::msg(format!(
            "Failed to find the root module with id: {} in index",
            crate_id.0
        )));
    };
    let Some(root_module_name) = &root_module.name else {
        return Err(anyhow::Error::msg("Root module cannot have no name"));
    };
    let ItemEnum::Module(Module {
        is_crate,
        items,
        is_stripped,
    }) = &root_module.inner
    else {
        return Err(anyhow::Error::msg(
            "root module can't have inner other than Module",
        ));
    };
    path.push_str(root_module_name);
    gen_path(&path)?;
    for item in items {
        let Some(item) = crate_type.index.get(&item) else {
            return Err(anyhow::Error::msg(format!(
                "Failed to find item in Index with id: {}",
                item.0
            )));
        };
        let Some(item_name) = &item.name else {
            continue;
        };
        let mut path = format!("{}/", path);
        let mut file_content = format!("# {}\n\n", item_name);

        match &item.inner {
            ItemEnum::Struct(_) => {
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
        fs::write(path, file_content).expect("TODO: panic message at `src/wiki/generator.rs`");
    }

    Ok(())
}
