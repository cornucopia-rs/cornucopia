use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use crate::error::PersistError;

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

    /// Add a new file
    pub fn add(&mut self, path: impl Into<PathBuf>, content: impl Into<String>) {
        assert!(self.fs.insert(path.into(), content.into()).is_none())
    }

    /// Materialize on real file system, overwrite existing directory if any
    pub fn persist(self, destination: impl AsRef<Path>) -> Result<(), PersistError> {
        let destination = destination.as_ref();
        // First write in a temporary directory to prevent leaving the destination in a bad state
        let tmp = tempfile::tempdir().map_err(PersistError::wrap("tempfile"))?;
        for (path, content) in self.fs {
            let path = tmp.path().join(path);
            let parent = path
                .parent()
                .expect("Must at least has 'destination' as parent");
            std::fs::create_dir_all(parent).ok(); // Might already exist
            std::fs::write(&path, content).map_err(PersistError::wrap(path))?;
        }
        // Swap destination and tmp as atomically as possible
        // TODO is it possible to do this atomically for some platform ?
        std::fs::remove_dir_all(destination).ok(); // Might not exist
        std::fs::create_dir_all(destination).map_err(PersistError::wrap(destination))?;
        std::fs::rename(tmp.into_path(), destination).map_err(PersistError::wrap(destination))?;
        Ok(())
    }
}
