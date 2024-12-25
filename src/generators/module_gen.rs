use crate::gen_path;
use crate::generators::struct_gen::StructGenerator;
use crate::generators::{ExternalCrates, Generator, Index, Paths};
use anyhow::Result;
use rustdoc_types::{Item, ItemEnum, Module};
use std::fs;

pub const MODULE_FILE_NAME: &str = "README";

#[derive(Default)]
pub struct ModuleItems<'a> {
    pub modules: Vec<ModuleField<'a>>,
    pub traits: Vec<ModuleField<'a>>,
    pub functions: Vec<ModuleField<'a>>,
    pub macros: Vec<ModuleField<'a>>,
    pub re_exports: Vec<ModuleField<'a>>,
    pub structs: Vec<ModuleField<'a>>,
    pub enums: Vec<ModuleField<'a>>,
    pub consts: Vec<ModuleField<'a>>,
}

pub struct ModuleField<'a> {
    pub name: &'a str,
    pub link: String,
    pub description: &'a str,
}

impl<'a> ModuleField<'a> {
    pub fn to_string(&self) -> String {
        let mut field_string = String::new();

        field_string.push_str("- [`");
        field_string.push_str(self.name);
        field_string.push_str("`](");
        field_string.push_str(&self.link);
        field_string.push_str(")\n");

        if !self.description.is_empty() {
            field_string.push_str("\n\t");
            field_string.push_str(self.description);
            field_string.push_str("\n");
        }

        field_string
    }
}

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
        let mut module_information = ModuleItems::default();

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

            let item_description = match &item.docs {
                Some(doc) => &doc[..doc.len().min(50)],
                None => ""
            };

            let mut path = format!("{}", path);
            let mut file_content = format!("# {}\n\n", item_name);

            match &item.inner {
                ItemEnum::Module(_) => {
                    module_information.modules.push(ModuleField {
                        name: item_name,
                        link: format!("./{}/{}.md", item_name, self.module_file_name),
                        description: &item_description,
                    });

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
                    module_information.structs.push(ModuleField {
                        name: item_name,
                        link: format!("./struct.{}.md", item_name),
                        description: &item_description,
                    });

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

        path.push_str("/");
        path.push_str(self.module_file_name);
        path.push_str(".md");

        module_file_content.push_str(&Self::generate_module_docs(&module_information));
        fs::write(path, module_file_content).expect(
            "TODO: panic message at `src/generators/module_gen.rs` while saving module file",
        );

        Ok(())
    }

    pub fn generate_module_docs(module_information: &ModuleItems) -> String {
        let mut module_information_string = String::new();

        // Modules
        if module_information.modules.len() > 0 {
            module_information_string.push_str("\n## Modules\n\n");
            for field in &module_information.modules {
                module_information_string.push_str(&field.to_string());
            }
        }

        // Constants
        if module_information.consts.len() > 0 {
            module_information_string.push_str("## Constants\n\n");
            for field in &module_information.consts {
                module_information_string.push_str(&field.to_string());
            }
        }

        // Structs
        if module_information.structs.len() > 0 {
            module_information_string.push_str("## Structs\n\n");
            for field in &module_information.structs {
                module_information_string.push_str(&field.to_string());
            }
        }

        // Enums
        if module_information.enums.len() > 0 {
            module_information_string.push_str("## Enums\n\n");
            for field in &module_information.enums {
                module_information_string.push_str(&field.to_string());
            }
        }

        // Traits
        if module_information.traits.len() > 0 {
            module_information_string.push_str("## Traits\n\n");
            for field in &module_information.traits {
                module_information_string.push_str(&field.to_string());
            }
        }

        // Functions
        if module_information.functions.len() > 0 {
            module_information_string.push_str("## Functions\n\n");
            for field in &module_information.functions {
                module_information_string.push_str(&field.to_string());
            }
        }

        // Macros
        if module_information.macros.len() > 0 {
            module_information_string.push_str("## Macros\n\n");
            for field in &module_information.macros {
                module_information_string.push_str(&field.to_string());
            }
        }

        // Re-exports
        if module_information.re_exports.len() > 0 {
            module_information_string.push_str("## Re-exports\n\n");
            for field in &module_information.re_exports {
                module_information_string.push_str(&field.to_string());
            }
        }

        module_information_string
    }
}
