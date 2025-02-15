use rustdoc_types::{Id, ItemEnum};

use crate::blocks::{
    inline::{CodeSpan, Text},
    Document, NLines, RawBlock, Title,
};

use super::{fn_gen::FunctionGenerator, generic_gen::GenericGenerator, type_gen::TypeGenerator};

pub struct ImplsGenerator;

impl ImplsGenerator {
    pub fn generate_impls(
        impls: &Vec<Id>,
        index: &super::Index,
    ) -> anyhow::Result<[Box<Document>; 3]> {
        let mut implementations: Document =
            vec![Box::new(Title::new(2, Text::from("Implementations")))];

        let mut trait_implementations: Document =
            vec![Box::new(Title::new(2, Text::from("Trait Implementations")))];

        let mut auto_trait_implementations: Document = vec![Box::new(Title::new(
            2,
            Text::from("Auto Trait Implementations"),
        ))];

        for id in impls {
            if let Some(item) = index.get(id) {
                if let ItemEnum::Impl(impl_info) = &item.inner {
                    let current_impl = if impl_info.trait_.is_some() {
                        if impl_info.is_synthetic {
                            &mut auto_trait_implementations
                        } else {
                            &mut trait_implementations
                        }
                    } else {
                        &mut implementations
                    };

                    let Ok((params, where_predicate)) =
                        GenericGenerator::generate_generics(&impl_info.generics)
                    else {
                        return Err(anyhow::Error::msg(
                            "Failed to gerate syntax for generics in impl block",
                        ));
                    };

                    if let Some(trait_) = &impl_info.trait_ {
                        let mut content = String::from("impl");

                        content.push_str(&params);
                        content.push_str(" ");

                        if impl_info.is_negative {
                            content.push_str("!");
                        }

                        content.push_str(&TypeGenerator::path_to_string(trait_));
                        content.push_str(" for ");
                        content.push_str(&&TypeGenerator::type_to_string(&impl_info.for_));
                        content.push_str(&where_predicate);

                        current_impl.push(Box::new(NLines::new(2)));
                        current_impl.push(Box::new(CodeSpan::from(content)));
                        current_impl.push(Box::new(NLines::new(2)));
                    }

                    // Generate syntax for functions
                    for function_id in &impl_info.items {
                        if let Some(function_item) = index.get(function_id) {
                            let Some(function_name) = &function_item.name else {
                                continue;
                            };
                            if let ItemEnum::Function(function) = &function_item.inner {
                                let docs = match &function_item.docs {
                                    Some(docs) => {
                                        if !docs.is_empty() {
                                            let mut new_docs = String::new();
                                            for line in docs.lines() {
                                                new_docs.push_str("/// ");
                                                new_docs.push_str(line);
                                                new_docs.push_str("\n");
                                            }
                                            new_docs
                                        } else {
                                            String::new()
                                        }
                                    }
                                    None => String::new(),
                                };

                                let mut content = String::from("```rust\n");
                                content.push_str(&docs);

                                content.push_str(&FunctionGenerator::generate_syntax(
                                    function,
                                    function_name,
                                )?);

                                content.push_str("\n```");

                                current_impl.push(Box::new(RawBlock::from(content)));
                            }
                        }
                    }
                }
            }
        }

        Ok([
            Box::new(implementations),
            Box::new(trait_implementations),
            Box::new(auto_trait_implementations),
        ])
    }
}
