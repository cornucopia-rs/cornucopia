use std::rc::Rc;

use indexmap::{map::Entry, IndexMap};
use postgres::Client;
use postgres_types::{Kind, Type};

use crate::{
    parser::{Span, TypeAnnotation},
    read_queries::ModuleInfo,
    type_registrar::CornucopiaType,
    type_registrar::TypeRegistrar,
    validation,
};

use self::error::Error;

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
    pub(crate) name: Span<String>,
    pub(crate) fields: Vec<PreparedField>,
    pub(crate) is_copy: bool,
    pub(crate) queries: Vec<usize>,
}

/// A returned row struct
#[derive(Debug, Clone)]
pub(crate) struct PreparedRow {
    pub(crate) name: Span<String>,
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
    pub(crate) info: ModuleInfo,
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
        name: Span<String>,
        fields: Vec<PreparedField>,
    ) -> Result<(usize, Vec<usize>), Error> {
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
                v.insert(PreparedRow {
                    name: name.clone(),
                    fields: fields.clone(),
                    is_copy,
                });
                self.add_row(registrar, name, fields)
            }
        }
    }

    fn add_param(&mut self, name: Span<String>, query_idx: usize) -> Result<usize, Error> {
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
                v.insert(PreparedParams {
                    name: name.clone(),
                    fields: fields.to_vec(),
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
    modules: Vec<validation::Module>,
) -> Result<Preparation, Error> {
    let mut registrar = TypeRegistrar::default();
    let mut tmp = Preparation {
        modules: Vec::new(),
        types: IndexMap::new(),
    };
    let declared: Vec<_> = modules
        .iter()
        .flat_map(|it| &it.types)
        .map(|ty| (*ty).clone())
        .collect();

    for module in modules {
        tmp.modules
            .push(prepare_module(client, module, &mut registrar)?);
    }

    // Prepare types grouped by schema
    for ((schema, name), ty) in &registrar.types {
        if let Some(ty) = prepare_type(&registrar, name, ty, &declared) {
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
    types: &[TypeAnnotation],
) -> Option<PreparedType> {
    if let CornucopiaType::Custom {
        pg_ty,
        struct_name,
        is_copy,
        is_params,
        ..
    } = ty
    {
        let declared = types
            .iter()
            .find(|it| it.name.value == pg_ty.name())
            .map(|it| it.fields.as_slice())
            .unwrap_or(&[]);
        let content = match pg_ty.kind() {
            Kind::Enum(variants) => PreparedContent::Enum(variants.to_vec()),
            Kind::Domain(_) => return None,
            Kind::Composite(fields) => PreparedContent::Composite(
                fields
                    .iter()
                    .map(|field| {
                        let nullity = declared.iter().find(|it| it.name.value == field.name());
                        PreparedField {
                            name: field.name().to_string(),
                            ty: registrar.ref_of(field.type_()),
                            is_nullable: nullity.map(|it| it.nullable).unwrap_or(false),
                            is_inner_nullable: nullity.map(|it| it.inner_nullable).unwrap_or(false),
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
    module: validation::Module,
    registrar: &mut TypeRegistrar,
) -> Result<PreparedModule, Error> {
    let mut tmp_prepared_module = PreparedModule {
        info: module.info.clone(),
        queries: IndexMap::new(),
        params: IndexMap::new(),
        rows: IndexMap::new(),
    };

    for query in module.queries {
        prepare_query(
            client,
            &mut tmp_prepared_module,
            registrar,
            &module.types,
            query,
            &module.info,
        )?;
    }

    Ok(tmp_prepared_module)
}

/// Prepares a query
fn prepare_query(
    client: &mut Client,
    module: &mut PreparedModule,
    registrar: &mut TypeRegistrar,
    types: &[TypeAnnotation],
    validation::Query {
        name,
        params,
        bind_params,
        row,
        sql_str,
        sql_span,
    }: validation::Query,
    module_info: &ModuleInfo,
) -> Result<(), Error> {
    // Prepare the statement
    let stmt = client
        .prepare(&sql_str)
        .map_err(|e| Error::new_db_err(e, module_info, &sql_span, &name))?;

    let (nullable_params_fields, params_name) =
        params.name_and_fields(types, &name, Some("Params"));
    let (nullable_row_fields, row_name) = row.name_and_fields(types, &name, None);
    let params_fields = {
        let stmt_params = stmt.params();
        let params = bind_params
            .iter()
            .zip(stmt_params)
            .map(|(a, b)| (a.to_owned(), b.to_owned()))
            .collect::<Vec<(Span<String>, Type)>>();
        for nullable_col in nullable_params_fields {
            // If none of the row's columns match the nullable column
            validation::nullable_param_name(&module.info, nullable_col, &params)
                .map_err(Error::from)?;
        }

        let mut param_fields = Vec::new();
        for (col_name, col_ty) in params {
            let nullity = nullable_params_fields
                .iter()
                .find(|x| x.name.value == col_name.value);
            // Register type
            param_fields.push(PreparedField {
                name: col_name.value.clone(),
                ty: registrar
                    .register(&col_name.value, &col_ty, &name, module_info)?
                    .clone(),
                is_nullable: nullity.map(|it| it.nullable).unwrap_or(false),
                is_inner_nullable: nullity.map(|it| it.inner_nullable).unwrap_or(false),
            });
        }
        param_fields
    };

    let row_fields = {
        let stmt_cols = stmt.columns();
        // Check for row declaration on execute
        validation::row_on_execute(&module.info, &name, &sql_span, &row, stmt_cols)?;
        // Check for duplicate names
        validation::duplicate_sql_col_name(&module.info, &name, stmt_cols).map_err(Error::from)?;
        for nullable_col in nullable_row_fields {
            // If none of the row's columns match the nullable column
            validation::nullable_column_name(&module.info, nullable_col, stmt_cols)
                .map_err(Error::from)?;
        }

        let mut row_fields = Vec::new();
        for (col_name, col_ty) in stmt_cols.iter().map(|c| (c.name().to_owned(), c.type_())) {
            let nullity = nullable_row_fields
                .iter()
                .find(|x| x.name.value == col_name);
            // Register type
            let ty = registrar
                .register(&col_name, col_ty, &name, module_info)?
                .clone();
            row_fields.push(PreparedField {
                name: normalize_rust_name(&col_name),
                ty,
                is_nullable: nullity.map(|it| it.nullable).unwrap_or(false),
                is_inner_nullable: nullity.map(|it| it.inner_nullable).unwrap_or(false),
            });
        }
        row_fields
    };

    let params_empty = params_fields.is_empty();
    let row_idx = if !row_fields.is_empty() {
        Some(module.add_row(registrar, row_name, row_fields)?)
    } else {
        None
    };
    let query_idx = module.add_query(name.value.clone(), params_fields, row_idx, sql_str);
    if !params_empty {
        module.add_param(params_name, query_idx)?;
    };

    Ok(())
}

pub(crate) mod error {
    use miette::{Diagnostic, NamedSource, SourceSpan};
    use thiserror::Error as ThisError;

    use crate::{
        parser::Span, read_queries::ModuleInfo, type_registrar::error::Error as PostgresTypeError,
        utils::db_err, validation::error::Error as ValidationError,
    };

    #[derive(Debug, ThisError, Diagnostic)]
    pub enum Error {
        #[error("Couldn't prepare query: {msg}")]
        Db {
            msg: String,
            #[help]
            help: Option<String>,
            #[source_code]
            src: NamedSource,
            #[label("error occurs near this location")]
            err_span: Option<SourceSpan>,
        },
        #[error(transparent)]
        #[diagnostic(transparent)]
        PostgresType(#[from] PostgresTypeError),
        #[error(transparent)]
        #[diagnostic(transparent)]
        Validation(#[from] ValidationError),
    }

    impl Error {
        pub(crate) fn new_db_err(
            err: postgres::Error,
            module_info: &ModuleInfo,
            query_span: &SourceSpan,
            query_name: &Span<String>,
        ) -> Self {
            let msg = format!("{:#}", err);
            if let Some((position, msg, help)) = db_err(err) {
                Self::Db {
                    msg,
                    help,
                    src: module_info.into(),
                    err_span: Some((query_span.offset() + position as usize - 1).into()),
                }
            } else {
                Self::Db {
                    msg,
                    help: None,
                    src: module_info.into(),
                    err_span: Some(query_name.span),
                }
            }
        }
    }
}
