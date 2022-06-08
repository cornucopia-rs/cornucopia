use std::rc::Rc;

use crate::prepare_queries::PreparedField;
use crate::read_queries::ModuleInfo;
use crate::utils::has_duplicate;

use crate::parser::{Parsed, ParsedModule, Query, QueryDataStructure, TypeDataStructure};

#[derive(Debug)]
pub(crate) struct ValidatedModule {
    pub(crate) info: Rc<ModuleInfo>,
    pub(crate) param_types: Vec<TypeDataStructure>,
    pub(crate) row_types: Vec<TypeDataStructure>,
    pub(crate) _db_types: Vec<TypeDataStructure>,
    pub(crate) queries: Vec<ValidatedQuery>,
}

#[derive(Debug)]
pub(crate) struct ValidatedQuery {
    pub(crate) name: Parsed<String>,
    pub(crate) params: QueryDataStructure,
    pub(crate) bind_params: Vec<Parsed<String>>,
    pub(crate) row: QueryDataStructure,
    pub(crate) sql_str: String,
}

use error::{Error, ErrorVariant};
use postgres::Column;
use postgres_types::Type;

pub(crate) fn duplicate_nullable_ident(
    info: &Rc<ModuleInfo>,
    idents: &[Parsed<String>],
) -> Result<(), Error> {
    if let Some(dup) = has_duplicate(idents, |p| &p.value) {
        return Err(Error {
            err: ErrorVariant::DuplicateCol { pos: dup.start },
            info: info.clone(),
        });
    }
    Ok(())
}

pub(crate) fn query_name_already_used(
    info: &Rc<ModuleInfo>,
    queries: &[Query],
) -> Result<(), Error> {
    for (i, query) in queries.iter().enumerate() {
        if let Some((_, q)) = queries
            .iter()
            .enumerate()
            .find(|(j, q)| *j != i && q.annotation.name == query.annotation.name)
        {
            return Err(Error {
                err: ErrorVariant::QueryNameAlreadyUsed {
                    name1: query.annotation.name.clone(),
                    name2: q.annotation.name.clone(),
                },
                info: info.clone(),
            });
        }
    }

    has_duplicate(queries.iter(), |q| &q.annotation.name);

    Ok(())
}

pub(crate) fn nullable_column_name(
    info: &Rc<ModuleInfo>,
    nullable_col: &Parsed<String>,
    stmt_cols: &[Column],
) -> Result<(), Error> {
    // If none of the row's columns match the nullable column
    if stmt_cols
        .iter()
        .any(|row_col| row_col.name() == nullable_col.value)
    {
        Ok(())
    } else {
        Err(Error {
            err: ErrorVariant::InvalidNullableColumnName {
                nullable_col: nullable_col.clone(),
            },
            info: info.clone(),
        })
    }
}

pub(crate) fn nullable_param_name(
    info: &Rc<ModuleInfo>,
    nullable_col: &Parsed<String>,
    params: &[(Parsed<String>, Type)],
) -> Result<(), Error> {
    // If none of the row's columns match the nullable column
    if params
        .iter()
        .any(|(name, _)| name.value == nullable_col.value)
    {
        Ok(())
    } else {
        Err(Error {
            err: ErrorVariant::InvalidNullableColumnName {
                nullable_col: nullable_col.clone(),
            },
            info: info.clone(),
        })
    }
}

pub(crate) fn named_struct_field(
    info: &Rc<ModuleInfo>,
    name: &Parsed<String>,
    prev_fields: &[PreparedField],
    fields: &[PreparedField],
) -> Result<(), Error> {
    // If the row doesn't contain the same fields as a previously
    // registered row with the same name...
    if prev_fields.len() == fields.len() || prev_fields.iter().all(|f| fields.contains(f)) {
        Ok(())
    } else {
        Err(Error {
            err: ErrorVariant::NamedStructInvalidFields {
                expected: prev_fields.to_owned(),
                actual: fields.to_owned(),
                name: name.clone(),
            },
            info: info.clone(),
        })
    }
}

pub(crate) fn validate_query(info: &Rc<ModuleInfo>, query: Query) -> Result<ValidatedQuery, Error> {
    if let QueryDataStructure::Implicit { idents } = &query.annotation.param {
        duplicate_nullable_ident(info, idents)?;
    };
    if let QueryDataStructure::Implicit { idents } = &query.annotation.row {
        duplicate_nullable_ident(info, idents)?;
    };
    let mut bind_params = query.sql.bind_params.clone();
    bind_params.sort();
    bind_params.dedup();

    let sql_str = query.sql.normalize_sql(query.sql_start);
    let validated_query = ValidatedQuery {
        name: query.annotation.name,
        params: query.annotation.param,
        bind_params,
        row: query.annotation.row,
        sql_str,
    };

    Ok(validated_query)
}

pub(crate) fn validate_module(
    info: Rc<ModuleInfo>,
    module: ParsedModule,
) -> Result<ValidatedModule, Error> {
    query_name_already_used(&info, &module.queries)?;
    for ty in module
        .param_types
        .iter()
        .chain(module.row_types.iter())
        .chain(module.db_types.iter())
    {
        duplicate_nullable_ident(&info, &ty.fields)?;
    }
    let mut validated_queries = Vec::new();
    for query in module.queries {
        validated_queries.push(validate_query(&info, query)?);
    }
    Ok(ValidatedModule {
        info,
        param_types: module.param_types,
        row_types: module.row_types,
        _db_types: module.db_types,
        queries: validated_queries,
    })
}

pub mod error {
    use std::{fmt::Display, rc::Rc};

    use crate::{
        parser::Parsed, prepare_queries::PreparedField, read_queries::ModuleInfo,
        utils::compute_line,
    };

    #[derive(Debug)]
    pub enum ErrorVariant {
        DuplicateCol {
            pos: usize,
        },
        InvalidNullableColumnName {
            nullable_col: Parsed<String>,
        },
        NamedStructInvalidFields {
            expected: Vec<PreparedField>,
            actual: Vec<PreparedField>,
            name: Parsed<String>,
        },
        QueryNameAlreadyUsed {
            name1: Parsed<String>,
            name2: Parsed<String>,
        },
    }

    #[derive(Debug)]
    pub struct Error {
        pub(crate) err: ErrorVariant,
        pub(crate) info: Rc<ModuleInfo>,
    }

    impl Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let head = format!(
                "Error while validating queries [path: \"{}\"]:\n",
                self.info.path
            );
            match &self.err {
                ErrorVariant::DuplicateCol { pos } => {
                    let msg = ["Column name is already used."];
                    write!(f, "{head}{}", format_err(&self.info, *pos, &msg))
                }
                ErrorVariant::InvalidNullableColumnName { nullable_col } => {
                    let msg = format!(
                        "No column named `{}` found for this query.",
                        nullable_col.value
                    );
                    write!(
                        f,
                        "{head}{}",
                        format_err(&self.info, nullable_col.start, &[&msg])
                    )
                }
                // Move into another module
                ErrorVariant::NamedStructInvalidFields {
                    name,
                    expected,
                    actual,
                } => {
                    let msg1 = format!("This query's named row struct `{}` has already been used, but the fields don't match.", name.value);
                    let msg2 = format!("Expected fields: {expected:#?}");
                    let msg3 = format!("Got fields: {actual:#?}");
                    write!(
                        f,
                        "{head}{}",
                        format_err(&self.info, name.start, &[&msg1, &msg2, &msg3])
                    )
                }
                ErrorVariant::QueryNameAlreadyUsed { name1, name2 } => {
                    let msg1 = format!("A query named `{}` already exists.", name1.value);
                    let msg2 = format!("Query `{}` first defined here.", name2.value);
                    write!(
                        f,
                        "{head}{}\n{}",
                        format_err(&self.info, name1.start, &[&msg1]),
                        format_err(&self.info, name2.start, &[&msg2])
                    )
                }
            }
        }
    }
    impl std::error::Error for Error {}

    fn format_err(info: &ModuleInfo, pos: usize, messages: &[&str]) -> String {
        let msg = messages.join("\n  = ");
        let (col, line, line_str) = compute_line(&info.content, pos);
        let cursor = format!("{}^---", " ".repeat(col - 1));
        format!(" --> {line}:{col}\n  | \n  | {line_str}\n  | {cursor}\n  | \n  = {msg}")
    }
}
