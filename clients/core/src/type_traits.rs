use std::borrow::Cow;

use crate::domain::escape_domain_to_sql;
use postgres_protocol::types::{self, ArrayDimension};
use postgres_types::{private::BytesMut, to_sql_checked, IsNull, Kind, ToSql, Type};

pub trait StringSql: std::fmt::Debug + ToSql + Sync {}
impl<T: StringSql> StringSql for &T {}
impl StringSql for String {}
impl StringSql for &str {}
impl StringSql for Cow<'_, str> {}
impl StringSql for Box<str> {}

pub trait BytesSql: std::fmt::Debug + ToSql + Send + Sync {}
impl<T: BytesSql> BytesSql for &T {}
impl BytesSql for Vec<u8> {}
impl BytesSql for &[u8] {}

#[cfg(feature = "with-serde_json-1")]
pub trait JsonSql: std::fmt::Debug + ToSql + Sync + Send {}
#[cfg(feature = "with-serde_json-1")]
impl<T: JsonSql> JsonSql for &T {}
#[cfg(feature = "with-serde_json-1")]
impl JsonSql for serde_json_1::value::Value {}
#[cfg(feature = "with-serde_json-1")]
impl<T: serde_1::ser::Serialize + std::fmt::Debug + Sync + Send> JsonSql
    for postgres_types::Json<T>
{
}

pub trait ArraySql: std::fmt::Debug + ToSql + Send + Sync {
    type Item;
    fn escape_domain_to_sql(
        &self,
        ty: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>>;
}
impl<T: std::fmt::Debug + ToSql + Sync, A: ArraySql<Item = T>> ArraySql for &A {
    type Item = T;

    fn escape_domain_to_sql(
        &self,
        ty: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        A::escape_domain_to_sql(self, ty, w)
    }
}
impl<T: std::fmt::Debug + ToSql + Send + Sync> ArraySql for Vec<T> {
    type Item = T;

    fn escape_domain_to_sql(
        &self,
        ty: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        escape_domain_to_sql(ty, w, self.iter())
    }
}

impl<T: std::fmt::Debug + ToSql + Sync> ArraySql for &[T] {
    type Item = T;

    fn escape_domain_to_sql(
        &self,
        ty: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        escape_domain_to_sql(ty, w, self.iter())
    }
}

impl<
        T: std::fmt::Debug + ToSql + Send + Sync,
        I: Iterator<Item = T> + ExactSizeIterator,
        F: Fn() -> I + Send + Sync,
    > ArraySql for IterSql<T, I, F>
{
    type Item = T;

    fn escape_domain_to_sql(
        &self,
        ty: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        escape_domain_to_sql(ty, w, (self.0)())
    }
}

pub struct IterSql<T: ToSql, I: Iterator<Item = T> + ExactSizeIterator, F: Fn() -> I + Sync>(pub F);

impl<T: ToSql, I: Iterator<Item = T> + ExactSizeIterator, F: Fn() -> I + Sync> std::fmt::Debug
    for IterSql<T, I, F>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ArrayFn").finish()
    }
}

// Taken from `postgres`
impl<T: ToSql, I: Iterator<Item = T> + ExactSizeIterator, F: Fn() -> I + Sync> ToSql
    for IterSql<T, I, F>
{
    fn to_sql(
        &self,
        ty: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let member_type = match *ty.kind() {
            Kind::Array(ref member) => member,
            _ => panic!("expected array type"),
        };

        let iter = (self.0)();

        let dimension = ArrayDimension {
            len: downcast(iter.len())?,
            lower_bound: 1,
        };

        types::array_to_sql(
            Some(dimension),
            member_type.oid(),
            iter,
            |e, w| match e.to_sql(member_type, w)? {
                IsNull::No => Ok(postgres_protocol::IsNull::No),
                IsNull::Yes => Ok(postgres_protocol::IsNull::Yes),
            },
            w,
        )?;
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        match *ty.kind() {
            Kind::Array(ref member) => T::accepts(member),
            _ => false,
        }
    }

    to_sql_checked!();
}

// https://github.com/sfackler/rust-postgres/blob/765395f288861209a644c621bf72172acd482515/postgres-types/src/lib.rs
fn downcast(len: usize) -> Result<i32, Box<dyn std::error::Error + Sync + Send>> {
    if len > i32::max_value() as usize {
        Err("value too large to transmit".into())
    } else {
        Ok(len as i32)
    }
}
