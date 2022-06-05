use std::collections::HashSet;

use error::{Error, ValidationError};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser as Pest;

use self::validate::validate_module;

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

#[derive(Debug, Clone)]
pub(crate) struct ParsedPosition {
    pub(crate) span: (usize, usize),
    pub(crate) line: usize,
    pub(crate) col: usize,
    pub(crate) line_str: String,
}

impl<'a> From<&Pair<'a, Rule>> for ParsedPosition {
    fn from(pair: &Pair<'a, Rule>) -> Self {
        let span = pair.as_span();
        let pos = span.start_pos();
        let (line, col) = pos.line_col();
        let line_str = pos.line_of().to_owned();
        Self {
            line,
            col,
            line_str,
            span: (span.start(), span.end()),
        }
    }
}

impl<T: FromPair> FromPair for Parsed<T> {
    fn from_pair(pair: Pair<Rule>) -> Self {
        Self {
            pos: ParsedPosition::from(&pair),
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
pub(crate) struct Parsed<T> {
    pub(crate) pos: ParsedPosition,
    pub(crate) value: T,
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
            pos: self.pos.clone(),
            value: f(&self.value),
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

#[derive(Debug)]
pub enum NullableIdent {
    Nullable(String),
    NonNullable(String),
}

impl NullableIdent {
    fn ident<'a>(&'a self) -> &'a str {
        match self {
            NullableIdent::Nullable(i) => i,
            NullableIdent::NonNullable(i) => i,
        }
    }
}

impl<'a> FromPair for NullableIdent {
    fn from_pair(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::ident => Self::NonNullable(pair.as_str().into()),
            Rule::nullable => Self::Nullable(pair.as_str().into()),
            _ => unreachable!(),
        }
    }
}

struct TypeAnnotation {
    kind: TypeAnnotationKind,
    ty_name: Parsed<String>,
    fields: Vec<Parsed<NullableIdent>>,
}

impl FromPair for TypeAnnotation {
    fn from_pair(pair: Pair<Rule>) -> Self {
        let mut tokens = pair.into_inner();
        dbg!(&tokens);
        let kind = TypeAnnotationKind::from_pair(tokens.next().unwrap());
        let mut inner_tokens = tokens.next().unwrap().into_inner();
        let ident = Parsed::<String>::from_pair(inner_tokens.next().unwrap());
        let fields = inner_tokens
            .next()
            .unwrap()
            .into_inner()
            .map(Parsed::<NullableIdent>::from_pair)
            .collect();
        Self {
            kind,
            ty_name: ident,
            fields,
        }
    }
}

#[derive(Debug)]
pub(crate) struct TypeAnnotationListItem {
    pub(crate) ty_name: Parsed<String>,
    pub(crate) fields: Vec<Parsed<NullableIdent>>,
}

impl FromPair for TypeAnnotationListItem {
    fn from_pair(pair: Pair<Rule>) -> Self {
        dbg!(&pair);
        let rule = pair.as_rule();
        let mut tokens = pair.into_inner();
        let ident = Parsed::<String>::from_pair(tokens.next().unwrap());
        let mut fields = Vec::new();
        if let Rule::type_with_nullable_cols = rule {
            fields = tokens
                .next()
                .unwrap()
                .into_inner()
                .map(Parsed::<NullableIdent>::from_pair)
                .collect();
        }

        Self {
            ty_name: ident,
            fields,
        }
    }
}

struct TypeAnnotationList {
    kind: TypeAnnotationKind,
    types: Vec<TypeAnnotationListItem>,
}

impl FromPair for TypeAnnotationList {
    fn from_pair(pair: Pair<Rule>) -> Self {
        let mut tokens = pair.into_inner();
        dbg!(&tokens);
        let kind = TypeAnnotationKind::from_pair(tokens.next().unwrap());
        let types = tokens
            .next()
            .unwrap()
            .into_inner()
            .map(TypeAnnotationListItem::from_pair)
            .collect();
        Self { kind, types }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BindParameter {
    PgCompatible(usize),
    Extended(String),
}

impl FromPair for BindParameter {
    fn from_pair(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::number => BindParameter::PgCompatible(pair.as_str().parse::<usize>().unwrap()),
            Rule::ident => BindParameter::Extended(pair.as_str().to_string()),
            _ => {
                unreachable!()
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct QuerySql {
    pub(crate) sql_str: String,
    pub(crate) bind_params: Vec<Parsed<BindParameter>>,
}

impl QuerySql {
    pub(crate) fn normalize_sql(self, sql_start: usize) -> String {
        let mut deduped_bind_params = self.bind_params.clone();
        deduped_bind_params.sort();
        deduped_bind_params.dedup();

        let mut replacing_values = self
            .bind_params
            .iter()
            .map(|bind_param| {
                let index = deduped_bind_params
                    .iter()
                    .position(|bp| bp == bind_param)
                    .unwrap();
                let start = bind_param.pos.span.0 - sql_start - 1_usize;
                let end = bind_param.pos.span.1 - sql_start - 1_usize;
                ((start, end), format!("{}", index + 1))
            })
            .collect::<Vec<((usize, usize), String)>>();
        replaced_in_string(self.sql_str, &mut replacing_values)
    }
}

impl FromPair for QuerySql {
    fn from_pair(pair: Pair<Rule>) -> Self {
        let sql_str = pair.as_str().into();
        let mut bind_params: Vec<Parsed<BindParameter>> = pair
            .into_inner()
            .map(Parsed::<BindParameter>::from_pair)
            .collect();

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
    pub(crate) sql_start: usize,
}

impl FromPair for Query {
    fn from_pair(pair: Pair<Rule>) -> Self {
        let mut tokens = pair.into_inner();
        let annotation = QueryAnnotation::from_pair(tokens.next().unwrap());
        let sql_tokens = tokens.next().unwrap();
        let sql_start = sql_tokens.as_span().start();
        let sql = QuerySql::from_pair(sql_tokens);
        Self {
            annotation,
            sql,
            sql_start,
        }
    }
}

#[derive(Debug)]
pub(crate) enum QueryDataStructure {
    Implicit { idents: Vec<Parsed<NullableIdent>> },
    Named(Parsed<String>),
}

impl Default for QueryDataStructure {
    fn default() -> Self {
        Self::Implicit { idents: Vec::new() }
    }
}

impl FromPair for QueryDataStructure {
    fn from_pair(pair: Pair<Rule>) -> Self {
        dbg!(&pair.as_rule());
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::ident => QueryDataStructure::Named(Parsed::<String>::from_pair(pair)),
            Rule::query_field_list => {
                let idents = pair
                    .into_inner()
                    .map(Parsed::<NullableIdent>::from_pair)
                    .collect();
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
pub(crate) struct Module {
    pub(crate) param_types: Vec<TypeAnnotationListItem>,
    pub(crate) row_types: Vec<TypeAnnotationListItem>,
    pub(crate) db_types: Vec<TypeAnnotationListItem>,
    pub(crate) queries: Vec<Query>,
}

impl FromPair for Module {
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
                    let ty_item = TypeAnnotationListItem { ty_name, fields };
                    match kind {
                        TypeAnnotationKind::Param => param_types.push(ty_item),
                        TypeAnnotationKind::Row => row_types.push(ty_item),
                        TypeAnnotationKind::Db => db_types.push(ty_item),
                    }
                }
                Rule::type_annotation_list => {
                    let TypeAnnotationList { kind, mut types } = TypeAnnotationList::from_pair(it);
                    match kind {
                        TypeAnnotationKind::Param => param_types.append(&mut types),
                        TypeAnnotationKind::Row => row_types.append(&mut types),
                        TypeAnnotationKind::Db => db_types.append(&mut types),
                    }
                }
                Rule::query => {
                    queries.push(Query::from_pair(it));
                }
                _ => unreachable!(),
            }
        }

        Module {
            param_types,
            row_types,
            db_types,
            queries,
        }
    }
}

/// Holds all the data known to Cornucopia about a query after parsing it.
/// The query is not yet fully known though, as it has not yet been prepared.
#[derive(Debug, Clone)]
pub(crate) struct ParsedQuery {
    pub(crate) line: usize,
    pub(crate) name: Parsed<String>,
    pub(crate) named_param_struct: Option<Parsed<String>>,
    pub(crate) params: Vec<Parsed<String>>,
    pub(crate) named_return_struct: Option<Parsed<String>>,
    pub(crate) nullable_columns: Vec<Parsed<String>>,
    pub(crate) sql_str: String,
}

#[derive(Debug)]
pub(crate) enum ValidatedQuery {
    PgCompatible {
        name: Parsed<String>,
        params: Vec<Parsed<NullableIdent>>,
        row: Vec<Parsed<NullableIdent>>,
        sql_str: String,
    },
    Extended {
        name: Parsed<String>,
        params: QueryDataStructure,
        row: QueryDataStructure,
        sql_str: String,
    },
}

impl ValidatedQuery {
    pub(crate) fn name(&self) -> &Parsed<String> {
        match self {
            ValidatedQuery::PgCompatible { name, .. } => name,
            ValidatedQuery::Extended { name, .. } => name,
        }
    }
    pub(crate) fn sql_str(&self) -> &String {
        match self {
            ValidatedQuery::PgCompatible { sql_str, .. } => sql_str,
            ValidatedQuery::Extended { sql_str, .. } => sql_str,
        }
    }
}

#[derive(Debug)]
pub(crate) struct ValidatedModule {
    pub(crate) param_types: Vec<TypeAnnotationListItem>,
    pub(crate) row_types: Vec<TypeAnnotationListItem>,
    pub(crate) db_types: Vec<TypeAnnotationListItem>,
    pub(crate) queries: Vec<ValidatedQuery>,
}

/// Parse queries in in the input string using the grammar file (`grammar.pest`).
pub(crate) fn parse_query_module(input: &str) -> Result<ValidatedModule, Error> {
    let x = CornucopiaParser::parse(Rule::parser, input)
        .unwrap()
        .next()
        .unwrap();

    let y = Module::from_pair(x);

    validate_module(y)
}

/// Utility that replaces all the replacing values into the target string.
fn replaced_in_string(mut s: String, replacing_values: &mut [((usize, usize), String)]) -> String {
    replacing_values.sort_by(|a, b| a.0 .0.cmp(&b.0 .0));
    for ((start, end), value) in replacing_values.iter().rev() {
        s.replace_range(start..=end, value)
    }
    s
}

pub(crate) mod validate {
    use crate::utils::has_duplicate;

    use super::{
        error::{Error, ValidationError},
        BindParameter, Module, NullableIdent, Parsed, ParsedPosition, ParsedQuery, Query,
        QueryAnnotation, QueryDataStructure, ValidatedModule, ValidatedQuery,
    };

    fn ambiguous_bind_param(bind_params: &[Parsed<BindParameter>]) -> Result<bool, Error> {
        // We're taking the first bind parameter as the gauge of what syntax is used.
        // This is pretty ad-hoc, it might worthwhile to add an explicit syntax marker (or smth similar).
        let syntax_is_extended = bind_params
            .get(0)
            .map(|bind_param| matches!(bind_param.value, BindParameter::Extended(_)))
            .unwrap_or(true);
        for bind_param in bind_params {
            if !(syntax_is_extended && matches!(bind_param.value, BindParameter::Extended(_))) {
                return Err(Error::Validation(ValidationError::AmbiguousBindParam {
                    pos: bind_param.pos.clone(),
                }));
            }
        }

        Ok(syntax_is_extended)
    }

    fn duplicate_nullable_ident(idents: &[Parsed<NullableIdent>]) -> Result<(), Error> {
        if let Some(dup) = has_duplicate(idents, |p| p.value.ident()) {
            return Err(Error::Validation(ValidationError::DuplicateParam {
                pos: dup.pos.clone(),
            }));
        }
        Ok(())
    }

    pub(crate) fn named_struct_in_pg_query(
        annotation: QueryAnnotation,
    ) -> Result<(Vec<Parsed<NullableIdent>>, Vec<Parsed<NullableIdent>>), Error> {
        if let QueryDataStructure::Named(name) = annotation.param {
            return Err(Error::Validation(ValidationError::NamedStructInPgQuery {
                pos: name.pos,
            }));
        };
        if let QueryDataStructure::Named(name) = annotation.row {
            return Err(Error::Validation(ValidationError::NamedStructInPgQuery {
                pos: name.pos,
            }));
        };

        let param = match annotation.param {
            QueryDataStructure::Implicit { idents } => idents,
            QueryDataStructure::Named(_) => unreachable!(),
        };
        let row = match annotation.row {
            QueryDataStructure::Implicit { idents } => idents,
            QueryDataStructure::Named(_) => unreachable!(),
        };
        Ok((param, row))
    }

    fn more_bind_params_than_params(
        params: &[Parsed<NullableIdent>],
        deduped_bind_params: &[Parsed<i16>],
    ) -> Result<(), Error> {
        let params_len = params.len();
        if let Some(bind_param) = deduped_bind_params
            .iter()
            .find(|bind_param| bind_param.value as usize > params_len)
        {
            return Err(Error::Validation(
                ValidationError::MoreBindParamsThanParams {
                    nb_params: params.len(),
                    pos: bind_param.pos.clone(),
                },
            ));
        }
        Ok(())
    }

    fn unused_param(
        params: &[Parsed<NullableIdent>],
        bind_params: &[Parsed<i16>],
    ) -> Result<(), Error> {
        if let Some((index, p)) = params.iter().enumerate().find(|(index, _)| {
            !bind_params
                .iter()
                .any(|bind_index| bind_index.value as usize == *index + 1)
        }) {
            return Err(Error::Validation(ValidationError::UnusedParam {
                index,
                pos: p.pos.clone(),
            }));
        };
        Ok(())
    }

    pub(crate) fn i16_index(
        Parsed { pos, value }: Parsed<BindParameter>,
    ) -> Result<Parsed<i16>, Error> {
        let usize_index = match value {
            BindParameter::PgCompatible(index) => index,
            BindParameter::Extended(_) => unreachable!(),
        };
        // Check that the index can be parsed as a i16 (required by postgres wire protocol)
        let i16_index = i16::try_from(usize_index).map_err(|_| {
            Error::Validation(ValidationError::InvalidI16Index { pos: pos.clone() })
        })?;

        // Check that the index is also non-zero (postgres bind params are 1-indexed)
        if i16_index == 0 {
            return Err(Error::Validation(ValidationError::InvalidI16Index { pos }));
        };

        Ok(Parsed {
            pos,
            value: i16_index,
        })
    }

    fn query_name_already_used(queries: &[Query]) -> Result<(), Error> {
        for (i, query) in queries.iter().enumerate() {
            if let Some((_, q)) = queries
                .iter()
                .enumerate()
                .find(|(j, q)| *j != i && q.annotation.name == query.annotation.name)
            {
                return Err(Error::Validation(ValidationError::QueryNameAlreadyUsed {
                    name1: query.annotation.name.clone(),
                    name2: q.annotation.name.clone(),
                }));
            }
        }

        has_duplicate(queries.iter(), |q| &q.annotation.name);

        Ok(())
    }

    pub(crate) fn validate_query(query: Query) -> Result<ValidatedQuery, Error> {
        if let QueryDataStructure::Implicit { idents } = &query.annotation.param {
            duplicate_nullable_ident(idents)?;
        };
        if let QueryDataStructure::Implicit { idents } = &query.annotation.row {
            duplicate_nullable_ident(idents)?;
        };
        let name = query.annotation.name.clone();
        let is_extended_syntax = ambiguous_bind_param(&query.sql.bind_params)?;
        let validated_query = if is_extended_syntax {
            let sql_str = query.sql.normalize_sql(query.sql_start);
            ValidatedQuery::Extended {
                name: query.annotation.name,
                params: query.annotation.param,
                row: query.annotation.row,
                sql_str,
            }
        } else {
            let bind_params = &query
                .sql
                .bind_params
                .into_iter()
                .map(|bind_param| Ok(i16_index(bind_param)?))
                .collect::<Result<Vec<Parsed<i16>>, Error>>()?;
            let mut deduped_bind_params = bind_params.clone();
            deduped_bind_params.sort();
            deduped_bind_params.dedup();

            let (params, row) = named_struct_in_pg_query(query.annotation)?;

            more_bind_params_than_params(&params, &deduped_bind_params)?;
            unused_param(&params, &bind_params)?;

            ValidatedQuery::PgCompatible {
                name,
                params,
                row,
                sql_str: query.sql.sql_str,
            }
        };

        Ok(validated_query)
    }

    pub(crate) fn validate_module(module: Module) -> Result<ValidatedModule, Error> {
        query_name_already_used(&module.queries)?;
        for ty in module
            .param_types
            .iter()
            .chain(module.row_types.iter())
            .chain(module.db_types.iter())
        {
            duplicate_nullable_ident(&ty.fields)?;
        }
        let mut validated_queries = Vec::new();
        for query in module.queries {
            validated_queries.push(validate_query(query)?);
        }
        Ok(ValidatedModule {
            param_types: module.param_types,
            row_types: module.row_types,
            db_types: module.db_types,
            queries: validated_queries,
        })
    }
}

pub(crate) mod error {
    use crate::prepare_queries::PreparedField;

    use super::{BindParameter, Parsed, ParsedPosition, Rule};
    use std::fmt::Display;

    #[derive(Debug)]
    pub(crate) enum Error {
        Parser(pest::error::Error<Rule>),
        Validation(ValidationError),
    }

    #[derive(Debug)]
    pub(crate) enum ValidationError {
        AmbiguousBindParam {
            pos: ParsedPosition,
        },
        InvalidI16Index {
            pos: ParsedPosition,
        },
        DuplicateParam {
            pos: ParsedPosition,
        },
        MoreBindParamsThanParams {
            nb_params: usize,
            pos: ParsedPosition,
        },
        UnusedParam {
            index: usize,
            pos: ParsedPosition,
        },
        InvalidNullableColumnName {
            name: String,
            pos: ParsedPosition,
        },
        NamedStructInvalidFields {
            expected: Vec<PreparedField>,
            actual: Vec<PreparedField>,
            name: String,
            pos: ParsedPosition,
        },
        QueryNameAlreadyUsed {
            name1: Parsed<String>,
            name2: Parsed<String>,
        },
        NamedStructInPgQuery {
            pos: ParsedPosition,
        },
    }

    impl Display for ValidationError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ValidationError::InvalidI16Index { pos } => {
                    let msg = ["Index must be between 1 and 32767."];
                    write!(f, "{}", format_err(pos, &msg))
                }
                ValidationError::DuplicateParam { pos } => {
                    let msg = ["Parameter is already used in parameter list."];
                    write!(f, "{}", format_err(pos, &msg))
                }
                ValidationError::MoreBindParamsThanParams { pos, nb_params } => {
                    let msg = format!(
                        "Index is higher than the number of parameters supplied ({nb_params})."
                    );
                    write!(f, "{}", format_err(pos, &[&msg]))
                }
                ValidationError::UnusedParam { pos, index } => {
                    let msg = format!("Parameter `${index}` is never used in the query.");
                    write!(f, "{}", format_err(pos, &[&msg]))
                }
                ValidationError::InvalidNullableColumnName { name, pos } => {
                    let msg = format!("No column named `{name}` found for this query.");
                    write!(f, "{}", format_err(pos, &[&msg]))
                }
                // Move into another module
                ValidationError::NamedStructInvalidFields {
                    name,
                    pos,
                    expected,
                    actual,
                } => {
                    let msg1 = format!("This query's named row struct `{name}` has already been used, but the fields don't match.");
                    let msg2 = format!("Expected fields: {expected:#?}");
                    let msg3 = format!("Got fields: {actual:#?}");
                    write!(f, "{}", format_err(pos, &[&msg1, &msg2, &msg3]))
                }
                ValidationError::QueryNameAlreadyUsed { name1, name2 } => {
                    let msg1 = format!("A query named `{}` already exists.", name1.value);
                    let msg2 = format!("Query `{}` first defined here.", name2.value);
                    write!(
                        f,
                        "{}\n{}",
                        format_err(&name1.pos, &[&msg1]),
                        format_err(&name2.pos, &[&msg2])
                    )
                }
                ValidationError::AmbiguousBindParam { pos } => {
                    let msg = [
                                "Cannot mix bind parameter syntaxes in the same query.", 
                                "Please use either named (`:named_ident`) or indexed (`$n`) bind parameters, but not both."
                            ];
                    write!(f, "{}", format_err(pos, &msg))
                }
                ValidationError::NamedStructInPgQuery { pos } => {
                    let msg = ["Named query structs are not allowed when using the PostgreSQL-compatible syntax.",
                    "Either use anonymous structs instead, or use the extended query syntax."];
                    write!(f, "{}", format_err(pos, &msg))
                }
            }
        }
    }
    impl std::error::Error for ValidationError {}

    fn format_err(
        ParsedPosition {
            line,
            col,
            line_str,
            ..
        }: &ParsedPosition,
        messages: &[&str],
    ) -> String {
        let msg = messages.join("\n  = ");
        let line_str = line_str.trim_end();
        let cursor = format!("{}^---", " ".repeat(col - 1));
        format!(" --> {line}:{col}\n  | \n  | {line_str}\n  | {cursor}\n  | \n  = {msg}")
    }

    impl Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &self {
                Error::Parser(e) => write!(f, "{e}"),
                Error::Validation(e) => write!(f, "{e}"),
            }
        }
    }

    impl std::error::Error for Error {}
}

#[cfg(test)]
mod test {
    use pest::Parser;

    use crate::parser::{FromPair, Module};

    use super::{CornucopiaParser, Rule};

    #[test]
    fn test() {
        let input = r#"
--: ROW Hello(a,b?)

--: PARAM (
--:     hello, world()
--: )

--! query (first?, second?, third)
asd

--! query : (first?, second?)
asd $1

--! query (first?, second?, third) : (first?, second?)
asd :first
        "#;

        let x = CornucopiaParser::parse(Rule::parser, input)
            .unwrap()
            .next()
            .unwrap();

        let y = Module::from_pair(x);

        println!("{:#?}", &y);
    }
}
