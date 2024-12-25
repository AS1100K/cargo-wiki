use rustdoc_types::{GenericBound, GenericParamDef, GenericParamDefKind, Generics, Type, WherePredicate};
use anyhow::Result;
use crate::generators::type_gen::TypeGenerator;

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
                    WherePredicate::BoundPredicate { type_, bounds, generic_params } => {
                        if let Type::Generic(type_name) = type_ {
                            let generic_params = Self::generate_generic_params(generic_params)?;
                            // TODO
                        }

                        return Err(anyhow::Error::msg("Only Generic Types are allowed in Where Predicate type_"))
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
                        params.push_str("'");
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
                        if let Some(default) = default {
                            params.push_str(" = ");
                            params.push_str(default);
                        } else {
                            params.push_str(": ");
                            params.push_str(&TypeGenerator::type_to_string(type_));
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
            GenericBound::TraitBound { trait_, generic_params, modifier } => {
                // TODO
            }
            GenericBound::Outlives(lifetime) => {
                bound_string.push_str("'");
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
}