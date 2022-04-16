use std::{borrow::Borrow, collections::HashMap};

use async_recursion::async_recursion;
use deadpool_postgres::Client;
use heck::ToUpperCamelCase;
use postgres_types::{Field, Kind};
use tokio_postgres::types::Type;

use self::error::{Error, UnsupportedPostgresTypeError};

#[derive(PartialEq, Eq, Debug, Clone)]

pub struct CornucopiaType {
    pub pg_ty: Type,
    pub kind: CornucopiaTypeKind,
    pub rust_ty_usage_path: String,
    pub rust_ty_definition_path: String,
    pub rust_ty_name: String,
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

    pub fn borrowed_rust_ty(&self) -> String {
        if self.rust_ty_usage_path == "String" {
            String::from("&str")
        } else {
            format!("&{}", self.rust_ty_usage_path)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CornucopiaField {
    pub name: String,
    pub ty: CornucopiaType,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CornucopiaTypeKind {
    Base,
    Composite(Vec<CornucopiaField>),
    Domain(Box<CornucopiaType>),
    Enum(Vec<String>),
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
        }
    }
}

pub struct TypeRegistrar {
    pub base_types: HashMap<String, CornucopiaType>,
    pub custom_types: HashMap<(String, String), CornucopiaType>,
}

impl TypeRegistrar {
    pub fn base_type(&self, alias: &str) -> Option<&CornucopiaType> {
        self.base_types.get(alias)
    }

    pub fn custom_type(&self, schema: &str, name: &str) -> Option<&CornucopiaType> {
        self.custom_types.get(&(schema.to_owned(), name.to_owned()))
    }

    pub fn get(&self, schema: &str, name: &str) -> Option<&CornucopiaType> {
        if schema == "pg_catalog" {
            self.base_type(name)
        } else {
            self.custom_type(schema, name)
        }
    }

    fn new() -> Self {
        Self {
            base_types: HashMap::new(),
            custom_types: HashMap::new(),
        }
    }

    fn insert_base_type(&mut self, alias: &'static str, ty: CornucopiaType) {
        self.base_types.insert(alias.to_owned(), ty);
    }

    fn insert(&mut self, ty: CornucopiaType) {
        self.custom_types.insert(
            (ty.pg_ty.schema().to_owned(), ty.pg_ty.name().to_owned()),
            ty,
        );
    }

    async fn type_info(
        client: &Client,
        schema: String,
        name: String,
    ) -> Result<(String, u32, char, String), tokio_postgres::Error> {
        let row = client
            .query_one(
                "SELECT pg_type.oid, pg_type.typtype
FROM  pg_type 
JOIN pg_namespace ON pg_namespace.oid = pg_type.typnamespace
WHERE pg_type.typname = $1
AND pg_namespace.nspname = $2;",
                &[&name, &schema],
            )
            .await?;

        let oid: u32 = row.get(0);
        let typtype: i8 = row.get(1);
        Ok((name, oid, typtype as u8 as char, schema))
    }

    async fn enum_variants(
        client: &Client,
        oid: &u32,
    ) -> Result<Vec<String>, tokio_postgres::Error> {
        let x = client
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
            .collect::<Vec<String>>();
        Ok(x)
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

    async fn type_kind(
        &mut self,
        client: &Client,
        name: &str,
        oid: &u32,
        typtype: &char,
    ) -> Result<CornucopiaTypeKind, Error> {
        match typtype {
            'e' => {
                let variants = Self::enum_variants(client, oid).await?;
                Ok(CornucopiaTypeKind::Enum(variants))
            }
            'd' => {
                let (base_name, _, _, base_schema) = Self::domain_base_type(client, oid).await?;
                let sub_type = self.register(client, base_schema, base_name).await?;
                Ok(CornucopiaTypeKind::Domain(Box::new(sub_type)))
            }
            'c' => {
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
            _ => Err(Error::UnsupportedPostgresType(
                UnsupportedPostgresTypeError {
                    name: name.to_owned(),
                },
            )),
        }
    }

    pub async fn register_type(
        &mut self,
        client: &Client,
        ty: &Type,
    ) -> Result<CornucopiaType, Error> {
        self.register(client, ty.schema().to_owned(), ty.name().to_owned())
            .await
    }

    #[async_recursion]
    pub async fn register(
        &mut self,
        client: &Client,
        schema: String,
        name: String,
    ) -> Result<CornucopiaType, Error> {
        if let Some(t) = self.get(&schema, &name) {
            Ok(t.clone())
        } else {
            let (name, oid, typtype, schema) = Self::type_info(client, schema, name).await?;
            let kind = self.type_kind(client, &name, &oid, &typtype).await?;
            let rust_ty_name = name.to_upper_camel_case();
            let pg_ty = Type::new(name, oid, kind.borrow().into(), schema);
            let new_type = CornucopiaType::new_custom(pg_ty, kind, rust_ty_name);
            self.insert(new_type.clone());
            Ok(new_type)
        }
    }
}

impl<const N: usize> From<[(&'static str, Type, &'static str); N]> for TypeRegistrar {
    fn from(base_types: [(&'static str, Type, &'static str); N]) -> Self {
        let mut registrar = Self::new();
        for (name, ty, rust_ty_name) in base_types {
            registrar.insert_base_type(name, CornucopiaType::new_base(ty, rust_ty_name.to_owned()));
        }
        registrar
    }
}

impl Default for TypeRegistrar {
    fn default() -> Self {
        TypeRegistrar::from([
            ("bool", Type::BOOL, "bool"),
            ("boolean", Type::BOOL, "bool"),
            ("char", Type::CHAR, "i8"),
            ("smallint", Type::INT2, "i16"),
            ("int2", Type::INT2, "i16"),
            ("smallserial", Type::INT2, "i16"),
            ("serial2", Type::INT2, "i16"),
            ("int", Type::INT4, "i32"),
            ("int4", Type::INT4, "i32"),
            ("serial", Type::INT4, "i32"),
            ("serial4", Type::INT4, "i32"),
            ("bigint", Type::INT8, "i64"),
            ("int8", Type::INT8, "i64"),
            ("bigserial", Type::INT8, "i64"),
            ("serial8", Type::INT8, "i64"),
            ("float4", Type::FLOAT4, "f32"),
            ("real", Type::FLOAT4, "f32"),
            ("float8", Type::FLOAT8, "f64"),
            ("double precision", Type::FLOAT8, "f64"),
            ("text", Type::TEXT, "String"),
            ("varchar", Type::VARCHAR, "String"),
            ("bytea", Type::BYTEA, "Vec<u8>"),
            ("timestamp", Type::TIMESTAMP, "time::PrimitiveDateTime"),
            (
                "timestamp without time zone",
                Type::TIMESTAMP,
                "time::PrimitiveDateTime",
            ),
            ("timestamptz", Type::TIMESTAMPTZ, "time::OffsetDateTime"),
            (
                "timestamp with time zone",
                Type::TIMESTAMPTZ,
                "time::OffsetDateTime",
            ),
            ("date", Type::DATE, "time::Date"),
            ("time", Type::TIME, "time::Time"),
            ("json", Type::JSON, "serde_json::Value"),
            ("jsonb", Type::JSONB, "serde_json::Value"),
            ("uuid", Type::UUID, "uuid::Uuid"),
            ("inet", Type::INET, "std::net::IpAddr"),
            ("macaddr", Type::MACADDR, "eui48::MacAddress"),
        ])
    }
}
pub mod error {
    use postgres_types::Kind;
    use thiserror::Error as ThisError;
    #[derive(Debug, ThisError)]
    #[error("encoutered unsupported type `{name}` while parsing queries")]
    pub struct UnsupportedPostgresTypeError {
        pub name: String,
    }

    #[derive(Debug, ThisError)]
    #[error("encountered error while attempting to discover postgres type")]
    pub enum Error {
        Db(#[from] tokio_postgres::Error),
        UnsupportedPostgresType(#[from] UnsupportedPostgresTypeError),
        #[error("unsupported postgres type kind {0:?}")]
        UnsupportedPostgresKindError(Kind),
    }
}
