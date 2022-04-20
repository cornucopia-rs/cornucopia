use error::Error;
use std::fs::File;

/// A query that has been sanitized. The `meta` string should
/// not contain any whitespace whatsoever, and should be stripped
/// of SQL comments and the meta-token '--!'.
#[derive(Default)]
pub(crate) struct SanitizedQuery {
    pub(crate) meta: String,
    pub(crate) sql: String,
}

/// This function implements a line-by-line coarse parsing
/// step by finding each query and its corresponding meta-comment.
pub(crate) fn sanitize(
    lines: std::io::Lines<std::io::BufReader<File>>,
) -> Result<Vec<SanitizedQuery>, Error> {
    let mut reader_state = QueryReaderState::Uninit;
    let mut sanitized_queries = Vec::new();
    let mut meta = String::new();
    let mut sql = String::new();

    // Accumulate tokens from each line
    for line in lines {
        let line_string = line.map_err(|_| Error::UnreadableLine)?.trim().to_owned();
        let line_type = LineType::from_str(&line_string);
        let next_state = reader_state.next(&line_type)?;
        if line_type.is_ignored() {
            continue;
        } else {
            reader_state.accumulate(
                &next_state,
                &mut sanitized_queries,
                &mut meta,
                &mut sql,
                &line_string,
            );
            reader_state = next_state;
        }
    }

    // Special case for the last line
    match reader_state {
        QueryReaderState::Uninit => Err(Error::NoQueriesFound),
        QueryReaderState::CreateNewQuery => Err(Error::MissingSQL),
        QueryReaderState::AccumulateMeta => Err(Error::MissingSQL),
        QueryReaderState::AccumulateSQL => {
            sanitized_queries.push(SanitizedQuery { meta, sql });
            Ok(sanitized_queries)
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum LineType {
    Meta,
    Sql,
    Empty,
    Comment,
}

fn without_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect::<String>()
}

fn contains_whitespace(s: &str) -> Option<bool> {
    s.chars().map(|c| c.is_whitespace()).reduce(|a, b| a && b)
}

impl LineType {
    fn from_str(s: &str) -> Self {
        if s.starts_with("--!") {
            LineType::Meta
        } else if s.starts_with("--") {
            LineType::Comment
        } else if contains_whitespace(s).unwrap_or(true) {
            LineType::Empty
        } else {
            LineType::Sql
        }
    }

    fn is_ignored(&self) -> bool {
        match self {
            LineType::Meta => false,
            LineType::Sql => false,
            LineType::Empty => true,
            LineType::Comment => true,
        }
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
enum QueryReaderState {
    Uninit,
    CreateNewQuery,
    AccumulateMeta,
    AccumulateSQL,
}

impl Default for QueryReaderState {
    fn default() -> Self {
        QueryReaderState::Uninit
    }
}

impl QueryReaderState {
    fn next(&self, line_type: &LineType) -> Result<Self, Error> {
        match self {
            QueryReaderState::Uninit => match line_type {
                LineType::Meta => Ok(QueryReaderState::CreateNewQuery),
                LineType::Sql => Err(Error::MissingMeta),
                _ => Ok(*self),
            },
            QueryReaderState::CreateNewQuery => match line_type {
                LineType::Meta => Ok(QueryReaderState::AccumulateMeta),
                LineType::Sql => Ok(QueryReaderState::AccumulateSQL),
                _ => Ok(*self),
            },
            QueryReaderState::AccumulateMeta => match line_type {
                LineType::Meta => Ok(QueryReaderState::AccumulateMeta),
                LineType::Sql => Ok(QueryReaderState::AccumulateSQL),
                _ => Ok(*self),
            },
            QueryReaderState::AccumulateSQL => match line_type {
                LineType::Meta => Ok(QueryReaderState::CreateNewQuery),
                LineType::Sql => Ok(QueryReaderState::AccumulateSQL),
                _ => Ok(*self),
            },
        }
    }
    fn accumulate(
        &self,
        new_state: &QueryReaderState,
        sanitized_queries: &mut Vec<SanitizedQuery>,
        meta: &mut String,
        sql: &mut String,
        line_str: &str,
    ) {
        match &new_state {
            QueryReaderState::Uninit => (),
            QueryReaderState::AccumulateMeta => {
                // Trim unwanted tokens ('--!', ' ', '\t')
                let sanitized = sanitize_meta(line_str);
                // Push to accumulator
                meta.push_str(&sanitized)
            }
            QueryReaderState::AccumulateSQL => {
                // Push new SQL line to accumulator.
                sql.push_str(&format!("{}\n", line_str))
            }
            QueryReaderState::CreateNewQuery => {
                if &QueryReaderState::Uninit != self {
                    // Push accumulators to sanitized queries
                    sanitized_queries.push(SanitizedQuery {
                        meta: meta.clone(),
                        sql: sql.clone(),
                    });
                    // Reset accumulators
                    meta.clear();
                    sql.clear();
                }
                // Trim unwanted tokens ('--!', ' ', '\t')
                let sanitized = sanitize_meta(line_str);
                // Push to accumulator
                meta.push_str(&sanitized);
            }
        }
    }
}

fn sanitize_meta(line: &str) -> String {
    without_whitespace(line).replace("--!", "")
}

pub(crate) mod error {
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    #[error("sanitizing process encountered an error")]
    pub(crate) enum Error {
        #[error("query missing meta-comment. Check that every query has a meta-comment")]
        MissingMeta,
        #[error("meta-comment is missing SQL query. Check that every comment has a query")]
        MissingSQL,
        #[error("file does not contain any queries")]
        NoQueriesFound,
        #[error("file contains characters that are not valid utf8")]
        UnreadableLine,
    }
}
