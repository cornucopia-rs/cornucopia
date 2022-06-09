use std::{fmt::Display, ops::Range};

use chumsky::{prelude::*, text::whitespace};
use error::Error;
use heck::ToUpperCamelCase;

/// Th    if is data structure holds a value and the context in which it was parsed.
/// This context is used for error reporting.
#[derive(Debug, Clone)]
pub struct Parsed<T> {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) value: T,
}

impl<T: std::hash::Hash> std::hash::Hash for Parsed<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T: PartialEq> PartialEq<Self> for Parsed<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Display> Display for Parsed<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T: Eq> Eq for Parsed<T> {}

impl<T: PartialOrd + PartialEq> PartialOrd<Self> for Parsed<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T: Ord> Ord for Parsed<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T> Parsed<T> {
    pub(crate) fn map<U>(&self, f: impl Fn(&T) -> U) -> Parsed<U> {
        Parsed {
            value: f(&self.value),
            start: self.start,
            end: self.end,
        }
    }
}

fn ident() -> impl Parser<char, Parsed<String>, Error = Simple<char>> {
    filter(|c: &char| c.is_ascii_alphabetic())
        .chain(filter(|c: &char| c.is_ascii_alphanumeric() || *c == '_').repeated())
        .collect()
        .map_with_span(|value: String, span: Range<usize>| Parsed {
            start: span.start(),
            end: span.end(),
            value,
        })
}
fn ln() -> impl Parser<char, (), Error = Simple<char>> {
    one_of("\n\r").repeated().ignored()
}
fn space() -> impl Parser<char, (), Error = Simple<char>> {
    one_of(" \t").repeated().ignored()
}
fn blank() -> impl Parser<char, (), Error = Simple<char>> {
    whitespace().or(space()
        .ignore_then(just("--"))
        .ignore_then(filter(|c: &char| *c != '\n').repeated())
        .ignore_then(ln()))
}
fn parse_nullable_ident() -> impl Parser<char, Vec<Parsed<String>>, Error = Simple<char>> {
    space()
        .ignore_then(ident())
        .then_ignore(just('?'))
        .then_ignore(space())
        .separated_by(just(','))
        .allow_trailing()
        .delimited_by(just('('), just(')'))
}

#[derive(Debug, Clone, Copy)]
pub enum TypeAnnotationKind {
    Param,
    Row,
    Db,
}

impl TypeAnnotationKind {
    fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        ident().map(|it| match it.value.to_lowercase().as_str() {
            "param" => Self::Param,
            "row" => Self::Row,
            "db" => Self::Db,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug)]
pub(crate) struct TypeDataStructure {
    pub(crate) name: Parsed<String>,
    pub(crate) fields: Vec<Parsed<String>>,
}

#[derive(Debug)]
pub struct TypeAnnotation {
    pub kind: TypeAnnotationKind,
    pub name: Parsed<String>,
    pub fields: Vec<Parsed<String>>,
}

impl TypeAnnotation {
    fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        just("--:")
            .ignore_then(space())
            .ignore_then(TypeAnnotationKind::parser())
            .then_ignore(space())
            .then(ident())
            .then_ignore(space())
            .then(parse_nullable_ident())
            .map(|((kind, name), fields)| Self { kind, name, fields })
    }
}

#[derive(Debug)]
pub(crate) struct QuerySql {
    pub(crate) sql_str: String,
    pub(crate) bind_params: Vec<Parsed<String>>,
}

impl QuerySql {
    fn parse_bind() -> impl Parser<char, Vec<Parsed<String>>, Error = Simple<char>> {
        just(':')
            .ignore_then(ident())
            .separated_by(filter(|c: &char| *c != ':').repeated())
            .allow_leading()
            .allow_trailing()
    }

    fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        filter(|c: &char| *c != ';')
            .repeated()
            .then_ignore(just(';'))
            .collect::<String>()
            .map_with_span(|it, span: Range<usize>| Parsed {
                start: span.start,
                end: span.end,
                value: it,
            })
            .map(|sql_str| {
                let sql_start = sql_str.start;
                let mut sql_str = sql_str.value;
                let bind_params: Vec<_> = Self::parse_bind()
                    .parse(sql_str.clone())
                    .unwrap()
                    .into_iter()
                    .map(|mut it| {
                        it.start += sql_start;
                        it.end += sql_start;
                        it
                    })
                    .collect();

                // Normalize
                let mut deduped_bind_params = bind_params.clone();
                deduped_bind_params.sort_unstable();
                deduped_bind_params.dedup();

                for bind_param in bind_params.iter().rev() {
                    let index = deduped_bind_params
                        .iter()
                        .position(|bp| bp == bind_param)
                        .unwrap();
                    let start = bind_param.start - sql_start - 1;
                    let end = bind_param.end - sql_start - 1;
                    sql_str.replace_range(start..=end, &format!("${}", index + 1))
                }
                Self {
                    sql_str,
                    bind_params,
                }
            })
    }
}

#[derive(Debug)]
pub(crate) struct Query {
    pub(crate) annotation: QueryAnnotation,
    pub(crate) sql: QuerySql,
}

impl Query {
    fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        QueryAnnotation::parser()
            .then_ignore(ln())
            .then(QuerySql::parser())
            .map(|(annotation, sql)| Self { annotation, sql })
    }
}

#[derive(Debug)]
pub(crate) enum QueryDataStruct {
    Implicit { idents: Vec<Parsed<String>> },
    Named(Parsed<String>),
}

impl QueryDataStruct {
    pub(crate) fn name_and_fields(
        self,
        registered_structs: &[TypeDataStructure],
        query_name: &Parsed<String>,
        name_suffix: Option<&str>,
    ) -> (Vec<Parsed<String>>, Parsed<String>) {
        match self {
            QueryDataStruct::Implicit { idents } => (
                idents,
                query_name.map(|x| {
                    format!(
                        "{}{}",
                        x.to_upper_camel_case(),
                        name_suffix.unwrap_or_default()
                    )
                }),
            ),
            QueryDataStruct::Named(name) => (
                registered_structs
                    .iter()
                    .find_map(|it| (it.name == name).then(|| it.fields.clone()))
                    .unwrap_or_default(),
                name,
            ),
        }
    }
}

impl Default for QueryDataStruct {
    fn default() -> Self {
        Self::Implicit { idents: Vec::new() }
    }
}

impl QueryDataStruct {
    fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        parse_nullable_ident()
            .map(|idents| Self::Implicit { idents })
            .or(ident().map(Self::Named))
    }
}

#[derive(Debug)]
pub(crate) struct QueryAnnotation {
    pub(crate) name: Parsed<String>,
    pub(crate) param: QueryDataStruct,
    pub(crate) row: QueryDataStruct,
}

impl QueryAnnotation {
    fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        just("--!")
            .ignore_then(space())
            .ignore_then(ident())
            .then_ignore(space())
            .then(QueryDataStruct::parser().or_not())
            .then_ignore(space())
            .then(
                just(':')
                    .ignore_then(space())
                    .ignore_then(QueryDataStruct::parser())
                    .or_not(),
            )
            .map(|((name, param), row)| Self {
                name,
                param: param.unwrap_or_default(),
                row: row.unwrap_or_default(),
            })
    }
}

#[derive(Debug)]
enum Statement {
    Type(TypeAnnotation),
    Query(Query),
}

#[derive(Debug)]
pub(crate) struct ParsedModule {
    pub(crate) param_types: Vec<TypeDataStructure>,
    pub(crate) row_types: Vec<TypeDataStructure>,
    pub(crate) db_types: Vec<TypeDataStructure>,
    pub(crate) queries: Vec<Query>,
}

impl FromIterator<Statement> for ParsedModule {
    fn from_iter<T: IntoIterator<Item = Statement>>(iter: T) -> Self {
        let mut param_types = Vec::new();
        let mut row_types = Vec::new();
        let mut db_types = Vec::new();
        let mut queries = Vec::new();
        for item in iter {
            match item {
                Statement::Type(TypeAnnotation { kind, name, fields }) => {
                    let ty_item = TypeDataStructure { name, fields };
                    match kind {
                        TypeAnnotationKind::Param => param_types.push(ty_item),
                        TypeAnnotationKind::Row => row_types.push(ty_item),
                        TypeAnnotationKind::Db => db_types.push(ty_item),
                    }
                }
                Statement::Query(it) => queries.push(it),
            }
        }

        ParsedModule {
            param_types,
            row_types,
            db_types,
            queries,
        }
    }
}

/// Parse queries in in the input string using the grammar file (`grammar.pest`).
pub(crate) fn parse_query_module(path: &str, input: &str) -> Result<ParsedModule, Error> {
    TypeAnnotation::parser()
        .map(Statement::Type)
        .or(Query::parser().map(Statement::Query))
        .separated_by(blank())
        .collect()
        .parse(input)
        .map_err(|e| Error {
            path: path.to_string(),
            err: e,
        })
}

pub(crate) mod error {

    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    #[error("Error while parsing queries [path: \"{path}\"]:\n{err:?}.")]
    pub struct Error {
        pub path: String,
        pub err: Vec<chumsky::error::Simple<char>>,
    }
}
