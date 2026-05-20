use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{config::StaticFile, error::PersistError};

/// In memory storage of file
/// Can only fail while persisting
pub struct Vfs {
    fs: BTreeMap<PathBuf, String>,
}

impl Vfs {
    pub fn empty() -> Self {
        Self {
            fs: BTreeMap::new(),
        }
    }

    pub(crate) fn rustfmt(path: impl AsRef<Path>) -> bool {
        let path = path.as_ref();

        // Check if rustfmt is available
        if Command::new("rustfmt").arg("--version").output().is_err() {
            // rustfmt not installed - return true since this isn't a critical error
            return true;
        }

        Command::new("rustfmt")
            .args([
                "--edition",
                "2024",
                path.join("src/lib.rs").to_str().unwrap(),
            ])
            .status()
            .unwrap()
            .success()
    }

    /// Add a new file
    pub fn add(&mut self, path: impl Into<PathBuf>, content: proc_macro2::TokenStream) {
        let path_buf = path.into();
        let warning = "// This file was generated with `cornucopia`. Do not modify.\n\n";

        let syntax_tree = syn::parse2(content).unwrap_or_else(|_| {
            panic!(
                "Failed to parse generated code. Trying to generate '{}'",
                path_buf.display()
            )
        });

        let formatted = prettyplease::unparse(&syntax_tree);

        let file_content = format!("{warning}{formatted}");
        assert!(self.fs.insert(path_buf, file_content).is_none())
    }

    /// Add a new file from a string
    pub fn add_string(&mut self, path: impl Into<PathBuf>, content: impl Into<String>) {
        assert!(self.fs.insert(path.into(), content.into()).is_none())
    }

    pub fn persist(
        self,
        destination: impl AsRef<Path>,
        static_files: Vec<StaticFile>,
    ) -> Result<(), PersistError> {
        let destination = destination.as_ref();
        // First write in a temporary directory to prevent leaving the destination in a bad state
        let tmp = tempfile::tempdir().map_err(PersistError::wrap("tempfile"))?;

        // Write files to temp directory
        for (path, content) in self.fs {
            let path = tmp.path().join(path);
            let parent = path
                .parent()
                .expect("Must at least has 'destination' as parent");
            std::fs::create_dir_all(parent).ok(); // Might already exist
            std::fs::write(&path, content).map_err(PersistError::wrap(path))?;
        }

        // Copy static files to temp directory
        if !static_files.is_empty() {
            for file in static_files {
                let (source_path, destination_path, hard_link) = match file {
                    StaticFile::Simple(path) => (path, None, false),
                    StaticFile::Detailed {
                        path,
                        destination,
                        hard_link,
                    } => (path, destination, hard_link),
                };

                if !source_path.exists() {
                    return Err(PersistError::wrap(source_path)(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Static file not found",
                    )));
                }

                let destination_path = if let Some(dest) = destination_path {
                    dest
                } else {
                    // Use original filename in the codegen root if no destination specified
                    let target = source_path.file_name().ok_or_else(|| {
                        PersistError::wrap(&source_path)(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "Invalid file name",
                        ))
                    })?;
                    PathBuf::from(target)
                };

                let dst = tmp.path().join(destination_path);

                // Create parent directories if they don't exist
                if let Some(parent) = dst.parent() {
                    std::fs::create_dir_all(parent).map_err(PersistError::wrap(parent))?;
                }

                if hard_link {
                    std::fs::hard_link(&source_path, dst)
                        .map_err(PersistError::wrap(&source_path))?;
                } else {
                    std::fs::copy(&source_path, dst).map_err(PersistError::wrap(&source_path))?;
                }
            }
        }

        // Format with rustfmt
        Vfs::rustfmt(tmp.path());

        // Copy directory contents recursively for moving files
        fn copy_dir_recursive(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
            let src = src.as_ref();
            let dst = dst.as_ref();

            if src.is_dir() {
                for entry in std::fs::read_dir(src)? {
                    let entry = entry?;
                    let ty = entry.file_type()?;
                    let path = entry.path();

                    let dst_path = dst.join(path.file_name().expect("file name exists"));

                    if ty.is_dir() {
                        std::fs::create_dir_all(&dst_path)?;
                        copy_dir_recursive(&path, &dst_path)?;
                    } else if ty.is_file() {
                        std::fs::copy(&path, &dst_path)?;
                    }
                }
            }
            Ok(())
        }

        // Create a backup of the destination if it exists
        let backup_dir = if destination.exists() {
            // Create a temporary directory for backup
            let backup = tempfile::tempdir().map_err(PersistError::wrap("backup tempfile"))?;
            let backup_path = backup.path();

            // Copy existing files to backup
            copy_dir_recursive(destination, backup_path)
                .map_err(PersistError::wrap("backing up existing destination"))?;

            // Now we can safely remove the destination
            std::fs::remove_dir_all(destination).map_err(PersistError::wrap(destination))?;

            Some(backup)
        } else {
            None
        };

        // Create destination directory
        // only needed for linux/macos, this will break the rename on windows
        // https://github.com/barosl/rust/commit/bcbc9e5346941011f36f71f66c808675b263a589
        if cfg!(not(target_os = "windows")) {
            std::fs::create_dir_all(destination).map_err(PersistError::wrap(destination))?;
        }

        // Try to move the generated files to the destination
        let result = match std::fs::rename(tmp.path(), destination) {
            Ok(_) => Ok(()), // Rename successful
            Err(e) if e.raw_os_error() == Some(18) => {
                // EXDEV error, fall back to copy
                copy_dir_recursive(tmp.path(), destination).map_err(PersistError::wrap(destination))
            }
            Err(e) => Err(PersistError::wrap(destination)(e)),
        };

        // If something went wrong and we have a backup, restore it
        if result.is_err()
            && let Some(backup_dir) = backup_dir
        {
            // Clean the destination directory if it exists after a failed operation
            if destination.exists() {
                let _ = std::fs::remove_dir_all(destination);
            }

            // Ensure the destination directory exists for restoration
            let _ = std::fs::create_dir_all(destination);

            // Restore from backup
            if let Err(restore_err) = copy_dir_recursive(backup_dir.path(), destination) {
                // If restoration also fails, return a compound error
                return Err(PersistError::wrap(
                    "failed to restore backup after generation error",
                )(restore_err));
            }
        }

        result
    }
}
