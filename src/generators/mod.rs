use anyhow::Result;
use rustdoc_types::{Id, Item, ItemSummary};
use std::collections::HashMap;

pub mod generic_gen;
pub mod struct_gen;
pub mod type_gen;
pub mod visibility_gen;

pub type Index = HashMap<Id, Item>;
pub type Paths = HashMap<Id, ItemSummary>;
pub type ExternalCrates = HashMap<u32, rustdoc_types::ExternalCrate>;

pub trait Generator {
    fn generate_syntax(
        item: &Item,
        index: &Index,
        paths: &Paths,
        external_crates: &ExternalCrates,
    ) -> Result<String>;
}
