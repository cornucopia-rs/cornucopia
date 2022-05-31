use std::borrow::Cow;

use error::{Error, UnsupportedPostgresTypeError};
use heck::ToUpperCamelCase;
use indexmap::{Equivalent, IndexMap};
use postgres_types::{Kind, Type};

/// A struct containing a postgres type and its Rust-equivalent.
#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) struct CornucopiaType {
    pub(crate) pg_ty: Type,
    pub(crate) rust_ty_name: Cow<'static, str>,
    pub(crate) rust_path_from_queries: Cow<'static, str>,
    pub(crate) rust_path_from_types: Cow<'static, str>,
    pub(crate) is_copy: bool,
    pub(crate) is_params: bool,
}

impl CornucopiaType {
    fn new_base(
        pg_ty: Type,
        rust_ty_name: Cow<'static, str>,
        is_copy: bool,
        is_params: bool,
    ) -> Self {
        Self {
            rust_path_from_queries: rust_ty_name.clone(),
            rust_path_from_types: rust_ty_name.clone(),
            pg_ty,
            rust_ty_name,
            is_copy,
            is_params,
        }
    }

    fn new_custom(pg_ty: Type, rust_ty_name: String, is_copy: bool, is_params: bool) -> Self {
        Self {
            rust_path_from_queries: format!(
                "super::super::types::{}::{}",
                pg_ty.schema(),
                rust_ty_name
            )
            .into(),
            rust_path_from_types: format!("super::{}::{}", pg_ty.schema(), rust_ty_name).into(),
            pg_ty,
            rust_ty_name: rust_ty_name.into(),
            is_copy,
            is_params,
        }
    }
}

impl CornucopiaType {
    pub(crate) fn owning_call(&self, var_name: &str, is_nullable: bool) -> String {
        if self.is_copy {
            return "".into();
        }

        fn from_json(name: &str) -> String {
            format!("postgres_types::Json(serde_json::from_str({name}.0.get()).unwrap())")
        }

        match self.pg_ty.kind() {
            Kind::Array(inner) => {
                let into = if matches!(*inner, Type::JSON | Type::JSONB) {
                    from_json("v")
                } else {
                    "v.into()".to_string()
                };

                if is_nullable {
                    format!("{var_name}.map(|v| v.map(|v| {into}).collect())")
                } else {
                    format!("{var_name}.map(|v| {into}).collect()")
                }
            }
            Kind::Domain(_) | Kind::Composite(_) => format!("{var_name}.into()"),
            _ => {
                if is_nullable {
                    if matches!(self.pg_ty, Type::JSON | Type::JSONB) {
                        format!("{var_name}.map(|v| {}).unwrap()))", from_json("v"))
                    } else {
                        format!("{var_name}.map(|v| v.into())")
                    }
                } else if matches!(self.pg_ty, Type::JSON | Type::JSONB) {
                    from_json(var_name)
                } else {
                    format!("{var_name}.into()")
                }
            }
        }
    }
    /// String representing a borrowed rust equivalent of this type. Notably, if
    /// a Rust equivalent is a String or a Vec<T>, it will return a &str and a &[T] respectively.
    pub(crate) fn borrowed_rust_ty(
        &self,
        type_registrar: &TypeRegistrar,
        lifetime: Option<&'static str>,
        is_param: bool,
    ) -> String {
        // Special case for copy types
        if self.is_copy {
            return self.rust_path_from_queries.to_string();
        }

        let lifetime = lifetime.unwrap_or("'a");

        // Special case for domains and composites
        if matches!(self.pg_ty.kind(), Kind::Domain(_) | Kind::Composite(_)) {
            return if is_param && !self.is_params {
                format!("{}Params<{lifetime}>", self.rust_path_from_queries)
            } else {
                format!("{}Borrowed<{lifetime}>", self.rust_path_from_queries)
            };
        }

        // Special case for PostgreSQL arrays
        if let Kind::Array(inner_ty) = self.pg_ty.kind() {
            let inner_ty = type_registrar.get(inner_ty).unwrap();

            // Its more practical for users to use a slice
            if is_param {
                return format!(
                    "&{lifetime} [{}]",
                    inner_ty.borrowed_rust_ty(type_registrar, Some(lifetime), is_param)
                );
            } else {
                return format!(
                    "cornucopia_client::ArrayIterator<{lifetime}, {}>",
                    inner_ty.borrowed_rust_ty(type_registrar, Some(lifetime), is_param)
                );
            }
        }

        // Special case for non copy simple types
        match self.pg_ty {
            Type::BYTEA => format!("&{lifetime} [u8]"),
            Type::TEXT | Type::VARCHAR => format!("&{lifetime} str"),
            Type::JSON | Type::JSONB => {
                format!("postgres_types::Json<&{lifetime} serde_json::value::RawValue>")
            }
            _ => unreachable!(),
        }
    }
}

/// Allows us to query a hashmap without having to own the key strings
#[derive(PartialEq, Eq, Hash)]
struct TypeRegistrarKey<'a> {
    schema: &'a str,
    name: &'a str,
}

impl<'a> From<&'a Type> for TypeRegistrarKey<'a> {
    fn from(ty: &'a Type) -> Self {
        TypeRegistrarKey {
            schema: ty.schema(),
            name: ty.name(),
        }
    }
}

impl<'a> Equivalent<(String, String)> for TypeRegistrarKey<'a> {
    fn equivalent(&self, key: &(String, String)) -> bool {
        key.0.as_str().equivalent(&self.schema) && key.1.as_str().equivalent(&self.name)
    }
}

/// Data structure holding all types known to this particular run of Cornucopia.
#[derive(Debug, Clone, Default)]
pub(crate) struct TypeRegistrar {
    pub types: IndexMap<(String, String), CornucopiaType>,
}

impl TypeRegistrar {
    pub(crate) fn register<'a>(&'a mut self, ty: &Type) -> Result<&'a CornucopiaType, Error> {
        if let Some(idx) = self.types.get_index_of(&TypeRegistrarKey::from(ty)) {
            return Ok(&self.types[idx]);
        }

        Ok(match ty.kind() {
            Kind::Enum(_) => {
                self.insert_custom(ty.clone(), ty.name().to_upper_camel_case(), true, true)
            }
            Kind::Array(inner_ty) => {
                let a_rust_ty_name = &self.register(inner_ty)?.rust_path_from_queries;
                let rust_ty_name = format!("Vec<{}>", a_rust_ty_name);
                self.insert_base(ty.clone(), rust_ty_name.into(), false, false)
            }
            Kind::Domain(inner_ty) => {
                let inner = self.register(inner_ty)?;
                let (is_copy, is_params) = (inner.is_copy, inner.is_params);
                self.insert_custom(
                    ty.clone(),
                    ty.name().to_upper_camel_case(),
                    is_copy,
                    is_params,
                )
            }
            Kind::Composite(composite_fields) => {
                let mut is_copy = true;
                let mut is_params = true;
                for field in composite_fields {
                    let field_ty = self.register(field.type_())?;
                    is_copy &= field_ty.is_copy;
                    is_params &= field_ty.is_params;
                }
                self.insert_custom(
                    ty.clone(),
                    ty.name().to_upper_camel_case(),
                    is_copy,
                    is_params,
                )
            }
            Kind::Simple => {
                let (name, is_copy) = match *ty {
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
                    Type::JSON | Type::JSONB => ("postgres_types::Json<serde_json::Value>", false),
                    Type::UUID => ("uuid::Uuid", true),
                    Type::INET => ("std::net::IpAddr", true),
                    Type::MACADDR => ("eui48::MacAddress", true),
                    _ => {
                        return Err(Error::UnsupportedPostgresType(
                            UnsupportedPostgresTypeError {
                                name: ty.name().to_owned(),
                            },
                        ))
                    }
                };
                self.insert_base(ty.clone(), name.into(), is_copy, true)
            }
            _ => {
                return Err(Error::UnsupportedPostgresType(
                    UnsupportedPostgresTypeError {
                        name: ty.name().to_owned(),
                    },
                ))
            }
        })
    }

    pub(crate) fn get(&self, ty: &Type) -> Option<&CornucopiaType> {
        self.types.get(&TypeRegistrarKey::from(ty))
    }

    fn insert_base(
        &mut self,
        ty: Type,
        rust_ty_name: Cow<'static, str>,
        is_copy: bool,
        is_params: bool,
    ) -> &CornucopiaType {
        let index = match self
            .types
            .entry((ty.schema().to_owned(), ty.name().to_owned()))
        {
            indexmap::map::Entry::Occupied(o) => o.index(),
            indexmap::map::Entry::Vacant(v) => {
                let index = v.index();
                v.insert(CornucopiaType::new_base(
                    ty,
                    rust_ty_name,
                    is_copy,
                    is_params,
                ));
                index
            }
        };

        &self.types[index]
    }

    fn insert_custom(
        &mut self,
        ty: Type,
        rust_ty_name: String,
        is_copy: bool,
        is_params: bool,
    ) -> &CornucopiaType {
        let index = match self
            .types
            .entry((ty.schema().to_owned(), ty.name().to_owned()))
        {
            indexmap::map::Entry::Occupied(o) => o.index(),
            indexmap::map::Entry::Vacant(v) => {
                let index = v.index();
                v.insert(CornucopiaType::new_custom(
                    ty,
                    rust_ty_name,
                    is_copy,
                    is_params,
                ));
                index
            }
        };

        &self.types[index]
    }
}

pub(crate) mod error {
    use thiserror::Error as ThisError;
    #[derive(Debug, ThisError)]
    #[error("Unsupported type `{name}`")]
    pub(crate) struct UnsupportedPostgresTypeError {
        pub(crate) name: String,
    }

    #[derive(Debug, ThisError)]
    #[error("{0}")]
    pub(crate) enum Error {
        Db(#[from] postgres::Error),
        UnsupportedPostgresType(#[from] UnsupportedPostgresTypeError),
    }
}
