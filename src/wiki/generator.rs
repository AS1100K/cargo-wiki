use crate::Configuration;
use anyhow::Result;
use rustdoc_types::Crate;

pub fn generate(configuration: &Configuration, crate_type: Crate) -> Result<()> {
    println!("{:?}", crate_type.crate_version);
    Ok(())
}