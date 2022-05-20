use self::error::WriteFileError;
use super::prepare_queries::PreparedModule;
use crate::{
    prepare_queries::PreparedQuery,
    type_registrar::{CornucopiaType, TypeRegistrar},
};
use error::Error;
use heck::ToUpperCamelCase;
use postgres_types::Kind;
use std::collections::HashMap;

fn is_reserved_keyword(s: &str) -> bool {
    [
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do",
        "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
        "union",
    ]
    .contains(&s)
}

fn generate_custom_type(
    type_registrar: &TypeRegistrar,
    ty: &CornucopiaType,
) -> Result<String, Error> {
    let type_def = match &ty.pg_ty.kind() {
        Kind::Enum(variants) => {
            let name = &ty.rust_ty_name;
            format!(
                "#[derive(Clone, Copy, PartialEq, Eq)]\npub enum {} {{ {} }}",
                name,
                variants.join(",")
            )
        }
        Kind::Domain(domain_inner_ty) => {
            let inner_ty = type_registrar.get(domain_inner_ty).unwrap();
            format!(
                "#[derive(Clone, PartialEq)]\npub struct {} ({})",
                ty.rust_ty_name, inner_ty.rust_path_from_types
            )
        }
        Kind::Composite(fields) => {
            let fields_str = fields
                .iter()
                .map(|f| {
                    let f_ty = type_registrar.get(f.type_()).unwrap();
                    format!("pub {} : {}", f.name(), f_ty.rust_path_from_types)
                })
                .collect::<Vec<String>>()
                .join(",");

            format!(
                "#[derive(Clone, PartialEq)]\npub struct {} {{ {} }}",
                ty.rust_ty_name, fields_str
            )
        }
        _ => unreachable!(),
    };

    Ok(format!(
        "#[derive(Debug, postgres_types::ToSql, postgres_types::FromSql)]\n#[postgres(name = \"{}\")]\n{}",
        ty.pg_ty.name(),
        type_def
    ))
}

fn generate_type_modules(type_registrar: &TypeRegistrar) -> Result<String, Error> {
    // Group the custom types by schema name
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
    // Generate each module
    let modules_str = modules
        .iter()
        .map(|(mod_name, tys)| {
            let tys_str = tys
                .iter()
                .map(|ty| generate_custom_type(type_registrar, ty))
                .collect::<Result<Vec<String>, Error>>()?
                .join("\n\n");
            Ok(format!("pub mod {mod_name} {{ {tys_str} }}"))
        })
        .collect::<Result<Vec<String>, Error>>()?
        .join("\n\n");

    // Return to overarching `types` module
    Ok(format!("pub mod types {{ {modules_str} }}"))
}

fn generate_query(module_name: &str, query: &PreparedQuery) -> String {
    let query_struct_name = query.name.to_upper_camel_case();

    let params_struct = if query.params.is_empty() {
        String::new()
    } else {
        let params_struct_fields = query
            .params
            .iter()
            .map(|p| format!("pub {} : &'a {}", p.name, p.ty.borrowed_rust_ty()))
            .collect::<Vec<String>>()
            .join(",");
        format!("pub struct {query_struct_name}Params<'a> {{ {params_struct_fields} }}")
    };

    let borrowed_ret_struct = if query.ret_fields.is_empty() {
        String::new()
    } else {
        let ret_struct_fields = query
            .ret_fields
            .iter()
            .map(|p| format!("pub {} : &'a {}", p.name, p.ty.borrowed_rust_ty()))
            .collect::<Vec<String>>()
            .join(",");
        format!("pub struct {query_struct_name}Borrowed<'a> {{ {ret_struct_fields} }}")
    };
    let ret_struct = if query.ret_fields.is_empty() {
        String::new()
    } else {
        let ret_struct_fields = query
            .ret_fields
            .iter()
            .map(|p| format!("pub {} : {}", p.name, p.ty.rust_path_from_queries))
            .collect::<Vec<String>>()
            .join(",");
        format!("pub struct {query_struct_name} {{ {ret_struct_fields} }}")
    };

    let from_impl = if query.ret_fields.is_empty() {
        String::new()
    } else {
        let fields_names = query
            .ret_fields
            .iter()
            .map(|f| f.name.clone())
            .collect::<Vec<String>>()
            .join(",");
        let borrowed_fields_to_owned = query
            .ret_fields
            .iter()
            .map(|f| format!("{}: {}.to_owned()", f.name, f.name))
            .collect::<Vec<String>>()
            .join(",");
        format!(
            "impl<'a> From<{query_struct_name}Borrowed<'a>> for {query_struct_name} {{
    fn from({query_struct_name}Borrowed {{ {fields_names} }}: {query_struct_name}Borrowed<'a>) -> Self {{
        Self {{ {borrowed_fields_to_owned} }}
    }}
}}"
        )
    };

    format!("{params_struct}\n{borrowed_ret_struct}\n{ret_struct}\n{from_impl}")
}

pub(crate) fn generate(
    type_registrar: &TypeRegistrar,
    modules: Vec<PreparedModule>,
    destination: &str,
) -> Result<(), Error> {
    let type_modules = generate_type_modules(type_registrar)?;

    let mut query_modules = Vec::new();
    for module in modules {
        let mut query_strings = Vec::new();
        for query in module.queries {
            let query_string = generate_query(&module.name, &query);
            query_strings.push(query_string);
        }
        let queries_string = query_strings.join("\n\n");
        let module_name = module.name;

        query_modules.push(format!("pub mod {module_name} {{ {queries_string} }}"));
    }
    let query_modules_string = format!("pub mod queries {{ {} }}", query_modules.join("\n\n"));
    let top_level_comment = "// This file was generated with `cornucopia`. Do not modify.";

    let generated_modules =
        format!("{top_level_comment}\n\n{type_modules}\n\n{query_modules_string}");

    std::fs::write(destination, generated_modules).map_err(|err| {
        Error::Io(WriteFileError {
            err,
            path: String::from(destination),
        })
    })?;

    Ok(())
}

pub(crate) mod error {
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    #[error("{0}")]
    pub(crate) enum Error {
        Io(#[from] WriteFileError),
    }

    #[derive(Debug, ThisError)]
    #[error("Error while trying to write to destination file \"{path}\": {err}.")]
    pub(crate) struct WriteFileError {
        pub(crate) err: std::io::Error,
        pub(crate) path: String,
    }
}
