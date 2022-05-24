use crate::{
    parser::{
        error::{ErrorPosition, ValidationError},
        NullableColumn, Parsed, ParsedQuery,
    },
    read_queries::Module,
    type_registrar::CornucopiaType,
    type_registrar::TypeRegistrar,
};
use error::Error;
use error::ErrorVariant;
use tokio_postgres::Client;

/// This data structure is used by Cornucopia to generate all constructs related to this particular query.
#[derive(Debug)]
pub(crate) struct PreparedQuery {
    pub(crate) name: String,
    pub(crate) params: Vec<PreparedParameter>,
    pub(crate) ret_fields: Vec<PreparedColumn>,
    pub(crate) sql: String,
}

#[derive(Debug)]
pub(crate) struct PreparedParameter {
    pub(crate) name: String,
    pub(crate) ty: CornucopiaType,
}

#[derive(Debug)]
pub(crate) struct PreparedColumn {
    pub(crate) name: String,
    pub(crate) ty: CornucopiaType,
    pub(crate) is_nullable: bool,
}

/// A struct containing the module name and the list of all
/// the queries it contains.
#[derive(Debug)]
pub(crate) struct PreparedModule {
    pub(crate) name: String,
    pub(crate) queries: Vec<PreparedQuery>,
}

fn has_duplicate<T, U>(
    iter: T,
    mapper: fn(<T as IntoIterator>::Item) -> U,
) -> Option<<T as IntoIterator>::Item>
where
    T: IntoIterator + Clone,
    U: Eq + std::hash::Hash + Clone,
{
    let mut uniq = std::collections::HashSet::new();
    iter.clone()
        .into_iter()
        .zip(iter.into_iter().map(mapper))
        .find(|(_, u)| !uniq.insert(u.clone()))
        .map(|(t, _)| t)
}

/// Prepares all modules
pub(crate) async fn prepare(
    client: &Client,
    type_registrar: &mut TypeRegistrar,
    modules: Vec<Module>,
) -> Result<Vec<PreparedModule>, Error> {
    let mut prepared_modules = Vec::new();
    for module in modules {
        prepared_modules.push(prepare_module(client, module, type_registrar).await?);
    }
    Ok(prepared_modules)
}

/// Prepares all queries in this module
async fn prepare_module(
    client: &Client,
    module: Module,
    type_registrar: &mut TypeRegistrar,
) -> Result<PreparedModule, Error> {
    let mut queries = Vec::new();
    for query in module.queries {
        queries.push(prepare_query(client, type_registrar, query, &module.path).await?);
    }
    Ok(PreparedModule {
        name: module.name,
        queries,
    })
}

/// Prepares a query
async fn prepare_query(
    client: &Client,
    type_registrar: &mut TypeRegistrar,
    query: ParsedQuery,
    module_path: &str,
) -> Result<PreparedQuery, Error> {
    // Prepare the statement
    let stmt = client
        .prepare(&query.sql_str)
        .await
        .map_err(|e| Error::new(e, &query, module_path))?;

    // Get parameter parameters
    let mut params = Vec::new();
    for (name, ty) in query.params.iter().zip(stmt.params().iter()) {
        // Register type
        let ty = type_registrar
            .register(client, ty)
            .await
            .map_err(|e| Error::new(e, &query, module_path))?;
        let name = name.value.to_owned();
        params.push(PreparedParameter {
            name,
            ty: ty.to_owned(),
        });
    }

    // Get return columns
    let stmt_cols = stmt.columns();
    // Check for duplicate names
    if let Some(duplicate_col) = has_duplicate(stmt_cols.iter(), |col| col.name()) {
        return Err(Error::new(
            ErrorVariant::ColumnNameAlreadyTaken {
                name: duplicate_col.name().to_owned(),
            },
            &query,
            module_path,
        ));
    };

    // Nullable columns
    let mut nullable_cols = Vec::new();
    for nullable_col in query.nullable_columns {
        match &nullable_col.value {
            crate::parser::NullableColumn::Index(index) => {
                // Get name from column index
                let name = match stmt_cols.get(*index as usize - 1) {
                    Some(col) => col.name(),
                    None => {
                        return Err(Error {
                            err: ErrorVariant::Validation(
                                ValidationError::InvalidNullableColumnIndex {
                                    index: *index as usize,
                                    max_col_index: stmt_cols.len(),
                                    pos: ErrorPosition {
                                        line: nullable_col.line,
                                        col: nullable_col.col,
                                        line_str: nullable_col.line_str,
                                    },
                                },
                            ),
                            query_name: query.name,
                            query_start_line: Some(query.line),
                            path: module_path.to_owned(),
                        })
                    }
                };

                // Check if `nullable_cols` already contains this column. If not, add it.
                if let Some((p, n)) = nullable_cols
                    .iter()
                    .find(|(_, n): &&(Parsed<NullableColumn>, String)| n == name)
                {
                    return Err(Error {
                        err: ErrorVariant::Validation(ValidationError::ColumnAlreadyNullable {
                            name: n.to_owned(),
                            pos: ErrorPosition {
                                line: p.line,
                                col: p.col,
                                line_str: p.line_str.to_owned(),
                            },
                        }),
                        query_name: query.name,
                        query_start_line: Some(query.line),
                        path: module_path.to_owned(),
                    });
                } else {
                    nullable_cols.push((nullable_col, name.to_owned()))
                }
            }
            crate::parser::NullableColumn::Named(name) => {
                // Check that the nullable column's name corresponds to one of the returned columns'.
                if stmt_cols.iter().any(|y| y.name() == name) {
                    nullable_cols.push((nullable_col.clone(), name.to_owned()))
                } else {
                    return Err(Error {
                        err: ErrorVariant::Validation(ValidationError::InvalidNullableColumnName {
                            name: name.to_owned(),
                            pos: ErrorPosition {
                                line: nullable_col.line,
                                col: nullable_col.col,
                                line_str: nullable_col.line_str,
                            },
                        }),
                        query_name: query.name.to_owned(),
                        query_start_line: Some(query.line),
                        path: module_path.to_owned(),
                    });
                };
            }
        }
    }

    // Now that we know all the nullable columns by name, check if there are duplicates.
    if let Some((p, u)) = has_duplicate(nullable_cols.iter(), |(_, n)| n) {
        return Err(Error {
            query_name: query.name,
            query_start_line: Some(query.line),
            err: ErrorVariant::Validation(ValidationError::ColumnAlreadyNullable {
                name: u.to_owned(),
                pos: ErrorPosition {
                    line: p.line,
                    col: p.col,
                    line_str: p.line_str.to_owned(),
                },
            }),
            path: module_path.to_owned(),
        });
    };

    // Get return columns
    let mut ret_fields = Vec::new();
    for column in stmt_cols {
        let ty = type_registrar
            .register(client, column.type_())
            .await
            .map_err(|e| Error {
                query_start_line: Some(query.line),
                err: e.into(),
                path: String::from(module_path),
                query_name: query.name.clone(),
            })?;
        let name = column.name().to_owned();
        let is_nullable = nullable_cols.iter().any(|(_, n)| *n == name);
        ret_fields.push(PreparedColumn {
            is_nullable,
            name,
            ty: ty.clone(),
        });
    }

    Ok(PreparedQuery {
        name: query.name,
        params,
        ret_fields,
        sql: query.sql_str,
    })
}

pub(crate) mod error {
    use std::fmt::Display;

    use crate::parser::{error::ValidationError, ParsedQuery};
    use crate::type_registrar::error::Error as PostgresTypeError;
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    #[error("{0}")]
    pub(crate) enum ErrorVariant {
        Db(#[from] tokio_postgres::Error),
        PostgresType(#[from] PostgresTypeError),
        Validation(#[from] ValidationError),
        #[error("Two or more columns have the same name: `{name}`. Consider disambiguing the column names with `AS` clauses.")]
        ColumnNameAlreadyTaken {
            name: String,
        },
    }

    #[derive(Debug)]
    pub struct Error {
        pub(crate) query_name: String,
        pub(crate) query_start_line: Option<usize>,
        pub(crate) err: ErrorVariant,
        pub(crate) path: String,
    }

    impl Error {
        pub(crate) fn new<E: Into<ErrorVariant>>(err: E, query: &ParsedQuery, path: &str) -> Self {
            Self {
                query_start_line: Some(query.line),
                err: err.into(),
                path: String::from(path),
                query_name: query.name.clone(),
            }
        }
    }

    impl Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &self.err {
                ErrorVariant::Db(e) => write!(
                    f,
                    "Error while preparing query \"{}\" [file: \"{}\", line: {}] ({})",
                    self.query_name,
                    self.path,
                    self.query_start_line.unwrap_or_default(),
                    e.as_db_error().unwrap().message()
                ),
                _ => match self.query_start_line {
                    Some(line) => {
                        write!(
                            f,
                            "Error while preparing query \"{}\" [file: \"{}\", line: {}]:\n{}",
                            self.query_name, self.path, line, self.err
                        )
                    }
                    None => {
                        write!(
                            f,
                            "Error while preparing query \"{}\" [file: \"{}\"]: {}",
                            self.query_name, self.path, self.err
                        )
                    }
                },
            }
        }
    }

    impl std::error::Error for Error {}
}
