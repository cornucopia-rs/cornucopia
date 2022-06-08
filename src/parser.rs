use std::fmt::Display;

use error::Error;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser as Pest;

#[derive(Pest)]
#[grammar = "../grammar.pest"]
struct CornucopiaParser;

trait FromPair {
    fn from_pair(pair: Pair<Rule>) -> Self;
}

impl FromPair for String {
    fn from_pair(pair: Pair<Rule>) -> Self {
        pair.as_str().to_string()
    }
}

impl<T: FromPair> FromPair for Parsed<T> {
    fn from_pair(pair: Pair<Rule>) -> Self {
        Self {
            start: pair.as_span().start(),
            end: pair.as_span().end(),
            value: T::from_pair(pair),
        }
    }
}

impl<T: std::hash::Hash> std::hash::Hash for Parsed<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

/// Th    if is data structure holds a value and the context in which it was parsed.
/// This context is used for error reporting.
#[derive(Debug, Clone)]
pub struct Parsed<T> {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) value: T,
}

impl<T: Display> Display for Parsed<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T: PartialEq> PartialEq<Self> for Parsed<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
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
    pub(crate) fn map<U>(&self, f: fn(&T) -> U) -> Parsed<U> {
        Parsed {
            value: f(&self.value),
            start: self.start,
            end: self.end,
        }
    }
}

enum TypeAnnotationKind {
    Param,
    Row,
    Db,
}

impl<'a> FromPair for TypeAnnotationKind {
    fn from_pair(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::row => Self::Row,
            Rule::param => Self::Param,
            Rule::db => Self::Db,
            _ => unreachable!(),
        }
    }
}

struct TypeAnnotation {
    kind: TypeAnnotationKind,
    ty_name: Parsed<String>,
    fields: Vec<Parsed<String>>,
}

impl FromPair for TypeAnnotation {
    fn from_pair(pair: Pair<Rule>) -> Self {
        let mut tokens = pair.into_inner();
        let kind = TypeAnnotationKind::from_pair(tokens.next().unwrap());
        let mut inner_tokens = tokens.next().unwrap().into_inner();
        let ident = Parsed::<String>::from_pair(inner_tokens.next().unwrap());
        let fields = if let Some(x) = inner_tokens.next() {
            x.into_inner().map(Parsed::<String>::from_pair).collect()
        } else {
            Vec::new()
        };
        Self {
            kind,
            ty_name: ident,
            fields,
        }
    }
}

#[derive(Debug)]
pub(crate) struct TypeDataStructure {
    pub(crate) name: Parsed<String>,
    pub(crate) fields: Vec<Parsed<String>>,
}

#[derive(Debug)]
pub(crate) struct QuerySql {
    pub(crate) sql_str: String,
    pub(crate) bind_params: Vec<Parsed<String>>,
}

impl FromPair for QuerySql {
    fn from_pair(pair: Pair<Rule>) -> Self {
        let sql_start = pair.as_span().start();
        let mut sql_str = pair.as_str().to_string();
        let bind_params: Vec<Parsed<String>> =
            pair.into_inner().map(Parsed::<String>::from_pair).collect();

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
    }
}

#[derive(Debug)]
pub(crate) struct Query {
    pub(crate) annotation: QueryAnnotation,
    pub(crate) sql: QuerySql,
}

impl FromPair for Query {
    fn from_pair(pair: Pair<Rule>) -> Self {
        let mut tokens = pair.into_inner();
        let annotation = QueryAnnotation::from_pair(tokens.next().unwrap());
        let sql_tokens = tokens.next().unwrap();
        let sql = QuerySql::from_pair(sql_tokens);
        Self { annotation, sql }
    }
}

#[derive(Debug)]
pub(crate) enum QueryDataStructure {
    Implicit { idents: Vec<Parsed<String>> },
    Named(Parsed<String>),
}

impl Default for QueryDataStructure {
    fn default() -> Self {
        Self::Implicit { idents: Vec::new() }
    }
}

impl FromPair for QueryDataStructure {
    fn from_pair(pair: Pair<Rule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::ident => QueryDataStructure::Named(Parsed::<String>::from_pair(pair)),
            Rule::field_list => {
                let idents = pair.into_inner().map(Parsed::<String>::from_pair).collect();
                QueryDataStructure::Implicit { idents }
            }
            _ => {
                unreachable!()
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct QueryAnnotation {
    pub(crate) name: Parsed<String>,
    pub(crate) param: QueryDataStructure,
    pub(crate) row: QueryDataStructure,
}

impl FromPair for QueryAnnotation {
    fn from_pair(pair: Pair<Rule>) -> Self {
        let mut tokens = pair.into_inner();
        let name = Parsed::<String>::from_pair(tokens.next().unwrap());
        let (mut param, mut row) = <(QueryDataStructure, QueryDataStructure)>::default();
        for it in tokens {
            match it.as_rule() {
                Rule::query_param => param = QueryDataStructure::from_pair(it),
                Rule::query_row => row = QueryDataStructure::from_pair(it),
                _ => {
                    unreachable!()
                }
            }
        }
        Self { name, param, row }
    }
}

#[derive(Debug)]
pub(crate) struct ParsedModule {
    pub(crate) param_types: Vec<TypeDataStructure>,
    pub(crate) row_types: Vec<TypeDataStructure>,
    pub(crate) db_types: Vec<TypeDataStructure>,
    pub(crate) queries: Vec<Query>,
}

impl FromPair for ParsedModule {
    fn from_pair(pair: Pair<Rule>) -> Self {
        let mut param_types = Vec::new();
        let mut row_types = Vec::new();
        let mut db_types = Vec::new();
        let mut queries = Vec::new();
        for it in pair.into_inner() {
            match it.as_rule() {
                Rule::type_annotation => {
                    let TypeAnnotation {
                        kind,
                        ty_name,
                        fields,
                    } = TypeAnnotation::from_pair(it);
                    let ty_item = TypeDataStructure {
                        name: ty_name,
                        fields,
                    };

                    match kind {
                        TypeAnnotationKind::Param => param_types.push(ty_item),
                        TypeAnnotationKind::Row => row_types.push(ty_item),
                        TypeAnnotationKind::Db => db_types.push(ty_item),
                    }
                }
                Rule::query => {
                    queries.push(Query::from_pair(it));
                }
                _ => unreachable!(),
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
pub(crate) fn parse_query_module(module_path: &str, input: &str) -> Result<ParsedModule, Error> {
    let parsed = CornucopiaParser::parse(Rule::parser, input)
        .map_err(|e| Error {
            err: e,
            path: module_path.to_owned(),
        })?
        .next()
        .unwrap();
    Ok(ParsedModule::from_pair(parsed))
}

pub(crate) mod error {

    use super::Rule;
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    #[error("Error while parsing queries [path: \"{path}\"]:\n{err}.")]
    pub struct Error {
        pub path: String,
        pub err: pest::error::Error<Rule>,
    }
}
