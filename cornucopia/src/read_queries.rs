use std::path::{Path, PathBuf};

use miette::NamedSource;

use self::error::Error;

#[derive(Debug, Clone)]
pub(crate) struct ModuleInfo {
    pub(crate) path: String,
    pub(crate) name: String,
    pub(crate) content: String,
}

impl From<ModuleInfo> for NamedSource {
    fn from(m: ModuleInfo) -> Self {
        Self::new(m.path, m.content)
    }
}

impl From<&ModuleInfo> for NamedSource {
    fn from(m: &ModuleInfo) -> Self {
        Self::new(&m.path, m.content.clone())
    }
}

/// Reads queries in the directory. Only .sql files are considered.
///
/// # Error
/// Returns an error if `dir_path` does not point to a valid directory or if a query file cannot be parsed.
pub(crate) fn read_query_modules(dir_path: &str) -> Result<Vec<ModuleInfo>, Error> {
    let mut modules_info = Vec::new();
    for entry_result in std::fs::read_dir(dir_path).map_err(|err| Error {
        err,
        path: String::from(dir_path),
    })? {
        // Directory entry
        let entry = entry_result.map_err(|err| Error {
            err,
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
                err,
                path: dir_path.to_owned(),
            })?;

            modules_info.push(ModuleInfo {
                path: String::from(path_buf.to_string_lossy()),
                name: module_name,
                content: file_contents,
            });
        }
    }
    // Sort module for consistent codegen
    modules_info.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(modules_info)
}

/// Reads queries in the directory and checks each directory found within given path.
/// Only .sql files are considered.
///
/// # Error
/// Returns an error if `dir_path` does not point to a valid directory or if a query file cannot be parsed.
pub(crate) fn read_query_modules_recursive(dir_path: &str) -> Result<Vec<ModuleInfo>, Error> {
    let mut modules_info = Vec::new();
    for entry_result in std::fs::read_dir(dir_path).map_err(|err| Error {
        err,
        path: String::from(dir_path),
    })? {
        // Directory entry
        let entry = entry_result.map_err(|err| Error {
            err,
            path: dir_path.to_owned(),
        })?;
        let path_buf = entry.path();

        let path_bufs = if path_buf.is_dir() {
            find_queries(&path_buf, Vec::<PathBuf>::new())
        } else {
            vec![path_buf]
        };

        // Check we're dealing with a .sql file
        for path_buf in path_bufs {
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
                    err,
                    path: dir_path.to_owned(),
                })?;

                modules_info.push(ModuleInfo {
                    path: String::from(path_buf.to_string_lossy()),
                    name: module_name,
                    content: file_contents,
                });
            }
        }
    }
    // Sort module for consistent codegen
    modules_info.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(modules_info)
}

fn find_queries(start: &Path, mut queries: Vec<PathBuf>) -> Vec<PathBuf> {
    for entry in start.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            queries = find_queries(&path, queries);
        } else {
            queries.push(path);
        }
    }

    queries
}

pub(crate) mod error {
    use miette::Diagnostic;
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError, Diagnostic)]
    #[error("[{path}] : {err:#}")]
    pub struct Error {
        pub(crate) err: std::io::Error,
        pub(crate) path: String,
    }
}
