use crate::{blocks::inline::Link, generators::generic_gen::GenericGenerator};
use rustdoc_types::{DynTrait, ItemKind, ItemSummary, Path, Type};

use super::{ExternalCrates, Paths};

pub struct TypeGenerator;

impl TypeGenerator {
    pub fn type_to_string(type_: &Type) -> String {
        let mut type_string = String::new();
        match type_ {
            Type::ResolvedPath(path) => type_string.push_str(&Self::path_to_string(&path)),
            Type::DynTrait(DynTrait { traits, lifetime }) => {
                // TODO
            }
            Type::Generic(generic_string) => type_string.push_str(generic_string),
            Type::Primitive(primitive_type) => type_string.push_str(primitive_type),
            Type::FunctionPointer(_) => {
                // TODO
            }
            Type::Tuple(tuple_types) => {
                type_string.push_str("(");
                for (i, tuple_type) in tuple_types.iter().enumerate() {
                    if i != 0 {
                        type_string.push_str(", ");
                        type_string.push_str(&Self::type_to_string(tuple_type))
                    }
                }
                type_string.push_str(")");
            }
            Type::Slice(type_) => {
                type_string.push_str("[");
                type_string.push_str(&Self::type_to_string(type_));
                type_string.push_str("]");
            }
            Type::Array { type_, len } => {
                type_string.push_str("[");
                type_string.push_str(&Self::type_to_string(type_));
                type_string.push_str("; ");
                type_string.push_str(len);
                type_string.push_str("]");
            }
            Type::Pat { .. } => {
                // TODO
            }
            Type::ImplTrait(_) => {
                // TODO
            }
            Type::Infer => type_string.push_str("_"),
            Type::RawPointer { is_mutable, type_ } => {
                if *is_mutable {
                    type_string.push_str("*mut ");
                } else {
                    type_string.push_str("*const ");
                }
                type_string.push_str(&Self::type_to_string(type_));
            }
            Type::BorrowedRef {
                lifetime,
                is_mutable,
                type_,
            } => {
                type_string.push_str("&");
                if let Some(lifetime) = lifetime {
                    type_string.push_str(lifetime);
                    type_string.push_str(" ");
                }
                if *is_mutable {
                    type_string.push_str("mut ");
                }
                type_string.push_str(&Self::type_to_string(type_));
            }
            Type::QualifiedPath { .. } => {
                // TODO
            }
        }

        type_string
    }

    pub fn type_to_link(type_: &Type, paths: &Paths, external_crates: &ExternalCrates) -> Link {
        match type_ {
            Type::ResolvedPath(path) => {
                let item_summary = paths.get(&path.id);

                if let Some(item_summary) = item_summary {
                    let external_crate = external_crates.get(&item_summary.crate_id);

                    if let Some(external_crate) = external_crate {
                        let root_url = if let Some(root_url) = &external_crate.html_root_url {
                            root_url.to_owned()
                        } else {
                            String::new()
                        };

                        Link::new(
                            TypeGenerator::path_to_string(path),
                            format!(
                                "{}{}",
                                root_url,
                                TypeGenerator::item_summary_to_url(item_summary)
                            ),
                        )
                    } else {
                        Link::empty()
                    }
                } else {
                    Link::empty()
                }
            }
            Type::DynTrait(dyn_trait) => Link::empty(),
            Type::Generic(generic) => Link::empty(),
            Type::Primitive(primitive) => Link::empty(),
            Type::FunctionPointer(function_pointer) => Link::empty(),
            Type::Tuple(tuples) => Link::empty(),
            Type::Slice(slice) => Link::empty(),
            Type::Array { type_, len } => Link::empty(),
            Type::Pat {
                type_,
                __pat_unstable_do_not_use,
            } => Link::empty(),
            Type::ImplTrait(generic_bounds) => Link::empty(),
            Type::Infer => Link::empty(),
            Type::RawPointer { is_mutable, type_ } => Link::empty(),
            Type::BorrowedRef {
                lifetime,
                is_mutable,
                type_,
            } => Link::empty(),
            Type::QualifiedPath {
                name,
                args,
                self_type,
                trait_,
            } => Link::empty(),
        }
    }

    pub fn path_to_string(path: &Path) -> String {
        let mut path_string = String::new();

        path_string.push_str(path.path.split("::").last().unwrap_or_default());

        if let Some(args) = &path.args {
            path_string.push_str(&GenericGenerator::generate_generic_args(args));
        }

        path_string
    }

    pub fn item_summary_to_url(path: &ItemSummary) -> String {
        let mut url = String::new();

        for (i, path_piece) in path.path.iter().enumerate() {
            if i != (path.path.len() - 1) {
                url.push_str(path_piece);
                url.push_str("/");
            } else {
                match &path.kind {
                    ItemKind::Module => {
                        url.push_str(path_piece);
                        url.push_str("/index.html");
                    }
                    ItemKind::ExternCrate => {}
                    ItemKind::Use => {}
                    ItemKind::Struct => {
                        url.push_str("struct.");
                        url.push_str(path_piece);
                        url.push_str(".html");
                    }
                    ItemKind::StructField => {}
                    ItemKind::Union => {}
                    ItemKind::Enum => {
                        url.push_str("enum.");
                        url.push_str(path_piece);
                        url.push_str(".html");
                    }
                    ItemKind::Variant => {}
                    ItemKind::Function => {}
                    ItemKind::TypeAlias => {}
                    ItemKind::Constant => {}
                    ItemKind::Trait => {}
                    ItemKind::TraitAlias => {}
                    ItemKind::Impl => {}
                    ItemKind::Static => {}
                    ItemKind::ExternType => {}
                    ItemKind::Macro => {}
                    ItemKind::ProcAttribute => {}
                    ItemKind::ProcDerive => {}
                    ItemKind::AssocConst => {}
                    ItemKind::AssocType => {}
                    ItemKind::Primitive => {}
                    ItemKind::Keyword => {}
                }
            }
        }

        url
    }
}
