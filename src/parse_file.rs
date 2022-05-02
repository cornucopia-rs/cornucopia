use error::{Error, ErrorVariants};
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use super::{
    parse::{parse_query_meta, ParsedQuery},
    sanitize::sanitize,
};

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub(crate) fn parse_file<P>(path: P) -> Result<Vec<ParsedQuery>, Error>
where
    P: AsRef<Path>,
{
    let lines = read_lines(&path).map_err(|err| Error {
        err: err.into(),
        line: None,
    })?;
    sanitize(lines)
        .map_err(|err| Error {
            line: Some(err.line),
            err: err.into(),
        })?
        .into_iter()
        .map(|sanitized| {
            Ok(ParsedQuery {
                line: sanitized.line,
                meta: parse_query_meta(&sanitized.meta).map_err(|e| Error {
                    line: Some(sanitized.line),
                    err: ErrorVariants::Parse(e),
                })?,
                sql: sanitized.sql,
            })
        })
        .collect()
}

pub(crate) mod error {
    use std::fmt::Display;

    use crate::parse::error::Error as ParserError;
    use crate::sanitize::error::Error as SanitizeError;
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    #[error("{0}")]
    pub(crate) enum ErrorVariants {
        IO(#[from] std::io::Error),
        Sanitize(#[from] SanitizeError),
        Parse(#[from] ParserError),
    }

    #[derive(Debug)]
    pub(crate) struct Error {
        pub(crate) err: ErrorVariants,
        pub(crate) line: Option<usize>,
    }

    impl Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.err)
        }
    }

    impl std::error::Error for Error {}
}
