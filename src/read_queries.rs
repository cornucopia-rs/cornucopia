use crate::parse::ParsedQuery;
use crate::parse_file::parse_file;

use self::error::*;

#[derive(Debug)]
pub(crate) struct Module {
    pub(crate) path: String,
    pub(crate) name: String,
    pub(crate) queries: Vec<ParsedQuery>,
}

pub(crate) fn read_queries(path: &str) -> Result<Vec<Module>, Error> {
    let mut modules = Vec::new();
    for entry_result in std::fs::read_dir(path).map_err(|err| Error {
        err: err.into(),
        path: String::from(path),
        line: None,
    })? {
        let entry = entry_result.map_err(|err| Error {
            err: err.into(),
            path: String::from(path),
            line: None,
        })?;
        let path_buf = entry.path();

        if path_buf
            .extension()
            .map(|extension| extension == "sql")
            .unwrap_or_default()
        {
            let module_name = path_buf
                .file_stem()
                .expect("is a file")
                .to_str()
                .expect("file name is valid utf8")
                .to_string();

            let module = Module {
                path: String::from(path_buf.to_string_lossy()),
                name: module_name,
                queries: parse_file(&path_buf).map_err(|err| {
                    let line = err.line;
                    Error {
                        err: Box::new(err).into(),
                        path: String::from(path_buf.to_string_lossy()),
                        line,
                    }
                })?,
            };

            modules.push(module);
        } else {
            continue;
        }
    }
    Ok(modules)
}

pub(crate) mod error {
    use std::fmt::Display;

    use crate::parse_file::error::Error as FileParserError;
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    #[error("{0}")]
    pub(crate) enum ErrorVariants {
        Io(#[from] std::io::Error),
        Parser(#[from] Box<FileParserError>),
    }

    #[derive(Debug)]
    pub(crate) struct Error {
        pub(crate) line: Option<usize>,
        pub(crate) err: ErrorVariants,
        pub(crate) path: String,
    }

    impl Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self.line {
                Some(line) => {
                    write!(
                        f,
                        "Error while reading queries [\"{}\", line: {}]: {}.",
                        self.path, line, self.err
                    )
                }
                None => {
                    write!(
                        f,
                        "Error while reading queries [\"{}\"]: {}.",
                        self.path, self.err
                    )
                }
            }
        }
    }

    impl std::error::Error for Error {}
}
