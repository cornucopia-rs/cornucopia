use error::{Error, ErrorPosition, ValidationError};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser as Pest;

#[derive(Pest)]
#[grammar = "../grammar.pest"]
struct CornucopiaParser;

#[derive(Debug, Clone)]
pub enum NullableColumn {
    Index(i16),
    Named(String),
}

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
    pub(crate) named_param_struct: Option<Parsed<String>>,
    pub(crate) params: Vec<Parsed<String>>,
    pub(crate) named_return_struct: Option<Parsed<String>>,
    pub(crate) nullable_columns: Vec<Parsed<NullableColumn>>,
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

    let mut parsed_queries = Vec::new();

    for query in parser_tokens.filter(|r| matches!(r.as_rule(), Rule::query)) {
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

        let mut parsed_query = ParsedQuery {
            line,
            name,
            params: Vec::new(),
            nullable_columns: Vec::new(),
            sql_str: sql_str.clone(),
            named_param_struct: None,
            named_return_struct: None,
        };

        // Parameter list and nullable column list
        match annotation_tokens.next() {
            Some(it) => match it.as_rule() {
                Rule::param_list => {
                    // Params
                    let params = parse_params(it)?;
                    let nb_params = params.len();

                    // Nullable columns
                    if let Some(nullable_col_tokens) = annotation_tokens.next() {
                        parsed_query.nullable_columns = parse_nullable_columns(nullable_col_tokens)?
                    };

                    // Bind params
                    if params.len() == 1 {
                        match parse_extended_bind_params(sql_tokens.clone(), sql_start, &sql_str) {
                            Ok((bind_params, normalized_sql)) => {
                                // Nullable columns
                                if let Some(return_struct_name) = annotation_tokens.next() {
                                    let pos = return_struct_name.as_span().start_pos();
                                    let (line, col) = pos.line_col();
                                    let line_str = pos.line_of().to_owned();
                                    parsed_query.named_return_struct = Some(Parsed {
                                        line,
                                        col,
                                        line_str,
                                        value: return_struct_name.as_str().to_string(),
                                    });
                                };
                                parsed_query.params = bind_params;
                                parsed_query.sql_str = normalized_sql;
                            }
                            Err(_) => {
                                let bind_params = parse_pg_bind_params(sql_tokens)?;
                                // Check if the bind parameter's index is greater than the number of parameters
                                Error::validate_more_bind_params_than_params(
                                    &bind_params,
                                    nb_params,
                                )?;
                                // Check that every param is used in the query
                                Error::validate_unused_param(&params, &bind_params)?;
                            }
                        }
                    } else {
                        let bind_params = parse_pg_bind_params(sql_tokens)?;
                        // Check if the bind parameter's index is greater than the number of parameters
                        Error::validate_more_bind_params_than_params(&bind_params, nb_params)?;
                        // Check that every param is used in the query
                        Error::validate_unused_param(&params, &bind_params)?;
                    }
                }
                // Extended syntax with nullabble columns
                Rule::nullable_column_list => {
                    // Nullable columns
                    let nullable_columns = parse_nullable_columns(it)?;
                    // Bind params and normalized sql
                    let (bind_params, normalized_sql) =
                        parse_extended_bind_params(sql_tokens, sql_start, &sql_str)?;
                    parsed_query.nullable_columns = nullable_columns;
                    parsed_query.params = bind_params;
                    parsed_query.sql_str = normalized_sql;
                }
                Rule::ident => {
                    let pos = it.as_span().start_pos();
                    let (line, col) = pos.line_col();
                    let line_str = pos.line_of().to_owned();
                    parsed_query.named_return_struct = Some(Parsed {
                        line,
                        col,
                        line_str,
                        value: it.as_str().to_string(),
                    });
                    // Nullable columns
                    if let Some(nullable_col_tokens) = annotation_tokens.next() {
                        parsed_query.nullable_columns = parse_nullable_columns(nullable_col_tokens)?
                    };
                    // Bind params and normalized sql
                    let (bind_params, normalized_sql) =
                        parse_extended_bind_params(sql_tokens, sql_start, &sql_str)?;
                    parsed_query.params = bind_params;
                    parsed_query.sql_str = normalized_sql;
                }
                _ => unreachable!(),
            },
            // Extended Syntax without nullable columns
            None => {
                // Bind params and normalized sql
                let (bind_params, normalized_sql) =
                    parse_extended_bind_params(sql_tokens, sql_start, &sql_str)?;
                parsed_query.params = bind_params;
                parsed_query.sql_str = normalized_sql;
            }
        };
        parsed_queries.push(parsed_query);
    }

    Ok(parsed_queries)
}

/// Parse query parameters. This is only applicable to postgres-compatible queries.
fn parse_params(pair: Pair<Rule>) -> Result<Vec<Parsed<String>>, Error> {
    let mut params = Vec::new();
    for it in pair.into_inner() {
        if it.as_rule() == Rule::ident {
            // Collect info about the span we're parsing
            let it_str = it.as_str().to_owned();

            // Check that this parameter is not already in the list, then add it to the list.
            let (line, col, line_str) = Error::validate_duplicate_param(it, &params, &it_str)?;
            params.push(Parsed {
                value: it_str,
                line,
                col,
                line_str,
            });
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
            let index = Error::validate_invalid_i16_index(&it_str, line, col, &line_str)?;

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
            return Err(Error::Validation(ValidationError::ExtendedParamInPgQuery {
                pos: ErrorPosition {
                    line,
                    col,
                    line_str,
                },
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
            return Err(Error::Validation(ValidationError::PgParamInExtendedQuery {
                pos: ErrorPosition {
                    line,
                    col,
                    line_str,
                },
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
fn parse_nullable_columns(pair: Pair<Rule>) -> Result<Vec<Parsed<NullableColumn>>, Error> {
    let mut cols = Vec::new();
    for it in pair.into_inner() {
        let pos = it.as_span().start_pos();
        let (line, col) = pos.line_col();
        let line_str = pos.line_of().to_owned();
        let it_str = it.as_str();
        let nullable_column = match it.as_rule() {
            // Named nullable column
            Rule::ident => NullableColumn::Named(it_str.to_owned()),
            // Indexed nullable column
            Rule::number => {
                // Check that the index can be parsed as a i16 (required by postgres wire protocol)
                let index = Error::validate_invalid_i16_index(it_str, line, col, &line_str)?;
                NullableColumn::Index(index)
            }
            _ => unreachable!(),
        };
        let parsed = Parsed {
            line,
            col,
            line_str,
            value: nullable_column,
        };
        cols.push(parsed);
    }
    Ok(cols)
}

pub(crate) mod error {
    use pest::iterators::Pair;

    use super::{Parsed, Rule};
    use std::fmt::Display;

    #[derive(Debug)]
    pub(crate) enum Error {
        Parser(pest::error::Error<Rule>),
        Validation(ValidationError),
    }

    impl Error {
        pub(crate) fn validate_more_bind_params_than_params(
            bind_params: &[Parsed<i16>],
            nb_params: usize,
        ) -> Result<(), Self> {
            if let Some(p) = bind_params.iter().find(|p| p.value > nb_params as i16) {
                return Err(Error::Validation(
                    ValidationError::MoreBindParamsThanParams {
                        nb_params,
                        pos: ErrorPosition {
                            line: p.line,
                            col: p.col,
                            line_str: p.line_str.to_owned(),
                        },
                    },
                ));
            } else {
                Ok(())
            }
        }

        pub(crate) fn validate_unused_param(
            params: &[Parsed<String>],
            bind_params: &[Parsed<i16>],
        ) -> Result<(), Self> {
            if let Some((index, p)) = params.iter().enumerate().find(|(index, _)| {
                !bind_params
                    .iter()
                    .any(|bind_index| bind_index.value == *index as i16 + 1)
            }) {
                return Err(Error::Validation(ValidationError::UnusedParam {
                    index: index + 1,
                    pos: ErrorPosition {
                        line: p.line,
                        col: p.col,
                        line_str: p.line_str.to_owned(),
                    },
                }));
            } else {
                Ok(())
            }
        }

        pub(crate) fn validate_duplicate_param(
            it: Pair<Rule>,
            params: &[Parsed<String>],
            param: &str,
        ) -> Result<(usize, usize, String), Self> {
            let pos = it.as_span().start_pos();
            let (line, col) = pos.line_col();
            let line_str = pos.line_of().to_owned();
            if params.iter().any(|p: &Parsed<String>| p.value == param) {
                return Err(Error::Validation(ValidationError::DuplicateParam {
                    pos: ErrorPosition {
                        line,
                        col,
                        line_str,
                    },
                }));
            } else {
                Ok((line, col, line_str))
            }
        }

        pub(crate) fn validate_invalid_i16_index(
            it_str: &str,
            line: usize,
            col: usize,
            line_str: &str,
        ) -> Result<i16, Error> {
            // Check that the index can be parsed as a i16 (required by postgres wire protocol)
            let index = it_str.parse::<i16>().map_err(|_| {
                Error::Validation(ValidationError::InvalidI16Index {
                    pos: ErrorPosition {
                        line,
                        col,
                        line_str: line_str.to_owned(),
                    },
                })
            })?;

            // Check that the index is also non-zero (postgres bind params are 1-indexed)
            if index == 0 {
                return Err(Error::Validation(ValidationError::InvalidI16Index {
                    pos: ErrorPosition {
                        line,
                        col,
                        line_str: line_str.to_owned(),
                    },
                }));
            };

            Ok(index)
        }
    }

    #[derive(Debug)]
    pub(crate) struct ErrorPosition {
        pub(crate) line: usize,
        pub(crate) col: usize,
        pub(crate) line_str: String,
    }

    #[derive(Debug)]
    pub(crate) enum ValidationError {
        PgParamInExtendedQuery {
            pos: ErrorPosition,
        },
        ExtendedParamInPgQuery {
            pos: ErrorPosition,
        },
        InvalidI16Index {
            pos: ErrorPosition,
        },
        DuplicateParam {
            pos: ErrorPosition,
        },
        MoreBindParamsThanParams {
            nb_params: usize,
            pos: ErrorPosition,
        },
        UnusedParam {
            index: usize,
            pos: ErrorPosition,
        },
        ColumnAlreadyNullable {
            name: String,
            pos: ErrorPosition,
        },
        InvalidNullableColumnIndex {
            index: usize,
            max_col_index: usize,
            pos: ErrorPosition,
        },
        InvalidNullableColumnName {
            name: String,
            pos: ErrorPosition,
        },
    }

    impl Display for ValidationError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ValidationError::PgParamInExtendedQuery { pos } => {
                    let msg = [
                            "Indexed bind parameters (`$index`) are not allowed when using the extended syntax.", 
                            "Consider using a named bind parameter like `:identifier`, or use the PostgreSQL-compatible syntax."
                        ];
                    write!(f, "{}", format_err(pos.line, pos.col, &pos.line_str, &msg))
                }
                ValidationError::ExtendedParamInPgQuery { pos } => {
                    let msg = [
                            "Named bind parameters like `:identifier` are not allowed when using the PostgreSQL-compatible syntax.", 
                            "Consider using an indexed bind parameter like `$index`, or use the extended syntax."
                        ];
                    write!(f, "{}", format_err(pos.line, pos.col, &pos.line_str, &msg))
                }
                ValidationError::InvalidI16Index { pos } => {
                    let msg = ["Index must be between 1 and 32767."];
                    write!(f, "{}", format_err(pos.line, pos.col, &pos.line_str, &msg))
                }
                ValidationError::DuplicateParam { pos } => {
                    let msg = ["Parameter is already used in parameter list."];
                    write!(f, "{}", format_err(pos.line, pos.col, &pos.line_str, &msg))
                }
                ValidationError::MoreBindParamsThanParams { pos, nb_params } => {
                    let msg = format!(
                        "Index is higher than the number of parameters supplied ({nb_params})."
                    );
                    write!(
                        f,
                        "{}",
                        format_err(pos.line, pos.col, &pos.line_str, &[&msg])
                    )
                }
                ValidationError::UnusedParam { pos, index } => {
                    let msg = format!("Parameter `${index}` is never used in the query.");
                    write!(
                        f,
                        "{}",
                        format_err(pos.line, pos.col, &pos.line_str, &[&msg])
                    )
                }
                ValidationError::ColumnAlreadyNullable { name, pos } => {
                    let msg = format!("Column `{name}` is already marked as nullable.");
                    write!(
                        f,
                        "{}",
                        format_err(pos.line, pos.col, &pos.line_str, &[&msg])
                    )
                }
                ValidationError::InvalidNullableColumnIndex {
                    index,
                    max_col_index,
                    pos,
                } => {
                    let msg = if *max_col_index > 1 {
                        format!("Bind parameter `${index}` is invalid (must be $1).")
                    } else {
                        format!("Bind parameter `${index}` is invalid (must be between $1 and ${max_col_index}).")
                    };
                    write!(
                        f,
                        "{}",
                        format_err(pos.line, pos.col, &pos.line_str, &[&msg])
                    )
                }
                ValidationError::InvalidNullableColumnName { name, pos } => {
                    let msg = format!("No column named `{name}` found for this query.");
                    write!(
                        f,
                        "{}",
                        format_err(pos.line, pos.col, &pos.line_str, &[&msg])
                    )
                }
            }
        }
    }
    impl std::error::Error for ValidationError {}

    fn format_err(line: usize, col: usize, line_str: &str, messages: &[&str]) -> String {
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
