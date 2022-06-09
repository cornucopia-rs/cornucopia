use crate::{
    parser::{Parsed, TypeDataStructure},
    read_queries::ModuleInfo,
    type_registrar::CornucopiaType,
    type_registrar::TypeRegistrar,
    utils::has_duplicate,
    validation::{self, ValidatedModule, ValidatedQuery},
};
use error::Error;
use error::ErrorVariant;

use indexmap::{map::Entry, IndexMap};
use postgres::Client;
use postgres_types::{Kind, Type};
use std::rc::Rc;

/// This data structure is used by Cornucopia to generate
/// all constructs related to this particular query.
#[derive(Debug, Clone)]
pub(crate) struct PreparedQuery {
    pub(crate) name: String,
    pub(crate) params: Vec<PreparedField>,
    pub(crate) row: Option<(usize, Vec<usize>)>, // None if execute
    pub(crate) sql: String,
}

/// A row or params field
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedField {
    pub(crate) name: String,
    pub(crate) ty: Rc<CornucopiaType>,
    pub(crate) is_nullable: bool,
    pub(crate) is_inner_nullable: bool, // Vec only
}

/// A params struct
#[derive(Debug, Clone)]
pub(crate) struct PreparedParams {
    pub(crate) name: Parsed<String>,
    pub(crate) fields: Vec<PreparedField>,
    pub(crate) is_copy: bool,
    pub(crate) queries: Vec<usize>,
}

/// A returned row struct
#[derive(Debug, Clone)]
pub(crate) struct PreparedRow {
    pub(crate) name: Parsed<String>,
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
    Composite(Vec<PreparedField>),
}

/// A struct containing the module name and the list of all
/// the queries it contains.
#[derive(Debug, Clone)]
pub(crate) struct PreparedModule {
    pub(crate) info: Rc<ModuleInfo>,
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
                let prev = &o.get();
                // If the row doesn't contain the same fields as a previously
                // registered row with the same name...
                validation::named_struct_field(
                    &self.info,
                    &prev.name,
                    &prev.fields,
                    &name,
                    &fields,
                )?;

                let indexes: Option<Vec<_>> = prev
                    .fields
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
                    name: name.clone(),
                    fields: tmp,
                    is_copy,
                });
                self.add_row(registrar, name, fields)
            }
        }
    }

    fn add_param(&mut self, name: Parsed<String>, query_idx: usize) -> Result<usize, ErrorVariant> {
        let fields = &self.queries.get_index(query_idx).unwrap().1.params;
        assert!(!fields.is_empty());
        match self.params.entry(name.value.clone()) {
            Entry::Occupied(mut o) => {
                let prev = o.get_mut();
                // If the param doesn't contain the same fields as a previously
                // registered param with the same name...
                validation::named_struct_field(
                    &self.info,
                    &prev.name,
                    &prev.fields,
                    &name,
                    fields,
                )?;

                prev.queries.push(query_idx);

                Ok(o.index())
            }
            Entry::Vacant(v) => {
                let is_copy = fields.iter().all(|f| f.ty.is_copy());
                let mut tmp = fields.to_vec();
                tmp.sort_unstable_by(|a, b| a.name.cmp(&b.name));
                v.insert(PreparedParams {
                    name: name.clone(),
                    fields: tmp,
                    is_copy,
                    queries: vec![],
                });
                self.add_param(name, query_idx)
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
        self.queries
            .insert_full(
                name.clone(),
                PreparedQuery {
                    name,
                    params,
                    row: row_idx,
                    sql,
                },
            )
            .0
    }
}

/// Prepares all modules
pub(crate) fn prepare(
    client: &mut Client,
    modules: Vec<ValidatedModule>,
) -> Result<Preparation, Error> {
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
    tmp.modules
        .sort_unstable_by(|a, b| a.info.name.cmp(&b.info.name));
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

fn normalize_rust_name(name: &str) -> String {
    name.replace(':', "_")
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
            Kind::Domain(_) => return None,
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
    validated_module: ValidatedModule,
    registrar: &mut TypeRegistrar,
) -> Result<PreparedModule, Error> {
    let mut tmp_prepared_module = PreparedModule {
        info: validated_module.info,
        queries: IndexMap::new(),
        params: IndexMap::new(),
        rows: IndexMap::new(),
    };

    for query in validated_module.queries {
        prepare_query(
            client,
            &mut tmp_prepared_module,
            registrar,
            &validated_module.param_types,
            &validated_module.row_types,
            query,
        )?;
    }

    Ok(tmp_prepared_module)
}

/// Prepares a query
fn prepare_query(
    client: &mut Client,
    module: &mut PreparedModule,
    registrar: &mut TypeRegistrar,
    param_types: &[TypeDataStructure],
    row_types: &[TypeDataStructure],
    ValidatedQuery {
        name,
        params,
        bind_params,
        row,
        sql_str,
    }: ValidatedQuery,
) -> Result<(), Error> {
    // Prepare the statement
    let stmt = client
        .prepare(&sql_str)
        .map_err(|e| Error::new(e, &name, module.info.clone()))?;

    let (nullable_params_fields, params_name) =
        params.name_and_fields(param_types, &name, Some("Params"));
    let (nullable_row_fields, row_name) = row.name_and_fields(row_types, &name, None);
    let params_fields = {
        let stmt_params = stmt.params();
        let params = bind_params
            .iter()
            .zip(stmt_params)
            .map(|(a, b)| (a.to_owned(), b.to_owned()))
            .collect::<Vec<(Parsed<String>, Type)>>();
        for nullable_col in &nullable_params_fields {
            // If none of the row's columns match the nullable column
            validation::nullable_param_name(&module.info, nullable_col, &params)
                .map_err(ErrorVariant::from)
                .map_err(|e| Error::new(e, &name, module.info.clone()))?;
        }

        let mut param_fields = Vec::new();
        for (col_name, col_ty) in params {
            let is_nullable = nullable_params_fields
                .iter()
                .any(|x| x.value == col_name.value);
            // Register type
            param_fields.push(PreparedField {
                name: col_name.value.clone(),
                ty: registrar
                    .register(&col_ty)
                    .map_err(|e| Error::new(e, &name, module.info.clone()))?
                    .clone(),
                is_nullable,
                is_inner_nullable: false, // TODO used when support null everywhere
            });
        }
        param_fields
    };

    let row_fields = {
        let stmt_cols = stmt.columns();
        // Check for duplicate names
        if let Some(duplicate_col) = has_duplicate(stmt_cols.iter(), |col| col.name()) {
            return Err(Error::new(
                ErrorVariant::DuplicateSqlColName {
                    name: duplicate_col.name().to_owned(),
                },
                &name,
                module.info.clone(),
            ));
        };
        for nullable_col in &nullable_row_fields {
            // If none of the row's columns match the nullable column
            validation::nullable_column_name(&module.info, nullable_col, stmt_cols)
                .map_err(ErrorVariant::from)
                .map_err(|e| Error::new(e, &name, module.info.clone()))?;
        }

        let mut row_fields = Vec::new();
        for (col_name, col_ty) in stmt_cols.iter().map(|c| (c.name().to_owned(), c.type_())) {
            let is_nullable = nullable_row_fields.iter().any(|x| x.value == col_name);

            // Register type
            let ty = registrar
                .register(col_ty)
                .map_err(|e| Error::new(e, &name, module.info.clone()))?
                .clone();
            row_fields.push(PreparedField {
                name: normalize_rust_name(&col_name),
                ty,
                is_nullable,
                is_inner_nullable: false, // TODO used when support null everywhere
            });
        }
        row_fields
    };

    let params_empty = params_fields.is_empty();
    let row_idx = if !row_fields.is_empty() {
        Some(
            module
                .add_row(registrar, row_name, row_fields)
                .map_err(|e| Error {
                    err: e,
                    query_name: name.clone(),
                    module_info: module.info.clone(),
                })?,
        )
    } else {
        None
    };
    let query_idx = module.add_query(name.value.clone(), params_fields, row_idx, sql_str);
    if !params_empty {
        module
            .add_param(params_name, query_idx)
            .map_err(|e| Error {
                err: e,
                query_name: name.clone(),
                module_info: module.info.clone(),
            })?;
    };

    Ok(())
}

pub(crate) mod error {
    use std::fmt::Display;
    use std::rc::Rc;

    use crate::parser::Parsed;
    use crate::read_queries::ModuleInfo;
    use crate::type_registrar::error::Error as PostgresTypeError;
    use crate::utils::compute_line;
    use crate::validation::error::Error as ValidationError;
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    #[error("{0}")]
    pub(crate) enum ErrorVariant {
        Db(#[from] postgres::Error),
        PostgresType(#[from] PostgresTypeError),
        Validation(#[from] ValidationError),
        #[error("Two or more columns have the same name: `{name}`. Consider disambiguing the column names with `AS` clauses.")]
        DuplicateSqlColName {
            name: String,
        },
    }

    #[derive(Debug)]
    pub struct Error {
        pub(crate) query_name: Parsed<String>,
        pub(crate) err: ErrorVariant,
        pub(crate) module_info: Rc<ModuleInfo>,
    }

    impl Error {
        pub(crate) fn new<E: Into<ErrorVariant>>(
            err: E,
            query_name: &Parsed<String>,
            info: Rc<ModuleInfo>,
        ) -> Self {
            Self {
                err: err.into(),
                query_name: query_name.clone(),
                module_info: info,
            }
        }
    }

    impl Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // Retrieve line index and content
            let (_, line, _) = compute_line(&self.module_info.content, self.query_name.start);
            match &self.err {
                ErrorVariant::Db(e) => write!(
                    f,
                    "Error while preparing query \"{}\" [file: \"{}\", line: {}] ({})",
                    self.query_name.value,
                    self.module_info.path,
                    line,
                    e.as_db_error().unwrap().message()
                ),
                ErrorVariant::Validation(e) => e.fmt(f),
                _ => write!(
                    f,
                    "Error while preparing query \"{}\" [file: \"{}\", line: {}]:\n{}",
                    self.query_name.value, self.module_info.path, line, self.err
                ),
            }
        }
    }

    impl std::error::Error for Error {}
}
