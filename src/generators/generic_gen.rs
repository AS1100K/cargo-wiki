use crate::generators::type_gen::TypeGenerator;
use anyhow::Result;
use rustdoc_types::{
    GenericArg, GenericArgs, GenericBound, GenericParamDef, GenericParamDefKind, Generics,
    TraitBoundModifier, Type, WherePredicate,
};

pub struct GenericGenerator;

impl GenericGenerator {
    pub fn generate_generics(generics: &Generics) -> Result<(String, String)> {
        let params = Self::generate_generic_params(&generics.params)?;
        let mut where_predicates = String::new();

        if generics.where_predicates.len() > 0 {
            where_predicates.push_str("\nwhere\n");

            for (i, where_pred) in generics.where_predicates.iter().enumerate() {
                if i != 0 {
                    where_predicates.push_str(",\n");
                }

                match where_pred {
                    WherePredicate::BoundPredicate {
                        type_,
                        bounds,
                        generic_params,
                    } => {
                        if let Type::Generic(type_name) = type_ {
                            let generic_params = Self::generate_generic_params(generic_params)?;
                            // TODO
                        }

                        return Err(anyhow::Error::msg(
                            "Only Generic Types are allowed in Where Predicate type_",
                        ));
                    }
                    WherePredicate::LifetimePredicate { lifetime, outlives } => {
                        // TODO
                    }
                    WherePredicate::EqPredicate { lhs, rhs } => {
                        // TODO
                    }
                }
            }
        }

        Ok((params, where_predicates))
    }

    pub fn generate_generic_params(generic_params: &Vec<GenericParamDef>) -> Result<String> {
        let mut params = String::new();

        if generic_params.len() > 0 {
            params.push_str("<");
            for (i, param_def) in generic_params.iter().enumerate() {
                // Add a comma before, just for some nice formatting
                if i != 0 {
                    params.push_str(", ")
                }

                match &param_def.kind {
                    GenericParamDefKind::Lifetime { outlives } => {
                        // name already contains '
                        params.push_str(&param_def.name);

                        if outlives.len() > 0 {
                            params.push_str(": ");

                            for (x, other_name) in outlives.iter().enumerate() {
                                if x != 0 {
                                    // Add a comma before, just for some nice formatting
                                    params.push_str(" + ");
                                    params.push_str(&other_name);
                                }
                            }
                        }
                    }
                    GenericParamDefKind::Type {
                        bounds,
                        default,
                        is_synthetic,
                    } => {
                        params.push_str(&param_def.name);
                        if let Some(default_type) = default {
                            params.push_str(" = ");
                            params.push_str(&TypeGenerator::type_to_string(default_type));
                        } else {
                            for (i, bound) in bounds.iter().enumerate() {
                                if i != 0 {
                                    params.push_str(" + ");
                                }
                                params.push_str(&Self::generate_generic_bounds(bound)?);
                            }
                        }
                        // TODO use `is_synthetic` field
                    }
                    GenericParamDefKind::Const { type_, default } => {
                        params.push_str("const ");
                        params.push_str(&param_def.name);

                        params.push_str(": ");
                        params.push_str(&TypeGenerator::type_to_string(type_));
                        if let Some(default) = default {
                            params.push_str(" = ");
                            params.push_str(default);
                        }
                    }
                }
            }
            params.push_str(">");
        }

        Ok(params)
    }

    pub fn generate_generic_bounds(bound: &GenericBound) -> Result<String> {
        let mut bound_string = String::new();
        match bound {
            GenericBound::TraitBound {
                trait_,
                generic_params,
                modifier,
            } => {
                bound_string.push_str(match modifier {
                    TraitBoundModifier::None => "",
                    TraitBoundModifier::Maybe => "?",
                    // Not really sure if this is correct
                    TraitBoundModifier::MaybeConst => "const? ",
                });

                bound_string.push_str("for");
                bound_string.push_str(&Self::generate_generic_params(generic_params)?);
                bound_string.push_str(" ");

                bound_string.push_str(&TypeGenerator::path_to_string(&trait_));
            }
            GenericBound::Outlives(lifetime) => {
                bound_string.push_str(lifetime);
            }
            GenericBound::Use(uses) => {
                bound_string.push_str("use<");
                for (i, use_) in uses.iter().enumerate() {
                    if i != 0 {
                        bound_string.push_str(", ");
                    }
                    bound_string.push_str(use_);
                }
                bound_string.push_str(">");
            }
        }
        Ok(bound_string)
    }

    pub fn generate_generic_args(generic_args: &GenericArgs) -> String {
        let mut generic_arg_string = String::new();

        match generic_args {
            GenericArgs::AngleBracketed { args, constraints } => {
                if args.len() > 0 {
                    generic_arg_string.push_str("<");

                    for (i, arg) in args.iter().enumerate() {
                        if i != 0 {
                            generic_arg_string.push_str(", ");
                        }

                        match arg {
                            GenericArg::Lifetime(lifetime) => generic_arg_string.push_str(lifetime),
                            GenericArg::Type(type_) => {
                                generic_arg_string.push_str(&TypeGenerator::type_to_string(type_))
                            }
                            // TODO: Account for other variables in constant i.e. value and is_literal
                            GenericArg::Const(constant) => {
                                generic_arg_string.push_str(&format!("{{ {} }}", constant.expr))
                            }
                            GenericArg::Infer => generic_arg_string.push_str("_"),
                        }

                        // TODO: Also, account for constraints
                    }

                    generic_arg_string.push_str(">");
                }
            }
            GenericArgs::Parenthesized { inputs, output } => {
                generic_arg_string.push_str("Fn(");

                for (i, input) in inputs.iter().enumerate() {
                    if i != 0 {
                        generic_arg_string.push_str(", ");
                    }
                    generic_arg_string.push_str(&TypeGenerator::type_to_string(input));
                }

                generic_arg_string.push_str(")");

                if let Some(type_) = output {
                    generic_arg_string.push_str(" -> ");
                    generic_arg_string.push_str(&TypeGenerator::type_to_string(type_));
                }
            }
        }

        generic_arg_string
    }
}
