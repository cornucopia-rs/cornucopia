use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
};

use indexmap::Equivalent;
use postgres::error::ErrorPosition;
use postgres_types::Type;

pub struct Lazy<F: Fn(&mut Formatter)> {
    f: RefCell<Option<F>>,
}

impl<F: Fn(&mut Formatter)> Lazy<F> {
    pub fn new(f: F) -> Self {
        Self {
            f: RefCell::new(Some(f)),
        }
    }
}

impl<F: Fn(&mut Formatter)> Display for Lazy<F> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(f) = self.f.take() {
            f(fmt);
        }
        Ok(())
    }
}

/// Allows us to query a map using type schema as key without having to own the key strings
#[derive(PartialEq, Eq, Hash)]
pub struct SchemaKey<'a> {
    schema: &'a str,
    name: &'a str,
}

impl<'a> From<&'a Type> for SchemaKey<'a> {
    fn from(ty: &'a Type) -> Self {
        SchemaKey {
            schema: ty.schema(),
            name: ty.name(),
        }
    }
}

impl<'a> Equivalent<(String, String)> for SchemaKey<'a> {
    fn equivalent(&self, key: &(String, String)) -> bool {
        key.0.as_str().equivalent(&self.schema) && key.1.as_str().equivalent(&self.name)
    }
}

pub fn find_duplicate<T>(slice: &[T], eq: fn(&T, &T) -> bool) -> Option<(&T, &T)> {
    for (i, first) in slice.iter().enumerate() {
        if let Some(second) = slice[i + 1..].iter().find(|second| eq(first, second)) {
            return Some((first, second));
        }
    }
    None
}

/// Extracts useful info from a `postgres`-generated error.
pub(crate) fn db_err(err: &postgres::Error) -> Option<(u32, String, Option<String>)> {
    if let Some(db_err) = err.as_db_error() {
        if let Some(ErrorPosition::Original(position)) = db_err.position() {
            Some((
                *position,
                db_err.message().to_string(),
                db_err.hint().map(String::from),
            ))
        } else {
            None
        }
    } else {
        None
    }
}
