use std::{fmt::Display, ops::Range};

use chumsky::prelude::*;
use error::Error;
use heck::ToUpperCamelCase;
use miette::SourceSpan;

use crate::read_queries::ModuleInfo;

/// This data structure holds a value and the context in which it was parsed.
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

fn plain_ident<'src>() -> impl Parser<'src, &'src str, Span<String>, extra::Err<Simple<'src, char>>>
{
    any::<&'src str, _>()
        .filter(|c: &char| c.is_ascii_alphanumeric() || *c == '_')
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map_with(|value, e| {
            let span: SimpleSpan = e.span();
            let range: Range<usize> = span.start()..span.end();

            Span {
                value,
                span: range.into(),
            }
        })
}

fn quoted_ident<'src>() -> impl Parser<'src, &'src str, Span<String>, extra::Err<Simple<'src, char>>>
{
    none_of('"')
        .repeated()
        .at_least(1)
        .collect::<String>()
        .delimited_by(just('"'), just('"'))
        .map_with(|value, e| {
            let span: SimpleSpan = e.span();
            let range: Range<usize> = span.start()..span.end();

            Span {
                value,
                span: range.into(),
            }
        })
}

fn ident<'src>() -> impl Parser<'src, &'src str, Span<String>, extra::Err<Simple<'src, char>>> {
    plain_ident().or(quoted_ident())
}

fn ln<'src>() -> impl Parser<'src, &'src str, (), extra::Err<Simple<'src, char>>> {
    just("\n").or(just("\n\r")).ignored()
}

fn space<'src>() -> impl Parser<'src, &'src str, (), extra::Err<Simple<'src, char>>> {
    any::<&'src str, _>()
        .filter(|c: &char| c.is_whitespace() && *c != '\n')
        .repeated()
        .ignored()
}

fn blank<'src>() -> impl Parser<'src, &'src str, (), extra::Err<Simple<'src, char>>> {
    // We want to escape valid SQL comment beginning with -- while not escaping our syntax --: or --!
    let comment = just("--")
        .then(none_of(":!#").rewind())
        .then(none_of('\n').repeated());

    any::<&'src str, _>()
        .filter(|c: &char| c.is_whitespace())
        .ignored()
        .or(comment.ignored())
        .repeated()
        .ignored()
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldSegment {
    pub name: Span<String>,
    pub nullable: bool,
    pub is_array: bool,
}

#[derive(Debug, Clone)]
pub struct NullableIdent {
    pub name: Span<String>,
    pub nullable: bool,
    pub inner_nullable: bool,
    pub nested_fields: Vec<FieldSegment>,
}

impl NullableIdent {
    /// Get all field nullability specifications for this identifier
    pub fn get_field_nullability(&self) -> impl Iterator<Item = (&str, bool)> + '_ {
        self.nested_fields
            .iter()
            .map(|segment| (segment.name.value.as_str(), segment.nullable))
    }
}

fn parse_field_segment<'src>()
-> impl Parser<'src, &'src str, FieldSegment, extra::Err<Simple<'src, char>>> {
    ident()
        .then(just('?').or_not())
        .map(|(name, nullable)| FieldSegment {
            name,
            nullable: nullable.is_some(),
            is_array: false,
        })
}

fn parse_nullable_ident<'src>()
-> impl Parser<'src, &'src str, Vec<NullableIdent>, extra::Err<Simple<'src, char>>> {
    let single_ident = space()
        .ignore_then(
            ident()
                .then(just('?').or_not())
                .then(just("[?]").or_not())
                .then(
                    // Parse nested field paths like .field or [].field
                    choice((
                        // Handle [].field? syntax for array element field access
                        just("[]")
                            .ignore_then(just('.'))
                            .ignore_then(parse_field_segment())
                            .map(|segment| (segment, true)), // true indicates this is an array access
                        // Handle .field? syntax for direct field access
                        just('.')
                            .ignore_then(parse_field_segment())
                            .map(|segment| (segment, false)), // false indicates this is direct access
                    ))
                    .repeated()
                    .collect::<Vec<_>>(),
                )
                .map(|(((name, nullable), array_nullable), nested_accesses)| {
                    let mut nested_fields = Vec::new();

                    for (mut segment, is_array_access) in nested_accesses {
                        segment.is_array = is_array_access;
                        nested_fields.push(segment);
                    }

                    NullableIdent {
                        name,
                        nullable: nullable.is_some(),
                        inner_nullable: array_nullable.is_some(),
                        nested_fields,
                    }
                }),
        )
        .then_ignore(space());

    single_ident
        .separated_by(just(','))
        .allow_trailing()
        .collect::<Vec<_>>()
        .delimited_by(just('('), just(')'))
}

#[derive(Debug, Clone)]
pub struct TypeAnnotation {
    pub name: Span<String>,
    pub fields: Vec<NullableIdent>,
    pub traits: Vec<String>,
    pub attributes: Vec<String>,
    pub attributes_borrowed: Vec<String>,
}

impl TypeAnnotation {
    fn path_ident<'src>()
    -> impl Parser<'src, &'src str, Span<String>, extra::Err<Simple<'src, char>>> {
        let path_segment = any::<&'src str, _>()
            .filter(|c: &char| c.is_ascii_alphanumeric() || *c == '_')
            .repeated()
            .at_least(1)
            .collect::<String>();

        path_segment
            .separated_by(just("::"))
            .at_least(1)
            .collect::<Vec<_>>()
            .map(|segments| segments.join("::"))
            .map_with(|value, e| {
                let span: SimpleSpan = e.span();
                let range: Range<usize> = span.start()..span.end();

                Span {
                    value,
                    span: range.into(),
                }
            })
    }

    fn parse_attributes<'src>()
    -> impl Parser<'src, &'src str, Vec<String>, extra::Err<Simple<'src, char>>> {
        just("--#")
            .ignore_then(space())
            .ignore_then(none_of('\n').repeated().collect::<String>())
            .map(|s| s.trim().to_string())
            .then_ignore(ln())
            .repeated()
            .collect()
    }

    fn parse_attributes_borrowed<'src>()
    -> impl Parser<'src, &'src str, Vec<String>, extra::Err<Simple<'src, char>>> {
        just("--&")
            .ignore_then(space())
            .ignore_then(none_of('\n').repeated().collect::<String>())
            .map(|s| s.trim().to_string())
            .then_ignore(ln())
            .repeated()
            .collect()
    }

    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<Simple<'src, char>>> {
        let trait_parser = Self::path_ident()
            .map(|s: Span<String>| s.value)
            .separated_by(just(',').padded())
            .collect::<Vec<_>>();

        just("--:")
            .ignore_then(space())
            .ignore_then(ident())
            .then_ignore(space())
            .then(parse_nullable_ident())
            .then_ignore(space())
            .then(
                just(':')
                    .ignore_then(space())
                    .ignore_then(trait_parser)
                    .or_not()
                    .map(|opt| opt.unwrap_or_default()),
            )
            .then_ignore(space())
            .then_ignore(ln())
            .then(Self::parse_attributes())
            .then(Self::parse_attributes_borrowed())
            .map(
                |((((name, fields), traits), attributes), attributes_borrowed)| Self {
                    name,
                    fields,
                    traits,
                    attributes,
                    attributes_borrowed,
                },
            )
    }
}

#[derive(Debug)]
pub(crate) struct Query {
    pub(crate) name: Span<String>,
    pub(crate) param: QueryDataStruct,
    pub(crate) comments: Vec<String>,
    pub(crate) row: QueryDataStruct,
    pub(crate) sql_span: SourceSpan,
    pub(crate) sql_str: String,
    pub(crate) bind_params: Vec<Span<String>>,
    pub(crate) attributes: Vec<String>,
}

/// Tracks SQL lexical context (strings, identifiers, comments, dollar quotes)
/// to distinguish characters that are part of SQL syntax from those inside
/// quoted or commented regions.
struct SqlScanner {
    chars: Vec<char>,
    i: usize,
    in_string: bool,
    in_escape_string: bool,
    in_identifier: bool,
    in_comment: bool,
    in_dollar_quote: bool,
    dollar_tag: String,
}

impl SqlScanner {
    fn new(sql: &str) -> Self {
        Self {
            chars: sql.chars().collect(),
            i: 0,
            in_string: false,
            in_escape_string: false,
            in_identifier: false,
            in_comment: false,
            in_dollar_quote: false,
            dollar_tag: String::new(),
        }
    }

    fn in_context(&self) -> bool {
        self.in_string || self.in_identifier || self.in_comment || self.in_dollar_quote
    }

    /// Process the current character for context transitions.
    /// Returns `true` if the character was consumed by context handling
    /// (the caller should loop to the next character).
    /// Returns `false` if the character is outside any context and not a
    /// context-switching character (the caller should handle it and advance `i`).
    fn process_context(&mut self) -> bool {
        let i = self.i;
        let c = self.chars[i];

        match c {
            '\'' => {
                if !self.in_comment && !self.in_dollar_quote {
                    if self.in_string {
                        if i + 1 < self.chars.len() && self.chars[i + 1] == '\'' {
                            self.i += 2;
                            return true;
                        }
                        self.in_string = false;
                        self.in_escape_string = false;
                    } else if !self.in_identifier {
                        self.in_string = true;
                    }
                }
                self.i += 1;
                true
            }

            'e' | 'E' => {
                if !self.in_context() && i + 1 < self.chars.len() && self.chars[i + 1] == '\'' {
                    self.i += 2;
                    self.in_string = true;
                    self.in_escape_string = true;
                    return true;
                }
                if self.in_context() {
                    self.i += 1;
                    return true;
                }
                false
            }

            '\\' => {
                if self.in_escape_string && i + 1 < self.chars.len() {
                    self.i += 2;
                    return true;
                }
                if self.in_context() {
                    self.i += 1;
                    return true;
                }
                false
            }

            '"' => {
                if !self.in_string && !self.in_comment && !self.in_dollar_quote {
                    if self.in_identifier {
                        if i + 1 < self.chars.len() && self.chars[i + 1] == '"' {
                            self.i += 2;
                            return true;
                        }
                        self.in_identifier = false;
                    } else {
                        self.in_identifier = true;
                    }
                }
                self.i += 1;
                true
            }

            '-' => {
                if !self.in_context() && i + 1 < self.chars.len() && self.chars[i + 1] == '-' {
                    self.i += 2;
                    self.in_comment = true;
                    return true;
                }
                if self.in_context() {
                    self.i += 1;
                    return true;
                }
                false
            }

            '\n' => {
                if self.in_comment {
                    self.in_comment = false;
                    self.i += 1;
                    return true;
                }
                if self.in_context() {
                    self.i += 1;
                    return true;
                }
                false
            }

            '$' => {
                if !self.in_context() {
                    let tag_start = i + 1;
                    let mut tag_end = tag_start;

                    while tag_end < self.chars.len()
                        && (self.chars[tag_end].is_alphanumeric() || self.chars[tag_end] == '_')
                    {
                        tag_end += 1;
                    }

                    if tag_end < self.chars.len() && self.chars[tag_end] == '$' {
                        self.dollar_tag = self.chars[tag_start..tag_end].iter().collect();
                        self.in_dollar_quote = true;
                        self.i = tag_end + 1;
                        return true;
                    }
                    return false;
                }
                if self.in_dollar_quote {
                    let tag_length = self.dollar_tag.len();

                    if i + 1 + tag_length < self.chars.len() {
                        let potential_end: String =
                            self.chars[i + 1..i + 1 + tag_length].iter().collect();

                        if potential_end == self.dollar_tag && self.chars[i + 1 + tag_length] == '$'
                        {
                            self.in_dollar_quote = false;
                            self.i = i + tag_length + 2;
                            self.dollar_tag.clear();
                            return true;
                        }
                    }
                }
                self.i += 1;
                true
            }

            _ => {
                if self.in_context() {
                    self.i += 1;
                    return true;
                }
                false
            }
        }
    }
}

impl Query {
    /// Remove all comments from a query
    fn clean_sql_comments(sql: &str) -> String {
        let mut result = String::new();
        let mut chars = sql.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                // Preserve everything in quotes
                '$' | '\'' | '"' => {
                    let mut content = String::from(c);
                    let end_marker = if c == '$' {
                        // For dollar quotes, read until $ and use that as tag
                        for x in chars.by_ref() {
                            content.push(x);
                            if x == '$' {
                                break;
                            }
                        }
                        content.clone()
                    } else {
                        content.clone()
                    };

                    while let Some(x) = chars.next() {
                        content.push(x);
                        if x == '\\' && c != '$' {
                            // Handle escapes in regular strings
                            if let Some(escaped) = chars.next() {
                                content.push(escaped);
                            }
                        } else if content.ends_with(&end_marker) {
                            break;
                        }
                    }
                    result.push_str(&content);
                }
                // Remove comments
                '-' if chars.peek() == Some(&'-') => {
                    chars.next();
                    while let Some(&x) = chars.peek() {
                        if x == '\n' {
                            break;
                        }
                        chars.next();
                    }
                }
                _ => result.push(c),
            }
        }
        result
    }

    /// Extract bind parameters from SQL, using `SqlScanner` to skip
    /// characters inside string literals, comments, and other quoted contexts.
    fn extract_bind_params(sql: &str) -> Vec<Span<String>> {
        let mut s = SqlScanner::new(sql);
        let mut params = Vec::new();

        while s.i < s.chars.len() {
            if s.process_context() {
                continue;
            }

            if s.chars[s.i] == ':' {
                if s.i + 1 < s.chars.len() && s.chars[s.i + 1] == ':' {
                    s.i += 2; // skip type cast (::)
                } else {
                    let param_start = s.i + 1;
                    let mut param_end = param_start;

                    while param_end < s.chars.len()
                        && (s.chars[param_end].is_alphanumeric() || s.chars[param_end] == '_')
                    {
                        param_end += 1;
                    }

                    if param_end > param_start {
                        let param: String = s.chars[param_start..param_end].iter().collect();
                        let byte_start: usize =
                            s.chars[..param_start].iter().map(|c| c.len_utf8()).sum();
                        let byte_end: usize = byte_start + param.len();
                        params.push(Span {
                            value: param,
                            span: (byte_start..byte_end).into(),
                        });
                        s.i = param_end;
                    } else {
                        s.i += 1;
                    }
                }
            } else {
                s.i += 1;
            }
        }

        params
    }

    /// Find the character index of the statement-terminating semicolon.
    /// Returns `None` if no terminating semicolon is found.
    /// Uses `SqlScanner` to correctly skip semicolons inside string literals,
    /// dollar-quoted strings, quoted identifiers, and comments.
    fn find_statement_end(sql: &str) -> Option<usize> {
        let mut s = SqlScanner::new(sql);

        while s.i < s.chars.len() {
            if s.process_context() {
                continue;
            }

            if s.chars[s.i] == ';' {
                return Some(s.i);
            }

            s.i += 1;
        }

        None
    }

    /// Remove duplicates from bind parameters while preserving order of first occurrence
    fn dedup_bind_params(params: Vec<Span<String>>) -> Vec<Span<String>> {
        let mut seen = std::collections::HashSet::new();
        params
            .into_iter()
            .filter(|param| seen.insert(param.value.clone()))
            .collect()
    }

    /// Parse sql query, normalizing named parameters.
    /// Uses a context-aware state machine to find the statement-terminating
    /// semicolon, correctly skipping semicolons inside string literals,
    /// dollar-quoted strings, quoted identifiers, and comments.
    fn parse_sql_query<'src>() -> impl Parser<
        'src,
        &'src str,
        (String, SourceSpan, Vec<Span<String>>),
        extra::Err<Simple<'src, char>>,
    > {
        custom(|inp| {
            let before = inp.cursor();
            let remaining: &str = inp.slice_from(&before..);

            match Self::find_statement_end(remaining) {
                Some(semi_char_pos) => {
                    for _ in 0..semi_char_pos {
                        inp.skip();
                    }
                    let sql: &str = inp.slice_since(&before..);
                    inp.skip(); // skip the semicolon
                    Ok(sql.to_string())
                }
                None => Err(Simple::new(None, inp.span_since(&before))),
            }
        })
        .map_with(move |sql_str: String, e| {
            let span: SimpleSpan = e.span();
            let range: Range<usize> = span.start()..span.end();
            let source_span: SourceSpan = range.into();

            let mut sql_str = Self::clean_sql_comments(&sql_str)
                .lines()
                .filter(|line| !line.trim().is_empty())
                .collect::<Vec<_>>()
                .join("\n");

            let bind_params = Self::extract_bind_params(&sql_str);
            let dedup_params = Self::dedup_bind_params(bind_params.clone());

            for bind_param in bind_params.iter().rev() {
                let index = dedup_params.iter().position(|bp| bp == bind_param).unwrap();
                let start = bind_param.span.offset() - 1;
                let end = start + bind_param.span.len();
                sql_str.replace_range(start..=end, &format!("${}", index + 1));
            }

            (sql_str, source_span, dedup_params)
        })
    }

    fn parse_comments<'src>()
    -> impl Parser<'src, &'src str, Vec<String>, extra::Err<Simple<'src, char>>> {
        just("---")
            .ignore_then(
                none_of('\n')
                    .repeated()
                    .collect::<String>()
                    .map(|s| s.trim().to_string()),
            )
            .then_ignore(ln())
            .repeated()
            .collect()
    }

    fn parse_query_annotation<'src>() -> impl Parser<
        'src,
        &'src str,
        (Span<String>, QueryDataStruct, QueryDataStruct),
        extra::Err<Simple<'src, char>>,
    > {
        just("--!")
            .ignore_then(space())
            .ignore_then(plain_ident())
            .then_ignore(space())
            .then(QueryDataStruct::parser())
            .then_ignore(space())
            .then(
                just(":")
                    .ignore_then(space())
                    .ignore_then(QueryDataStruct::parser())
                    .or_not(),
            )
            .map(|((name, param), row)| (name, param, row.unwrap_or_default()))
    }

    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<Simple<'src, char>>> {
        Self::parse_query_annotation()
            .then_ignore(space())
            .then_ignore(ln())
            .then(TypeAnnotation::parse_attributes())
            .then(Self::parse_comments())
            .then(Self::parse_sql_query())
            .map(
                |(
                    (((name, param, row), attributes), comments),
                    (sql_str, sql_span, bind_params),
                )| Self {
                    name,
                    param,
                    comments,
                    row,
                    sql_span,
                    sql_str,
                    bind_params,
                    attributes,
                },
            )
    }
}

#[derive(Debug)]
pub(crate) struct QueryDataStruct {
    pub span: SourceSpan,
    pub name: Option<Span<String>>,
    pub idents: Option<Vec<NullableIdent>>,
}

/// Return type for name_and_fields method containing:
/// - nullable_fields: `&'a [NullableIdent]`
/// - traits: `Vec<String>`
/// - name: `Span<String>`
/// - attributes: `Vec<String>`
/// - attributes_borrowed: `Vec<String>`
type StructInfo<'a> = (
    &'a [NullableIdent],
    Vec<String>,
    Span<String>,
    Vec<String>,
    Vec<String>,
);

impl QueryDataStruct {
    pub fn is_implicit(&self) -> bool {
        self.name.is_none()
    }

    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.idents.is_none()
    }

    pub fn inlined(&self) -> bool {
        self.idents.is_some() && self.name.is_some()
    }

    pub(crate) fn name_and_fields<'a>(
        &'a self,
        registered_structs: &'a [TypeAnnotation],
        query_name: &Span<String>,
        name_suffix: Option<&str>,
    ) -> StructInfo<'a> {
        if let Some(named) = &self.name {
            let registered = registered_structs.iter().find(|it| it.name == *named);
            (
                self.idents.as_ref().map_or_else(
                    || registered.map(|it| it.fields.as_slice()).unwrap_or(&[]),
                    Vec::as_slice,
                ),
                registered.map(|it| it.traits.clone()).unwrap_or_default(),
                named.clone(),
                registered
                    .map(|it| it.attributes.clone())
                    .unwrap_or_default(),
                registered
                    .map(|it| it.attributes_borrowed.clone())
                    .unwrap_or_default(),
            )
        } else {
            (
                self.idents.as_ref().map_or(&[], Vec::as_slice),
                vec![],
                query_name.map(|x| {
                    format!(
                        "{}{}",
                        x.to_owned().to_upper_camel_case(),
                        name_suffix.unwrap_or_default()
                    )
                }),
                vec![],
                vec![],
            )
        }
    }

    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<Simple<'src, char>>> {
        plain_ident()
            .or_not()
            .then_ignore(space())
            .then(parse_nullable_ident().or_not())
            .map_with(|(name, idents), e| {
                let span: SimpleSpan = e.span();
                let range: Range<usize> = span.start()..span.end();

                Self {
                    span: range.into(),
                    name,
                    idents,
                }
            })
    }
}

impl Default for QueryDataStruct {
    fn default() -> Self {
        Self {
            span: (0..0).into(),
            name: None,
            idents: None,
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
enum Statement {
    Type(TypeAnnotation),
    Query(Query),
}

#[derive(Debug)]
pub(crate) struct Module {
    pub(crate) info: ModuleInfo,
    pub(crate) types: Vec<TypeAnnotation>,
    pub(crate) queries: Vec<Query>,
}

pub(crate) fn parse_query_module(info: ModuleInfo) -> Result<Module, Error> {
    let result = TypeAnnotation::parser()
        .map(Statement::Type)
        .or(Query::parser().map(Statement::Query))
        .separated_by(blank())
        .allow_leading()
        .allow_trailing()
        .collect::<Vec<_>>()
        .then_ignore(end())
        .parse(&info.content);

    match result.into_result() {
        Ok(statements) => {
            let mut types = Vec::new();
            let mut queries = Vec::new();
            for item in statements {
                match item {
                    Statement::Type(it) => types.push(it),
                    Statement::Query(it) => queries.push(it),
                }
            }
            Ok(Module {
                info,
                types,
                queries,
            })
        }
        Err(e) => {
            let span = e[0].span();
            let range: Range<usize> = span.start()..span.end();
            let source_span: SourceSpan = range.into();

            Err(Error {
                src: (&info).into(),
                err_span: source_span,
                help: e[0].to_string().replace("\n", "\\n"),
            })
        }
    }
}

pub(crate) mod error {
    use miette::{Diagnostic, NamedSource, SourceSpan};
    use std::sync::Arc;
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError, Diagnostic)]
    #[error("Couldn't parse queries")]
    pub struct Error {
        #[source_code]
        pub src: NamedSource<Arc<String>>,

        #[help]
        pub help: String,

        #[label("unexpected token")]
        pub err_span: SourceSpan,
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::{Query, parse_query_module};
    use crate::read_queries::ModuleInfo;

    #[test]
    fn find_statement_end_simple() {
        assert_eq!(Query::find_statement_end("SELECT 1;"), Some(8));
    }

    #[test]
    fn find_statement_end_single_quoted_string() {
        assert_eq!(Query::find_statement_end("SELECT 'a;b';"), Some(12));
    }

    #[test]
    fn find_statement_end_dollar_quoted_string() {
        assert_eq!(Query::find_statement_end("SELECT $$a;b$$;"), Some(14));
    }

    #[test]
    fn find_statement_end_tagged_dollar_quote() {
        assert_eq!(Query::find_statement_end("SELECT $t$a;b$t$;"), Some(16));
    }

    #[test]
    fn find_statement_end_escape_string() {
        assert_eq!(Query::find_statement_end("SELECT E'a\\;b';"), Some(14));
    }

    #[test]
    fn find_statement_end_double_quoted_identifier() {
        assert_eq!(Query::find_statement_end("SELECT \"a;b\";"), Some(12));
    }

    #[test]
    fn find_statement_end_line_comment() {
        assert_eq!(
            Query::find_statement_end("SELECT 1 -- comment;\n;"),
            Some(21)
        );
    }

    #[test]
    fn find_statement_end_no_semicolon() {
        assert_eq!(Query::find_statement_end("SELECT 1"), None);
    }

    #[test]
    fn find_statement_end_multiple_contexts() {
        assert_eq!(
            Query::find_statement_end("SELECT 'a;b', $$c;d$$, \"e;f\";"),
            Some(28)
        );
    }

    #[test]
    fn find_statement_end_escaped_quote_in_string() {
        assert_eq!(
            Query::find_statement_end("SELECT 'it''s ; here';"),
            Some(21)
        );
    }

    fn module_from(sql: &str) -> ModuleInfo {
        ModuleInfo {
            path: "test.sql".into(),
            name: "test".to_string(),
            full_module_path: "test".to_string(),
            content: Arc::new(sql.to_string()),
        }
    }

    #[test]
    fn parse_query_bind_param_after_utf8_quoted_identifier() {
        let sql = "--! q\nSELECT \"à\" FROM t WHERE \"à\" = :v;";
        let module = parse_query_module(module_from(sql))
            .expect("query with UTF-8 quoted identifier should parse successfully");
        assert_eq!(module.queries.len(), 1);
        let query = &module.queries[0];
        assert_eq!(query.bind_params.len(), 1);
        assert_eq!(query.bind_params[0].value, "v");
        assert_eq!(query.sql_str, "SELECT \"à\" FROM t WHERE \"à\" = $1");
    }
}
