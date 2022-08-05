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
/*
pub fn test<'a>(str: &'a str) {
    let cow: Cow<'a, str> = str.into();
    let cow: Cow<'a, str> = str.to_string().into();
    let cow: Cow<'a, str> = str.to_string().into_boxed_str().as_ref().into();
}*/
