use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use miette::NamedSource;

use self::error::Error;
use crate::config::Config;

#[derive(Debug, Clone)]
pub(crate) struct ModuleInfo {
    pub(crate) path: PathBuf,
    // The name field contains just the file name without extension (e.g. "roles" for "roles.sql")
    pub(crate) name: String,
    // Full module path with namespace (e.g. "users::roles" for "queries/users/roles.sql")
    pub(crate) full_module_path: String,
    pub(crate) content: Arc<String>,
}

impl From<ModuleInfo> for NamedSource<Arc<String>> {
    fn from(m: ModuleInfo) -> Self {
        Self::new(m.path.to_string_lossy(), m.content)
    }
}

impl From<&ModuleInfo> for NamedSource<Arc<String>> {
    fn from(m: &ModuleInfo) -> Self {
        Self::new(m.path.to_string_lossy(), m.content.clone())
    }
}

/// Reads queries in the directory, recursively traversing subdirectories.
/// Only .sql files are considered.
/// If `config.ignore_underscore_files` is true, files with names prefixed with '_' are ignored.
///
/// # Error
/// Returns an error if `dir_path` does not point to a valid directory or if a query file cannot be parsed.
pub(crate) fn read_query_modules(
    dir_path: &Path,
    config: &Config,
) -> Result<Vec<ModuleInfo>, Error> {
    // Using the recursive function but passing the same path for base and current
    // to maintain backward compatibility
    let modules_info = read_query_modules_recursive(dir_path, dir_path, config)?;
    Ok(modules_info)
}

/// Recursively reads queries from a directory and its subdirectories.
/// Maps the directory structure to module paths.
fn read_query_modules_recursive(
    base_dir: &Path,
    current_dir: &Path,
    config: &Config,
) -> Result<Vec<ModuleInfo>, Error> {
    let mut modules_info = Vec::new();

    // Determine the relative path from base_dir to current_dir
    let relative_path = match current_dir.strip_prefix(base_dir) {
        Ok(path) => path,
        Err(_) => Path::new(""), // Fallback if prefix stripping fails
    };

    // Process entries in the current directory
    for entry_result in std::fs::read_dir(current_dir).map_err(|err| Error {
        err,
        path: current_dir.to_owned(),
    })? {
        let entry = entry_result.map_err(|err| Error {
            err,
            path: current_dir.to_owned(),
        })?;
        let path_buf = entry.path();

        if path_buf.is_dir() {
            // Skip directories starting with underscore if configured to do so
            if let Some(dir_name) = path_buf.file_name() {
                if let Some(dir_name_str) = dir_name.to_str() {
                    if config.ignore_underscore_files && dir_name_str.starts_with('_') {
                        continue;
                    }
                }
            }

            // Recursively process subdirectory
            let nested_modules = read_query_modules_recursive(base_dir, &path_buf, config)?;
            modules_info.extend(nested_modules);
        } else if path_buf
            .extension()
            .map(|extension| extension == "sql")
            .unwrap_or_default()
        {
            let file_name = path_buf
                .file_name()
                .expect("is a file")
                .to_str()
                .expect("file name is valid utf8");

            // Skip files starting with underscore if configured to do so
            if config.ignore_underscore_files && file_name.starts_with('_') {
                continue;
            }

            // Get the basic module name from the file stem
            let module_name = path_buf
                .file_stem()
                .expect("is a file")
                .to_str()
                .expect("file name is valid utf8")
                .to_string();

            // Construct the full module path based on directory structure
            let full_module_path = if relative_path.as_os_str().is_empty() {
                // Root-level file
                module_name.clone()
            } else {
                // Create a path that represents the module hierarchy
                let rel_path_str = path_to_module_path(relative_path);
                format!("{rel_path_str}::{module_name}")
            };

            let file_contents = std::fs::read_to_string(&path_buf).map_err(|err| Error {
                err,
                path: current_dir.to_owned(),
            })?;

            modules_info.push(ModuleInfo {
                path: path_buf,
                name: module_name,
                full_module_path,
                content: Arc::new(file_contents),
            });
        }
    }

    // Sort modules for consistent codegen
    modules_info.sort_by(|a, b| a.full_module_path.cmp(&b.full_module_path));
    Ok(modules_info)
}

/// Converts a Path to a module path string (directory separators to ::)
fn path_to_module_path(path: &Path) -> String {
    path.components()
        .filter_map(|comp| match comp {
            std::path::Component::Normal(s) => s.to_str(),
            _ => None,
        })
        .collect::<Vec<&str>>()
        .join("::")
}

/// Gets module hierarchy information for all modules
pub(crate) fn build_module_hierarchy(
    modules: &[ModuleInfo],
) -> BTreeMap<String, Vec<(String, bool)>> {
    let mut module_tree = BTreeMap::new();

    for module in modules {
        let path_components: Vec<&str> = module.full_module_path.split("::").collect();

        // Add each level of the hierarchy
        let mut current_path = Vec::new();
        for (i, component) in path_components.iter().enumerate() {
            current_path.push(*component);
            let current_path_str = current_path.join("::");

            if i < path_components.len() - 1 {
                // This is a directory, not a leaf module
                module_tree
                    .entry(current_path_str)
                    .or_insert_with(Vec::new)
                    .push((
                        path_components[i + 1].to_string(),
                        i == path_components.len() - 2,
                    ));
            }
        }
    }

    module_tree
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
