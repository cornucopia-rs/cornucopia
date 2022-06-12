use std::rc::Rc;

use crate::prepare_queries::PreparedField;
use crate::read_queries::ModuleInfo;
use crate::utils::has_duplicate;

use crate::parser::{NullableIdent, Parsed, ParsedModule, Query, QueryDataStruct, TypeAnnotation};

// TODO check params and rows name are unique across both declared and generated

#[derive(Debug)]
pub(crate) struct ValidatedModule {
    pub(crate) info: Rc<ModuleInfo>,
    pub(crate) types: Vec<TypeAnnotation>,
    pub(crate) queries: Vec<ValidatedQuery>,
}

#[derive(Debug)]
pub(crate) struct ValidatedQuery {
    pub(crate) name: Parsed<String>,
    pub(crate) params: QueryDataStruct,
    pub(crate) bind_params: Vec<Parsed<String>>,
    pub(crate) row: QueryDataStruct,
    pub(crate) sql_str: String,
}

use error::{Error, ErrorVariant};
use postgres::Column;
use postgres_types::Type;

pub(crate) fn duplicate_nullable_ident(
    info: &Rc<ModuleInfo>,
    idents: &[NullableIdent],
) -> Result<(), Error> {
    if let Some(dup) = has_duplicate(idents, |p| &p.name.value) {
        return Err(Error {
            err: ErrorVariant::DuplicateCol {
                pos: dup.name.start,
            },
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
                err: ErrorVariant::DuplicateQueryName {
                    name1: q.annotation.name.clone(),
                    name2: query.annotation.name.clone(),
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
    nullable_col: &NullableIdent,
    stmt_cols: &[Column],
) -> Result<(), Error> {
    // If none of the row's columns match the nullable column
    if stmt_cols
        .iter()
        .any(|row_col| row_col.name() == nullable_col.name.value)
    {
        Ok(())
    } else {
        Err(Error {
            err: ErrorVariant::InvalidNullableColumnName {
                nullable_col: nullable_col.name.clone(),
            },
            info: info.clone(),
        })
    }
}

pub(crate) fn nullable_param_name(
    info: &Rc<ModuleInfo>,
    nullable_col: &NullableIdent,
    params: &[(Parsed<String>, Type)],
) -> Result<(), Error> {
    // If none of the row's columns match the nullable column
    if params
        .iter()
        .any(|(name, _)| name.value == nullable_col.name.value)
    {
        Ok(())
    } else {
        Err(Error {
            err: ErrorVariant::InvalidNullableColumnName {
                nullable_col: nullable_col.name.clone(),
            },
            info: info.clone(),
        })
    }
}

pub(crate) fn named_struct_field(
    info: &Rc<ModuleInfo>,
    name: &Parsed<String>,
    fields: &[PreparedField],
    prev_name: &Parsed<String>,
    prev_fields: &[PreparedField],
) -> Result<(), Error> {
    if prev_fields.len() == fields.len() && prev_fields.iter().all(|f| fields.contains(f)) {
        Ok(())
    } else {
        Err(Error {
            err: ErrorVariant::NamedStructInvalidFields {
                expected_name: name.to_owned(),
                expected_fields: fields.to_owned(),
                actual_name: prev_name.to_owned(),
                actual_fields: prev_fields.to_owned(),
            },
            info: info.clone(),
        })
    }
}

pub(crate) fn validate_query(info: &Rc<ModuleInfo>, query: Query) -> Result<ValidatedQuery, Error> {
    if let QueryDataStruct::Implicit { idents } = &query.annotation.param {
        duplicate_nullable_ident(info, idents)?;
    };
    if let QueryDataStruct::Implicit { idents } = &query.annotation.row {
        duplicate_nullable_ident(info, idents)?;
    };
    let mut bind_params = query.sql.bind_params.clone();
    bind_params.sort();
    bind_params.dedup();

    let validated_query = ValidatedQuery {
        name: query.annotation.name,
        params: query.annotation.param,
        bind_params,
        row: query.annotation.row,
        sql_str: query.sql.sql_str,
    };

    Ok(validated_query)
}

pub(crate) fn validate_module(
    info: Rc<ModuleInfo>,
    module: ParsedModule,
) -> Result<ValidatedModule, Error> {
    query_name_already_used(&info, &module.queries)?;
    for ty in module.types.iter() {
        duplicate_nullable_ident(&info, &ty.fields)?;
    }
    let mut validated_queries = Vec::new();
    for query in module.queries {
        validated_queries.push(validate_query(&info, query)?);
    }
    Ok(ValidatedModule {
        info,
        types: module.types,
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
            expected_name: Parsed<String>,
            expected_fields: Vec<PreparedField>,
            actual_name: Parsed<String>,
            actual_fields: Vec<PreparedField>,
        },
        DuplicateQueryName {
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
                ErrorVariant::NamedStructInvalidFields {
                    expected_name,
                    expected_fields,
                    actual_name,
                    actual_fields,
                } => {
                    let expected_fields = expected_fields
                        .iter()
                        .map(|f| format!("  - {}: {}", f.name, f.ty.pg_ty()))
                        .collect::<Vec<String>>()
                        .join("\n");
                    let got_fields = actual_fields
                        .iter()
                        .map(|f| format!("  - {}: {}", f.name, f.ty.pg_ty()))
                        .collect::<Vec<String>>()
                        .join("\n");
                    let msg = "This named data structure has been defined elsewhere, but the fields don't match.\n";
                    let err1 = format_err(&self.info, actual_name.start, &[msg]);
                    let err2 = format_err(&self.info, expected_name.start, &["First defined here"]);
                    write!(
                        f,
                        "{head}{err1}{err2}\n\nExpected:\n{expected_fields}\nGot:\n{got_fields}",
                    )
                }
                ErrorVariant::DuplicateQueryName { name1, name2 } => {
                    let msg1 = "This query's name is already used";
                    let msg2 = "First defined here.";
                    write!(
                        f,
                        "{head}{}\n{}",
                        format_err(&self.info, name1.start, &[msg1]),
                        format_err(&self.info, name2.start, &[msg2]),
                    )
                }
            }
        }
    }
    impl std::error::Error for Error {}

    fn format_err(info: &ModuleInfo, pos: usize, messages: &[&str]) -> String {
        let msg = messages
            .iter()
            .map(|m| format!("\n  = {}", m))
            .collect::<Vec<String>>()
            .join("");
        let (col, line, line_str) = compute_line(&info.content, pos);
        let cursor = format!("{}^---", " ".repeat(col - 1));
        format!(" --> {line}:{col}\n  | \n  | {line_str}\n  | {cursor}\n  | {msg}")
    }
}
