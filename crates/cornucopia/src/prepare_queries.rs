use std::rc::Rc;

use heck::ToUpperCamelCase;
use indexmap::{map::Entry, IndexMap};
use postgres::Client;
use postgres_types::{Kind, Type};

use crate::{
    codegen::{DependencyAnalysis, GenCtx, ModCtx},
    parser::{Module, NullableIdent, Query, Span, TypeAnnotation},
    read_queries::ModuleInfo,
    type_registrar::CornucopiaType,
    type_registrar::TypeRegistrar,
    utils::KEYWORD,
    validation,
};

use self::error::Error;

/// This data structure is used by Cornucopia to generate
/// all constructs related to this particular query.
#[derive(Debug, Clone)]
pub(crate) struct PreparedQuery {
    pub(crate) ident: Ident,
    pub(crate) param: Option<(usize, Vec<usize>)>,
    pub(crate) row: Option<(usize, Vec<usize>)>,
    pub(crate) sql: String,
}

/// A normalized ident replacing all non-alphanumeric characters with an underscore (`_`)
/// and escaping it with a raw identifier prefix (`r#`) if it clashes with a keyword reserved in Rust.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident {
    /// Database original ident
    pub(crate) db: String,
    /// Normalized ident for rust code usage
    pub(crate) rs: String,
}

impl Ident {
    pub(crate) fn new(db: String) -> Self {
        Self {
            rs: Self::normalize_ident(&db),
            db,
        }
    }

    pub(crate) fn type_ident(&self) -> String {
        self.rs.to_upper_camel_case()
    }

    /// Normalize identifier by replacing all non-alphanumeric characters with an underscore (`_`) and
    /// escaping it with a raw identifier prefix (`r#`) if it clashes with a keyword reserved in Rust.
    fn normalize_ident(ident: &str) -> String {
        let ident = ident.replace(|c: char| !c.is_ascii_alphanumeric() && c != '_', "_");

        if KEYWORD.binary_search(&ident.as_str()).is_ok() {
            format!("r#{ident}")
        } else {
            ident
        }
    }
}

/// A row or params field
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedField {
    pub(crate) ident: Ident,
    pub(crate) ty: Rc<CornucopiaType>,
    pub(crate) is_nullable: bool,
    pub(crate) is_inner_nullable: bool, // Vec only
}

impl PreparedField {
    pub(crate) fn new(
        db_ident: String,
        ty: Rc<CornucopiaType>,
        nullity: Option<&NullableIdent>,
    ) -> Self {
        Self {
            ident: Ident::new(db_ident),
            ty,
            is_nullable: nullity.map_or(false, |it| it.nullable),
            is_inner_nullable: nullity.map_or(false, |it| it.inner_nullable),
        }
    }
}

impl PreparedField {
    pub fn unwrapped_name(&self) -> String {
        self.own_struct(&GenCtx::new(ModCtx::Types, false, false))
            .replace(['<', '>', '_'], "")
            .to_upper_camel_case()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PreparedItem {
    pub(crate) name: Span<String>,
    pub(crate) fields: Vec<PreparedField>,
    pub(crate) is_copy: bool,
    pub(crate) is_named: bool,
    pub(crate) is_ref: bool,
}

impl PreparedItem {
    pub fn new(name: Span<String>, fields: Vec<PreparedField>, is_implicit: bool) -> Self {
        Self {
            name,
            is_copy: fields.iter().all(|f| f.ty.is_copy()),
            is_ref: fields.iter().any(|f| f.ty.is_ref()),
            is_named: !is_implicit || fields.len() > 1,
            fields,
        }
    }

    pub fn path(&self, ctx: &GenCtx) -> String {
        match ctx.hierarchy {
            ModCtx::Types | ModCtx::SchemaTypes => {
                unreachable!()
            }
            ModCtx::Queries => self.name.to_string(),
            ModCtx::CLientQueries => format!("super::{}", self.name),
        }
    }
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
    Enum(Vec<Ident>),
    Composite(Vec<PreparedField>),
}

/// A struct containing the module name and the list of all
/// the queries it contains.
#[derive(Debug, Clone)]
pub(crate) struct PreparedModule {
    pub(crate) info: ModuleInfo,
    pub(crate) queries: IndexMap<Span<String>, PreparedQuery>,
    pub(crate) params: IndexMap<Span<String>, PreparedItem>,
    pub(crate) rows: IndexMap<Span<String>, PreparedItem>,
}

#[derive(Debug, Clone)]
pub(crate) struct Preparation {
    pub(crate) modules: Vec<PreparedModule>,
    pub(crate) types: IndexMap<String, Vec<PreparedType>>,
    pub(crate) dependency_analysis: DependencyAnalysis,
}

impl PreparedModule {
    fn add(
        info: &ModuleInfo,
        map: &mut IndexMap<Span<String>, PreparedItem>,
        name: Span<String>,
        fields: Vec<PreparedField>,
        is_implicit: bool,
    ) -> Result<(usize, Vec<usize>), Error> {
        assert!(!fields.is_empty());
        match map.entry(name.clone()) {
            Entry::Occupied(o) => {
                let prev = &o.get();
                // If the row doesn't contain the same fields as a previously
                // registered row with the same name...
                let indexes: Vec<_> = if prev.is_named {
                    validation::named_struct_field(info, &prev.name, &prev.fields, &name, &fields)?;
                    prev.fields
                        .iter()
                        .map(|f| fields.iter().position(|it| it == f).unwrap())
                        .collect()
                } else {
                    vec![0]
                };

                Ok((o.index(), indexes))
            }
            Entry::Vacant(v) => {
                v.insert(PreparedItem::new(name.clone(), fields.clone(), is_implicit));
                Self::add(info, map, name, fields, is_implicit)
            }
        }
    }

    fn add_row(
        &mut self,
        name: Span<String>,
        fields: Vec<PreparedField>,
        is_implicit: bool,
    ) -> Result<(usize, Vec<usize>), Error> {
        let fuck = if fields.len() == 1 && is_implicit {
            name.map(|_| fields[0].unwrapped_name())
        } else {
            name
        };
        Self::add(&self.info, &mut self.rows, fuck, fields, is_implicit)
    }

    fn add_param(
        &mut self,
        name: Span<String>,
        fields: Vec<PreparedField>,
        is_implicit: bool,
    ) -> Result<(usize, Vec<usize>), Error> {
        Self::add(&self.info, &mut self.params, name, fields, is_implicit)
    }

    fn add_query(
        &mut self,
        name: Span<String>,
        param_idx: Option<(usize, Vec<usize>)>,
        row_idx: Option<(usize, Vec<usize>)>,
        sql: String,
    ) {
        self.queries.insert(
            name.clone(),
            PreparedQuery {
                ident: Ident::new(name.value),
                row: row_idx,
                sql,
                param: param_idx,
            },
        );
    }
}

/// Prepares all modules
pub(crate) fn prepare(client: &mut Client, modules: Vec<Module>) -> Result<Preparation, Error> {
    let mut registrar = TypeRegistrar::default();
    let mut prepared_types: IndexMap<String, Vec<PreparedType>> = IndexMap::new();
    let mut prepared_modules = Vec::new();

    let declared: Vec<_> = modules
        .iter()
        .flat_map(|it| &it.types)
        .map(|ty| (*ty).clone())
        .collect();

    for module in modules {
        prepared_modules.push(prepare_module(client, module, &mut registrar)?);
    }

    // Prepare types grouped by schema
    for ((schema, name), ty) in &registrar.types {
        if let Some(ty) = prepare_type(&registrar, name, ty, &declared) {
            match prepared_types.entry(schema.clone()) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().push(ty);
                }
                Entry::Vacant(entry) => {
                    entry.insert(vec![ty]);
                }
            }
        }
    }
    Ok(Preparation {
        modules: prepared_modules,
        types: prepared_types,
        dependency_analysis: registrar.dependency_analysis,
    })
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
            Kind::Enum(variants) => {
                PreparedContent::Enum(variants.clone().into_iter().map(Ident::new).collect())
            }

            Kind::Domain(_) => return None,
            Kind::Composite(fields) => PreparedContent::Composite(
                fields
                    .iter()
                    .map(|field| {
                        let nullity = declared.iter().find(|it| it.name.value == field.name());
                        PreparedField::new(
                            field.name().to_string(),
                            registrar.ref_of(field.type_()),
                            nullity,
                        )
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
            param_fields.push(PreparedField::new(
                col_name.value.clone(),
                registrar
                    .register(&col_name.value, &col_ty, &name, module_info)?
                    .clone(),
                nullity,
            ));
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
            row_fields.push(PreparedField::new(
                normalize_rust_name(&col_name),
                ty,
                nullity,
            ));
        }
        row_fields
    };

    let row_idx = if row_fields.is_empty() {
        None
    } else {
        Some(module.add_row(row_name, row_fields, row.is_implicit())?)
    };
    let param_idx = if params_fields.is_empty() {
        None
    } else {
        Some(module.add_param(params_name, params_fields, param.is_implicit())?)
    };
    module.add_query(name.clone(), param_idx, row_idx, sql_str);

    Ok(())
}

pub(crate) mod error {
    use std::sync::Arc;

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
            src: NamedSource<Arc<String>>,
            #[label("error occurs near this location")]
            err_span: Option<SourceSpan>,
        },
        #[error(transparent)]
        #[diagnostic(transparent)]
        PostgresType(#[from] PostgresTypeError),
        #[error(transparent)]
        #[diagnostic(transparent)]
        Validation(#[from] Box<ValidationError>),
    }

    impl Error {
        pub(crate) fn new_db_err(
            err: &postgres::Error,
            module_info: &ModuleInfo,
            query_span: &SourceSpan,
            query_name: &Span<String>,
        ) -> Self {
            let msg = format!("{err:#}");
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
