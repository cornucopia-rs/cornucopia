use crate::prepare_queries::PreparedField;
use crate::read_queries::ModuleInfo;
use crate::utils::has_duplicate;

use crate::parser::{self, NullableIdent, Parsed, QueryDataStruct, TypeAnnotation};

#[derive(Debug)]
pub(crate) struct Module {
    pub(crate) info: ModuleInfo,
    pub(crate) types: Vec<TypeAnnotation>,
    pub(crate) queries: Vec<Query>,
}

#[derive(Debug)]
pub(crate) struct Query {
    pub(crate) name: Parsed<String>,
    pub(crate) params: QueryDataStruct,
    pub(crate) bind_params: Vec<Parsed<String>>,
    pub(crate) row: QueryDataStruct,
    pub(crate) sql_start: usize,
    pub(crate) sql_str: String,
}

use error::Error;
use miette::NamedSource;
use postgres::Column;
use postgres_types::Type;

pub(crate) fn duplicate_nullable_ident(
    info: &ModuleInfo,
    idents: &[NullableIdent],
) -> Result<(), Error> {
    for (i, ident1) in idents.iter().enumerate() {
        if let Some((_, ident2)) = idents
            .iter()
            .enumerate()
            .find(|(j, ident2)| *j != i && ident1.name == ident2.name)
        {
            return Err(Error::DuplicateNullableCol {
                src: NamedSource::new(info.path.as_str(), info.content.to_string()),
                dup_pos: (ident2.name.start..ident2.name.end).into(),
                pos: (ident1.name.start..ident1.name.end).into(),
            });
        }
    }

    Ok(())
}

pub(crate) fn duplicate_sql_col_name(
    info: &ModuleInfo,
    query_name: &Parsed<String>,
    cols: &[Column],
) -> Result<(), Error> {
    if has_duplicate(cols, |col| col.name()).is_some() {
        Err(Error::DuplicateSqlColName {
            src: info.to_owned().into(),
            pos: query_name.span().into(),
        })
    } else {
        Ok(())
    }
}

pub(crate) fn query_name_already_used(
    info: &ModuleInfo,
    queries: &[parser::Query],
) -> Result<(), Error> {
    for (i, query) in queries.iter().enumerate() {
        if let Some((_, q)) = queries
            .iter()
            .enumerate()
            .find(|(j, q)| *j != i && q.annotation.name == query.annotation.name)
        {
            return Err(Error::DuplicateQueryName {
                src: NamedSource::new(info.path.as_str(), info.content.to_string()),
                pos1: (query.annotation.name.start..query.annotation.name.end).into(),
                pos2: (q.annotation.name.start..q.annotation.name.end).into(),
            });
        }
    }

    has_duplicate(queries.iter(), |q| &q.annotation.name);

    Ok(())
}

pub(crate) fn nullable_column_name(
    info: &ModuleInfo,
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
        Err(Error::InvalidNullableColumnName {
            src: NamedSource::new(info.path.as_str(), info.content.to_string()),
            pos: (nullable_col.name.start..nullable_col.name.end).into(),
        })
    }
}

pub(crate) fn nullable_param_name(
    info: &ModuleInfo,
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
        Err(Error::InvalidNullableColumnName {
            src: NamedSource::new(info.path.as_str(), info.content.to_string()),
            pos: (nullable_col.name.start..nullable_col.name.end).into(),
        })
    }
}

pub(crate) fn named_struct_field(
    info: &ModuleInfo,
    name: &Parsed<String>,
    fields: &[PreparedField],
    prev_name: &Parsed<String>,
    prev_fields: &[PreparedField],
) -> Result<(), Error> {
    for field in fields {
        let mut found = false;
        for prev_field in prev_fields {
            if prev_field.name == field.name {
                if prev_field.ty != field.ty {
                    return Err(Error::IncompatibleNamedStructs {
                        src: NamedSource::new(info.path.as_str(), info.content.to_string()),
                        label2: format!(
                            "column `{}` is defined with type `{}` here",
                            field.name,
                            field.ty.pg_ty()
                        ),
                        pos1: (prev_name.start..prev_name.end).into(),
                        label1: format!("but here it has type `{}`", prev_field.ty.pg_ty()),
                        pos2: (name.start..name.end).into(),
                    });
                } else {
                    found = true;
                }
            }
        }
        if !found {
            return Err(Error::IncompatibleNamedStructs {
                src: NamedSource::new(info.path.as_str(), info.content.to_string()),
                label1: format!("column `{}` defined here", &field.name),
                pos2: (prev_name.start..prev_name.end).into(),
                label2: format!("column `{}` not found", &field.name),
                pos1: (name.start..name.end).into(),
            });
        }
    }

    for prev_field in prev_fields {
        let mut found = false;
        for field in fields {
            if prev_field.name == field.name {
                found = true;
            }
        }
        if !found {
            return Err(Error::IncompatibleNamedStructs {
                src: NamedSource::new(info.path.as_str(), info.content.to_string()),
                label1: format!("column `{}` defined here", &prev_field.name),
                pos1: (prev_name.start..prev_name.end).into(),
                label2: format!("column `{}` not found", &prev_field.name),
                pos2: (name.start..name.end).into(),
            });
        }
    }

    Ok(())
}

pub(crate) fn validate_module(info: ModuleInfo, module: parser::Module) -> Result<Module, Error> {
    query_name_already_used(&info, &module.queries)?;
    for ty in module.types.iter() {
        duplicate_nullable_ident(&info, &ty.fields)?;
    }
    let mut validated_queries = Vec::new();
    for query in module.queries {
        if let QueryDataStruct::Implicit { idents } = &query.annotation.param {
            duplicate_nullable_ident(&info, idents)?;
        };
        if let QueryDataStruct::Implicit { idents } = &query.annotation.row {
            duplicate_nullable_ident(&info, idents)?;
        };

        let validated_query = Query {
            name: query.annotation.name,
            params: query.annotation.param,
            bind_params: query.sql.bind_params,
            row: query.annotation.row,
            sql_str: query.sql.sql_str,
            sql_start: query.sql.start,
        };

        validated_queries.push(validated_query);
    }
    Ok(Module {
        info,
        types: module.types,
        queries: validated_queries,
    })
}

pub mod error {
    use std::fmt::Debug;

    use miette::{Diagnostic, NamedSource, SourceSpan};
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError, Diagnostic)]
    #[error("Couldn't validate queries.")]
    pub enum Error {
        #[diagnostic(
            code(cornucopia::validation::duplicate_sql_col_name),
            help("consider disambiguing the column names in your SQL using an `AS` clause.")
        )]
        DuplicateSqlColName {
            #[source_code]
            src: NamedSource,
            #[label("query returns one or more columns with the same name")]
            pos: SourceSpan,
        },
        #[diagnostic(code(cornucopia::validation::duplicate_nullable_col))]
        DuplicateNullableCol {
            #[source_code]
            src: NamedSource,
            #[label("this nullable column name is already in use")]
            dup_pos: SourceSpan,
            #[label("first used here")]
            pos: SourceSpan,
        },
        #[diagnostic(code(cornucopia::validation::duplicate_query_name))]
        DuplicateQueryName {
            #[source_code]
            src: NamedSource,
            #[label("this query name is already in use")]
            pos2: SourceSpan,
            #[label("first defined here")]
            pos1: SourceSpan,
        },
        #[diagnostic(code(cornucopia::validation::invalid_nullable_col))]
        InvalidNullableColumnName {
            #[source_code]
            src: NamedSource,
            #[label("no column with this name found")]
            pos: SourceSpan,
        },
        #[diagnostic(code(cornucopia::validation::incompatible_named_structs))]
        IncompatibleNamedStructs {
            #[source_code]
            src: NamedSource,
            label1: String,
            #[label("{label1}")]
            pos1: SourceSpan,
            label2: String,
            #[label("{label2}")]
            pos2: SourceSpan,
        },
    }
}
