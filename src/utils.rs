use std::{
    cell::RefCell,
    fmt::{Display, Formatter, Write},
};

use indexmap::Equivalent;
use postgres_types::Type;

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

/// Lazily join of a formatted iterator
pub struct Joiner<T, I: IntoIterator<Item = T>, F: Fn(&mut Formatter, T)> {
    sep: char,
    /// Use interior mutability because Display::fmt takes &self
    inner: RefCell<Option<I>>,
    mapper: F,
}

impl<T, I: IntoIterator<Item = T>, F: Fn(&mut Formatter, T)> Display for Joiner<T, I, F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for item in self.inner.borrow_mut().take().unwrap().into_iter() {
            if first {
                first = false;
            } else {
                f.write_char(self.sep)?;
            }
            (self.mapper)(f, item);
        }
        Ok(())
    }
}

/// Join a formatted iterator using a separator
pub fn join<T, I: IntoIterator<Item = T>, F: Fn(&mut Formatter, T)>(
    iter: I,
    map: F,
    sep: char,
) -> Joiner<T, I, F> {
    Joiner {
        sep,
        inner: RefCell::new(Some(iter)),
        mapper: map,
    }
}

/// Join a formatted iterator with comma
pub fn join_comma<T, I: IntoIterator<Item = T>, F: Fn(&mut Formatter, T)>(
    iter: I,
    map: F,
) -> Joiner<T, I, F> {
    join(iter, map, ',')
}

/// Join a formatted iterator with newline
pub fn join_ln<T, I: IntoIterator<Item = T>, F: Fn(&mut Formatter, T)>(
    iter: I,
    map: F,
) -> Joiner<T, I, F> {
    join(iter, map, '\n')
}

pub fn has_duplicate<T, U>(
    iter: T,
    mapper: fn(<T as IntoIterator>::Item) -> U,
) -> Option<<T as IntoIterator>::Item>
where
    T: IntoIterator + Clone,
    U: Eq + std::hash::Hash + Clone,
{
    let mut uniq = std::collections::HashSet::new();
    iter.clone()
        .into_iter()
        .zip(iter.into_iter().map(mapper))
        .find(|(_, u)| !uniq.insert(u.clone()))
        .map(|(t, _)| t)
}
