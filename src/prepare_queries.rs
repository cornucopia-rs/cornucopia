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
use heck::ToUpperCamelCase;
use indexmap::{map::Entry, IndexMap};
use postgres::Client;

/// This data structure is used by Cornucopia to generate all constructs related to this particular query.
#[derive(Debug, Clone)]
pub(crate) struct PreparedQuery {
    pub(crate) name: String,
    pub(crate) params: Vec<PreparedField>,
    pub(crate) row: Option<(usize, Vec<usize>)>, // None if execute
    pub(crate) sql: String,
}

/// A row or params field
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreparedField {
    pub(crate) name: String,
    pub(crate) ty: CornucopiaType,
    pub(crate) is_nullable: bool,
}

/// A params struct
#[derive(Debug, Clone)]
pub(crate) struct PreparedParams {
    pub(crate) name: String,
    pub(crate) fields: Vec<PreparedField>,
    pub(crate) queries: Vec<usize>,
}

/// A returned row
#[derive(Debug, Clone)]
pub(crate) struct PreparedRow {
    pub(crate) name: String,
    pub(crate) fields: Vec<PreparedField>,
    pub(crate) is_copy: bool,
}

/// A struct containing the module name and the list of all
/// the queries it contains.
#[derive(Debug, Clone)]
pub(crate) struct PreparedModule {
    pub(crate) name: String,
    pub(crate) queries: IndexMap<String, PreparedQuery>,
    pub(crate) params: IndexMap<String, PreparedParams>,
    pub(crate) rows: IndexMap<String, PreparedRow>,
}

impl PreparedModule {
    fn add_row(&mut self, name: String, fields: &[PreparedField]) -> (usize, Vec<usize>) {
        assert!(!fields.is_empty());
        match self.rows.entry(name.clone()) {
            Entry::Occupied(o) => {
                // TODO return an error
                let prev = &o.get().fields;
                assert!(prev.len() == fields.len());
                let indexes: Option<Vec<_>> = prev
                    .iter()
                    .map(|f| fields.iter().position(|it| it.name == f.name))
                    .collect();
                (o.index(), indexes.unwrap())
            }
            Entry::Vacant(v) => {
                let is_copy = fields.iter().all(|f| f.ty.is_copy);
                let mut tmp = fields.to_vec();
                tmp.sort_unstable_by(|a, b| a.name.cmp(&b.name));
                v.insert(PreparedRow {
                    name: name.clone(),
                    fields: tmp,
                    is_copy,
                });
                self.add_row(name, fields)
            }
        }
    }

    fn add_query(
        &mut self,
        name: String,
        params: Vec<PreparedField>,
        row_idx: Option<(usize, Vec<usize>)>,
        sql: String,
    ) -> usize {
        match self.queries.entry(name.clone()) {
            Entry::Occupied(_o) => {
                // TODO return an error
                unreachable!()
            }
            Entry::Vacant(v) => {
                let index = v.index();
                v.insert(PreparedQuery {
                    name,
                    params,
                    row: row_idx,
                    sql,
                });
                index
            }
        }
    }

    fn add_params(&mut self, name: String, query_idx: usize) -> usize {
        let params = &self.queries.get_index(query_idx).unwrap().1.params;
        assert!(!params.is_empty());

        match self.params.entry(name.clone()) {
            Entry::Occupied(mut o) => {
                let prev = o.get_mut();
                // TODO return an error
                assert!(prev.fields.len() == params.len());
                assert!(prev
                    .fields
                    .iter()
                    .all(|f| params.iter().position(|it| it.name == f.name).is_some()));
                prev.queries.push(query_idx);
                o.index()
            }
            Entry::Vacant(v) => {
                let mut fields = params.to_vec();
                fields.sort_unstable_by(|a, b| a.name.cmp(&b.name));
                let index = v.index();
                v.insert(PreparedParams {
                    name,
                    fields,
                    queries: vec![query_idx],
                });
                index
            }
        }
    }
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
pub(crate) fn prepare(
    client: &mut Client,
    type_registrar: &mut TypeRegistrar,
    modules: Vec<Module>,
) -> Result<Vec<PreparedModule>, Error> {
    let mut prepared_modules = Vec::new();
    for module in modules {
        prepared_modules.push(prepare_module(client, module, type_registrar)?);
    }
    Ok(prepared_modules)
}

/// Prepares all queries in this module
fn prepare_module(
    client: &mut Client,
    module: Module,
    type_registrar: &mut TypeRegistrar,
) -> Result<PreparedModule, Error> {
    let mut tmp = PreparedModule {
        name: module.name,
        queries: IndexMap::new(),
        params: IndexMap::new(),
        rows: IndexMap::new(),
    };
    for query in module.queries {
        prepare_query(client, &mut tmp, type_registrar, query, &module.path)?;
    }
    Ok(tmp)
}

/// Prepares a query
fn prepare_query(
    client: &mut Client,
    module: &mut PreparedModule,
    type_registrar: &mut TypeRegistrar,
    query: ParsedQuery,
    module_path: &str,
) -> Result<(), Error> {
    // Prepare the statement
    let stmt = client
        .prepare(&query.sql_str)
        .map_err(|e| Error::new(e, &query, module_path))?;

    // Get parameter parameters
    let mut params = Vec::new();
    for (name, ty) in query.params.iter().zip(stmt.params().iter()) {
        // Register type
        let ty = type_registrar
            .register(client, ty)
            .map_err(|e| Error::new(e, &query, module_path))?;
        let name = name.value.to_owned();
        params.push(PreparedField {
            name,
            ty: ty.to_owned(),
            is_nullable: false, // TODO used when support null everywhere
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
                let name = if let Some(col) = stmt_cols.get(*index as usize - 1) {
                    col.name()
                } else {
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
                    });
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
    let mut row_fields = Vec::new();
    for column in stmt_cols {
        let ty = type_registrar
            .register(client, column.type_())
            .map_err(|e| Error {
                query_start_line: Some(query.line),
                err: e.into(),
                path: String::from(module_path),
                query_name: query.name.clone(),
            })?;
        let name = column.name().to_owned();
        let is_nullable = nullable_cols.iter().any(|(_, n)| *n == name);
        row_fields.push(PreparedField {
            is_nullable,
            name,
            ty: ty.clone(),
        });
    }

    let nb_params = params.len();

    let name = query.name.to_upper_camel_case();
    let row_idx = (!row_fields.is_empty()).then(|| module.add_row(name.clone(), &row_fields));
    let query_idx = module.add_query(query.name.clone(), params, row_idx, query.sql_str);
    if nb_params > 0 {
        module.add_params(format!("{name}Params"), query_idx);
    }

    Ok(())
}

pub(crate) mod error {
    use std::fmt::Display;

    use crate::parser::{error::ValidationError, ParsedQuery};
    use crate::type_registrar::error::Error as PostgresTypeError;
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    #[error("{0}")]
    pub(crate) enum ErrorVariant {
        Db(#[from] postgres::Error),
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
