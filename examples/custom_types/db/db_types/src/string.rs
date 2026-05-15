use bytes::BytesMut;
use postgres_types::{FromSql, IsNull, ToSql, Type, to_sql_checked};
use std::error::Error;

/// An owned custom string type that wraps a String.
/// This demonstrates how to use the `borrowed-type` configuration option.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CustomString(pub String);

/// A borrowed custom string type that wraps a &str.
/// This is the borrowed counterpart to `CustomString`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomStringRef<'a>(pub &'a str);

impl<'a> From<CustomStringRef<'a>> for CustomString {
    fn from(borrowed: CustomStringRef<'a>) -> Self {
        CustomString(borrowed.0.to_owned())
    }
}

impl<'a> FromSql<'a> for CustomStringRef<'a> {
    fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let s = std::str::from_utf8(raw)?;
        Ok(CustomStringRef(s))
    }

    fn accepts(ty: &Type) -> bool {
        matches!(*ty, Type::TEXT | Type::VARCHAR)
    }
}

impl<'a> FromSql<'a> for CustomString {
    fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let s = std::str::from_utf8(raw)?;
        Ok(CustomString(s.to_owned()))
    }

    fn accepts(ty: &Type) -> bool {
        matches!(*ty, Type::TEXT | Type::VARCHAR)
    }
}

impl ToSql for CustomString {
    fn to_sql(
        &self,
        _ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        out.extend_from_slice(self.0.as_bytes());
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        matches!(*ty, Type::TEXT | Type::VARCHAR)
    }

    to_sql_checked!();
}

impl<'a> ToSql for CustomStringRef<'a> {
    fn to_sql(
        &self,
        _ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        out.extend_from_slice(self.0.as_bytes());
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        matches!(*ty, Type::TEXT | Type::VARCHAR)
    }

    to_sql_checked!();
}
