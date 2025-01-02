use crate::generators::module_gen::{ModuleContent, ModuleDocumentation, ModuleGenerator, MODULE_FILE_NAME};
use crate::{gen_path, save_file, Configuration, WikiStructure, WIKI_CACHE_PATH};
use anyhow::Result;
use rustdoc_types::Crate;

pub fn generate_wiki(configuration: &Configuration, crate_type: Crate) -> Result<()> {
    let crate_id = &crate_type.root;
    let path = WIKI_CACHE_PATH.to_string();
    gen_path(&path).expect("Failed to create file");

    let Some(root_module) = crate_type.index.get(&crate_type.root) else {
        return Err(anyhow::Error::msg(format!(
            "Failed to find the root module with id: {} in index",
            crate_id.0
        )));
    };

    let Some(root_module_name) = &root_module.name else {
        return Err(anyhow::Error::msg("The crate must have a name, Empty name is not allowed."))
    };

    let module_generator = ModuleGenerator::new(
        configuration,
        path,
        MODULE_FILE_NAME,
        root_module,
        &crate_type.index,
        &crate_type.paths,
        &crate_type.external_crates,
    );

    let module_docs = module_generator.auto()?.generate_docs(configuration)?;
    if !module_docs.is_empty() {
        let path = format!("{}/{}.md", WIKI_CACHE_PATH, root_module_name);
        save_file(&path, &module_docs)?;
    }

    Ok(())
}

impl<'a> ModuleDocumentation<'a> {
    pub fn generate_docs(self, configuration: &Configuration) -> Result<String> {
        let mut file_content = format!("# {} \n\n", self.title);
        file_content.push_str(&self.module_items.generate_docs());

        match configuration.structure {
            WikiStructure::Directory => {
                for content in self.content {
                    content.save_to_file()?
                }

                for module in self.inner_modules {
                    module.generate_docs(configuration)?;
                }

                save_file(&self.file_path, &file_content)?;
                Ok(String::new())
            }
            WikiStructure::SingleFile => {
                for content in self.content {
                    file_content.push_str("\n## ");
                    file_content.push_str(&content.title);
                    file_content.push_str("\n\n");
                    file_content.push_str(&content.page);
                }

                for module in self.inner_modules {
                    let inner_module_doc = module.generate_docs(configuration)?;
                    file_content.push_str("\n\n");
                    file_content.push_str(&inner_module_doc);
                }

                Ok(file_content)
            }
        }
    }
}

impl<'a> ModuleContent<'a> {
    pub fn save_to_file(self) -> Result<()> {
        let mut file_content = format!("# {}\n\n", self.title);
        file_content.push_str(&self.page);

        save_file(&self.file_path, &file_content)
    }
}