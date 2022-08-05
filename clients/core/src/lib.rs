mod array_iterator;
mod domain;
mod utils;

use std::borrow::Cow;

pub use array_iterator::ArrayIterator;
pub use domain::{Domain, DomainArray};
use postgres_protocol::types::{self, ArrayDimension};
use postgres_types::{private::BytesMut, to_sql_checked, IsNull, Kind, ToSql, Type};
pub use utils::slice_iter;

pub trait StringSql: std::fmt::Debug + ToSql + Sync {}
impl StringSql for String {}
impl StringSql for &str {}
impl StringSql for Cow<'_, str> {}
impl StringSql for Box<str> {}

pub trait BytesSql: std::fmt::Debug + ToSql + Sync {}
impl BytesSql for Vec<u8> {}
impl BytesSql for &[u8] {}

pub trait ArraySql<T: std::fmt::Debug + ToSql + Sync>: std::fmt::Debug + ToSql + Sync {
    fn slice(&self) -> &[T];
}
impl<T: std::fmt::Debug + ToSql + Sync> ArraySql<T> for Vec<T> {
    fn slice(&self) -> &[T] {
        self.as_slice()
    }
}
impl<T: std::fmt::Debug + ToSql + Sync> ArraySql<T> for &[T] {
    fn slice(&self) -> &[T] {
        self
    }
}
impl<
        T: std::fmt::Debug + ToSql + Sync,
        I: Iterator<Item = T> + ExactSizeIterator,
        F: Fn() -> I + Sync,
    > ArraySql<T> for IterSql<T, I, F>
{
    fn slice(&self) -> &[T] {
        todo!("Can't use IterSql with Domains yet")
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
