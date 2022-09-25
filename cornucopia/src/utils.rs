use std::{
    cell::RefCell,
    fmt::{Display, Formatter, Write},
};

use indexmap::Equivalent;
use postgres::error::ErrorPosition;
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
        for item in self.inner.borrow_mut().take().unwrap() {
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

/// Sorted list of rust reserved keywords
pub(crate) const KEYWORD: [&str; 52] = [
    "Self", "abstract", "as", "async", "await", "become", "box", "break", "const", "continue",
    "crate", "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl",
    "in", "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref",
    "return", "self", "static", "struct", "super", "trait", "true", "try", "type", "typeof",
    "union", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

/// Escape ident if clash with rust reserved keywords
pub(crate) fn escape_keyword(ident: String) -> String {
    if KEYWORD.binary_search(&ident.as_str()).is_ok() {
        format!("r#{ident}")
    } else {
        ident
    }
}

/// Unescape ident
pub(crate) fn unescape_keyword(ident: &str) -> &str {
    ident.trim_start_matches("r#")
}
