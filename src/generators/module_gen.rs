use crate::gen_path;
use crate::generators::struct_gen::StructGenerator;
use crate::generators::{ExternalCrates, Generator, Index, Paths};
use anyhow::Result;
use rustdoc_types::{Item, ItemEnum, Module};
use std::fs;

pub const MODULE_FILE_NAME: &str = "README.md";

pub struct ModuleGenerator<'a> {
    pub prefix_path: String,
    pub module_file_name: &'a str,
    pub root_item: &'a Item,
    pub index: &'a Index,
    pub paths: &'a Paths,
    pub external_crate: &'a ExternalCrates,
}

impl<'a> ModuleGenerator<'a> {
    pub fn new(
        prefix_path: String,
        module_file_name: &'a str,
        root_item: &'a Item,
        index: &'a Index,
        paths: &'a Paths,
        external_crate: &'a ExternalCrates,
    ) -> Self {
        Self {
            prefix_path,
            module_file_name,
            root_item,
            index,
            paths,
            external_crate,
        }
    }

    pub fn auto(self) -> Result<()> {
        let Some(module_name) = &self.root_item.name else {
            return Err(anyhow::Error::msg(format!(
                "Every module should have a name. Id: {}",
                self.root_item.id.0
            )));
        };
        let ItemEnum::Module(Module {
            items, is_stripped, ..
        }) = &self.root_item.inner
        else {
            return Err(anyhow::Error::msg(
                "The Root module can't have inner item type anything other than Module. If \
                you think this is an error, please open the issue at \
                https://github.com/as1100k/cargo-wiki/issues with appropriate logs.",
            ));
        };
        let mut path = format!("{}/{}", self.prefix_path, module_name);
        let mut module_file_content = format!("# {}\n\n", module_name);

        gen_path(&path)?;

        for item in items {
            let Some(item) = self.index.get(item) else {
                eprintln!("Failed to find item with id: {} in index", item.0);
                continue;
            };
            let Some(item_name) = &item.name else {
                // Ignoring the items that don't have name as these items are later fetched
                // by there Id's where they are required
                continue;
            };

            let mut path = format!("{}", path);
            let mut file_content = format!("# {}\n\n", item_name);

            match &item.inner {
                ItemEnum::Module(_) => {
                    let new_module_generator = Self::new(
                        path,
                        &self.module_file_name,
                        item,
                        self.index,
                        self.paths,
                        self.external_crate,
                    );
                    new_module_generator.auto()?;
                    // Move to the next item as module will document itself separately
                    continue;
                }
                ItemEnum::Struct(_) => {
                    path.push_str("/struct.");
                    path.push_str(item_name);

                    let syntax = StructGenerator::generate_syntax(
                        item,
                        self.index,
                        self.paths,
                        self.external_crate,
                    )?;
                    file_content.push_str(&syntax);
                }
                _ => continue,
            }

            path.push_str(".md");
            fs::write(path, file_content)
                .expect("TODO: panic message at `src/generators/module_gen.rs`");
        }

        Ok(())
    }
}
