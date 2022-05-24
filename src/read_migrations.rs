use std::path::Path;

use error::{Error, ErrorVariants};

#[derive(Debug)]
pub(crate) struct Migration {
    pub(crate) path: String,
    pub(crate) name: String,
    pub(crate) timestamp: i64,
    pub(crate) sql: String,
}

/// Reads migrations in the directory. Only .sql files are considered.
///
/// # Error
/// Returns an error if `dir_path` does not point to a valid directory or a migration has a filename that cannot be parsed.
pub(crate) fn read_migrations(dir_path: &str) -> Result<Vec<Migration>, Error> {
    let mut migrations = Vec::new();
    for entry_result in
        std::fs::read_dir(dir_path).map_err(|err| Error::new(err.into(), dir_path.to_string()))?
    {
        // Directory entry
        let entry = entry_result.map_err(|err| Error::new(err.into(), dir_path.to_string()))?;
        let path_buf = entry.path();

        // Check we're dealing with a .sql file
        if path_buf
            .extension()
            .map(|extension| extension == "sql")
            .unwrap_or_default()
        {
            // Create migration
            let (timestamp, name) = parse_migration_filename(&path_buf)?;
            let sql = std::fs::read_to_string(&path_buf)
                .map_err(|err| Error::new(err.into(), dir_path.to_string()))?;
            let migration = Migration {
                path: path_buf.to_str().unwrap().to_string(),
                timestamp,
                name: name.to_string(),
                sql,
            };
            migrations.push(migration);
        } else {
            // If not a .sql file, ignore
            continue;
        }
    }
    // Sort migrations by timestamp.
    migrations.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    Ok(migrations)
}

/// Parse a migration filename from this format: `<unix_timestamp>_<migration_name>.sql`.
///
/// # Errors
/// Returns an error if the filename is not compatible with the format.
///
/// # Panics
/// Panics if the path does not point to a file.
fn parse_migration_filename(path_buf: &Path) -> Result<(i64, &str), Error> {
    let (timestamp_str, name) = path_buf
        .file_stem()
        .unwrap() // ! We already checked we're dealing with a file
        .to_str()
        .ok_or_else(|| {
            Error::new(
                ErrorVariants::InvalidMigrationFilename,
                path_buf.to_string_lossy().to_string(),
            )
        })?
        .split_once('_')
        .ok_or_else(|| {
            Error::new(
                ErrorVariants::InvalidMigrationFilename,
                path_buf.to_string_lossy().to_string(),
            )
        })?;

    let timestamp = timestamp_str.parse::<i64>().map_err(|_| {
        Error::new(
            ErrorVariants::InvalidTimestamp(timestamp_str.to_string()),
            path_buf.to_string_lossy().to_string(),
        )
    })?;

    Ok((timestamp, name))
}

pub(crate) mod error {
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    pub(crate) enum ErrorVariants {
        #[error("{0}")]
        Io(#[from] std::io::Error),
        #[error("Migrations must be named with this pattern '<timestamp>_<name>' where <timestamp> is a unix timestamp and <name> is a valid identifier")]
        InvalidMigrationFilename,
        #[error("timestamp \"{0}\" is not a valid unix timestamp")]
        InvalidTimestamp(String),
    }

    #[derive(Debug, ThisError)]
    #[error("Error while reading migration [file: \"{path}\"]: {err}.")]
    pub(crate) struct Error {
        pub(crate) err: ErrorVariants,
        pub(crate) path: String,
    }

    impl Error {
        pub(crate) fn new(err: ErrorVariants, path: String) -> Self {
            Self { path, err }
        }
    }
}
