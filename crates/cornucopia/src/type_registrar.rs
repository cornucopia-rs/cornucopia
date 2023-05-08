use std::rc::Rc;

use heck::ToUpperCamelCase;
use indexmap::{map::Entry, IndexMap};
use postgres_types::{Kind, Type};

use crate::{
    codegen::{idx_char, GenCtx},
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
        rust_name: &'static str,
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
            CornucopiaType::Simple { pg_ty, .. } => match *pg_ty {
                Type::BYTEA | Type::TEXT | Type::VARCHAR | Type::JSON | Type::JSONB => false,
                _ => !self.is_copy(),
            },
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
    pub(crate) fn sql_wrapped(&self, name: &str, ctx: &GenCtx) -> String {
        let client_name = ctx.client_name();
        match self {
            CornucopiaType::Domain { inner, .. } => {
                format!(
                    "&{client_name}::private::Domain({})",
                    inner.sql_wrapped(name, ctx)
                )
            }
            CornucopiaType::Array { inner } => match inner.as_ref() {
                CornucopiaType::Domain { inner, .. } => {
                    format!(
                        "&{client_name}::private::DomainArray({})",
                        inner.sql_wrapped(name, ctx)
                    )
                }
                _ => name.to_string(),
            },
            _ => name.to_string(),
        }
    }

    /// Wrap type to escape domains when writing to sql
    pub(crate) fn accept_to_sql(&self, ctx: &GenCtx) -> String {
        let client_name = ctx.client_name();
        match self {
            CornucopiaType::Domain { inner, .. } => format!(
                "{client_name}::private::Domain::<{}>",
                inner.accept_to_sql(ctx)
            ),
            CornucopiaType::Array { inner } => match inner.as_ref() {
                CornucopiaType::Domain { inner, .. } => {
                    let ty = inner.accept_to_sql(ctx);
                    format!("{client_name}::private::DomainArray::<{ty}, &[{ty}]>")
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
            } => custom_ty_path(pg_ty.schema(), struct_name, ctx),
        }
    }

    /// Corresponding borrowed ergonomic parameter type (using traits if possible)
    pub(crate) fn param_ergo_ty(
        &self,
        is_inner_nullable: bool,
        traits: &mut Vec<String>,
        ctx: &GenCtx,
    ) -> String {
        let client_name = "crate::client";
        match self {
            CornucopiaType::Simple { pg_ty, .. } => match *pg_ty {
                Type::BYTEA => {
                    traits.push(format!("{client_name}::BytesSql"));
                    idx_char(traits.len())
                }
                Type::TEXT | Type::VARCHAR => {
                    traits.push(format!("{client_name}::StringSql"));
                    idx_char(traits.len())
                }
                Type::JSON | Type::JSONB => {
                    traits.push(format!("{client_name}::JsonSql"));
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
                traits.push(format!("{client_name}::ArraySql<Item = {inner}>"));
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
            CornucopiaType::Simple { pg_ty, .. } => match *pg_ty {
                Type::JSON | Type::JSONB => "&'a serde_json::value::Value".to_string(),
                _ => self.brw_ty(is_inner_nullable, true, ctx),
            },
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
                    let path = custom_ty_path(pg_ty.schema(), struct_name, ctx);
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
                pg_ty, rust_name, ..
            } => match *pg_ty {
                Type::BYTEA => format!("&{lifetime} [u8]"),
                Type::TEXT | Type::VARCHAR => format!("&{lifetime} str"),
                Type::JSON | Type::JSONB => {
                    format!("postgres_types::Json<&{lifetime} serde_json::value::RawValue>")
                }
                _ => (*rust_name).to_string(),
            },
            CornucopiaType::Array { inner, .. } => {
                let inner = inner.brw_ty(is_inner_nullable, has_lifetime, ctx);
                let inner = if is_inner_nullable {
                    format!("Option<{inner}>")
                } else {
                    inner
                };
                // Its more practical for users to use a slice
                let lifetime = if has_lifetime { lifetime } else { "'_" };
                let client_name = "crate::client";
                format!("{client_name}::ArrayIterator<{lifetime}, {inner}>")
            }
            CornucopiaType::Domain { inner, .. } => inner.brw_ty(false, has_lifetime, ctx),
            CornucopiaType::Custom {
                is_copy,
                pg_ty,
                struct_name,
                ..
            } => {
                let path = custom_ty_path(pg_ty.schema(), struct_name, ctx);
                if *is_copy {
                    path
                } else {
                    format!("{path}Borrowed<{lifetime}>")
                }
            }
        }
    }
}

pub fn custom_ty_path(schema: &str, struct_name: &str, ctx: &GenCtx) -> String {
    if ctx.depth == 0 {
        format!("{schema}::{struct_name}")
    } else if ctx.depth == 1 {
        format!("super::{schema}::{struct_name}")
    } else {
        ctx.path(ctx.depth, format_args!("types::{schema}::{struct_name}"))
    }
}

/// Data structure holding all types known to this particular run of Cornucopia.
#[derive(Debug, Clone, Default)]
pub(crate) struct TypeRegistrar {
    pub types: IndexMap<(String, String), Rc<CornucopiaType>>,
}

impl TypeRegistrar {
    pub(crate) fn register(
        &mut self,
        name: &str,
        ty: &Type,
        query_name: &Span<String>,
        module_info: &ModuleInfo,
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

        if let Some(idx) = self.types.get_index_of(&SchemaKey::from(ty)) {
            return Ok(&self.types[idx]);
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
                let mut is_copy = true;
                let mut is_params = true;
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
                    Type::TEXT | Type::VARCHAR => ("String", false),
                    Type::BYTEA => ("Vec<u8>", false),
                    Type::TIMESTAMP => ("time::PrimitiveDateTime", true),
                    Type::TIMESTAMPTZ => ("time::OffsetDateTime", true),
                    Type::DATE => ("time::Date", true),
                    Type::TIME => ("time::Time", true),
                    Type::JSON | Type::JSONB => ("serde_json::Value", false),
                    Type::UUID => ("uuid::Uuid", true),
                    Type::INET => ("std::net::IpAddr", true),
                    Type::MACADDR => ("eui48::MacAddress", true),
                    Type::NUMERIC => ("rust_decimal::Decimal", true),
                    _ => {
                        return Err(Error::UnsupportedPostgresType {
                            src: module_info.clone().into(),
                            query: query_name.span,
                            col_name: name.to_string(),
                            col_ty: ty.to_string(),
                        })
                    }
                };
                self.insert(ty, || CornucopiaType::Simple {
                    pg_ty: ty.clone(),
                    rust_name,
                    is_copy,
                })
            }
            _ => {
                return Err(Error::UnsupportedPostgresType {
                    src: module_info.clone().into(),
                    query: query_name.span,
                    col_name: name.to_string(),
                    col_ty: ty.to_string(),
                })
            }
        })
    }

    pub(crate) fn ref_of(&self, ty: &Type) -> Rc<CornucopiaType> {
        self.types
            .get(&SchemaKey::from(ty))
            .expect("type must already be registered")
            .clone()
    }

    fn insert(&mut self, ty: &Type, call: impl Fn() -> CornucopiaType) -> &Rc<CornucopiaType> {
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
    use miette::{Diagnostic, NamedSource, SourceSpan};
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError, Diagnostic)]
    #[error("Couldn't register SQL type.")]
    pub enum Error {
        Db(#[from] postgres::Error),
        UnsupportedPostgresType {
            #[source_code]
            src: NamedSource,
            #[label("this query contains an unsupported type (name: {col_name}, type: {col_ty})")]
            query: SourceSpan,
            col_name: String,
            col_ty: String,
        },
    }
}
