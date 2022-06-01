use error::{Error, UnsupportedPostgresTypeError};
use heck::ToUpperCamelCase;
use indexmap::{Equivalent, IndexMap};
use postgres_types::{Kind, Type};

use crate::prepare_queries::PreparedField;

/// A struct containing a postgres type and its Rust-equivalent.
#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) enum CornucopiaType {
    Simple {
        pg_ty: Type,
        rust_name: &'static str,
        is_copy: bool,
    },
    Array(Type),
    Custom {
        pg_ty: Type,
        // TODO Maybe a hashmap if different module declare it, should we allow it ?
        // TODO an alternative would be to use a different type registrar per module
        // which will naturally isolate modules nullability declarations
        content: Box<CustomContent>,
        struct_name: String,
        struct_path: String,
        is_copy: bool,
        is_params: bool,
    },
}
#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) enum CustomContent {
    Enum(Vec<String>),
    Domain(PreparedField),
    Composite(Vec<PreparedField>),
}

impl CornucopiaType {
    pub fn is_copy(&self) -> bool {
        match self {
            CornucopiaType::Simple { is_copy, .. } | CornucopiaType::Custom { is_copy, .. } => {
                *is_copy
            }
            CornucopiaType::Array(_) => false,
        }
    }

    pub fn is_params(&self) -> bool {
        match self {
            CornucopiaType::Simple { .. } => true,
            CornucopiaType::Array { .. } => false,
            CornucopiaType::Custom { is_params, .. } => *is_params,
        }
    }

    pub(crate) fn owning_call(
        &self,
        var_name: &str,
        is_nullable: bool,
        is_inner_nullable: bool,
    ) -> String {
        if self.is_copy() {
            return var_name.into();
        }

        fn from_json(name: &str) -> String {
            format!("postgres_types::Json(serde_json::from_str({name}.0.get()).unwrap())")
        }

        match self {
            CornucopiaType::Simple { pg_ty, .. } => {
                if is_nullable {
                    if matches!(*pg_ty, Type::JSON | Type::JSONB) {
                        format!("{var_name}.map(|v| {}).unwrap()))", from_json("v"))
                    } else {
                        format!("{var_name}.map(|v| v.into())")
                    }
                } else if matches!(*pg_ty, Type::JSON | Type::JSONB) {
                    from_json(var_name)
                } else {
                    format!("{var_name}.into()")
                }
            }
            CornucopiaType::Array(inner_ty) => {
                let into = if matches!(*inner_ty, Type::JSON | Type::JSONB) {
                    from_json("v")
                } else {
                    "v.into()".to_string()
                };
                let inner = if is_inner_nullable {
                    format!("v.map(|v| {into})")
                } else {
                    into
                };

                if is_nullable {
                    format!("{var_name}.map(|v| v.map(|v| {inner}).collect())")
                } else {
                    format!("{var_name}.map(|v| {inner}).collect()")
                }
            }
            CornucopiaType::Custom { .. } => {
                if is_nullable {
                    format!("{var_name}.map(|v| v.into())")
                } else {
                    format!("{var_name}.into()")
                }
            }
        }
    }

    pub(crate) fn own_struct(
        &self,
        type_registrar: &TypeRegistrar,
        is_inner_nullable: bool,
    ) -> String {
        match self {
            CornucopiaType::Simple { rust_name, .. } => rust_name.to_string(),
            CornucopiaType::Array(inner_ty) => {
                let inner = type_registrar
                    .get(inner_ty)
                    .unwrap()
                    .own_struct(type_registrar, false);

                let inner = if is_inner_nullable {
                    format!("Option<{inner}>")
                } else {
                    inner
                };
                format!("Vec<{inner}>")
            }
            CornucopiaType::Custom { struct_path, .. } => struct_path.to_string(),
        }
    }

    /// String representing a borrowed rust equivalent of this type. Notably, if
    /// a Rust equivalent is a String or a Vec<T>, it will return a &str and a &[T] respectively.
    pub(crate) fn brw_struct(
        &self,
        type_registrar: &TypeRegistrar,
        for_params: bool,
        is_inner_nullable: bool,
    ) -> String {
        let lifetime = "'a";
        match self {
            CornucopiaType::Simple {
                pg_ty, rust_name, ..
            } => match *pg_ty {
                Type::BYTEA => format!("&{lifetime} [u8]"),
                Type::TEXT | Type::VARCHAR => format!("&{lifetime} str"),
                Type::JSON | Type::JSONB => {
                    format!("postgres_types::Json<&{lifetime} serde_json::value::RawValue>")
                }
                _ => rust_name.to_string(),
            },
            CornucopiaType::Array(inner_ty) => {
                let inner = type_registrar.get(inner_ty).unwrap().brw_struct(
                    type_registrar,
                    for_params,
                    false,
                );

                let inner = if is_inner_nullable {
                    format!("Option<{inner}>")
                } else {
                    inner
                };
                // Its more practical for users to use a slice
                if for_params {
                    format!("&{lifetime} [{inner}]")
                } else {
                    format!("cornucopia_client::ArrayIterator<{lifetime}, {inner}>")
                }
            }
            CornucopiaType::Custom {
                struct_path,
                is_params,
                is_copy,
                ..
            } => {
                if *is_copy {
                    struct_path.to_string()
                } else if for_params && !is_params {
                    format!("{}Params<{lifetime}>", struct_path)
                } else {
                    format!("{}Borrowed<{lifetime}>", struct_path)
                }
            }
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
            Kind::Enum(_) => self.insert_custom(ty, true, true),
            Kind::Array(inner_ty) => {
                self.register(inner_ty)?;
                self.insert(ty, || CornucopiaType::Array(inner_ty.clone()))
            }
            Kind::Domain(inner_ty) => {
                let inner = self.register(inner_ty)?;
                let (is_copy, is_params) = (inner.is_copy(), inner.is_params());
                self.insert_custom(ty, is_copy, is_params)
            }
            Kind::Composite(composite_fields) => {
                let mut is_copy = true;
                let mut is_params = true;
                for field in composite_fields {
                    let field_ty = self.register(field.type_())?;
                    is_copy &= field_ty.is_copy();
                    is_params &= field_ty.is_params();
                }
                self.insert_custom(ty, is_copy, is_params)
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
                self.insert(ty, || CornucopiaType::Simple {
                    pg_ty: ty.clone(),
                    rust_name,
                    is_copy,
                })
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

    fn insert(&mut self, ty: &Type, call: impl Fn() -> CornucopiaType) -> &CornucopiaType {
        let index = match self
            .types
            .entry((ty.schema().to_owned(), ty.name().to_owned()))
        {
            indexmap::map::Entry::Occupied(o) => o.index(),
            indexmap::map::Entry::Vacant(v) => {
                let index = v.index();
                v.insert(call());
                index
            }
        };

        &self.types[index]
    }

    fn insert_custom(&mut self, ty: &Type, is_copy: bool, is_params: bool) -> &CornucopiaType {
        let content = match ty.kind() {
            Kind::Enum(variants) => CustomContent::Enum(variants.to_vec()),
            Kind::Domain(inner) => {
                CustomContent::Domain(PreparedField {
                    name: "inner".to_string(),
                    ty: self.get(inner).unwrap().clone(),
                    is_nullable: false,
                    is_inner_nullable: false, // TODO used when support null everywhere
                })
            }
            Kind::Composite(fields) => CustomContent::Composite(
                fields
                    .iter()
                    .map(|field| {
                        PreparedField {
                            name: field.name().to_string(),
                            ty: self.get(field.type_()).unwrap().clone(),
                            is_nullable: false, // TODO used when support null everywhere
                            is_inner_nullable: false, // TODO used when support null everywhere
                        }
                    })
                    .collect(),
            ),
            _ => unreachable!(),
        };
        let index = match self
            .types
            .entry((ty.schema().to_owned(), ty.name().to_owned()))
        {
            indexmap::map::Entry::Occupied(o) => o.index(),
            indexmap::map::Entry::Vacant(v) => {
                let index = v.index();
                let rust_ty_name = ty.name().to_upper_camel_case();

                v.insert(CornucopiaType::Custom {
                    pg_ty: ty.clone(),
                    content: Box::new(content),
                    struct_path: format!("super::super::types::{}::{}", ty.schema(), rust_ty_name),
                    struct_name: rust_ty_name,
                    is_copy,
                    is_params,
                });
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
