use std::collections::HashMap;

use error::Error;
use heck::ToUpperCamelCase;

use crate::{
    parse::Quantifier,
    pg_type::{CornucopiaType, CornucopiaTypeKind, TypeRegistrar},
    prepare_queries::{PreparedQuery, RustReturnType},
};

use super::prepare_queries::PreparedModule;

pub fn generate_query(query: &PreparedQuery) -> Result<String, Error> {
    let name = &query.name;
    let query_struct = generate_query_struct(query)?.unwrap_or_default();
    let params = generate_query_params(query)?;
    let ret_ty = generate_query_ret_ty(query)?;
    let ret = generate_query_quantified_ret_ty(query, &ret_ty);

    let body = generate_query_body(query, ret_ty)?;
    Ok(format!(
        r#"{query_struct}
pub async fn {name}(client:&Client, {params}) -> Result<{ret},Error> {{{body}}}
pub async fn {name}_tx<'a>(client:&Transaction<'a>, {params}) -> Result<{ret}, Error> {{{body}}}
"#
    ))
}

pub fn generate_custom_type(ty: &CornucopiaType) -> String {
    let type_def = match &ty.kind {
        CornucopiaTypeKind::Enum(variants) => {
            let name = &ty.rust_ty_name;
            format!("pub enum {} {{ {} }}", name, variants.join(","))
        }
        CornucopiaTypeKind::Domain(domain_inner_ty) => {
            let name = &domain_inner_ty.rust_ty_name;
            format!("pub struct {} ({})", ty.pg_ty.schema(), name)
        }
        CornucopiaTypeKind::Composite(fields) => {
            let fields_str = fields
                .iter()
                .map(|f| format!("pub {} : {}", f.name, f.ty.rust_ty_definition_path))
                .collect::<Vec<String>>()
                .join(",");

            let name = &ty.rust_ty_name;
            format!("pub struct {} {{ {} }}", name, fields_str)
        }
        CornucopiaTypeKind::Base => {
            panic!("got base type while generating custom types. This is a bug")
        }
    };

    format!(
        "#[postgres(name = \"{}\")]\n{}",
        ty.pg_ty.name(),
        type_def
    )
}

pub fn generate_type_modules(type_registrar: &TypeRegistrar) -> String {
    let mut modules = HashMap::<String, Vec<CornucopiaType>>::new();
    for ((schema, _), ty) in &type_registrar.custom_types {
        match modules.entry(schema.to_owned()) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                entry.get_mut().push(ty.clone());
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(vec![ty.clone()]);
            }
        }
    }
    let schema_modules = modules
        .iter()
        .map(|(mod_name, tys)| {
            let tys_str = tys
                .iter()
                .map(|ty| generate_custom_type(ty))
                .collect::<Vec<String>>()
                .join("\n\n");
            format!(
                "
        pub mod {mod_name} {{
            use postgres_types::{{FromSql, Kind, Oid, ToSql, Type}};

            {tys_str} 
        }}"
            )
        })
        .collect::<Vec<String>>()
        .join("\n\n");

    format!(
        "pub mod types {{
    {schema_modules}        
}}"
    )
}

pub fn generate_query_struct(query: &PreparedQuery) -> Result<Option<String>, Error> {
    if let RustReturnType::Struct(fields) = &query.ret {
        let mut field_strings = Vec::new();
        for field in fields {
            field_strings.push(format!(
                r#"pub {} : {}"#,
                field.name, field.ty.rust_ty_usage_path,
            ));
        }
        let fields_string = field_strings.join(",");
        let struct_name = query.name.to_upper_camel_case();

        Ok(Some(format!(
            r#"#[derive(Debug, Clone, PartialEq)]
pub struct {struct_name} {{{fields_string}}}"#
        )))
    } else {
        Ok(None)
    }
}

pub fn generate_query_quantified_ret_ty(query: &PreparedQuery, ret_ty: &str) -> String {
    if let RustReturnType::Void = query.ret {
        String::from("()")
    } else {
        match query.quantifier {
            Quantifier::ZeroOrMore => format!("Vec<{ret_ty}>"),
            Quantifier::ZeroOrOne => format!("Option<{ret_ty}>"),
            Quantifier::One => ret_ty.to_string(),
        }
    }
}

pub fn generate_query_params(query: &PreparedQuery) -> Result<String, Error> {
    let mut param_strings = Vec::new();
    for param in &query.params {
        let rs_ty_borrowed = &param.ty.borrowed_rust_ty();
        let param_string = format!("{} : {}", param.name, rs_ty_borrowed);
        param_strings.push(param_string);
    }
    Ok(param_strings.join(","))
}

pub fn generate_query_ret_ty(query: &PreparedQuery) -> Result<String, Error> {
    Ok(match &query.ret {
        crate::prepare_queries::RustReturnType::Void => String::from("()"),
        crate::prepare_queries::RustReturnType::Scalar(ty) => ty.rust_ty_usage_path.clone(),
        crate::prepare_queries::RustReturnType::Tuple(tys) => {
            let mut tys_string = Vec::new();
            for ty in tys {
                tys_string.push(ty.rust_ty_usage_path.clone())
            }
            format!("({})", tys_string.join(","))
        }
        crate::prepare_queries::RustReturnType::Struct(_) => query.name.to_upper_camel_case(),
    })
}

pub fn generate_query_body(query: &PreparedQuery, ret_ty: String) -> Result<String, Error> {
    let query_string = format!(r#""{}""#, &query.sql);
    let override_types = query
        .override_types
        .iter()
        .map(|ty| &ty.custom_pg_ty_path)
        .cloned()
        .collect::<Vec<String>>()
        .join(",");

    let query_method = if let RustReturnType::Void = query.ret {
        "execute"
    } else {
        match query.quantifier {
            Quantifier::ZeroOrMore => "query",
            Quantifier::ZeroOrOne => "query_opt",
            Quantifier::One => "query_one",
        }
    };

    let query_param_values = query
        .params
        .iter()
        .map(|field| format!("&{}", field.name))
        .collect::<Vec<String>>()
        .join(",");

    let ret_value = match &query.ret {
        RustReturnType::Void => String::from("Ok(())"),
        RustReturnType::Scalar(_) => match query.quantifier {
            Quantifier::ZeroOrMore => {
                format!("let return_value = res.iter().map(|row| {{let value : {ret_ty} = row.get(0); value}}).collect::<Vec<{ret_ty}>>(); Ok(return_value)")
            }
            Quantifier::ZeroOrOne => {
                format!("let return_value = res.map(|row| {{let value: {ret_ty} = row.get(0); value}}); Ok(return_value)")
            }
            Quantifier::One => {
                format!("let return_value: {ret_ty} = res.get(0); Ok(return_value)")
            }
        },
        RustReturnType::Tuple(tup) => {
            let mut rust_ret_values = Vec::new();
            let mut rust_ret_value_names = Vec::new();
            for (i, ty) in tup.iter().enumerate() {
                let rust_ret_value_name = format!("return_value_{}", i);
                rust_ret_values.push(format!(
                    "let {}: {} = res.get({});",
                    rust_ret_value_name, ty.rust_ty_usage_path, i
                ));
                rust_ret_value_names.push(rust_ret_value_name);
            }
            let tuple_value_string = format!(
                "{} ({})",
                rust_ret_values.join(" "),
                rust_ret_value_names.join(",")
            );
            match query.quantifier {
                Quantifier::ZeroOrMore => {
                    format!("let return_value = res.iter().map(|res| {{ {tuple_value_string} }}).collect::<Vec<{ret_ty}>>(); Ok(return_value)")
                }
                Quantifier::ZeroOrOne => {
                    format!("let return_value = res.map(|res| {{ {tuple_value_string} }}); Ok(return_value)")
                }
                Quantifier::One => {
                    format!("let return_value={{ {tuple_value_string} }}; Ok(return_value)")
                }
            }
        }
        RustReturnType::Struct(structure) => {
            let mut field_values = Vec::new();
            let mut rust_ret_values = Vec::new();
            for (i, field) in structure.iter().enumerate() {
                let rust_ty = &field.ty.rust_ty_usage_path;
                let rust_ret_value_name = format!("return_value_{}", i);
                rust_ret_values.push(format!(
                    "let {}: {} = res.get({});",
                    rust_ret_value_name, rust_ty, i
                ));
                field_values.push(format!("{} : {}", field.name, rust_ret_value_name));
            }

            let struct_value_string = format!(
                "{} {} {{ {} }}",
                rust_ret_values.join(" "),
                ret_ty,
                field_values.join(",")
            );

            match query.quantifier {
                Quantifier::ZeroOrMore => {
                    format!("let return_value = res.iter().map(|res| {{ {struct_value_string} }}).collect::<Vec<{ret_ty}>>(); Ok(return_value)")
                }
                Quantifier::ZeroOrOne => {
                    format!("let return_value = res.map(|res| {{ {struct_value_string} }}); Ok(return_value)")
                }
                Quantifier::One => {
                    format!("let return_value={{ {struct_value_string} }}; Ok(return_value)")
                }
            }
        }
    };

    Ok(format!(
        "let stmt = client.prepare_typed_cached({query_string}, &[{override_types}]).await?;
let res = client.{query_method}(&stmt, &[{query_param_values}]).await?;

{ret_value}"
    ))
}

pub fn generate(
    type_registrar: &TypeRegistrar,
    modules: Vec<PreparedModule>,
    destination: &str,
) -> Result<(), Error> {
    let imports = r#"use deadpool_postgres::{Client, Transaction};
use tokio_postgres::{types::Type, error::Error};"#;

    let type_modules = generate_type_modules(type_registrar);

    let mut query_modules = Vec::new();
    for module in modules {
        let mut query_strings = Vec::new();
        for query in module.queries {
            query_strings.push(generate_query(&query)?)
        }
        let queries_string = query_strings.join("\n\n");
        let module_name = module.name;

        query_modules.push(format!(
            r#"pub mod {module_name} {{
{imports}

{queries_string}
}}"#
        ));
    }

    let generated_modules = format!("{type_modules} {}", query_modules.join("\n\n"));

    std::fs::write(destination, generated_modules)?;

    Ok(())
}

pub mod error {
    use thiserror::Error as ThisError;

    use crate::pg_type::error::UnsupportedPostgresTypeError;

    #[derive(Debug, ThisError)]
    #[error("Encountered an error while generating Rust code")]
    pub enum Error {
        UnsupportedPostgresTypeError(#[from] UnsupportedPostgresTypeError),
        #[error("error while attempting to write generated modules")]
        Io(#[from] std::io::Error),
    }
}
