use error::Error;
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
    sanitize(read_lines(path)?)?
        .into_iter()
        .map(|sanitized| {
            Ok(ParsedQuery {
                meta: parse_query_meta(&sanitized.meta)?,
                sql: sanitized.sql,
            })
        })
        .collect()
}

pub(crate) mod error {
    use crate::parse::error::Error as ParserError;
    use crate::sanitize::error::Error as SanitizeError;
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    #[error("{0}")]
    pub(crate) enum Error {
        IO(#[from] std::io::Error),
        Sanitize(#[from] SanitizeError),
        Parse(#[from] ParserError),
    }
}
