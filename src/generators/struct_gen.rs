use crate::generators::{ExternalCrates, Generator, Index, Paths};
use anyhow::Result;
use rustdoc_types::{Item, ItemEnum, StructKind, Visibility};
use crate::generators::generic_gen::GenericGenerator;

pub struct StructGenerator;

impl Generator for StructGenerator {
    /// Generate Code Syntax for `Struct`. See [Template.html](https://github.com/AS1100K/cargo-wiki/blob/main/Template.md#struct-syntax-block)
    /// for more info.
    fn generate_syntax(
        item: &Item,
        index: &Index,
        paths: &Paths,
        external_crates: &ExternalCrates,
    ) -> Result<String> {
        if let ItemEnum::Struct(rustdoc_types::Struct {
            kind,
            generics,
            impls,
        }) = &item.inner
        {
            let mut syntax = String::from("```rust\n");
            match &item.visibility {
                Visibility::Public => syntax.push_str("pub "),
                Visibility::Default => {},
                Visibility::Crate => syntax.push_str("pub(crate) "),
                Visibility::Restricted { parent, path } => {
                    // TODO: Utilize parent
                    syntax.push_str("pub(");
                    syntax.push_str(path);
                    syntax.push_str(") ");
                }
            }
            syntax.push_str("struct ");

            if let Some(struct_name) = &item.name {
                let (generic_params, where_predicates) = GenericGenerator::generate_generics(&generics)?;
                match kind {
                    StructKind::Unit => {
                        syntax.push_str(struct_name);
                        syntax.push_str(&generic_params);
                        syntax.push_str(&where_predicates);
                        syntax.push_str(";");
                    }
                    StructKind::Tuple(unnamed_fields) => {
                        // TODO
                    }
                    StructKind::Plain {
                        fields,
                        has_stripped_fields,
                    } => {
                        // TODO
                    }
                }
                syntax.push_str("\n```\n");
                return Ok(syntax)
            }
            return Err(anyhow::Error::msg("Can't document a struct with no name"))
        }
        Err(anyhow::Error::msg(
            "Umm... Only Item with inner ItemEnum::Struct to be used here",
        ))
    }
}
