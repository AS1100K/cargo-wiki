use rustdoc_types::{GenericParamDef, GenericParamDefKind, Generics, Type, WherePredicate};
use anyhow::Result;

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
                        if let Some(default_type) = default {
                            params.push_str(&param_def.name);
                            params.push_str(" = ");
                            // TODO: params.push_str(&default_type);
                        } else {
                            for bound in bounds {
                                // TODO
                            }
                        }
                    }
                    GenericParamDefKind::Const { type_, default } => {
                        params.push_str(&param_def.name);
                        params.push_str(" = ");
                        // TODO: Param Type
                    }
                }
            }
            params.push_str(">");
        }

        Ok(params)
    }
}