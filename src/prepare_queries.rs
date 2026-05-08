use std::{collections::HashMap, rc::Rc};

use futures::{StreamExt, stream::FuturesUnordered};
use heck::ToUpperCamelCase;
use indexmap::{IndexMap, map::Entry};
use postgres_types::{Kind, Type};
use tokio_postgres::{Client, Statement};

use crate::{
    codegen::{DependencyAnalysis, GenCtx, ModCtx},
    config::Config,
    parser::{Module, NullableIdent, Query, Span, TypeAnnotation},
    read_queries::ModuleInfo,
    type_registrar::{ClorindeType, TypeRegistrar},
    utils::KEYWORD,
    validation,
};

use self::error::Error;

type ModuleNestedSpecs = std::collections::HashMap<String, std::collections::HashMap<String, bool>>;

/// This data structure is used by Clorinde to generate
/// all constructs related to this particular query.
#[derive(Debug, Clone)]
pub(crate) struct PreparedQuery {
    pub(crate) ident: Ident,
    pub(crate) param: Option<(usize, Vec<usize>)>,
    pub(crate) comments: Vec<String>,
    pub(crate) row: Option<(usize, Vec<usize>)>,
    pub(crate) sql: String,
    pub(crate) attributes: Vec<String>,
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
    pub(crate) ty: Rc<ClorindeType>,
    pub(crate) is_nullable: bool,
    pub(crate) is_inner_nullable: bool,          // Vec only
    pub(crate) attributes: Vec<String>,          // Custom field attributes
    pub(crate) attributes_borrowed: Vec<String>, // Custom field attributes for borrowed structs
    pub(crate) nested_nullability: std::collections::HashMap<String, bool>, // Field name -> nullable
}

impl PreparedField {
    pub(crate) fn new(
        db_ident: String,
        ty: Rc<ClorindeType>,
        nullity: Option<&NullableIdent>,
    ) -> Self {
        let mut nested_nullability = std::collections::HashMap::new();

        // Extract nested field nullability specifications
        if let Some(nullity) = nullity {
            for (field_name, is_nullable) in nullity.get_field_nullability() {
                nested_nullability.insert(field_name.to_string(), is_nullable);
            }
        }

        Self {
            ident: Ident::new(db_ident),
            ty,
            is_nullable: nullity.is_some_and(|it| it.nullable),
            is_inner_nullable: nullity.is_some_and(|it| it.inner_nullable),
            attributes: Vec::new(),
            attributes_borrowed: Vec::new(),
            nested_nullability,
        }
    }

    pub(crate) fn with_attributes(mut self, attributes: (Vec<String>, Vec<String>)) -> Self {
        self.attributes = attributes.0;
        self.attributes_borrowed = attributes.1;
        self
    }
}

impl PreparedField {
    pub fn unwrapped_name(&self) -> String {
        self.own_struct(&GenCtx::new(ModCtx::Types, false))
            .replace(['<', '>', '_'], "")
            .to_upper_camel_case()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PreparedItem {
    pub(crate) name: Span<String>,
    pub(crate) fields: Vec<PreparedField>,
    pub(crate) traits: Vec<String>,
    pub(crate) is_copy: bool,
    pub(crate) is_named: bool,
    pub(crate) is_ref: bool,
    pub(crate) attributes: Vec<String>,
    pub(crate) attributes_borrowed: Vec<String>,
}

impl PreparedItem {
    pub fn new(
        name: Span<String>,
        fields: Vec<PreparedField>,
        traits: Vec<String>,
        is_implicit: bool,
        attributes: Vec<String>,
        attributes_borrowed: Vec<String>,
    ) -> Self {
        Self {
            name,
            is_copy: fields.iter().all(|f| f.ty.is_copy()),
            is_ref: fields.iter().any(|f| f.ty.is_ref()),
            is_named: !is_implicit || fields.len() > 1,
            fields,
            traits,
            attributes,
            attributes_borrowed,
        }
    }

    pub fn path(&self, ctx: &GenCtx) -> String {
        match ctx.hierarchy {
            ModCtx::Types | ModCtx::SchemaTypes => {
                unreachable!()
            }
            ModCtx::Queries => self.name.to_string(),
            ModCtx::ClientQueries => format!("super::{}", self.name),
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
    pub(crate) traits: Vec<String>,
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

#[allow(clippy::result_large_err)]
impl PreparedModule {
    #[allow(clippy::too_many_arguments)]
    fn add(
        info: &ModuleInfo,
        map: &mut IndexMap<Span<String>, PreparedItem>,
        name: Span<String>,
        fields: Vec<PreparedField>,
        traits: Vec<String>,
        is_implicit: bool,
        attributes: Vec<String>,
        attributes_borrowed: Vec<String>,
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
                v.insert(PreparedItem::new(
                    name.clone(),
                    fields.clone(),
                    traits.clone(),
                    is_implicit,
                    attributes.clone(),
                    attributes_borrowed.clone(),
                ));
                Self::add(
                    info,
                    map,
                    name,
                    fields,
                    traits,
                    is_implicit,
                    attributes,
                    attributes_borrowed,
                )
            }
        }
    }

    #[allow(clippy::result_large_err)]
    fn add_row(
        &mut self,
        name: Span<String>,
        fields: Vec<PreparedField>,
        traits: Vec<String>,
        is_implicit: bool,
        attributes: Vec<String>,
        attributes_borrowed: Vec<String>,
    ) -> Result<(usize, Vec<usize>), Error> {
        let nom = if fields.len() == 1 && is_implicit {
            name.map(|_| fields[0].unwrapped_name())
        } else {
            name
        };
        Self::add(
            &self.info,
            &mut self.rows,
            nom,
            fields,
            traits,
            is_implicit,
            attributes,
            attributes_borrowed,
        )
    }

    #[allow(clippy::result_large_err)]
    fn add_param(
        &mut self,
        name: Span<String>,
        fields: Vec<PreparedField>,
        is_implicit: bool,
    ) -> Result<(usize, Vec<usize>), Error> {
        Self::add(
            &self.info,
            &mut self.params,
            name,
            fields,
            vec![],
            is_implicit,
            vec![],
            vec![],
        )
    }

    fn add_query(
        &mut self,
        name: Span<String>,
        comments: Vec<String>,
        param_idx: Option<(usize, Vec<usize>)>,
        row_idx: Option<(usize, Vec<usize>)>,
        sql: String,
        attributes: Vec<String>,
    ) {
        self.queries.insert(
            name.clone(),
            PreparedQuery {
                ident: Ident::new(name.value),
                row: row_idx,
                comments,
                sql,
                param: param_idx,
                attributes,
            },
        );
    }
}

#[allow(clippy::result_large_err)]
/// Prepares all modules
pub(crate) fn prepare(
    client: &Client,
    modules: Vec<Module>,
    config: &Config,
) -> Result<Preparation, Error> {
    let stmts = prepare_sql(client, &modules);
    let mut registrar = TypeRegistrar::new(config.clone());
    let mut prepared_types: IndexMap<String, Vec<PreparedType>> = IndexMap::new();
    let mut prepared_modules = Vec::new();
    let mut nested_nullability_specs: std::collections::HashMap<
        String,
        std::collections::HashMap<String, bool>,
    > = std::collections::HashMap::new();

    let declared: Vec<_> = modules
        .iter()
        .flat_map(|it| &it.types)
        .map(|ty| (*ty).clone())
        .collect();

    for module in modules {
        let (prepared_module, module_nested_specs) =
            prepare_module(&stmts, module, &mut registrar)?;

        prepared_modules.push(prepared_module);

        // Merge nested nullability specifications
        for (type_name, field_specs) in module_nested_specs {
            nested_nullability_specs
                .entry(type_name)
                .or_default()
                .extend(field_specs);
        }
    }

    // Prepare types grouped by schema
    for ((schema, name), ty) in &registrar.types {
        let type_nested_specs = nested_nullability_specs
            .get(name)
            .cloned()
            .unwrap_or_default();

        if let Some(ty) = prepare_type(&registrar, name, ty, &declared, &type_nested_specs) {
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
    ty: &ClorindeType,
    types: &[TypeAnnotation],
    nested_specs: &std::collections::HashMap<String, bool>,
) -> Option<PreparedType> {
    if let ClorindeType::Custom {
        pg_ty,
        struct_name,
        is_copy,
        is_params,
        ..
    } = ty
    {
        let type_annotation = types.iter().find(|it| it.name.value == pg_ty.name());
        let declared = type_annotation.map_or(&[] as &[NullableIdent], |it| it.fields.as_slice());
        let traits = type_annotation.map_or_else(Vec::new, |it| it.traits.clone());

        let content = match pg_ty.kind() {
            Kind::Enum(variants) => {
                PreparedContent::Enum(variants.clone().into_iter().map(Ident::new).collect())
            }
            Kind::Domain(_) => return None,
            Kind::Composite(fields) => PreparedContent::Composite(
                fields
                    .iter()
                    .map(|field| {
                        let mut nullity = declared
                            .iter()
                            .find(|it| it.name.value == field.name())
                            .cloned();

                        // Apply nested nullability specifications
                        if let Some(&should_be_nullable) = nested_specs.get(field.name()) {
                            if let Some(ref mut existing_nullity) = nullity {
                                existing_nullity.nullable = should_be_nullable;
                            } else if should_be_nullable {
                                // Create new nullity specification
                                nullity = Some(NullableIdent {
                                    name: crate::parser::Span {
                                        span: (0..0).into(),
                                        value: field.name().to_string(),
                                    },
                                    nullable: true,
                                    inner_nullable: false,
                                    nested_fields: Vec::new(),
                                });
                            }
                        }

                        let ty = registrar.ref_of(field.type_());

                        let is_nullable = nullity.as_ref().is_some_and(|n| n.nullable);
                        let attributes =
                            if let Some(mapping) = registrar.get_type_mapping(field.type_()) {
                                mapping.get_attributes(is_nullable)
                            } else {
                                (Vec::new(), Vec::new())
                            };

                        PreparedField::new(field.name().to_string(), ty, nullity.as_ref())
                            .with_attributes(attributes)
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
            traits,
        })
    } else {
        None
    }
}

#[allow(clippy::result_large_err)]
fn prepare_sql(
    client: &Client,
    modules: &[Module],
) -> HashMap<String, Result<Statement, tokio_postgres::Error>> {
    let queries: FuturesUnordered<_> = modules
        .iter()
        .flat_map(|m| m.queries.iter().map(|q| q.sql_str.clone()))
        .map(|query| async move {
            let stmt = client.prepare(&query).await;
            (query, stmt)
        })
        .collect();
    let results: HashMap<_, _> = futures::executor::block_on(queries.collect());
    results
}

#[allow(clippy::result_large_err)]
/// Prepares all queries in this module and collects nested nullability specifications
fn prepare_module(
    stmts: &HashMap<String, Result<Statement, tokio_postgres::Error>>,
    module: Module,
    registrar: &mut TypeRegistrar,
) -> Result<(PreparedModule, ModuleNestedSpecs), Error> {
    validation::validate_module(&module)?;

    let mut tmp_prepared_module = PreparedModule {
        info: module.info.clone(),
        queries: IndexMap::new(),
        params: IndexMap::new(),
        rows: IndexMap::new(),
    };

    let mut all_nested_specs: std::collections::HashMap<
        String,
        std::collections::HashMap<String, bool>,
    > = std::collections::HashMap::new();

    for query in module.queries {
        let query_nested_specs = prepare_query(
            stmts,
            &mut tmp_prepared_module,
            registrar,
            &module.types,
            query,
            &module.info,
        )?;

        // Merge nested specs from this query
        for (type_name, field_specs) in query_nested_specs {
            all_nested_specs
                .entry(type_name)
                .or_default()
                .extend(field_specs);
        }
    }

    validation::validate_preparation(&tmp_prepared_module)?;

    Ok((tmp_prepared_module, all_nested_specs))
}

#[allow(clippy::result_large_err)]
/// Prepares a query
fn prepare_query(
    stmts: &HashMap<String, Result<Statement, tokio_postgres::Error>>,
    module: &mut PreparedModule,
    registrar: &mut TypeRegistrar,
    types: &[TypeAnnotation],
    Query {
        name,
        param,
        comments,
        bind_params,
        row,
        sql_str,
        sql_span,
        attributes,
    }: Query,
    module_info: &ModuleInfo,
) -> Result<std::collections::HashMap<String, std::collections::HashMap<String, bool>>, Error> {
    let mut nested_specs: std::collections::HashMap<
        String,
        std::collections::HashMap<String, bool>,
    > = std::collections::HashMap::new();

    // Prepare the statement
    let stmt = stmts[&sql_str]
        .as_ref()
        .map_err(|e| Error::new_db_err(e, module_info, &sql_span, &name))?;

    let (nullable_params_fields, _, params_name, _, _) =
        param.name_and_fields(types, &name, Some("Params"));

    let (nullable_row_fields, traits, row_name, row_attributes, row_attributes_borrowed) =
        row.name_and_fields(types, &name, None);

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
            let ty = registrar
                .register(&col_name.value, &col_ty, &name, module_info)?
                .clone();

            let is_nullable = nullity.is_some_and(|n| n.nullable);
            let attributes = if let Some(mapping) = registrar.get_type_mapping(&col_ty) {
                mapping.get_attributes(is_nullable)
            } else {
                (Vec::new(), Vec::new())
            };

            param_fields.push(
                PreparedField::new(col_name.value.clone(), ty, nullity).with_attributes(attributes),
            );
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
            // Find ALL matching nullity entries (there may be multiple for the same field with different nested specs)
            let matching_nullities: Vec<&NullableIdent> = nullable_row_fields
                .iter()
                .filter(|x| x.name.value == col_name)
                .collect();

            let nullity = matching_nullities.first();

            // Collect nested nullability specifications from ALL matching entries
            let mut all_nested_specs = std::collections::HashMap::new();

            for nullity_entry in &matching_nullities {
                if !nullity_entry.nested_fields.is_empty()
                    && extract_composite_type_name(col_ty).is_some()
                {
                    // Collect field specifications
                    let field_specs: std::collections::HashMap<String, bool> = nullity_entry
                        .get_field_nullability()
                        .map(|(k, v)| (k.to_string(), v))
                        .collect();
                    all_nested_specs.extend(field_specs);
                }
            }

            if !all_nested_specs.is_empty() {
                let type_key = extract_composite_type_name(col_ty);
                if let Some(type_name) = type_key {
                    nested_specs.insert(type_name, all_nested_specs);
                }
            }

            // Register type
            let ty = registrar
                .register(&col_name, col_ty, &name, module_info)?
                .clone();

            let is_nullable = nullity.is_some_and(|n| n.nullable);
            let attributes = if let Some(mapping) = registrar.get_type_mapping(col_ty) {
                mapping.get_attributes(is_nullable)
            } else {
                (Vec::new(), Vec::new())
            };

            row_fields.push(
                PreparedField::new(normalize_rust_name(&col_name), ty, nullity.copied())
                    .with_attributes(attributes),
            );
        }
        row_fields
    };

    let row_idx = if row_fields.is_empty() {
        None
    } else {
        Some(module.add_row(
            row_name,
            row_fields,
            traits,
            row.is_implicit(),
            row_attributes,
            row_attributes_borrowed,
        )?)
    };

    let param_idx = if params_fields.is_empty() {
        None
    } else {
        Some(module.add_param(params_name, params_fields, param.is_implicit())?)
    };

    module.add_query(
        name.clone(),
        comments,
        param_idx,
        row_idx,
        sql_str,
        attributes,
    );

    Ok(nested_specs)
}

fn extract_composite_type_name(col_ty: &postgres_types::Type) -> Option<String> {
    match col_ty.kind() {
        Kind::Array(inner_type) => {
            // For arrays, get the inner type name
            Some(inner_type.name().to_string())
        }
        Kind::Composite(_) => {
            // For direct composite types
            Some(col_ty.name().to_string())
        }
        _ => None,
    }
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
            err: &tokio_postgres::Error,
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
