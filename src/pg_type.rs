use std::{borrow::Borrow, collections::HashMap};

use async_recursion::async_recursion;
use deadpool_postgres::Client;
use heck::ToUpperCamelCase;
use postgres_types::{Field, Kind};
use tokio_postgres::types::Type;

use self::error::{Error, UnsupportedPostgresTypeError};

#[derive(PartialEq, Eq, Debug, Clone)]

pub(crate) struct CornucopiaType {
    pub(crate) pg_ty: Type,
    pub(crate) kind: CornucopiaTypeKind,
    pub(crate) rust_ty_usage_path: String,
    pub(crate) rust_ty_definition_path: String,
    pub(crate) rust_ty_name: String,
}

impl CornucopiaType {
    fn new_custom(pg_ty: Type, kind: CornucopiaTypeKind, rust_ty_name: String) -> Self {
        Self {
            kind,

            rust_ty_usage_path: format!(
                "super::super::types::{}::{}",
                pg_ty.schema(),
                rust_ty_name
            ),
            rust_ty_definition_path: format!("super::{}::{}", pg_ty.schema(), rust_ty_name),
            rust_ty_name,
            pg_ty,
        }
    }

    fn new_base(pg_ty: Type, rust_ty_name: String) -> Self {
        Self {
            kind: CornucopiaTypeKind::Base,

            rust_ty_usage_path: rust_ty_name.to_owned(),
            rust_ty_definition_path: rust_ty_name.to_owned(),
            rust_ty_name,
            pg_ty,
        }
    }

    pub(crate) fn borrowed_rust_ty(&self) -> String {
        if self.rust_ty_usage_path == "String" {
            String::from("&str")
        } else if self.rust_ty_usage_path.starts_with("Vec<") {
            format!(
                "&[{}]",
                String::from(&self.rust_ty_usage_path[4..self.rust_ty_usage_path.len() - 1])
            )
        } else {
            format!("&{}", self.rust_ty_usage_path)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct CornucopiaField {
    pub(crate) name: String,
    pub(crate) ty: CornucopiaType,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum CornucopiaTypeKind {
    Base,
    Composite(Vec<CornucopiaField>),
    Domain(Box<CornucopiaType>),
    Enum(Vec<String>),
    Array(Box<CornucopiaType>),
}

impl From<&CornucopiaTypeKind> for Kind {
    fn from(value: &CornucopiaTypeKind) -> Self {
        match value {
            CornucopiaTypeKind::Base => Kind::Simple,
            CornucopiaTypeKind::Composite(fields) => Kind::Composite(
                fields
                    .iter()
                    .map(|field| Field::new(field.name.clone(), field.ty.pg_ty.clone()))
                    .collect(),
            ),
            CornucopiaTypeKind::Domain(domain_base_type) => {
                Kind::Domain(domain_base_type.pg_ty.clone())
            }
            CornucopiaTypeKind::Enum(variants) => Kind::Enum(variants.clone()),
            CornucopiaTypeKind::Array(array_ty) => Kind::Array(array_ty.pg_ty.clone()),
        }
    }
}

pub(crate) struct TypeRegistrar {
    pub(crate) base_types: HashMap<(String, String), CornucopiaType>,
    pub(crate) custom_types: HashMap<(String, String), CornucopiaType>,
}

impl TypeRegistrar {
    pub(crate) fn get_base_type(&self, schema: &str, alias: &str) -> Option<&CornucopiaType> {
        self.base_types.get(&(schema.to_owned(), alias.to_owned()))
    }

    pub(crate) fn get_custom_type(&self, schema: &str, name: &str) -> Option<&CornucopiaType> {
        self.custom_types.get(&(schema.to_owned(), name.to_owned()))
    }

    pub(crate) fn get(&self, schema: &str, name: &str) -> Option<&CornucopiaType> {
        self.get_base_type(schema, name)
            .or_else(|| self.get_custom_type(schema, name))
    }

    fn new() -> Self {
        Self {
            base_types: HashMap::new(),
            custom_types: HashMap::new(),
        }
    }

    fn insert_base(&mut self, schema: String, name: String, ty: CornucopiaType) {
        self.base_types.insert((schema, name), ty);
    }

    fn insert_custom(&mut self, ty: CornucopiaType) {
        self.custom_types.insert(
            (ty.pg_ty.schema().to_owned(), ty.pg_ty.name().to_owned()),
            ty,
        );
    }

    async fn type_info(
        client: &Client,
        schema: String,
        name: String,
    ) -> Result<(String, u32, char, char, String), tokio_postgres::Error> {
        let row = client
            .query_one(
                "SELECT pg_type.oid, pg_type.typtype, pg_type.typcategory
FROM  pg_type 
JOIN pg_namespace ON pg_namespace.oid = pg_type.typnamespace
WHERE pg_type.typname = $1
AND pg_namespace.nspname = $2;",
                &[&name, &schema],
            )
            .await?;
        let oid: u32 = row.get(0);
        let typtype: i8 = row.get(1);
        let typcategory: i8 = row.get(2);
        Ok((
            name,
            oid,
            typtype as u8 as char,
            typcategory as u8 as char,
            schema,
        ))
    }

    async fn enum_variants(
        client: &Client,
        oid: &u32,
    ) -> Result<Vec<String>, tokio_postgres::Error> {
        Ok(client
            .query(
                "select e.enumlabel
from pg_type t 
join pg_enum e on t.oid = e.enumtypid  
where t.oid = $1;",
                &[&oid],
            )
            .await?
            .iter()
            .map(|row| row.get(0))
            .collect::<Vec<String>>())
    }

    async fn domain_base_type(
        client: &Client,
        oid: &u32,
    ) -> Result<(String, u32, char, String), tokio_postgres::Error> {
        let row = client
            .query_one(
                "SELECT pg_type.typname, pg_type.oid, pg_type.typtype, pg_namespace.nspname
FROM pg_type
INNER JOIN pg_namespace
ON pg_namespace.oid = pg_type.typnamespace
WHERE pg_type.oid = (
select typbasetype 
from pg_type
where oid = $1
);",
                &[&oid],
            )
            .await?;

        let name: String = row.get(0);
        let oid: u32 = row.get(1);
        let typtype: i8 = row.get(2);
        let schema: String = row.get(3);

        Ok((name, oid, typtype as u8 as char, schema))
    }

    async fn composite_fields(
        client: &Client,
        oid: &u32,
    ) -> Result<Vec<(String, String, u32, char, String)>, tokio_postgres::Error> {
        let rows = client
            .query(
                "select attr.attname, ty.typname, ty.oid, ty.typtype, pg_namespace.nspname
FROM (
    select  attname, atttypid
    from pg_attribute 
    where attrelid = (
        select typrelid 
        from pg_type
        where oid = $1)
) as attr 
INNER JOIN pg_type as ty ON attr.atttypid = ty.oid
INNER JOIN pg_namespace ON pg_namespace.oid = ty.typnamespace
;",
                &[&oid],
            )
            .await?;

        Ok(rows
            .iter()
            .map(|row| {
                let name: String = row.get(0);
                let typname: String = row.get(1);
                let typoid: u32 = row.get(2);
                let typtype: i8 = row.get(3);
                let typschema: String = row.get(4);

                (name, typname, typoid, typtype as u8 as char, typschema)
            })
            .collect())
    }

    async fn array_elem_type(client: &Client, oid: &u32) -> Result<(String, String), Error> {
        let row = client
            .query_one(
                "
SELECT pg_namespace.nspname, pg_type.typname 
from pg_type 
INNER JOIN pg_namespace
    ON pg_namespace.oid = pg_type.typnamespace
WHERE pg_type.typarray = $1;",
                &[oid],
            )
            .await?;

        let schema: String = row.get(0);
        let name: String = row.get(1);

        Ok((schema, name))
    }

    async fn type_kind(
        &mut self,
        client: &Client,
        name: &str,
        oid: &u32,
        typtype: &char,
        typcategory: &char,
    ) -> Result<CornucopiaTypeKind, Error> {
        match (typtype, typcategory) {
            ('e', _) => {
                let variants = Self::enum_variants(client, oid).await?;
                Ok(CornucopiaTypeKind::Enum(variants))
            }
            ('d', _) => {
                let (base_name, _, _, base_schema) = Self::domain_base_type(client, oid).await?;
                let sub_type = self.register(client, base_schema, base_name).await?;
                Ok(CornucopiaTypeKind::Domain(Box::new(sub_type)))
            }
            ('c', _) => {
                let fields = Self::composite_fields(client, oid).await?;
                let mut tokio_pg_fields = Vec::new();
                for (field_name, field_ty_name, _, _, field_ty_schema) in fields {
                    let field_type = self
                        .register(client, field_ty_schema, field_ty_name)
                        .await?;

                    tokio_pg_fields.push(CornucopiaField {
                        name: field_name,
                        ty: field_type,
                    });
                }
                Ok(CornucopiaTypeKind::Composite(tokio_pg_fields))
            }
            ('b', 'A') => {
                let (elem_ty_schema, elem_ty_name) = Self::array_elem_type(client, oid).await?;
                let base_elem_ty = self.register(client, elem_ty_schema, elem_ty_name).await?;
                Ok(CornucopiaTypeKind::Array(Box::new(base_elem_ty)))
            }
            _ => Err(Error::UnsupportedPostgresType(
                UnsupportedPostgresTypeError {
                    name: name.to_owned(),
                },
            )),
        }
    }

    pub(crate) async fn register_type(
        &mut self,
        client: &Client,
        ty: &Type,
    ) -> Result<CornucopiaType, Error> {
        self.register(client, ty.schema().to_owned(), ty.name().to_owned())
            .await
    }

    #[async_recursion]
    pub(crate) async fn register(
        &mut self,
        client: &Client,
        schema: String,
        name: String,
    ) -> Result<CornucopiaType, Error> {
        if let Some(t) = self.get(&schema, &name) {
            Ok(t.clone())
        } else {
            let (name, oid, typtype, typcategory, schema) =
                Self::type_info(client, schema, name).await?;
            let kind = self
                .type_kind(client, &name, &oid, &typtype, &typcategory)
                .await?;
            let pg_ty = Type::new(name.clone(), oid, kind.borrow().into(), schema.clone());

            Ok(match &kind {
                CornucopiaTypeKind::Array(t) => {
                    let rust_ty_name = format!("Vec<{}>", t.rust_ty_usage_path);
                    let new_type = CornucopiaType::new_base(t.pg_ty.clone(), rust_ty_name);
                    self.insert_base(schema, name, new_type.clone());
                    new_type
                }
                _ => {
                    let rust_ty_name = name.to_upper_camel_case();
                    let new_type = CornucopiaType::new_custom(pg_ty, kind, rust_ty_name);
                    self.insert_custom(new_type.clone());
                    new_type
                }
            })
        }
    }
}

impl<const N: usize> From<[(Type, &'static str); N]> for TypeRegistrar {
    fn from(base_types: [(Type, &'static str); N]) -> Self {
        let mut registrar = Self::new();
        for (ty, rust_ty_name) in base_types {
            registrar.insert_base(
                ty.schema().to_owned(),
                ty.name().to_owned(),
                CornucopiaType::new_base(ty.clone(), rust_ty_name.to_owned()),
            );
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
    #[error("unsupported type `{name}`")]
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
