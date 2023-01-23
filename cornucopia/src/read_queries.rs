use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use miette::NamedSource;

use self::error::Error;

#[derive(Debug, Clone)]
pub(crate) struct ModuleInfo {
    pub(crate) path: PathBuf,
    pub(crate) name: String,
    pub(crate) content: Arc<String>,
}

impl From<ModuleInfo> for NamedSource {
    fn from(m: ModuleInfo) -> Self {
        Self::new(m.path.to_string_lossy(), m.content)
    }
}

impl From<&ModuleInfo> for NamedSource {
    fn from(m: &ModuleInfo) -> Self {
        Self::new(m.path.to_string_lossy(), m.content.clone())
    }
}

/// Reads queries in the directory. Only .sql files are considered.
///
/// # Error
/// Returns an error if `dir_path` does not point to a valid directory or if a query file cannot be parsed.
pub(crate) fn read_query_modules(dir_path: &Path) -> Result<Vec<ModuleInfo>, Error> {
    let mut modules_info = Vec::new();
    for entry_result in std::fs::read_dir(dir_path).map_err(|err| Error {
        err,
        path: dir_path.to_owned(),
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
                path: path_buf,
                name: module_name,
                content: Arc::new(file_contents),
            });
        }
    }
    // Sort module for consistent codegen
    modules_info.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(modules_info)
}

pub(crate) mod error {
    use std::path::PathBuf;

    use miette::Diagnostic;
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError, Diagnostic)]
    #[error("[{path}] : {err:#}")]
    pub struct Error {
        pub(crate) err: std::io::Error,
        pub(crate) path: PathBuf,
    }
}
