use crate::generators::generic_gen::GenericGenerator;
use crate::generators::type_gen::TypeGenerator;
use crate::generators::visibility_gen::VisibilityGenerator;
use crate::generators::{ExternalCrates, Generator, Index, Paths};
use anyhow::Result;
use rustdoc_types::{Item, ItemEnum, StructKind};

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
            let mut fields_section = String::new();
            let mut syntax = String::from("```rust\n");
            syntax.push_str(&VisibilityGenerator::generate_visibility(&item.visibility));
            syntax.push_str("struct ");

            if let Some(struct_name) = &item.name {
                let (generic_params, where_predicates) =
                    GenericGenerator::generate_generics(&generics)?;
                syntax.push_str(struct_name);
                syntax.push_str(&generic_params);
                syntax.push_str(&where_predicates);

                match kind {
                    StructKind::Unit => {
                        syntax.push_str(";");
                    }
                    StructKind::Tuple(unnamed_fields) => {
                        syntax.push_str("(");
                        for (i, unnamed_field) in unnamed_fields.iter().enumerate() {
                            if i != 0 {
                                syntax.push_str(", ");
                            }

                            if let Some(field_id) = unnamed_field {
                                let Some(field_item) = index.get(field_id) else {
                                    return Err(anyhow::Error::msg(format!(
                                        "Failed to find item with id: {} in index",
                                        field_id.0
                                    )));
                                };
                                let ItemEnum::StructField(type_) = &field_item.inner else {
                                    return Err(anyhow::Error::msg(format!("inner can't be anything other than `StructField in index id: {}", field_id.0)));
                                };
                                syntax.push_str(&TypeGenerator::type_to_string(type_));
                            } else {
                                syntax.push_str("/* private field */");
                            }
                        }
                        syntax.push_str(")");
                    }
                    StructKind::Plain {
                        fields,
                        has_stripped_fields,
                    } => {
                        if where_predicates.is_empty() {
                            syntax.push_str(" {\n");
                        } else {
                            syntax.push_str("\n{")
                        }

                        if fields.len() > 0 {
                            fields_section.push_str("## Fields\n")
                        }

                        for field_id in fields {
                            let Some(field_item) = index.get(field_id) else {
                                return Err(anyhow::Error::msg(format!(
                                    "Failed to find item with id: {} in index",
                                    field_id.0
                                )));
                            };
                            let Some(field_name) = &field_item.name else {
                                return Err(anyhow::Error::msg(format!(
                                    "Failed to find name of struct field with id: {} in index",
                                    field_id.0
                                )));
                            };
                            let ItemEnum::StructField(type_) = &field_item.inner else {
                                return Err(anyhow::Error::msg(format!("inner can't be anything other than `StructField in index id: {}", field_id.0)));
                            };

                            syntax.push_str("\t");
                            syntax.push_str(&VisibilityGenerator::generate_visibility(
                                &field_item.visibility,
                            ));
                            syntax.push_str(field_name);
                            syntax.push_str(": ");
                            syntax.push_str(&TypeGenerator::type_to_string(type_));
                            syntax.push_str(",\n");

                            fields_section.push_str("- `");
                            fields_section.push_str(field_name);
                            fields_section.push_str("` : `");
                            fields_section.push_str(&TypeGenerator::type_to_string(type_));
                            fields_section.push_str("`\n");
                        }

                        if *has_stripped_fields {
                            syntax.push_str("\t/* private fields */\n");
                        }
                        syntax.push_str("}");
                    }
                }
                syntax.push_str("\n```\n");
                syntax.push_str(&fields_section);
                return Ok(syntax);
            }
            return Err(anyhow::Error::msg("Can't document a struct with no name"));
        }
        Err(anyhow::Error::msg(
            "Umm... Only Item with inner ItemEnum::Struct to be used here",
        ))
    }
}
