mod array_iterator;
mod domain;
mod utils;

use std::borrow::Cow;

pub use array_iterator::ArrayIterator;
pub use domain::{Domain, DomainArray};
use postgres_types::ToSql;
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
