use std::rc::Rc;

use crate::{
    parser::{error::ValidationError, Parsed, ParsedQuery},
    read_queries::Module,
    type_registrar::CornucopiaType,
    type_registrar::TypeRegistrar,
};
use error::Error;
use error::ErrorVariant;
use heck::ToUpperCamelCase;
use indexmap::{map::Entry, IndexMap};
use postgres::Client;
use postgres_types::Kind;

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
    pub(crate) ty: Rc<CornucopiaType>,
    pub(crate) is_nullable: bool,
    pub(crate) is_inner_nullable: bool, // Vec only
}

/// A params struct
#[derive(Debug, Clone)]
pub(crate) struct PreparedParams {
    pub(crate) name: String,
    pub(crate) fields: Vec<PreparedField>,
    pub(crate) queries: Vec<usize>,
    pub(crate) is_copy: bool,
}

/// A returned row
#[derive(Debug, Clone)]
pub(crate) struct PreparedRow {
    pub(crate) name: String,
    pub(crate) fields: Vec<PreparedField>,
    pub(crate) is_copy: bool,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) struct PreparedType {
    pub(crate) name: String,
    pub(crate) struct_name: String,
    pub(crate) content: PreparedContent,
    pub(crate) is_copy: bool,
    pub(crate) is_params: bool,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) enum PreparedContent {
    Enum(Vec<String>),
    Domain(PreparedField),
    Composite(Vec<PreparedField>),
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

#[derive(Debug, Clone)]
pub(crate) struct Preparation {
    pub(crate) modules: Vec<PreparedModule>,
    pub(crate) types: IndexMap<String, Vec<PreparedType>>,
}

impl PreparedModule {
    fn add_row(
        &mut self,
        registrar: &TypeRegistrar,
        name: Parsed<String>,
        fields: Vec<PreparedField>,
    ) -> Result<(usize, Vec<usize>), ErrorVariant> {
        assert!(!fields.is_empty());
        match self.rows.entry(name.value.clone()) {
            Entry::Occupied(o) => {
                let prev = &o.get().fields;

                // If the row doesn't contain the same fields as a previously
                // registered row with the same name...
                if prev.len() != fields.len() || !prev.iter().all(|f| fields.contains(f)) {
                    return Err(ErrorVariant::Validation(
                        ValidationError::NamedRowInvalidFields {
                            expected: prev.clone(),
                            actual: fields,
                            name: name.value,
                            pos: name.pos,
                        },
                    ));
                }

                let indexes: Option<Vec<_>> = prev
                    .iter()
                    .map(|f| fields.iter().position(|it| it == f))
                    .collect();
                Ok((o.index(), indexes.unwrap()))
            }
            Entry::Vacant(v) => {
                let is_copy = fields.iter().all(|f| f.ty.is_copy());
                let mut tmp = fields.to_vec();
                tmp.sort_unstable_by(|a, b| a.name.cmp(&b.name));
                v.insert(PreparedRow {
                    name: name.value.clone(),
                    fields: tmp,
                    is_copy,
                });
                self.add_row(registrar, name, fields)
            }
        }
    }

    fn add_query(
        &mut self,
        name: Parsed<String>,
        params: Vec<PreparedField>,
        row_idx: Option<(usize, Vec<usize>)>,
        sql: String,
    ) -> Result<usize, ErrorVariant> {
        match self.queries.entry(name.value.clone()) {
            Entry::Occupied(_o) => Err(ErrorVariant::Validation(
                ValidationError::QueryNameAlreadyUsed {
                    name: name.value,
                    pos: name.pos,
                },
            )),
            Entry::Vacant(v) => {
                let index = v.index();
                v.insert(PreparedQuery {
                    name: name.value,
                    params,
                    row: row_idx,
                    sql,
                });
                Ok(index)
            }
        }
    }

    fn add_params(
        &mut self,
        name: Parsed<String>,
        query_idx: usize,
    ) -> Result<usize, ErrorVariant> {
        let params = &self.queries.get_index(query_idx).unwrap().1.params;
        assert!(!params.is_empty());

        match self.params.entry(name.value.clone()) {
            Entry::Occupied(mut o) => {
                let prev = o.get_mut();
                // If the param struct doesn't contain the same fields as a previously
                // registered param struct with the same name...
                if prev.fields.len() != params.len()
                    || !prev.fields.iter().all(|f| params.contains(f))
                {
                    return Err(ErrorVariant::Validation(
                        ValidationError::NamedParamStructInvalidFields {
                            name: name.value,
                            pos: name.pos,
                            expected: prev.fields.clone(),
                            actual: params.clone(),
                        },
                    ));
                }
                prev.queries.push(query_idx);
                Ok(o.index())
            }
            Entry::Vacant(v) => {
                let mut fields = params.to_vec();
                fields.sort_unstable_by(|a, b| a.name.cmp(&b.name));
                let index = v.index();
                v.insert(PreparedParams {
                    name: name.value,
                    is_copy: fields.iter().all(|a| a.ty.is_copy()),
                    fields,
                    queries: vec![query_idx],
                });
                Ok(index)
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
pub(crate) fn prepare(client: &mut Client, modules: Vec<Module>) -> Result<Preparation, Error> {
    let mut registrar = TypeRegistrar::default();
    let mut tmp = Preparation {
        modules: Vec::new(),
        types: IndexMap::new(),
    };
    for module in modules {
        tmp.modules
            .push(prepare_module(client, module, &mut registrar)?);
    }
    // Sort module for consistent codegen
    tmp.modules.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    // Prepare types grouped by schema
    for ((schema, name), ty) in &registrar.types {
        if let Some(ty) = prepare_type(&registrar, name, ty) {
            match tmp.types.entry(schema.clone()) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().push(ty);
                }
                Entry::Vacant(entry) => {
                    entry.insert(vec![ty]);
                }
            }
        }
    }
    Ok(tmp)
}

/// Prepares database custom types
fn prepare_type(
    registrar: &TypeRegistrar,
    name: &str,
    ty: &CornucopiaType,
) -> Option<PreparedType> {
    if let CornucopiaType::Custom {
        pg_ty,
        struct_name,
        is_copy,
        is_params,
        ..
    } = ty
    {
        let content = match pg_ty.kind() {
            Kind::Enum(variants) => PreparedContent::Enum(variants.to_vec()),
            Kind::Domain(inner) => {
                PreparedContent::Domain(PreparedField {
                    name: "inner".to_string(),
                    ty: registrar.ref_of(inner),
                    is_nullable: false,
                    is_inner_nullable: false, // TODO used when support null everywhere
                })
            }
            Kind::Composite(fields) => PreparedContent::Composite(
                fields
                    .iter()
                    .map(|field| {
                        PreparedField {
                            name: field.name().to_string(),
                            ty: registrar.ref_of(field.type_()),
                            is_nullable: false, // TODO used when support null everywhere
                            is_inner_nullable: false, // TODO used when support null everywhere
                        }
                    })
                    .collect(),
            ),
            _ => unreachable!(),
        };
        Some(PreparedType {
            name: name.to_string(),
            struct_name: struct_name.clone(),
            content,
            is_copy: *is_copy,
            is_params: *is_params,
        })
    } else {
        None
    }
}

/// Prepares all queries in this module
fn prepare_module(
    client: &mut Client,
    module: Module,
    registrar: &mut TypeRegistrar,
) -> Result<PreparedModule, Error> {
    let mut tmp = PreparedModule {
        name: module.name,
        queries: IndexMap::new(),
        params: IndexMap::new(),
        rows: IndexMap::new(),
    };
    for query in module.queries {
        prepare_query(client, &mut tmp, registrar, query, &module.path)?;
    }
    Ok(tmp)
}

/// Prepares a query
fn prepare_query(
    client: &mut Client,
    module: &mut PreparedModule,
    registrar: &mut TypeRegistrar,
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
        params.push(PreparedField {
            name: name.value.to_owned(),
            ty: registrar
                .register(ty)
                .map_err(|e| Error::new(e, &query, module_path))?
                .clone(),
            is_nullable: false,       // TODO used when support null everywhere
            is_inner_nullable: false, // TODO used when support null everywhere
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
        let name = &nullable_col.value;

        // Check that the nullable column's name corresponds to one of the returned columns'.
        if stmt_cols.iter().any(|y| y.name() == name) {
            nullable_cols.push((nullable_col.clone(), name.to_owned()))
        } else {
            return Err(Error {
                err: ErrorVariant::Validation(ValidationError::InvalidNullableColumnName {
                    name: name.to_owned(),
                    pos: nullable_col.pos,
                }),
                query_name: query.name.value.clone(),
                query_start_line: Some(query.line),
                path: module_path.to_owned(),
            });
        };
    }

    // Check if there are duplicate nullable columns
    if let Some((p, u)) = has_duplicate(nullable_cols.iter(), |(_, n)| n) {
        return Err(Error {
            query_name: query.name.value,
            query_start_line: Some(query.line),
            err: ErrorVariant::Validation(ValidationError::ColumnAlreadyNullable {
                name: u.to_owned(),
                pos: p.pos.clone(),
            }),
            path: module_path.to_owned(),
        });
    };

    // Get return columns
    let mut row_fields = Vec::new();
    for column in stmt_cols {
        let name = column.name().to_owned();
        row_fields.push(PreparedField {
            is_nullable: nullable_cols.iter().any(|(_, n)| *n == name),
            is_inner_nullable: false, // TODO used when support null everywhere
            name,
            ty: registrar
                .register(column.type_())
                .map_err(|e| Error {
                    query_start_line: Some(query.line),
                    err: e.into(),
                    path: String::from(module_path),
                    query_name: query.name.value.clone(),
                })?
                .clone(),
        });
    }

    let row_struct_name = query
        .named_return_struct
        .unwrap_or_else(|| query.name.map(|x| x.to_upper_camel_case()));
    let param_struct_name = query
        .named_param_struct
        .unwrap_or_else(|| query.name.map(|x| x.to_upper_camel_case() + "Params"));

    let row_idx = if !row_fields.is_empty() {
        Some(
            module
                .add_row(registrar, row_struct_name, row_fields)
                .map_err(|e| Error {
                    err: e,
                    query_name: query.name.value.clone(),
                    query_start_line: Some(query.line),
                    path: module_path.to_owned(),
                })?,
        )
    } else {
        None
    };

    let params_not_empty = !params.is_empty();

    let query_idx = module
        .add_query(query.name.clone(), params, row_idx, query.sql_str)
        .map_err(|e| Error {
            err: e,
            query_name: query.name.value.clone(),
            query_start_line: Some(query.line),
            path: module_path.to_owned(),
        })?;
    if params_not_empty {
        module
            .add_params(param_struct_name, query_idx)
            .map_err(|e| Error {
                err: e,
                query_name: query.name.value.clone(),
                query_start_line: Some(query.line),
                path: module_path.to_owned(),
            })?;
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
                query_name: query.name.value.clone(),
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
