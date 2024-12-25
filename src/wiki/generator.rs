use crate::generators::module_gen::{ModuleGenerator, MODULE_FILE_NAME};
use crate::{gen_path, Configuration, WIKI_CACHE_PATH};
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

    let module_generator = ModuleGenerator::new(
        path,
        MODULE_FILE_NAME,
        root_module,
        &crate_type.index,
        &crate_type.paths,
        &crate_type.external_crates,
    );
    module_generator.auto()?;

    Ok(())
}
