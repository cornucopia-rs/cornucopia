use async_recursion::async_recursion;
use deadpool_postgres::Client;
use error::{Error, UnsupportedPostgresTypeError};
use heck::ToUpperCamelCase;
use indexmap::{Equivalent, IndexMap};
use postgres_types::Kind;
use tokio_postgres::types::Type;

/// A struct containing a `tokio_postgres` type and its Rust-equivalent.
#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) struct CornucopiaType {
    pub(crate) pg_ty: Type,
    pub(crate) rust_ty_name: String,
    pub(crate) rust_path_from_queries: String,
    pub(crate) rust_path_from_types: String,
}

impl CornucopiaType {
    fn new_base(pg_ty: Type, rust_ty_name: String) -> Self {
        Self {
            rust_path_from_queries: rust_ty_name.clone(),
            rust_path_from_types: rust_ty_name.clone(),
            pg_ty,
            rust_ty_name,
        }
    }
    fn new_custom(pg_ty: Type, rust_ty_name: String) -> Self {
        Self {
            rust_path_from_queries: format!(
                "super::super::types::{}::{}",
                pg_ty.schema(),
                rust_ty_name
            ),
            rust_path_from_types: format!("super::{}::{}", pg_ty.schema(), rust_ty_name),
            pg_ty,
            rust_ty_name,
        }
    }
}

impl CornucopiaType {
    /// String representing a borrowed rust equivalent of this type. Notably, if
    /// a Rust equivalent is a String or a Vec<T>, it will return a &str and a &[T] respectively.
    pub(crate) fn borrowed_rust_ty(&self) -> String {
        if self.rust_path_from_queries == "String" {
            String::from("str")
        } else if self.rust_path_from_queries.starts_with("Vec<") {
            format!(
                "[{}]",
                String::from(
                    &self.rust_path_from_queries[4..self.rust_path_from_queries.len() - 1]
                )
            )
        } else {
            format!("{}", self.rust_path_from_queries)
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
    #[async_recursion]
    pub(crate) async fn register(
        &mut self,
        client: &Client,
        ty: &Type,
    ) -> Result<&CornucopiaType, Error> {
        if let Some(cty) = self.get_by_index(ty) {
            return match cty {
                TypeVariant::Base(index) => Ok(&self.base_types[index]),
                TypeVariant::Custom(index) => Ok(&self.custom_types[index]),
            };
        };

        Ok(match ty.kind() {
            Kind::Enum(_) => self.insert_custom(ty.clone(), ty.name().to_upper_camel_case()),
            Kind::Array(array_inner_ty) => {
                let a_rust_ty_name = &self
                    .register(client, array_inner_ty)
                    .await?
                    .rust_path_from_queries;
                let rust_ty_name = format!("Vec<{}>", a_rust_ty_name);
                self.insert_base(ty.clone(), rust_ty_name)
            }
            Kind::Domain(domain_inner_ty) => {
                self.register(client, domain_inner_ty).await?;
                self.insert_custom(ty.clone(), ty.name().to_upper_camel_case())
            }
            Kind::Composite(composite_fields) => {
                for field in composite_fields {
                    self.register(client, field.type_()).await?;
                }
                self.insert_custom(ty.clone(), ty.name().to_upper_camel_case())
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

    fn insert_base(&mut self, ty: Type, rust_ty_name: String) -> &CornucopiaType {
        let index = match self
            .base_types
            .entry((ty.schema().to_owned(), ty.name().to_owned()))
        {
            indexmap::map::Entry::Occupied(o) => o.index(),
            indexmap::map::Entry::Vacant(v) => {
                let index = v.index();
                v.insert(CornucopiaType::new_base(ty, rust_ty_name));
                index
            }
        };

        &self.base_types[index]
    }

    fn insert_custom(&mut self, ty: Type, rust_ty_name: String) -> &CornucopiaType {
        let index = match self
            .custom_types
            .entry((ty.schema().to_owned(), ty.name().to_owned()))
        {
            indexmap::map::Entry::Occupied(o) => o.index(),
            indexmap::map::Entry::Vacant(v) => {
                let index = v.index();
                v.insert(CornucopiaType::new_custom(ty, rust_ty_name));
                index
            }
        };

        &self.custom_types[index]
    }
}

impl<const N: usize> From<[(Type, &'static str); N]> for TypeRegistrar {
    fn from(base_types: [(Type, &'static str); N]) -> Self {
        let mut registrar = Self::new();
        for (ty, rust_ty_name) in base_types {
            registrar.insert_base(ty, rust_ty_name.to_owned());
        }
        registrar
    }
}

impl Default for TypeRegistrar {
    fn default() -> Self {
        TypeRegistrar::from([
            (Type::BOOL, "bool"),
            (Type::CHAR, "i8"),
            (Type::INT2, "i16"),
            (Type::INT4, "i32"),
            (Type::INT8, "i64"),
            (Type::FLOAT4, "f32"),
            (Type::FLOAT8, "f64"),
            (Type::TEXT, "String"),
            (Type::VARCHAR, "String"),
            (Type::BYTEA, "Vec<u8>"),
            (Type::TIMESTAMP, "time::PrimitiveDateTime"),
            (Type::TIMESTAMPTZ, "time::OffsetDateTime"),
            (Type::DATE, "time::Date"),
            (Type::TIME, "time::Time"),
            (Type::JSON, "serde_json::Value"),
            (Type::JSONB, "serde_json::Value"),
            (Type::UUID, "uuid::Uuid"),
            (Type::INET, "std::net::IpAddr"),
            (Type::MACADDR, "eui48::MacAddress"),
            (Type::BOOL_ARRAY, "Vec<bool>"),
            (Type::CHAR_ARRAY, "Vec<i8>"),
            (Type::INT2_ARRAY, "Vec<i16>"),
            (Type::INT4_ARRAY, "Vec<i32>"),
            (Type::INT8_ARRAY, "Vec<i64>"),
            (Type::FLOAT4_ARRAY, "Vec<f32>"),
            (Type::FLOAT8_ARRAY, "Vec<f64>"),
            (Type::TEXT_ARRAY, "Vec<String>"),
            (Type::VARCHAR_ARRAY, "Vec<String>"),
            (Type::BYTEA_ARRAY, "Vec<Vec<u8>>"),
            (Type::TIMESTAMP_ARRAY, "Vec<time::PrimitiveDateTime>"),
            (Type::TIMESTAMPTZ_ARRAY, "Vec<time::OffsetDateTime>"),
            (Type::DATE_ARRAY, "Vec<time::Date>"),
            (Type::TIME_ARRAY, "Vec<time::Time>"),
            (Type::JSON_ARRAY, "Vec<serde_json::Value>"),
            (Type::JSONB_ARRAY, "Vec<serde_json::Value>"),
            (Type::UUID_ARRAY, "Vec<uuid::Uuid>"),
            (Type::INET_ARRAY, "Vec<std::net::IpAddr>"),
            (Type::MACADDR_ARRAY, "Vec<eui48::MacAddress>"),
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
        Db(#[from] tokio_postgres::Error),
        UnsupportedPostgresType(#[from] UnsupportedPostgresTypeError),
    }
}
