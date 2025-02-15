use crate::{blocks::Document, Configuration};
use anyhow::Result;
use rustdoc_types::{Id, Item, ItemSummary};
use std::collections::HashMap;

pub mod enum_gen;
pub mod fn_gen;
pub mod generic_gen;
pub mod impls_gen;
pub mod module_gen;
pub mod struct_gen;
pub mod type_gen;
pub mod visibility_gen;

pub type Index = HashMap<Id, Item>;
pub type Paths = HashMap<Id, ItemSummary>;
pub type ExternalCrates = HashMap<u32, rustdoc_types::ExternalCrate>;

/// A collection of generic generator functions. All the generators that implement this trait are
/// of those types that have a whole file based for them.
///
/// **NOTE:** [ModuleGenerator](module_gen::ModuleGenerator) doesn't follow `Generator` trait even
/// though module have a whole file for them as this is a special kind of generator.
pub trait Generator {
    fn generate_page(
        item: &Item,
        index: &Index,
        paths: &Paths,
        external_crates: &ExternalCrates,
        config: &Configuration,
    ) -> Result<Document>;
}
