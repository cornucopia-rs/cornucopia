use std::rc::Rc;

use heck::ToUpperCamelCase;
use indexmap::{IndexMap, map::Entry};
use postgres_types::{Kind, Type};

use crate::{
    codegen::{DependencyAnalysis, GenCtx, idx_char},
    config::{Config, TypeMapping},
    parser::Span,
    read_queries::ModuleInfo,
    utils::SchemaKey,
};

use self::error::Error;

/// A struct containing a postgres type and its Rust-equivalent.
#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) enum CornucopiaType {
    Simple {
        pg_ty: Type,
        rust_name: String,
        /// The borrowed counterpart type name, with explicit lifetime (e.g., `MyType<'a>`)
        borrowed_name: Option<String>,
        is_copy: bool,
    },
    Array {
        inner: Rc<CornucopiaType>,
    },
    Domain {
        pg_ty: Type,
        inner: Rc<CornucopiaType>,
    },
    Custom {
        pg_ty: Type,
        struct_name: String,
        is_copy: bool,
        is_params: bool,
    },
}

impl CornucopiaType {
    /// Is this type need a generic lifetime
    pub fn is_ref(&self) -> bool {
        match self {
            CornucopiaType::Simple {
                pg_ty:
                    Type::BYTEA | Type::TEXT | Type::VARCHAR | Type::BPCHAR | Type::JSON | Type::JSONB,
                ..
            } => false,
            CornucopiaType::Simple { pg_ty: ty, .. }
                if (ty.name() == "citext"
                    || ty.name() == "ltree"
                    || ty.name() == "lquery"
                    || ty.name() == "ltxtquery") =>
            {
                false
            }
            CornucopiaType::Simple {
                borrowed_name: Some(_),
                ..
            } => true,
            CornucopiaType::Simple { .. } => !self.is_copy(),
            CornucopiaType::Domain { inner, .. } | CornucopiaType::Array { inner } => {
                inner.is_ref()
            }
            _ => !self.is_copy(),
        }
    }

    /// Is this type copyable
    pub fn is_copy(&self) -> bool {
        match self {
            CornucopiaType::Simple { is_copy, .. } | CornucopiaType::Custom { is_copy, .. } => {
                *is_copy
            }
            CornucopiaType::Domain { inner, .. } => inner.is_copy(),
            CornucopiaType::Array { .. } => false,
        }
    }

    /// Can this used in parameters as it is
    pub fn is_params(&self) -> bool {
        match self {
            CornucopiaType::Simple { .. } => true,
            CornucopiaType::Array { .. } => false,
            CornucopiaType::Domain { inner, .. } => inner.is_params(),
            CornucopiaType::Custom { is_params, .. } => *is_params,
        }
    }

    /// Wrap type to escape domains in parameters
    pub(crate) fn sql_wrapped(&self, name: &str) -> String {
        match self {
            CornucopiaType::Domain { inner, .. } => {
                format!("&crate::Domain({})", inner.sql_wrapped(name))
            }
            CornucopiaType::Array { inner } => match inner.as_ref() {
                CornucopiaType::Domain { inner, .. } => {
                    format!("&crate::DomainArray({})", inner.sql_wrapped(name))
                }
                _ => name.to_string(),
            },
            _ => name.to_string(),
        }
    }

    /// Wrap type to escape domains when writing to sql
    pub(crate) fn accept_to_sql(&self, ctx: &GenCtx) -> String {
        match self {
            CornucopiaType::Domain { inner, .. } => {
                format!("crate::Domain::<{}>", inner.accept_to_sql(ctx))
            }
            CornucopiaType::Array { inner } => match inner.as_ref() {
                CornucopiaType::Domain { inner, .. } => {
                    let ty = inner.accept_to_sql(ctx);
                    format!("crate::DomainArray::<{ty}, &[{ty}]>")
                }
                _ => self.param_ty(false, ctx),
            },
            _ => self.param_ty(false, ctx),
        }
    }

    /// Corresponding postgres type
    pub(crate) fn pg_ty(&self) -> &Type {
        match self {
            CornucopiaType::Simple { pg_ty, .. }
            | CornucopiaType::Custom { pg_ty, .. }
            | CornucopiaType::Domain { pg_ty, .. } => pg_ty,
            CornucopiaType::Array { inner } => inner.pg_ty(),
        }
    }

    /// Code to transform its borrowed type to its owned one
    pub(crate) fn owning_call(
        &self,
        name: &str,
        is_nullable: bool,
        is_inner_nullable: bool,
    ) -> String {
        if self.is_copy() {
            return name.into();
        }

        if is_nullable {
            let into = self.owning_call("v", false, is_inner_nullable);
            return format!("{name}.map(|v| {into})");
        }

        match self {
            CornucopiaType::Simple { pg_ty, .. } if matches!(*pg_ty, Type::JSON | Type::JSONB) => {
                format!("serde_json::from_str({name}.0.get()).unwrap()")
            }
            CornucopiaType::Array { inner, .. } => {
                let inner = inner.owning_call("v", is_inner_nullable, false);
                format!("{name}.map(|v| {inner}).collect()")
            }
            CornucopiaType::Domain { inner, .. } => inner.owning_call(name, is_nullable, false),
            _ => {
                format!("{name}.into()")
            }
        }
    }

    /// Corresponding owned type
    pub(crate) fn own_ty(&self, is_inner_nullable: bool, ctx: &GenCtx) -> String {
        match self {
            CornucopiaType::Simple { rust_name, .. } => (*rust_name).to_string(),
            CornucopiaType::Array { inner, .. } => {
                let own_inner = inner.own_ty(false, ctx);
                if is_inner_nullable {
                    format!("Vec<Option<{own_inner}>>")
                } else {
                    format!("Vec<{own_inner}>")
                }
            }
            CornucopiaType::Domain { inner, .. } => inner.own_ty(false, ctx),
            CornucopiaType::Custom {
                struct_name, pg_ty, ..
            } => ctx.custom_ty_path(pg_ty.schema(), struct_name),
        }
    }

    /// Corresponding borrowed ergonomic parameter type (using traits if possible)
    pub(crate) fn param_ergo_ty(
        &self,
        is_inner_nullable: bool,
        traits: &mut Vec<String>,
        ctx: &GenCtx,
    ) -> String {
        match self {
            CornucopiaType::Simple { pg_ty, .. } => match *pg_ty {
                Type::BYTEA => {
                    traits.push("crate::BytesSql".to_string());
                    idx_char(traits.len())
                }
                Type::TEXT | Type::VARCHAR | Type::BPCHAR => {
                    traits.push("crate::StringSql".to_string());
                    idx_char(traits.len())
                }
                Type::JSON | Type::JSONB => {
                    traits.push("crate::JsonSql".to_string());
                    idx_char(traits.len())
                }
                ref ty
                    if (ty.name() == "citext"
                        || ty.name() == "ltree"
                        || ty.name() == "lquery"
                        || ty.name() == "ltxtquery") =>
                {
                    traits.push("crate::StringSql".to_string());
                    idx_char(traits.len())
                }
                _ => self.param_ty(is_inner_nullable, ctx),
            },
            CornucopiaType::Array { inner, .. } => {
                let inner = inner.param_ergo_ty(is_inner_nullable, traits, ctx);
                let inner = if is_inner_nullable {
                    format!("Option<{inner}>")
                } else {
                    inner
                };
                traits.push(format!("crate::ArraySql<Item = {inner}>"));
                idx_char(traits.len())
            }
            CornucopiaType::Domain { inner, .. } => {
                inner.param_ergo_ty(is_inner_nullable, traits, ctx)
            }
            CornucopiaType::Custom { .. } => self.param_ty(is_inner_nullable, ctx),
        }
    }

    /// Corresponding borrowed parameter type
    pub(crate) fn param_ty(&self, is_inner_nullable: bool, ctx: &GenCtx) -> String {
        match self {
            CornucopiaType::Simple {
                pg_ty,
                borrowed_name,
                ..
            } => {
                // If user explicitly provides a borrowed type, use it
                if borrowed_name.is_some() {
                    return self.brw_ty(is_inner_nullable, true, ctx);
                }
                // Otherwise use default param type based on pg_ty
                match *pg_ty {
                    Type::JSON | Type::JSONB => "&'a serde_json::value::Value".to_string(),
                    _ => self.brw_ty(is_inner_nullable, true, ctx),
                }
            }
            CornucopiaType::Array { inner, .. } => {
                let inner = inner.param_ty(is_inner_nullable, ctx);
                let inner = if is_inner_nullable {
                    format!("Option<{inner}>")
                } else {
                    inner
                };
                // Its more practical for users to use a slice
                format!("&'a [{inner}]")
            }
            CornucopiaType::Domain { inner, .. } => inner.param_ty(false, ctx),
            CornucopiaType::Custom {
                is_params,
                is_copy,
                pg_ty,
                struct_name,
                ..
            } => {
                if !is_copy && !is_params {
                    let path = ctx.custom_ty_path(pg_ty.schema(), struct_name);
                    format!("{path}Params<'a>")
                } else {
                    self.brw_ty(is_inner_nullable, true, ctx)
                }
            }
        }
    }

    /// String representing a borrowed rust equivalent of this type. Notably, if
    /// a Rust equivalent is a String or a Vec<T>, it will return a &str and a &[T] respectively.
    pub(crate) fn brw_ty(
        &self,
        is_inner_nullable: bool,
        has_lifetime: bool,
        ctx: &GenCtx,
    ) -> String {
        let lifetime = if has_lifetime { "'a" } else { "" };
        match self {
            CornucopiaType::Simple {
                pg_ty,
                rust_name,
                borrowed_name,
                ..
            } => {
                // If user explicitly provides a borrowed type, use it
                if let Some(borrowed) = borrowed_name {
                    return borrowed.clone();
                }
                // Otherwise use default borrowed type based on pg_ty
                match *pg_ty {
                    Type::BYTEA => format!("&{lifetime} [u8]"),
                    Type::TEXT | Type::VARCHAR | Type::BPCHAR => format!("&{lifetime} str"),
                    Type::JSON | Type::JSONB => {
                        format!("postgres_types::Json<&{lifetime} serde_json::value::RawValue>")
                    }
                    ref ty
                        if (ty.name() == "citext"
                            || ty.name() == "ltree"
                            || ty.name() == "lquery"
                            || ty.name() == "ltxtquery") =>
                    {
                        format!("&{lifetime} str")
                    }
                    _ => match rust_name.as_str() {
                        "String" => format!("&{lifetime} str"),
                        "Vec<u8>" => format!("&{lifetime} [u8]"),
                        _ => rust_name.to_string(),
                    },
                }
            }
            CornucopiaType::Array { inner, .. } => {
                let inner = inner.brw_ty(is_inner_nullable, has_lifetime, ctx);
                let inner = if is_inner_nullable {
                    format!("Option<{inner}>")
                } else {
                    inner
                };
                // Its more practical for users to use a slice
                let lifetime = if has_lifetime { lifetime } else { "'_" };
                format!("crate::ArrayIterator<{lifetime}, {inner}>")
            }
            CornucopiaType::Domain { inner, .. } => inner.brw_ty(false, has_lifetime, ctx),
            CornucopiaType::Custom {
                is_copy,
                pg_ty,
                struct_name,
                ..
            } => {
                let path = ctx.custom_ty_path(pg_ty.schema(), struct_name);
                if *is_copy {
                    path
                } else {
                    format!("{path}Borrowed<{lifetime}>")
                }
            }
        }
    }
}

/// Data structure holding all types known to this particular run of Cornucopia.
#[derive(Debug, Clone)]
pub(crate) struct TypeRegistrar {
    pub types: IndexMap<(String, String), Rc<CornucopiaType>>,
    pub dependency_analysis: DependencyAnalysis,
    config: Config,
}

impl TypeRegistrar {
    pub(crate) fn new(config: Config) -> Self {
        Self {
            types: IndexMap::default(),
            dependency_analysis: DependencyAnalysis::default(),
            config,
        }
    }

    /// Returns the type mapping for a specific type
    pub(crate) fn get_type_mapping(&self, ty: &Type) -> Option<&TypeMapping> {
        self.config.get_type_mapping(ty)
    }

    fn resolve_type(
        &mut self,
        ty: &Type,
        name: &str,
        query_name: &Span<String>,
        module_info: &ModuleInfo,
        default_is_copy: bool,
        default_is_params: bool,
    ) -> Result<&Rc<CornucopiaType>, Error> {
        fn custom(ty: &Type, is_copy: bool, is_params: bool) -> CornucopiaType {
            let rust_ty_name = ty.name().to_upper_camel_case();
            CornucopiaType::Custom {
                pg_ty: ty.clone(),
                struct_name: rust_ty_name,
                is_copy,
                is_params,
            }
        }

        fn domain(ty: &Type, inner: Rc<CornucopiaType>) -> CornucopiaType {
            CornucopiaType::Domain {
                pg_ty: ty.clone(),
                inner,
            }
        }

        Ok(match ty.kind() {
            Kind::Enum(_) => self.insert(ty, || custom(ty, true, true)),
            Kind::Array(inner_ty) => {
                let inner = self
                    .register(name, inner_ty, query_name, module_info)?
                    .clone();
                self.insert(ty, || CornucopiaType::Array {
                    inner: inner.clone(),
                })
            }
            Kind::Domain(inner_ty) => {
                let inner = self
                    .register(name, inner_ty, query_name, module_info)?
                    .clone();
                self.insert(ty, || domain(ty, inner.clone()))
            }
            Kind::Composite(composite_fields) => {
                let mut is_copy = default_is_copy;
                let mut is_params = default_is_params;
                for field in composite_fields {
                    let field_ty = self.register(name, field.type_(), query_name, module_info)?;
                    is_copy &= field_ty.is_copy();
                    is_params &= field_ty.is_params();
                }
                self.insert(ty, || custom(ty, is_copy, is_params))
            }
            Kind::Simple => {
                let (rust_name, is_copy) = match *ty {
                    Type::BOOL => ("bool", true),
                    Type::CHAR => ("i8", true),
                    Type::INT2 => ("i16", true),
                    Type::INT4 => ("i32", true),
                    Type::INT8 => ("i64", true),
                    Type::FLOAT4 => ("f32", true),
                    Type::FLOAT8 => ("f64", true),
                    Type::TEXT | Type::VARCHAR | Type::BPCHAR => ("String", false),
                    Type::BYTEA => ("Vec<u8>", false),
                    Type::TIMESTAMP => ("chrono::NaiveDateTime", true),
                    Type::TIMESTAMPTZ => ("chrono::DateTime<chrono::FixedOffset>", true),
                    Type::DATE => ("chrono::NaiveDate", true),
                    Type::TIME => ("chrono::NaiveTime", true),
                    Type::JSON | Type::JSONB => ("serde_json::Value", false),
                    Type::UUID => ("uuid::Uuid", true),
                    Type::INET => ("std::net::IpAddr", true),
                    Type::MACADDR => ("eui48::MacAddress", true),
                    Type::NUMERIC => ("rust_decimal::Decimal", true),
                    ref ty
                        if (ty.name() == "citext"
                            || ty.name() == "ltree"
                            || ty.name() == "lquery"
                            || ty.name() == "ltxtquery") =>
                    {
                        ("String", false)
                    }
                    _ => {
                        return Err(Error::UnsupportedPostgresType {
                            src: module_info.clone().into(),
                            query: query_name.span,
                            col_name: name.to_string(),
                            col_ty: ty.to_string(),
                        });
                    }
                };
                self.insert(ty, || CornucopiaType::Simple {
                    pg_ty: ty.clone(),
                    rust_name: rust_name.to_string(),
                    borrowed_name: None,
                    is_copy,
                })
            }
            _ => {
                return Err(Error::UnsupportedPostgresType {
                    src: module_info.clone().into(),
                    query: query_name.span,
                    col_name: name.to_string(),
                    col_ty: ty.to_string(),
                });
            }
        })
    }

    pub(crate) fn register(
        &mut self,
        name: &str,
        ty: &Type,
        query_name: &Span<String>,
        module_info: &ModuleInfo,
    ) -> Result<&Rc<CornucopiaType>, Error> {
        self.dependency_analysis.analyse(ty);

        if let Some(idx) = self.types.get_index_of(&SchemaKey::from(ty)) {
            return Ok(&self.types[idx]);
        }

        // check if there's a user-defined mapping first
        let mapping_result = if let Some(mapping) = self.config.get_type_mapping(ty) {
            match mapping {
                TypeMapping::Simple(name) => Some((name.to_string(), None, true)),
                TypeMapping::Detailed {
                    rust_type,
                    borrowed_type,
                    is_copy,
                    ..
                } => Some((rust_type.to_string(), borrowed_type.clone(), *is_copy)),
            }
        } else {
            None
        };

        if let Some((rust_name, borrowed_name, is_copy)) = mapping_result {
            return Ok(self.insert(ty, || CornucopiaType::Simple {
                pg_ty: ty.clone(),
                rust_name,
                borrowed_name,
                is_copy,
            }));
        }

        self.resolve_type(ty, name, query_name, module_info, true, true)
    }

    pub(crate) fn ref_of(&self, ty: &Type) -> Rc<CornucopiaType> {
        self.types
            .get(&SchemaKey::from(ty))
            .expect("type must already be registered")
            .clone()
    }

    fn insert(&mut self, ty: &Type, call: impl FnOnce() -> CornucopiaType) -> &Rc<CornucopiaType> {
        let index = match self
            .types
            .entry((ty.schema().to_owned(), ty.name().to_owned()))
        {
            Entry::Occupied(o) => o.index(),
            Entry::Vacant(v) => {
                let index = v.index();
                v.insert(Rc::new(call()));
                index
            }
        };
        &self.types[index]
    }
}

impl std::ops::Index<&Type> for TypeRegistrar {
    type Output = Rc<CornucopiaType>;

    fn index(&self, index: &Type) -> &Self::Output {
        &self.types[&SchemaKey::from(index)]
    }
}

pub(crate) mod error {
    use std::sync::Arc;

    use miette::{Diagnostic, NamedSource, SourceSpan};
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError, Diagnostic)]
    #[error("Couldn't register SQL type.")]
    pub enum Error {
        Db(#[from] tokio_postgres::Error),
        UnsupportedPostgresType {
            #[source_code]
            src: NamedSource<Arc<String>>,
            #[label("this query contains an unsupported type (name: {col_name}, type: {col_ty})")]
            query: SourceSpan,
            col_name: String,
            col_ty: String,
        },
    }
}
