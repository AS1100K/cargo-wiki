use rustdoc_types::{DynTrait, Path, Type};

pub struct TypeGenerator;

impl TypeGenerator {
    pub fn type_to_string(type_: &Type) -> String {
        let mut type_string = String::new();
        match type_ {
            Type::ResolvedPath(Path { name, args, id }) => {
                type_string.push_str(name);
                // TODO: parse args
            }
            Type::DynTrait(DynTrait { traits, lifetime }) => {
                // TODO
            }
            Type::Generic(generic_string) => { type_string.push_str(generic_string) }
            Type::Primitive(primitive_type) => { type_string.push_str(primitive_type) }
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
            Type::Array {  type_, len  } => {
                type_string.push_str("[");
                type_string.push_str(&Self::type_to_string(type_));
                type_string.push_str("; ");
                type_string.push_str(len);
                type_string.push_str("]");
            }
            Type::Pat {..} => {
                // TODO
            }
            Type::ImplTrait(_) => {
                // TODO
            }
            Type::Infer => { type_string.push_str("_") }
            Type::RawPointer { is_mutable, type_ } => {
                if *is_mutable {
                    type_string.push_str("*mut ");
                } else {
                    type_string.push_str("*const ");
                }
                type_string.push_str(&Self::type_to_string(type_));
            }
            Type::BorrowedRef { lifetime, is_mutable, type_ } => {
                type_string.push_str("&");
                if let Some(lifetime) = lifetime {
                    type_string.push_str("'");
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
}