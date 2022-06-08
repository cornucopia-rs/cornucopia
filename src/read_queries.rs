use error::Error;

#[derive(Debug, Clone)]
pub(crate) struct ModuleInfo {
    pub(crate) path: String,
    pub(crate) name: String,
    pub(crate) content: String,
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
        } else {
            continue;
        }
    }
    Ok(modules_info)
}

pub(crate) mod error {
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    #[error("Error while reading queries [path: \"{path}\"]: {err}.")]
    pub struct Error {
        pub(crate) err: std::io::Error,
        pub(crate) path: String,
    }
}
