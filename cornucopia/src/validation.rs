use std::collections::BTreeMap;

use crate::{
    parser::{self, NullableIdent, QueryDataStruct, Span, TypeAnnotation},
    prepare_queries::{PreparedField, PreparedModule},
    read_queries::ModuleInfo,
    utils::has_duplicate,
};

#[derive(Debug)]
pub(crate) struct Module {
    pub(crate) info: ModuleInfo,
    pub(crate) types: Vec<TypeAnnotation>,
    pub(crate) queries: Vec<Query>,
}

#[derive(Debug)]
pub(crate) struct Query {
    pub(crate) name: Span<String>,
    pub(crate) params: QueryDataStruct,
    pub(crate) bind_params: Vec<Span<String>>,
    pub(crate) row: QueryDataStruct,
    pub(crate) sql_span: SourceSpan,
    pub(crate) sql_str: String,
}

use error::Error;
use heck::ToUpperCamelCase;
use miette::SourceSpan;
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
            return Err(Error::DuplicateFieldNullity {
                src: info.into(),
                name: ident1.name.value.clone(),
                first: ident1.name.span,
                second: ident2.name.span,
            });
        }
    }

    Ok(())
}

pub(crate) fn duplicate_sql_col_name(
    info: &ModuleInfo,
    query_name: &Span<String>,
    cols: &[Column],
) -> Result<(), Error> {
    if let Some(col) = has_duplicate(cols, Column::name) {
        Err(Error::DuplicateSqlColName {
            src: info.clone().into(),
            name: col.name().to_string(),
            pos: query_name.span,
        })
    } else {
        Ok(())
    }
}

pub(crate) fn query_name_already_used(
    info: &ModuleInfo,
    queries: &[parser::Query],
) -> Result<(), Error> {
    for (i, first) in queries.iter().enumerate() {
        if let Some(second) = queries[i + 1..]
            .iter()
            .find(|second| second.annotation.name == first.annotation.name)
        {
            return Err(Error::DuplicateName {
                src: info.into(),
                ty: "query",
                name: first.annotation.name.value.clone(),
                first: first.annotation.name.span,
                second: second.annotation.name.span,
            });
        }
    }

    Ok(())
}

pub(crate) fn named_type_already_used(
    info: &ModuleInfo,
    types: &[TypeAnnotation],
) -> Result<(), Error> {
    for (i, first) in types.iter().enumerate() {
        if let Some(second) = types[i + 1..]
            .iter()
            .find(|second| second.name == first.name)
        {
            return Err(Error::DuplicateName {
                src: info.into(),
                ty: "type",
                name: first.name.value.clone(),
                first: first.name.span,
                second: second.name.span,
            });
        }
    }

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
        Err(Error::UnknownFieldName {
            src: info.into(),
            pos: nullable_col.name.span,
            known: stmt_cols
                .iter()
                .map(|it| it.name().to_string())
                .collect::<Vec<_>>()
                .join(", "),
        })
    }
}

pub(crate) fn nullable_param_name(
    info: &ModuleInfo,
    nullable_col: &NullableIdent,
    params: &[(Span<String>, Type)],
) -> Result<(), Error> {
    // If none of the row's columns match the nullable column
    if params
        .iter()
        .any(|(name, _)| name.value == nullable_col.name.value)
    {
        Ok(())
    } else {
        Err(Error::UnknownFieldName {
            src: info.into(),
            pos: nullable_col.name.span,
            known: params
                .iter()
                .map(|it| it.0.value.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        })
    }
}

pub(crate) fn row_on_execute(
    info: &ModuleInfo,
    name: &Span<String>,
    query: &SourceSpan,
    row: &QueryDataStruct,
    columns: &[Column],
) -> Result<(), Error> {
    let row = match row {
        QueryDataStruct::Implicit { idents } => match (
            idents.first().map(|it| it.name.span),
            idents.last().map(|it| it.name.span),
        ) {
            (Some(first), Some(last)) => Some((first.offset()..last.offset() + last.len()).into()),
            _ => None,
        },
        QueryDataStruct::Named(name) => Some(name.span),
    };
    if let Some(row) = row {
        if columns.is_empty() {
            return Err(Error::RowOnExecute {
                src: info.into(),
                name: name.value.clone(),
                row,
                query: *query,
            });
        }
    }

    Ok(())
}

pub(crate) fn named_struct_field(
    info: &ModuleInfo,
    name: &Span<String>,
    fields: &[PreparedField],
    prev_name: &Span<String>,
    prev_fields: &[PreparedField],
) -> Result<(), Error> {
    if let Some((field, prev_field)) = fields.iter().find_map(|f| {
        prev_fields
            .iter()
            .find_map(|prev_f| (f.name == prev_f.name && f.ty != prev_f.ty).then(|| (f, prev_f)))
    }) {
        return Err(Error::IncompatibleNamedType {
            src: info.into(),
            name: name.value.clone(),
            first_label: format!(
                "column `{}` is expected to have type `{}` here",
                field.name,
                field.ty.pg_ty()
            ),
            second: prev_name.span,
            second_label: format!("but here it has type `{}`", prev_field.ty.pg_ty()),
            first: name.span,
        });
    }

    if let Some(field) = fields.iter().find(|f| !prev_fields.contains(f)) {
        return Err(Error::IncompatibleNamedType {
            src: info.into(),
            name: name.value.clone(),
            second_label: format!("column `{}` expected here", &field.name),
            second: name.span,
            first_label: format!("column `{}` not found", &field.name),
            first: prev_name.span,
        });
    }

    if let Some(prev_field) = prev_fields.iter().find(|f| !fields.contains(f)) {
        return Err(Error::IncompatibleNamedType {
            src: info.into(),
            name: name.value.clone(),
            second_label: format!("column `{}` expected here", &prev_field.name),
            second: prev_name.span,
            first_label: format!("column `{}` not found", &prev_field.name),
            first: name.span,
        });
    }

    Ok(())
}

pub(crate) fn validate_preparation(module: &PreparedModule) -> Result<(), Error> {
    // Check generated name clash
    let mut named_registrar = BTreeMap::new();

    let mut check_name = |name: String, span: SourceSpan, ty: &'static str| {
        if let Some(prev) = named_registrar.insert(name.clone(), (span, ty)) {
            // Sort by span
            let (first, second) = if prev.0.offset() < span.offset() {
                (prev, (span, ty))
            } else {
                ((span, ty), prev)
            };
            Err(Error::DuplicateGenName {
                src: (&module.info).into(),
                name,
                first: first.0,
                first_ty: first.1,
                second: second.0,
                second_ty: second.1,
            })
        } else {
            Ok(())
        }
    };

    for (origin, query) in &module.queries {
        check_name(
            format!("{}Stmt", query.name.to_upper_camel_case()),
            origin.span,
            "statement",
        )?;
    }
    for (origin, row) in &module.rows {
        if row.fields.len() > 1 || !row.is_implicit {
            check_name(row.name.value.clone(), origin.span, "row")?;

            if !row.is_copy {
                check_name(format!("{}Borrowed", row.name), origin.span, "borrowed row")?;
            };
        }
        check_name(format!("{}Query", row.name), origin.span, "query")?;
    }
    for (origin, params) in &module.params {
        if params.fields.len() > 1 || !params.is_implicit {
            check_name(params.name.value.clone(), origin.span, "params")?;
        }
    }
    Ok(())
}

pub(crate) fn validate_module(info: ModuleInfo, module: parser::Module) -> Result<Module, Error> {
    query_name_already_used(&info, &module.queries)?;
    named_type_already_used(&info, &module.types)?;
    for ty in &module.types {
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
            sql_span: query.sql.span,
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
    pub enum Error {
        #[error("column `{name}` appear multiple time")]
        #[diagnostic(help("disambiguate column names in your SQL using an `AS` clause"))]
        DuplicateSqlColName {
            #[source_code]
            src: NamedSource,
            name: String,
            #[label("query returns one or more columns with the same name")]
            pos: SourceSpan,
        },
        #[error("the field `{name}` is declared null multiple time")]
        #[diagnostic(help("remove one of the two declaration"))]
        DuplicateFieldNullity {
            #[source_code]
            src: NamedSource,
            name: String,
            #[label("previous nullity declaration")]
            first: SourceSpan,
            #[label("redeclared here")]
            second: SourceSpan,
        },
        #[error("the {ty} `{name}` is defined multiple time")]
        #[diagnostic(help("use a different name for one of those"))]
        DuplicateName {
            #[source_code]
            src: NamedSource,
            name: String,
            ty: &'static str,
            #[label("previous definition of the {ty} here")]
            first: SourceSpan,
            #[label("redefined here")]
            second: SourceSpan,
        },
        #[error("unknown field")]
        #[diagnostic(help("use one of those names: {known}"))]
        UnknownFieldName {
            #[source_code]
            src: NamedSource,
            #[label("no field with this name was found")]
            pos: SourceSpan,
            known: String,
        },
        #[error("named type `{name}` as conflicting usage")]
        #[diagnostic(help("use a different named type for each query"))]
        IncompatibleNamedType {
            #[source_code]
            src: NamedSource,
            name: String,
            first_label: String,
            #[label("{first_label}")]
            first: SourceSpan,
            second_label: String,
            #[label("{second_label}")]
            second: SourceSpan,
        },
        #[error("the query `{name}` declare a row but return nothing")]
        #[diagnostic(help("remove row declaration"))]
        RowOnExecute {
            #[source_code]
            src: NamedSource,
            name: String,
            #[label("row declared here")]
            row: SourceSpan,
            #[label("but query return nothing")]
            query: SourceSpan,
        },
        #[error("`{name}` is used multiple time")]
        #[diagnostic(help("use a different name for one of those"))]
        DuplicateGenName {
            #[source_code]
            src: NamedSource,
            name: String,
            first_ty: &'static str,
            #[label("previous definition as {first_ty} here")]
            first: SourceSpan,
            second_ty: &'static str,
            #[label("redefined as {second_ty} here")]
            second: SourceSpan,
        },
    }
}
