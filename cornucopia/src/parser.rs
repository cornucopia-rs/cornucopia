use std::{fmt::Display, ops::Range};

use chumsky::prelude::*;
use error::Error;
use heck::ToUpperCamelCase;
use miette::SourceSpan;

use crate::read_queries::ModuleInfo;

/// Th    if is data structure holds a value and the context in which it was parsed.
/// This context is used for error reporting.
#[derive(Debug, Clone)]
pub struct Span<T> {
    pub(crate) span: SourceSpan,
    pub(crate) value: T,
}

impl<T: std::hash::Hash> std::hash::Hash for Span<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T: PartialEq> PartialEq<Self> for Span<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Display> Display for Span<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T: Eq> Eq for Span<T> {}

impl<T: PartialOrd + PartialEq> PartialOrd<Self> for Span<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T: Ord> Ord for Span<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T> Span<T> {
    pub(crate) fn map<U>(&self, f: impl Fn(&T) -> U) -> Span<U> {
        Span {
            value: f(&self.value),
            span: self.span,
        }
    }
}

fn ident() -> impl Parser<char, Span<String>, Error = Simple<char>> {
    filter(|c: &char| c.is_ascii_alphabetic())
        .chain(filter(|c: &char| c.is_ascii_alphanumeric() || *c == '_').repeated())
        .collect()
        .map_with_span(|value: String, span: Range<usize>| Span {
            value,
            span: span.into(),
        })
}

fn ln() -> impl Parser<char, (), Error = Simple<char>> {
    just("\n").or(just("\n\r")).ignored()
}

fn space() -> impl Parser<char, (), Error = Simple<char>> {
    filter(|c: &char| c.is_whitespace() && *c != '\n')
        .repeated()
        .ignored()
}

fn blank() -> impl Parser<char, (), Error = Simple<char>> {
    // We want to escape valid SQL comment beginning with -- while not escaping our syntax --: or --!
    let comment = just("--")
        .then(none_of(":!").rewind())
        .then(none_of('\n').repeated());
    filter(|c: &char| c.is_whitespace())
        .ignored()
        .or(comment.ignored())
        .repeated()
        .ignored()
}

#[derive(Debug, Clone)]
pub struct NullableIdent {
    pub name: Span<String>,
    pub nullable: bool,
    pub inner_nullable: bool,
}

fn parse_nullable_ident() -> impl Parser<char, Vec<NullableIdent>, Error = Simple<char>> {
    space()
        .ignore_then(ident())
        .then(just('?').or_not())
        .then(just("[?]").or_not())
        .map(|((name, null), inner_null)| NullableIdent {
            name,
            nullable: null.is_some(),
            inner_nullable: inner_null.is_some(),
        })
        .then_ignore(space())
        .separated_by(just(','))
        .allow_trailing()
        .delimited_by(just('('), just(')'))
}

#[derive(Debug, Clone)]
pub struct TypeAnnotation {
    pub name: Span<String>,
    pub fields: Vec<NullableIdent>,
}

impl TypeAnnotation {
    fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        just("--:")
            .ignore_then(space())
            .ignore_then(ident())
            .then_ignore(space())
            .then(parse_nullable_ident().or_not())
            .map(|(name, fields)| Self {
                name,
                fields: fields.unwrap_or_default(),
            })
    }
}

#[derive(Debug)]
pub(crate) struct QuerySql {
    pub(crate) span: SourceSpan,
    pub(crate) sql_str: String,
    pub(crate) bind_params: Vec<Span<String>>,
}

impl QuerySql {
    /// Escape sql string and pattern that are not bind
    fn sql_escaping() -> impl Parser<char, (), Error = Simple<char>> {
        // https://www.postgresql.org/docs/current/sql-syntax-lexical.html

        // ":bind" TODO is this possible ?
        let constant = none_of("\"")
            .repeated()
            .delimited_by(just("\""), just("\""))
            .ignored();
        // ':bind'
        let string = none_of("'")
            .repeated()
            .delimited_by(just("'"), just("'"))
            .ignored();
        // E'\':bind\''
        let c_style_string = just("\\'")
            .or(just("''"))
            .ignored()
            .or(none_of("'").ignored())
            .repeated()
            .delimited_by(just("e'").or(just("E'")), just("'"))
            .ignored();
        // $:bind$:bind$:bind$
        let dollar_tag = just("$").then(none_of("$").repeated()).then(just("$"));
        let dollar_quoted = none_of("$")
            .repeated()
            .delimited_by(dollar_tag.clone(), dollar_tag)
            .ignored();

        c_style_string
            .or(string)
            .or(constant)
            .or(dollar_quoted)
            // Non c_style_string e
            .or(one_of("eE").then(none_of("'").rewind()).ignored())
            // Non binding sql
            .or(none_of("\"':$eE").ignored())
            .repeated()
            .at_least(1)
            .ignored()
    }

    /// Parse all bind from an SQL query
    fn parse_bind() -> impl Parser<char, Vec<Span<String>>, Error = Simple<char>> {
        just(':')
            .ignore_then(ident())
            .separated_by(Self::sql_escaping())
            .allow_leading()
            .allow_trailing()
    }

    fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        none_of(";")
            .repeated()
            .then_ignore(just(';'))
            .collect::<String>()
            .map_with_span(|mut sql_str, span: Range<usize>| {
                let bind_params: Vec<_> = Self::parse_bind().parse(sql_str.clone()).unwrap();
                // Remove duplicate
                let dedup_params: Vec<_> = bind_params
                    .iter()
                    .enumerate()
                    .rev()
                    .filter_map(|(i, u)| (!bind_params[..i].contains(u)).then(|| u.clone()))
                    .rev()
                    .collect();

                for bind_param in bind_params.iter().rev() {
                    let index = dedup_params.iter().position(|bp| bp == bind_param).unwrap();
                    let start = bind_param.span.offset() - 1;
                    let end = start + bind_param.span.len();
                    sql_str.replace_range(start..=end, &format!("${}", index + 1))
                }

                Self {
                    sql_str,
                    bind_params: dedup_params,
                    span: span.into(),
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
            .then_ignore(space())
            .then_ignore(ln())
            .then(QuerySql::parser())
            .map(|(annotation, sql)| Self { annotation, sql })
    }
}

#[derive(Debug)]
pub(crate) enum QueryDataStruct {
    Implicit { idents: Vec<NullableIdent> },
    Named(Span<String>),
}

impl QueryDataStruct {
    pub(crate) fn name_and_fields<'a>(
        &'a self,
        registered_structs: &'a [TypeAnnotation],
        query_name: &Span<String>,
        name_suffix: Option<&str>,
    ) -> (&'a [NullableIdent], Span<String>) {
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
                    .find_map(|it| (it.name == *name).then(|| it.fields.as_slice()))
                    .unwrap_or(&[]),
                name.clone(),
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
    pub(crate) name: Span<String>,
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
pub(crate) struct Module {
    pub(crate) types: Vec<TypeAnnotation>,
    pub(crate) queries: Vec<Query>,
}

impl FromIterator<Statement> for Module {
    fn from_iter<T: IntoIterator<Item = Statement>>(iter: T) -> Self {
        let mut types = Vec::new();
        let mut queries = Vec::new();
        for item in iter {
            match item {
                Statement::Type(it) => types.push(it),
                Statement::Query(it) => queries.push(it),
            }
        }
        Module { types, queries }
    }
}

pub(crate) fn parse_query_module(module_info: &ModuleInfo) -> Result<Module, Error> {
    TypeAnnotation::parser()
        .map(Statement::Type)
        .or(Query::parser().map(Statement::Query))
        .separated_by(blank())
        .allow_leading()
        .allow_trailing()
        .then_ignore(end())
        .collect()
        .parse(module_info.content.as_str())
        .map_err(|e| Error {
            src: module_info.into(),
            err_span: e[0].span().into(),
            help: e[0].to_string().replace('\n', "\\n"),
        })
}

pub(crate) mod error {
    use miette::{Diagnostic, NamedSource, SourceSpan};
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError, Diagnostic)]
    #[error("Couldn't parse queries")]
    pub struct Error {
        #[source_code]
        pub src: NamedSource,

        #[help]
        pub help: String,

        #[label("unexpected token")]
        pub err_span: SourceSpan,
    }
}
