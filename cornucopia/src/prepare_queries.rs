use std::{ops::Deref, rc::Rc};

use indexmap::{map::Entry, IndexMap};
use postgres::Client;
use postgres_types::{Kind, Type};

use crate::{
    parser::{Module, NullableIdent, Query, Span, TypeAnnotation},
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
    pub(crate) params: TypeKind,
    pub(crate) row: Option<(usize, Vec<usize>)>,
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

#[derive(Debug, Clone)]
pub(crate) enum TypeKind {
    Raw([PreparedField; 1]),
    Struct {
        fields: Vec<PreparedField>,
        is_copy: bool,
    },
}

impl TypeKind {
    pub fn new(fields: Vec<PreparedField>, is_implicit: bool) -> Self {
        if fields.len() == 1 && is_implicit {
            Self::Raw(fields.try_into().unwrap())
        } else {
            Self::Struct {
                is_copy: fields.iter().all(|f| f.ty.is_copy()),
                fields,
            }
        }
    }

    pub fn fields(&self) -> &[PreparedField] {
        match self {
            TypeKind::Raw(f) => f,
            TypeKind::Struct { fields, .. } => &fields,
        }
    }
}

/// A params struct
#[derive(Debug, Clone)]
pub(crate) struct PreparedParams {
    pub(crate) name: Span<String>,
    pub(crate) kind: TypeKind,
    pub(crate) queries: Vec<usize>,
}

/// A returned row struct
#[derive(Debug, Clone)]
pub(crate) struct PreparedRow {
    pub(crate) name: Span<String>,
    pub(crate) kind: TypeKind,
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
    pub(crate) queries: IndexMap<Span<String>, PreparedQuery>,
    pub(crate) params: IndexMap<Span<String>, PreparedParams>,
    pub(crate) rows: IndexMap<Span<String>, PreparedRow>,
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
        is_implicit: bool,
    ) -> Result<(usize, Vec<usize>), Error> {
        assert!(!fields.is_empty());
        let name = if fields.len() == 1 && is_implicit {
            name.map(|_| fields[0].ty.name(false))
        } else {
            name
        };
        match self.rows.entry(name.clone()) {
            Entry::Occupied(o) => {
                let prev = &o.get();
                // If the row doesn't contain the same fields as a previously
                // registered row with the same name...
                if let TypeKind::Struct {
                    fields: prev_fields,
                    ..
                } = &prev.kind
                {
                    validation::named_struct_field(
                        &self.info,
                        &prev.name,
                        prev_fields,
                        &name,
                        &fields,
                    )?;
                };

                let indexes: Vec<_> = match &prev.kind {
                    TypeKind::Raw(_) => vec![0],
                    TypeKind::Struct { fields, .. } => fields
                        .iter()
                        .map(|f| fields.iter().position(|it| it == f).unwrap())
                        .collect(),
                };
                Ok((o.index(), indexes))
            }
            Entry::Vacant(v) => {
                v.insert(PreparedRow {
                    name: name.clone(),
                    kind: TypeKind::new(fields.clone(), is_implicit),
                });
                self.add_row(registrar, name, fields, is_implicit)
            }
        }
    }

    fn add_param(&mut self, name: Span<String>, query_idx: usize) -> Result<usize, Error> {
        let kind = &self.queries.get_index(query_idx).unwrap().1.params;
        let name = match kind {
            TypeKind::Raw([field]) => name.map(|_| field.ty.name(false)),
            TypeKind::Struct { .. } => name,
        };

        match self.params.entry(name.clone()) {
            Entry::Occupied(mut o) => {
                let prev = o.get_mut();
                // If the param doesn't contain the same fields as a previously
                // registered param with the same name...
                if let TypeKind::Struct {
                    fields: prev_fields,
                    ..
                } = &prev.kind
                {
                    if let TypeKind::Struct { fields, .. } = &kind {
                        validation::named_struct_field(
                            &self.info,
                            &prev.name,
                            prev_fields,
                            &name,
                            fields,
                        )?;
                    }
                };
                prev.queries.push(query_idx);
                Ok(o.index())
            }
            Entry::Vacant(v) => {
                v.insert(PreparedParams {
                    name: name.clone(),
                    queries: vec![],
                    kind: kind.clone(),
                });
                self.add_param(name, query_idx)
            }
        }
    }

    fn add_query(
        &mut self,
        name: Span<String>,
        params: Vec<PreparedField>,
        row_idx: Option<(usize, Vec<usize>)>,
        is_implicit: bool,
        sql: String,
    ) -> usize {
        self.queries
            .insert_full(
                name.clone(),
                PreparedQuery {
                    name: name.value,
                    params: TypeKind::new(params, is_implicit),
                    row: row_idx,
                    sql,
                },
            )
            .0
    }
}

/// Prepares all modules
pub(crate) fn prepare(client: &mut Client, modules: Vec<Module>) -> Result<Preparation, Error> {
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
            .map_or(&[] as &[NullableIdent], |it| it.fields.as_slice());
        let content = match pg_ty.kind() {
            Kind::Enum(variants) => PreparedContent::Enum(variants.clone()),
            Kind::Domain(_) => return None,
            Kind::Composite(fields) => PreparedContent::Composite(
                fields
                    .iter()
                    .map(|field| {
                        let nullity = declared.iter().find(|it| it.name.value == field.name());
                        PreparedField {
                            name: field.name().to_string(),
                            ty: registrar.ref_of(field.type_()),
                            is_nullable: nullity.map_or(false, |it| it.nullable),
                            is_inner_nullable: nullity.map_or(false, |it| it.inner_nullable),
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
    validation::validate_module(&module)?;

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

    validation::validate_preparation(&tmp_prepared_module)?;

    Ok(tmp_prepared_module)
}

/// Prepares a query
fn prepare_query(
    client: &mut Client,
    module: &mut PreparedModule,
    registrar: &mut TypeRegistrar,
    types: &[TypeAnnotation],
    Query {
        name,
        param,
        bind_params,
        row,
        sql_str,
        sql_span,
    }: Query,
    module_info: &ModuleInfo,
) -> Result<(), Error> {
    // Prepare the statement
    let stmt = client
        .prepare(&sql_str)
        .map_err(|e| Error::new_db_err(&e, module_info, &sql_span, &name))?;

    let (nullable_params_fields, params_name) = param.name_and_fields(types, &name, Some("Params"));
    let (nullable_row_fields, row_name) = row.name_and_fields(types, &name, None);
    let params_fields = {
        let stmt_params = stmt.params();
        let params = bind_params
            .iter()
            .zip(stmt_params)
            .map(|(a, b)| (a.clone(), b.clone()))
            .collect::<Vec<(Span<String>, Type)>>();
        // Check for param declaration on simple query
        validation::param_on_simple_query(&module.info, &name, &sql_span, &param, &params)?;
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
                is_nullable: nullity.map_or(false, |it| it.nullable),
                is_inner_nullable: nullity.map_or(false, |it| it.inner_nullable),
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
                is_nullable: nullity.map_or(false, |it| it.nullable),
                is_inner_nullable: nullity.map_or(false, |it| it.inner_nullable),
            });
        }
        row_fields
    };

    let params_empty = params_fields.is_empty();
    let row_idx = if row_fields.is_empty() {
        None
    } else {
        Some(module.add_row(registrar, row_name, row_fields, row.is_implicit())?)
    };
    let query_idx = module.add_query(name, params_fields, row_idx, param.is_implicit(), sql_str);
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
            err: &postgres::Error,
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
