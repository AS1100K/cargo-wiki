use std::fmt::Debug;

use crate::blocks::Document;
use crate::generators::struct_gen::StructGenerator;
use crate::generators::{ExternalCrates, Generator, Index, Paths};
use crate::Configuration;
use anyhow::Result;
use rustdoc_types::{Item, ItemEnum, ItemKind, Module};

use super::enum_gen::EnumGenerator;

#[derive(Default, Debug, Clone)]
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

impl<'a> ModuleItems<'a> {
    pub fn generate_docs(&self) -> String {
        let mut module_information_string = String::new();

        // Modules
        if self.modules.len() > 0 {
            module_information_string.push_str("\n## Modules\n\n");
            for field in &self.modules {
                module_information_string.push_str(&field.to_string());
            }
        }

        // Constants
        if self.consts.len() > 0 {
            module_information_string.push_str("\n## Constants\n\n");
            for field in &self.consts {
                module_information_string.push_str(&field.to_string());
            }
        }

        // Structs
        if self.structs.len() > 0 {
            module_information_string.push_str("\n## Structs\n\n");
            for field in &self.structs {
                module_information_string.push_str(&field.to_string());
            }
        }

        // Enums
        if self.enums.len() > 0 {
            module_information_string.push_str("\n## Enums\n\n");
            for field in &self.enums {
                module_information_string.push_str(&field.to_string());
            }
        }

        // Traits
        if self.traits.len() > 0 {
            module_information_string.push_str("\n## Traits\n\n");
            for field in &self.traits {
                module_information_string.push_str(&field.to_string());
            }
        }

        // Functions
        if self.functions.len() > 0 {
            module_information_string.push_str("\n## Functions\n\n");
            for field in &self.functions {
                module_information_string.push_str(&field.to_string());
            }
        }

        // Macros
        if self.macros.len() > 0 {
            module_information_string.push_str("\n## Macros\n\n");
            for field in &self.macros {
                module_information_string.push_str(&field.to_string());
            }
        }

        // Re-exports
        if self.re_exports.len() > 0 {
            module_information_string.push_str("\n## Re-exports\n\n");
            for field in &self.re_exports {
                module_information_string.push_str(&field.to_string());
            }
        }

        module_information_string
    }
}

#[derive(Debug, Clone)]
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

#[derive(Default, Debug, Clone)]
// #[derive(Default)]
pub struct ModuleDocumentation<'a> {
    pub file_path: String,
    pub title: &'a str,
    pub module_items: ModuleItems<'a>,
    pub content: Vec<ModuleContent<'a>>,
    pub inner_modules: Vec<ModuleDocumentation<'a>>,
}

#[derive(Debug, Clone)]
pub struct ModuleContent<'a> {
    pub file_path: String,
    pub kind: ItemKind,
    pub title: &'a str,
    pub inner: Document,
}

pub struct ModuleGenerator<'a> {
    pub configuration: &'a Configuration,
    pub prefix_path: String,
    pub root_item: &'a Item,
    pub index: &'a Index,
    pub paths: &'a Paths,
    pub external_crate: &'a ExternalCrates,
}

impl<'a> ModuleGenerator<'a> {
    pub fn new(
        configuration: &'a Configuration,
        prefix_path: String,
        root_item: &'a Item,
        index: &'a Index,
        paths: &'a Paths,
        external_crate: &'a ExternalCrates,
    ) -> Self {
        Self {
            configuration,
            prefix_path,
            root_item,
            index,
            paths,
            external_crate,
        }
    }

    pub fn auto(self) -> Result<ModuleDocumentation<'a>> {
        let mut module_documentations = ModuleDocumentation::default();

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

        module_documentations.title = module_name;

        let mut path = format!("{}/{}", self.prefix_path, module_name);

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
                None => "",
            };

            let mut path = format!("{}", path);

            match &item.inner {
                ItemEnum::Module(_) => {
                    module_documentations
                        .module_items
                        .modules
                        .push(ModuleField {
                            name: item_name,
                            link: format!(
                                "./{}/{}{}",
                                item_name,
                                self.configuration.default_module_file_name,
                                self.configuration.default_link_file_extension
                            ),
                            description: &item_description,
                        });

                    let new_module_generator = Self::new(
                        &self.configuration,
                        path,
                        item,
                        self.index,
                        self.paths,
                        self.external_crate,
                    );
                    let n_module_documentation = new_module_generator.auto()?;
                    module_documentations
                        .inner_modules
                        .push(n_module_documentation);

                    // Move to the next item as module will document itself separately
                    continue;
                }
                ItemEnum::Struct(_) => {
                    module_documentations
                        .module_items
                        .structs
                        .push(ModuleField {
                            name: item_name,
                            link: format!(
                                "./struct.{}{}",
                                item_name, &self.configuration.default_link_file_extension
                            ),
                            description: &item_description,
                        });

                    path.push_str("/struct.");
                    path.push_str(item_name);
                    path.push_str(".md");

                    let inner = StructGenerator::generate_page(
                        item,
                        self.index,
                        self.paths,
                        self.external_crate,
                        self.configuration,
                    )?;

                    module_documentations.content.push(ModuleContent {
                        file_path: path.clone(),
                        kind: ItemKind::Struct,
                        title: item_name,
                        inner,
                    });
                }
                ItemEnum::Enum(_) => {
                    module_documentations.module_items.enums.push(ModuleField {
                        name: item_name,
                        link: format!(
                            "./enum.{}{}",
                            item_name, &self.configuration.default_link_file_extension
                        ),
                        description: &item_description,
                    });

                    path.push_str("/enum.");
                    path.push_str(item_name);
                    path.push_str(".md");

                    let inner = EnumGenerator::generate_page(
                        item,
                        self.index,
                        self.paths,
                        self.external_crate,
                        self.configuration,
                    )?;

                    module_documentations.content.push(ModuleContent {
                        file_path: path.clone(),
                        kind: ItemKind::Enum,
                        title: item_name,
                        inner,
                    });
                }
                _ => continue,
            }

            path.push_str(".md");
        }

        path.push_str("/");
        path.push_str(&self.configuration.default_module_file_name);
        path.push_str(".md");

        module_documentations.file_path = path;

        Ok(module_documentations.clone())
    }
}
