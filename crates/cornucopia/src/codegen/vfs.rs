use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

/// In memory storage of file
/// Can only fail while persisting
pub struct Vfs {
    fs: BTreeMap<PathBuf, Vec<u8>>,
}

impl Vfs {
    pub fn empty() -> Self {
        Self {
            fs: BTreeMap::new(),
        }
    }

    /// Add a new file
    pub fn add(&mut self, path: impl Into<PathBuf>, content: impl Into<Vec<u8>>) {
        assert!(self.fs.insert(path.into(), content.into()).is_none())
    }

    /// Materialize on real file system, overwrite existing directory if any
    pub fn persist(self, destination: impl AsRef<Path>) -> std::io::Result<()> {
        let destination = destination.as_ref();
        // First write in a temporary directory to prevent leaving the destination in a bad state
        let tmp = tempfile::tempdir()?;
        for (path, content) in self.fs {
            let path = tmp.path().join(path);
            let parent = path
                .parent()
                .expect("Must at least has 'destination' as parent");
            std::fs::create_dir_all(parent)?;
            std::fs::write(path, content)?;
        }
        // Swap destination and tmp as atomically as possible
        // TODO is it possible to do this atomically for some platform ?
        std::fs::remove_dir_all(destination).ok();
        std::fs::create_dir_all(destination)?;
        std::fs::rename(tmp.into_path(), destination)?;
        Ok(())
    }
}
