use rustdoc_types::{Id, ItemEnum};

use super::{
    fn_gen::FunctionGenerator, generic_gen::GenericGenerator, module_gen::InnerModuleContent,
    type_gen::TypeGenerator,
};

pub struct ImplsGenerator;

impl ImplsGenerator {
    pub fn generate_impls(
        impls: &Vec<Id>,
        index: &super::Index,
    ) -> anyhow::Result<[InnerModuleContent; 3]> {
        let mut implementations = InnerModuleContent {
            title: "Implementations".to_string(),
            content: String::new(),
        };

        let mut trait_implementations = InnerModuleContent {
            title: "Trait Implementations".to_string(),
            content: String::new(),
        };

        let mut auto_trait_implementations = InnerModuleContent {
            title: "Auto Trait Implementations".to_string(),
            content: String::new(),
        };

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
                        current_impl.content.push_str("`impl");
                        current_impl.content.push_str(&params);
                        current_impl.content.push_str(" ");

                        if impl_info.is_negative {
                            current_impl.content.push_str("!");
                        }

                        current_impl
                            .content
                            .push_str(&TypeGenerator::path_to_string(trait_));
                        current_impl.content.push_str(" for ");
                        current_impl
                            .content
                            .push_str(&&TypeGenerator::type_to_string(&impl_info.for_));
                        current_impl.content.push_str(&where_predicate);
                        current_impl.content.push_str("`\n\n");
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

                                current_impl.content.push_str("```rust\n");
                                current_impl.content.push_str(&docs);

                                current_impl
                                    .content
                                    .push_str(&FunctionGenerator::generate_syntax(
                                        function,
                                        function_name,
                                    )?);
                                current_impl.content.push_str("\n```\n\n");
                            }
                        }
                    }
                }
            }
        }

        Ok([
            implementations,
            trait_implementations,
            auto_trait_implementations,
        ])
    }
}
