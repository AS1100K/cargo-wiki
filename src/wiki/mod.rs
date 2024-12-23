pub mod generator;

use std::fs;
use std::path::Path;
use rustdoc_types::Crate;
use anyhow::Result;

pub trait CrateExt {
    fn from_file<P: AsRef<Path>>(file: P) -> Result<Crate>;
}

impl CrateExt for Crate {
    fn from_file<P: AsRef<Path>>(file: P) -> Result<Crate> {
        let bytes = fs::read(file)?;
        Ok(serde_json::from_slice(&bytes)?)
    }
}