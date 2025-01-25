use anyhow::Result;
use rustdoc_types::Function;

use super::{generic_gen::GenericGenerator, type_gen::TypeGenerator};

pub struct FunctionGenerator;

impl FunctionGenerator {
    pub fn generate_syntax(function: &Function, name: &str) -> Result<String> {
        let mut syntax = String::new();
        let (params, where_predicate) = GenericGenerator::generate_generics(&function.generics)?;

        let function_header = &function.header;
        if function_header.is_const {
            syntax.push_str("const ");
        }
        if function_header.is_async {
            syntax.push_str("async ");
        }
        if function_header.is_unsafe {
            syntax.push_str("unsafe ");
        }

        // TODO: Utilize `Abi`. Refer https://docs.rs/rustdoc-types/0.35.0/rustdoc_types/enum.Abi.html

        syntax.push_str("fn ");
        syntax.push_str(name);
        syntax.push_str(&params);
        syntax.push_str("(");

        for (i, (arg, type_)) in function.sig.inputs.iter().enumerate() {
            if i != 0 {
                syntax.push_str(", ");
            }

            syntax.push_str(arg);
            syntax.push_str(": ");
            syntax.push_str(&TypeGenerator::type_to_string(type_));
        }

        if function.sig.is_c_variadic {
            if function.sig.inputs.len() > 0 {
                syntax.push_str(", ");
            }
            syntax.push_str("...");
        }

        syntax.push_str(")");

        if let Some(output) = &function.sig.output {
            syntax.push_str(" -> ");
            syntax.push_str(&TypeGenerator::type_to_string(output));
        }

        syntax.push_str(&where_predicate);

        syntax.push_str(";");
        Ok(syntax)
    }
}
