use crate::blocks;
use rustdoc_types::{ItemEnum, VariantKind};

use super::{
    generic_gen::GenericGenerator, impls_gen::ImplsGenerator, visibility_gen::VisibilityGenerator,
    *,
};

pub struct EnumGenerator;

impl Generator for EnumGenerator {
    fn generate_page(
        item: &Item,
        index: &Index,
        paths: &Paths,
        external_crates: &ExternalCrates,
        config: &Configuration,
    ) -> Result<Document> {
        let ItemEnum::Enum(rustdoc_types::Enum {
            generics,
            has_stripped_variants,
            variants,
            impls,
        }) = &item.inner
        else {
            return Err(anyhow::Error::msg(
                "Umm... Only Item with inner ItemEnum::Enum to be used here",
            ));
        };

        let mut docs = match &item.docs {
            Some(docs) => docs.clone(),
            None => String::new(),
        };
        let mut variants_section = blocks::ListBlock::new_unordered_list();
        let mut syntax = String::from("```rust\n");
        syntax.push_str(&VisibilityGenerator::generate_visibility(&item.visibility));
        syntax.push_str("enum ");

        let Some(enum_name) = &item.name else {
            return Err(anyhow::Error::msg(
                "Enum name is required for generating documentation",
            ));
        };
        let (generic_params, where_predicates) = GenericGenerator::generate_generics(&generics)?;
        syntax.push_str(enum_name);
        syntax.push_str(&generic_params);
        syntax.push_str(&where_predicates);

        if where_predicates.is_empty() {
            syntax.push_str(" {\n");
        } else {
            syntax.push_str("\n{")
        }

        for variant_id in variants {
            let Some(variant) = index.get(variant_id) else {
                return Err(anyhow::Error::msg(format!(
                    "Failed to find item with id: {} in index",
                    variant_id.0
                )));
            };
            let ItemEnum::Variant(variant_item) = &variant.inner else {
                return Err(anyhow::Error::msg(format!(
                    "inner can't be anything other than `Variant` in index id: {}",
                    variant_id.0
                )));
            };
            let Some(variant_name) = &variant.name else {
                return Err(anyhow::Error::msg(format!(
                    "Failed to find name of struct field with id: {} in index",
                    variant_id.0
                )));
            };
            syntax.push_str("\t");
            syntax.push_str(variant_name);

            if let Some(variant_doc) = &variant.docs {
                variants_section.push(
                    blocks::inline::CodeSpan::from(variant_name),
                    blocks::inline::Text::from(variant_doc),
                );
            } else {
                variants_section.push(
                    blocks::inline::CodeSpan::from(variant_name),
                    blocks::EmptyElement,
                );
            }

            match &variant_item.kind {
                VariantKind::Plain => syntax.push_str(",\n"),
                VariantKind::Tuple(tuple_items) => {
                    syntax.push_str("(");
                    for (i, items) in tuple_items.iter().enumerate() {
                        if let Some(item_id) = items {
                            if let Some(item) = index.get(item_id) {
                                if let Some(item_name) = &item.name {
                                    if i != 0 {
                                        syntax.push_str(", ");
                                    }
                                    syntax.push_str(item_name);
                                }
                            }
                        }
                    }
                    syntax.push_str("),\n");
                }
                VariantKind::Struct {
                    fields,
                    has_stripped_fields,
                } => {
                    syntax.push_str("{\n");
                    for (i, field) in fields.iter().enumerate() {
                        if i != 0 {
                            syntax.push_str(",\n\t\t");
                        }
                        let Some(field_item) = index.get(field) else {
                            continue;
                        };
                        let Some(field_name) = &field_item.name else {
                            continue;
                        };
                        syntax.push_str(field_name);
                        syntax.push_str(": /* TODO Type Generation in Syntax */");
                    }

                    if *has_stripped_fields {
                        syntax.push_str(",\n\t\t/* Private Fields */");
                    }

                    syntax.push_str("\t}");
                }
            };
        }

        if *has_stripped_variants {
            syntax.push_str("\t/* Private Variants */\n");
        }

        syntax.push_str("}\n```");

        let [impl1, impl2, impl3] = ImplsGenerator::generate_impls(impls, index)?;

        Ok(vec![
            Box::new(blocks::RawBlock::from(syntax)),
            Box::new(blocks::RawBlock::from(docs)),
            Box::new(blocks::DropDown::new_closed(
                blocks::inline::Text::from("Variants"),
                variants_section,
            )),
            impl1,
            impl2,
            impl3,
        ])
    }
}
