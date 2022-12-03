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

/// Sorted list of rust reserved keywords that cannot be escaped
pub(crate) const STRICT_KEYWORD: [&str; 5] = ["Self", "_", "crate", "self", "super"];

/// Sorted list of rust reserved keywords
pub(crate) const KEYWORD: [&str; 53] = [
    "Self", "_", "abstract", "as", "async", "await", "become", "box", "break", "const", "continue",
    "crate", "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl",
    "in", "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref",
    "return", "self", "static", "struct", "super", "trait", "true", "try", "type", "typeof",
    "union", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];
