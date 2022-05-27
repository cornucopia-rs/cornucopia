use error::{Error, UnsupportedPostgresTypeError};
use heck::ToUpperCamelCase;
use indexmap::{Equivalent, IndexMap};
use postgres::Client;
use postgres_types::{Kind, Type};

/// A struct containing a postgres type and its Rust-equivalent.
#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) struct CornucopiaType {
    pub(crate) pg_ty: Type,
    pub(crate) rust_ty_name: String,
    pub(crate) rust_path_from_queries: String,
    pub(crate) rust_path_from_types: String,
    pub(crate) is_copy: bool,
}

impl CornucopiaType {
    fn new_base(pg_ty: Type, rust_ty_name: String, is_copy: bool) -> Self {
        Self {
            rust_path_from_queries: rust_ty_name.clone(),
            rust_path_from_types: rust_ty_name.clone(),
            pg_ty,
            rust_ty_name,
            is_copy,
        }
    }
    fn new_custom(pg_ty: Type, rust_ty_name: String, is_copy: bool) -> Self {
        Self {
            rust_path_from_queries: format!(
                "super::super::types::{}::{}",
                pg_ty.schema(),
                rust_ty_name
            ),
            rust_path_from_types: format!("super::{}::{}", pg_ty.schema(), rust_ty_name),
            pg_ty,
            rust_ty_name,
            is_copy,
        }
    }
}

impl CornucopiaType {
    pub(crate) fn owning_call(&self, var_name: &str, is_nullable: bool) -> String {
        if self.is_copy {
            return "".into();
        }

        if self.rust_path_from_queries == "postgres_types::Json<serde_json::Value>" {
            return format!(
                "postgres_types::Json(serde_json::from_str({var_name}.0.get()).unwrap())"
            );
        }

        match self.pg_ty.kind() {
            Kind::Array(_) => {
                if is_nullable {
                    format!("{var_name}.map(|v| v.map(|v| v.into()).collect())")
                } else {
                    format!("{var_name}.map(|v| v.into()).collect()")
                }
            }
            Kind::Domain(_) | Kind::Composite(_) => format!("{var_name}.into()"),
            _ => {
                if is_nullable {
                    format!("{var_name}.map(|v| v.into())")
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
            return self.rust_path_from_queries.to_owned();
        }
        // Special case for byte arrays
        if self.rust_path_from_queries == "Vec<u8>" {
            return format!("&{} [u8]", lifetime.unwrap_or(""));
        }
        // Special case for domains and composites
        if matches!(self.pg_ty.kind(), Kind::Domain(_) | Kind::Composite(_)) {
            return if is_param {
                format!(
                    "{}Params<{}>",
                    self.rust_path_from_queries,
                    lifetime.unwrap_or("'a")
                )
            } else {
                format!(
                    "{}Borrowed<{}>",
                    self.rust_path_from_queries,
                    lifetime.unwrap_or("'a")
                )
            };
        }

        // Special case for PostgreSQL arrays
        if let Kind::Array(inner_ty) = self.pg_ty.kind() {
            let inner_ty = type_registrar.get(inner_ty).unwrap();

            // Its more practical for users to use a slice
            if is_param {
                return format!(
                    "&{} [{}]",
                    lifetime.unwrap_or("'a"),
                    inner_ty.borrowed_rust_ty(type_registrar, lifetime, is_param)
                );
            } else {
                return format!(
                    "cornucopia_client::ArrayIterator<{}, {}>",
                    lifetime.unwrap_or("'a"),
                    inner_ty.borrowed_rust_ty(type_registrar, lifetime, is_param)
                );
            }
        }

        // Simple checks
        match self.rust_path_from_queries.as_str() {
            "String" => {
                format!("&{} str", lifetime.unwrap_or(""))
            }
            "postgres_types::Json<serde_json::Value>" => {
                format!(
                    "postgres_types::Json<&{} serde_json::value::RawValue>",
                    lifetime.unwrap_or("")
                )
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
pub(crate) struct TypeRegistrar {
    base_types: IndexMap<(String, String), CornucopiaType>,
    pub(crate) custom_types: IndexMap<(String, String), CornucopiaType>,
}

enum TypeVariant {
    Base(usize),
    Custom(usize),
}

impl TypeRegistrar {
    pub(crate) fn register(
        &mut self,
        client: &mut Client,
        ty: &Type,
    ) -> Result<&CornucopiaType, Error> {
        if let Some(cty) = self.get_by_index(ty) {
            return match cty {
                TypeVariant::Base(index) => Ok(&self.base_types[index]),
                TypeVariant::Custom(index) => Ok(&self.custom_types[index]),
            };
        };

        Ok(match ty.kind() {
            Kind::Enum(_) => self.insert_custom(ty.clone(), ty.name().to_upper_camel_case(), true),
            Kind::Array(array_inner_ty) => {
                let a_rust_ty_name = &self
                    .register(client, array_inner_ty)?
                    .rust_path_from_queries;
                let rust_ty_name = format!("Vec<{}>", a_rust_ty_name);
                self.insert_base(ty.clone(), rust_ty_name, false)
            }
            Kind::Domain(domain_inner_ty) => {
                let inner_is_copy = self.register(client, domain_inner_ty)?.is_copy;
                self.insert_custom(ty.clone(), ty.name().to_upper_camel_case(), inner_is_copy)
            }
            Kind::Composite(composite_fields) => {
                let mut is_copy = true;
                for field in composite_fields {
                    let field_ty = self.register(client, field.type_())?;
                    is_copy = is_copy && field_ty.is_copy;
                }
                self.insert_custom(ty.clone(), ty.name().to_upper_camel_case(), is_copy)
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
        self.base_types
            .get(&TypeRegistrarKey::from(ty))
            .or_else(|| self.custom_types.get(&TypeRegistrarKey::from(ty)))
    }

    fn get_by_index(&self, ty: &Type) -> Option<TypeVariant> {
        self.base_types
            .get_index_of(&TypeRegistrarKey::from(ty))
            .map(TypeVariant::Base)
            .or_else(|| {
                self.custom_types
                    .get_index_of(&TypeRegistrarKey::from(ty))
                    .map(TypeVariant::Custom)
            })
    }

    fn new() -> Self {
        Self {
            base_types: IndexMap::new(),
            custom_types: IndexMap::new(),
        }
    }

    fn insert_base(&mut self, ty: Type, rust_ty_name: String, is_copy: bool) -> &CornucopiaType {
        let index = match self
            .base_types
            .entry((ty.schema().to_owned(), ty.name().to_owned()))
        {
            indexmap::map::Entry::Occupied(o) => o.index(),
            indexmap::map::Entry::Vacant(v) => {
                let index = v.index();
                v.insert(CornucopiaType::new_base(ty, rust_ty_name, is_copy));
                index
            }
        };

        &self.base_types[index]
    }

    fn insert_custom(&mut self, ty: Type, rust_ty_name: String, is_copy: bool) -> &CornucopiaType {
        let index = match self
            .custom_types
            .entry((ty.schema().to_owned(), ty.name().to_owned()))
        {
            indexmap::map::Entry::Occupied(o) => o.index(),
            indexmap::map::Entry::Vacant(v) => {
                let index = v.index();
                v.insert(CornucopiaType::new_custom(ty, rust_ty_name, is_copy));
                index
            }
        };

        &self.custom_types[index]
    }
}

impl<const N: usize> From<[(Type, &'static str, bool); N]> for TypeRegistrar {
    fn from(base_types: [(Type, &'static str, bool); N]) -> Self {
        let mut registrar = Self::new();
        for (ty, rust_ty_name, is_copy) in base_types {
            registrar.insert_base(ty, rust_ty_name.to_owned(), is_copy);
        }
        registrar
    }
}

impl Default for TypeRegistrar {
    fn default() -> Self {
        TypeRegistrar::from([
            (Type::BOOL, "bool", true),
            (Type::CHAR, "i8", true),
            (Type::INT2, "i16", true),
            (Type::INT4, "i32", true),
            (Type::INT8, "i64", true),
            (Type::FLOAT4, "f32", true),
            (Type::FLOAT8, "f64", true),
            (Type::TEXT, "String", false),
            (Type::VARCHAR, "String", false),
            (Type::BYTEA, "Vec<u8>", false),
            (Type::TIMESTAMP, "time::PrimitiveDateTime", true),
            (Type::TIMESTAMPTZ, "time::OffsetDateTime", true),
            (Type::DATE, "time::Date", true),
            (Type::TIME, "time::Time", true),
            (Type::JSON, "postgres_types::Json<serde_json::Value>", false),
            (
                Type::JSONB,
                "postgres_types::Json<serde_json::Value>",
                false,
            ),
            (Type::UUID, "uuid::Uuid", true),
            (Type::INET, "std::net::IpAddr", true),
            (Type::MACADDR, "eui48::MacAddress", true),
            (Type::BOOL_ARRAY, "Vec<bool>", false),
            (Type::CHAR_ARRAY, "Vec<i8>", false),
            (Type::INT2_ARRAY, "Vec<i16>", false),
            (Type::INT4_ARRAY, "Vec<i32>", false),
            (Type::INT8_ARRAY, "Vec<i64>", false),
            (Type::FLOAT4_ARRAY, "Vec<f32>", false),
            (Type::FLOAT8_ARRAY, "Vec<f64>", false),
            (Type::TEXT_ARRAY, "Vec<String>", false),
            (Type::VARCHAR_ARRAY, "Vec<String>", false),
            (Type::BYTEA_ARRAY, "Vec<Vec<u8>>", false),
            (Type::TIMESTAMP_ARRAY, "Vec<time::PrimitiveDateTime>", false),
            (Type::TIMESTAMPTZ_ARRAY, "Vec<time::OffsetDateTime>", false),
            (Type::DATE_ARRAY, "Vec<time::Date>", false),
            (Type::TIME_ARRAY, "Vec<time::Time>", false),
            (
                Type::JSON_ARRAY,
                "Vec<postgres_types::Json<serde_json::Value>>",
                false,
            ),
            (
                Type::JSON_ARRAY,
                "Vec<postgres_types::Json<serde_json::Value>>",
                false,
            ),
            (Type::UUID_ARRAY, "Vec<uuid::Uuid>", false),
            (Type::INET_ARRAY, "Vec<std::net::IpAddr>", false),
            (Type::MACADDR_ARRAY, "Vec<eui48::MacAddress>", false),
        ])
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
