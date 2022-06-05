use crate::parser::{parse_query_module, ParsedQuery, ValidatedModule};
use error::Error;

#[derive(Debug)]
pub(crate) struct Module {
    pub(crate) path: String,
    pub(crate) name: String,
    pub(crate) inner: ValidatedModule,
}

/// Reads queries in the directory. Only .sql files are considered.
///
/// # Error
/// Returns an error if `dir_path` does not point to a valid directory or if a query file cannot be parsed.
pub(crate) fn read_query_modules(dir_path: &str) -> Result<Vec<Module>, Error> {
    let mut modules = Vec::new();
    for entry_result in std::fs::read_dir(dir_path).map_err(|err| Error {
        err: err.into(),
        path: String::from(dir_path),
    })? {
        // Directory entry
        let entry = entry_result.map_err(|err| Error {
            err: err.into(),
            path: dir_path.to_owned(),
        })?;
        let path_buf = entry.path();

        // Check we're dealing with a .sql file
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

            let file_contents = std::fs::read_to_string(&path_buf).map_err(|err| Error {
                err: err.into(),
                path: dir_path.to_owned(),
            })?;

            let module = Module {
                path: String::from(path_buf.to_string_lossy()),
                name: module_name,
                inner: parse_query_module(&file_contents).map_err(|err| Error {
                    err: err.into(),
                    path: String::from(path_buf.to_string_lossy()),
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
    use crate::parser::error::Error as ParseError;

    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    #[error("{0}")]
    pub(crate) enum ErrorVariants {
        Io(#[from] std::io::Error),
        Parser(#[from] ParseError),
    }

    #[derive(Debug, ThisError)]
    #[error("Error while reading queries [path: \"{path}\"]: {err}.")]
    pub struct Error {
        pub(crate) err: ErrorVariants,
        pub(crate) path: String,
    }
}
