use error::{Error, ValidationError};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser as Pest;

use self::error::Kind;

#[derive(Pest)]
#[grammar = "../grammar.pest"]
struct CornucopiaParser;

/// Th    if is data structure holds a value and the context in which it was parsed.
/// This context is used for error reporting.
#[derive(Debug, Clone)]
pub(crate) struct Parsed<T> {
    pub(crate) line: usize,
    pub(crate) col: usize,
    pub(crate) line_str: String,
    pub(crate) value: T,
}

/// Holds all the data known to Cornucopia about a query after parsing it.
/// The query is not yet fully known though, as it has not yet been prepared.
#[derive(Debug, Clone)]
pub(crate) struct ParsedQuery {
    pub(crate) line: usize,
    pub(crate) name: String,
    pub(crate) params: Vec<Parsed<String>>,
    pub(crate) nullable_columns: Vec<Parsed<String>>,
    pub(crate) sql_str: String,
}

/// Parse queries in in the input string using the grammar file (`grammar.pest`).
pub(crate) fn parse_queries(input: &str) -> Result<Vec<ParsedQuery>, Error> {
    // Get top level tokens
    let parser_tokens = CornucopiaParser::parse(Rule::parser, input)
        .map_err(Error::Parser)?
        .next()
        .unwrap()
        .into_inner();

    parser_tokens
        .filter(|r| matches!(r.as_rule(), Rule::query))
        .map(|query| {
            let (line, _) = query.as_span().start_pos().line_col();
            let mut query_tokens = query.into_inner();

            // Parse annotation
            let mut annotation_tokens = query_tokens.next().unwrap().into_inner();

            // Parse sql
            let sql_tokens = query_tokens.next().unwrap();
            let sql_str = sql_tokens.as_str().to_string();
            let sql_start = sql_tokens.as_span().start();

            // Name
            let name_tokens = annotation_tokens.next().unwrap();
            let name = name_tokens.as_str().to_string();

            // Parameter list and nullable column list
            Ok(match annotation_tokens.next() {
                Some(it) => match it.as_rule() {
                    // Postgres-compatible syntax
                    Rule::param_list => {
                        // Params
                        let params = parse_params(it)?;
                        let nb_params = params.len();

                        // Nullable columns
                        let nullable_columns = match annotation_tokens.next() {
                            Some(it_nullable_columns) => {
                                parse_nullable_columns(it_nullable_columns)?
                            }
                            None => Vec::new(),
                        };

                        // Bind params
                        let bind_params = parse_pg_bind_params(sql_tokens)?;

                        // Check if the bind parameter's index is greater than the number of parameters
                        if let Some(p) = bind_params.iter().find(|p| p.value > nb_params as i16) {
                            return Err(Error::Validation(ValidationError {
                                line: p.line,
                                col: p.col,
                                line_str: p.line_str.to_owned(),
                                kind: Kind::MoreBindParamsThanParams { nb_params },
                            }));
                        };

                        // Check that every param is used in the query
                        if let Some((index, p)) = params.iter().enumerate().find(|(index, _)| {
                            !bind_params
                                .iter()
                                .any(|bind_index| bind_index.value == *index as i16 + 1)
                        }) {
                            return Err(Error::Validation(ValidationError {
                                line: p.line,
                                col: p.col,
                                line_str: p.line_str.to_owned(),
                                kind: Kind::UnusedParam { index: index + 1 },
                            }));
                        }

                        ParsedQuery {
                            line,
                            name,
                            params,
                            nullable_columns,
                            sql_str,
                        }
                    }
                    // Extended syntax with nullabble columns
                    Rule::nullable_column_list => {
                        // Nullable columns
                        let nullable_columns = parse_nullable_columns(it)?;
                        // Bind params and normalized sql
                        let (bind_params, sql_str) =
                            parse_extended_bind_params(sql_tokens, sql_start, &sql_str)?;
                        ParsedQuery {
                            line,
                            name,
                            params: bind_params,
                            nullable_columns,
                            sql_str,
                        }
                    }
                    _ => unreachable!(),
                },
                // Extended Syntax without nullable columns
                None => {
                    // Nullable columns
                    let nullable_columns = Vec::new();
                    // Bind params and normalized sql
                    let (bind_params, sql_str) =
                        parse_extended_bind_params(sql_tokens, sql_start, &sql_str)?;
                    ParsedQuery {
                        line,
                        name,
                        params: bind_params,
                        nullable_columns,
                        sql_str,
                    }
                }
            })
        })
        .collect()
}

/// Parse query parameters. This is only applicable to postgres-compatible queries.
fn parse_params(pair: Pair<Rule>) -> Result<Vec<Parsed<String>>, Error> {
    let mut params = Vec::new();
    for it in pair.into_inner() {
        if it.as_rule() == Rule::ident {
            // Collect info about the span we're parsing
            let it_str = it.as_str().to_owned();
            let pos = it.as_span().start_pos();
            let (line, col) = pos.line_col();
            let line_str = pos.line_of().to_owned();
            // Check that this parameter is not already in the list, then add it to the list.
            if params.iter().any(|p: &Parsed<String>| p.value == it_str) {
                return Err(Error::Validation(ValidationError {
                    line,
                    col,
                    line_str,
                    kind: Kind::DuplicateParam,
                }));
            } else {
                params.push(Parsed {
                    value: it_str,
                    line,
                    col,
                    line_str,
                });
            }
        }
    }
    Ok(params)
}

/// Finds all bind parameters (indexed) from their usage inside the query sql.  This is only applicable to postgres-compatible queries.
fn parse_pg_bind_params(pair: Pair<Rule>) -> Result<Vec<Parsed<i16>>, Error> {
    let mut bind_params = Vec::new();
    for it in pair.into_inner() {
        // Collect info about the span we're parsing
        let pos = it.as_span().start_pos();
        let (line, col) = pos.line_col();
        let line_str = pos.line_of().to_owned();
        // Check that we have an indexed bind param (as opposed to named).
        // This is mandatory in postgres-compatible syntax queries
        if it.as_rule() == Rule::number {
            let it_str = it.as_str().to_owned();

            // Check that the index can be parsed as a i16 (required by postgres wire protocol)
            let index = it_str.parse::<i16>().map_err(|_| {
                Error::Validation(ValidationError {
                    line,
                    col,
                    line_str: line_str.to_owned(),
                    kind: Kind::InvalidI16Index,
                })
            })?;

            // Check that the index is also non-zero (postgres bind params are 1-indexed)
            if index == 0 {
                return Err(Error::Validation(ValidationError {
                    line,
                    col,
                    line_str,
                    kind: Kind::InvalidI16Index,
                }));
            };

            // If the bind param has not yet been seen, add it to the list
            if !bind_params.iter().any(|p: &Parsed<i16>| p.value == index) {
                bind_params.push(Parsed {
                    line,
                    col,
                    line_str,
                    value: index,
                });
            }
        } else {
            return Err(Error::Validation(ValidationError {
                line,
                col,
                line_str,
                kind: Kind::ExtendedParamInPgQuery,
            }));
        }
    }
    Ok(bind_params)
}

/// Finds all bind parameters (named) from their usage inside the query sql.  
/// This is only applicable to extended syntax queries.
fn parse_extended_bind_params(
    pair: Pair<Rule>,
    sql_start: usize,
    sql: &str,
) -> Result<(Vec<Parsed<String>>, String), Error> {
    // Accumulator for valid bind parameters
    let mut bind_params = Vec::new();
    // Accumulator for values to replace in the original string (normalizing process)
    let mut replacing_values = Vec::new();
    for it in pair.into_inner() {
        // Collect some info about the span we're parsing
        let span = it.as_span();
        let pos = span.start_pos();
        let span_start = span.start() - sql_start - 1_usize;
        let span_end = span.end() - sql_start - 1_usize;
        let (line, col) = pos.line_col();
        let line_str = pos.line_of().to_owned();

        // Check that we have a named bind param (as opposed to indexed)
        // This is mandatory in extended syntax queries
        if it.as_rule() == Rule::ident {
            let it_str = it.as_str().to_owned();
            let parsed = Parsed {
                line,
                col,
                line_str,
                value: it_str,
            };

            // If the bind parameter hasn't been seen yet, add it and and its replacing value
            // otherwise, just add the replacing value
            if let Some((index, _)) = bind_params
                .iter()
                .enumerate()
                .find(|(_, p): &(usize, &Parsed<String>)| p.value == parsed.value)
            {
                replacing_values.push((
                    (span_start, span_end),
                    format!("${}", &(index + 1).to_string()),
                ));
            } else {
                replacing_values.push((
                    (span_start, span_end),
                    format!("${}", &(bind_params.len() + 1).to_string()),
                ));
                bind_params.push(parsed);
            }
        } else {
            return Err(Error::Validation(ValidationError {
                line,
                col,
                line_str,
                kind: Kind::PgParamInExtendedQuery,
            }));
        }
    }
    let normalized_sql = replaced_in_string(sql.to_owned(), &mut replacing_values);
    Ok((bind_params, normalized_sql))
}

/// Utility that replaces all the replacing values into the target string.
fn replaced_in_string(mut s: String, replacing_values: &mut [((usize, usize), String)]) -> String {
    replacing_values.sort_by(|a, b| a.0 .0.cmp(&b.0 .0));
    for ((start, end), value) in replacing_values.iter().rev() {
        s.replace_range(start..=end, value)
    }
    s
}

/// Parse nullable column list. Applicable to both extended and postgres-compatible syntax.
fn parse_nullable_columns(pair: Pair<Rule>) -> Result<Vec<Parsed<String>>, Error> {
    let mut cols = Vec::new();
    for it in pair.into_inner() {
        let pos = it.as_span().start_pos();
        let (line, col) = pos.line_col();
        let line_str = pos.line_of().to_owned();
        let value = it.as_str().to_owned();
        let parsed = Parsed {
            line,
            col,
            line_str,
            value,
        };
        cols.push(parsed);
    }
    Ok(cols)
}

pub(crate) mod error {
    use super::Rule;
    use std::fmt::Display;

    #[derive(Debug)]
    pub(crate) enum Error {
        Parser(pest::error::Error<Rule>),
        Validation(ValidationError),
    }

    #[derive(Debug)]
    pub(crate) struct ValidationError {
        pub(crate) line: usize,
        pub(crate) col: usize,
        pub(crate) line_str: String,
        pub(crate) kind: Kind,
    }

    #[derive(Debug)]
    pub(crate) enum Kind {
        PgParamInExtendedQuery,
        ExtendedParamInPgQuery,
        InvalidI16Index,
        DuplicateParam,
        MoreBindParamsThanParams { nb_params: usize },
        UnusedParam { index: usize },
        ColumnAlreadyNullable { name: String },
        InvalidNullableColumnName { name: String },
    }

    impl Display for ValidationError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let Self {
                line,
                col,
                line_str,
                kind,
            } = self;
            // Format error position
            let line_str = line_str.trim_end();
            write!(
                f,
                " --> {line}:{col}\n  | \n  | {line_str}\n  |{:>col$}^---\n  | ",
                " "
            )?;
            // Format error msgs
            let mut err_msgs = |msgs: &[&str]| -> std::fmt::Result {
                for msg in msgs {
                    write!(f, "\n  = {}", msg)?;
                }
                Ok(())
            };
            match kind {
                Kind::PgParamInExtendedQuery => err_msgs(&[
                        "Indexed bind parameters (`$index`) are not allowed when using the extended syntax.", 
                        "Consider using a named bind parameter like `:identifier`, or use the PostgreSQL-compatible syntax."
                    ]),
                Kind::ExtendedParamInPgQuery => err_msgs( &[
                        "Named bind parameters like `:identifier` are not allowed when using the PostgreSQL-compatible syntax.", 
                        "Consider using an indexed bind parameter like `$index`, or use the extended syntax."
                    ]),
                Kind::InvalidI16Index => err_msgs( &["Index must be between 1 and 32767."]),
                Kind::DuplicateParam => err_msgs( &["Parameter is already used in parameter list."]),
                Kind::MoreBindParamsThanParams {  nb_params } =>     err_msgs(&[&format!(
                        "Index is higher than the number of parameters supplied ({nb_params})."
                    )]),
                Kind::UnusedParam { index } => err_msgs(&[&format!("Parameter `${index}` is never used in the query.")]),
                Kind::ColumnAlreadyNullable { name } => err_msgs(&[&format!("Column `{name}` is already marked as nullable.")]),
                Kind::InvalidNullableColumnName { name } => err_msgs(&[&format!("No column named `{name}` found for this query.")]),
            }
        }
    }
    impl std::error::Error for ValidationError {}

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
