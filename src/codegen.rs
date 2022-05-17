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
                "#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum {} {{ {} }}",
                name,
                variants.join(",")
            )
        }
        Kind::Domain(domain_inner_ty) => {
            let inner_ty = type_registrar.get(domain_inner_ty).unwrap();
            format!(
                "#[derive(Debug, Clone, PartialEq)]\npub struct {} ({})",
                ty.rust_ty_name, inner_ty.rust_path_from_types
            )
        }
        Kind::Composite(fields) => {
            let fields_str = fields
                .iter()
                .map(|f| {
                    let f_ty = type_registrar.get(f.type_()).unwrap();
                    format!("pub {} : {}", f_ty.rust_ty_name, f_ty.rust_path_from_types)
                })
                .collect::<Vec<String>>()
                .join(",");

            format!(
                "#[derive(Debug, Clone, PartialEq)]\npub struct {} {{ {} }}",
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
    todo!()
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
