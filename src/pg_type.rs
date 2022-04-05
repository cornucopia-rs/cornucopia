use tokio_postgres::types::Type;

use self::error::UnsupportedPostgresTypeError;

pub fn from_str(value: &str) -> Result<Type, UnsupportedPostgresTypeError> {
    Ok(match value.to_lowercase().as_str() {
        "bool" | "boolean" => Type::BOOL,
        "char" | "character" => Type::CHAR,
        "smallint" | "int2" | "smallserial" | "serial2" => Type::INT2,
        "int" | "int4" | "serial" | "serial4" => Type::INT4,
        "bigint" | "int8" | "bigserial" | "serial8" => Type::INT8,
        "real" | "float4" => Type::FLOAT4,
        "double precision" | "float8" => Type::FLOAT8,
        "text" => Type::TEXT,
        "varchar" => Type::VARCHAR,
        "byeta" => Type::BYTEA,
        "timestamp without time zone" | "timestamp" => Type::TIMESTAMP,
        "timestamp with time zone" | "timestamptz" => Type::TIMESTAMPTZ,
        "date" => Type::DATE,
        "time" => Type::TIME,
        "json" => Type::JSON,
        "jsonb" => Type::JSONB,
        "uuid" => Type::UUID,
        _ => return Err(UnsupportedPostgresTypeError { ty: value.into() }),
    })
}

pub fn to_equivalent_rust_string(ty: &Type) -> Result<String, UnsupportedPostgresTypeError> {
    let rust_type_str = match *ty {
        Type::BOOL => "bool",
        Type::CHAR => "i8",
        Type::INT2 => "i16",
        Type::INT4 => "i32",
        Type::INT8 => "i64",
        Type::FLOAT4 => "f32",
        Type::FLOAT8 => "f64",
        Type::TEXT => "String",
        Type::VARCHAR => "String",
        Type::BYTEA => "Vec<u8>",
        Type::TIMESTAMP => "time::PrimitiveDateTime",
        Type::TIMESTAMPTZ => "time::OffsetDateTime",
        Type::DATE => "time::Date",
        Type::TIME => "time::Time",
        Type::JSON => "serde_json::Value",
        Type::JSONB => "serde_json::Value",
        Type::UUID => "uuid::Uuid",
        _ => {
            return Err(UnsupportedPostgresTypeError {
                ty: String::from(ty.name()),
            })
        }
    };

    Ok(String::from(rust_type_str))
}

pub fn to_litteral_rust_string(ty: &Type) -> String {
    format!("Type::{}", ty.name().to_uppercase())
}

pub mod error {
    use thiserror::Error as ThisError;
    #[derive(Debug, ThisError)]
    #[error("encoutered unsupported type `{ty}` while parsing queries")]
    pub struct UnsupportedPostgresTypeError {
        pub ty: String,
    }
}
