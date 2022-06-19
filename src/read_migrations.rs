use std::path::Path;

use miette::NamedSource;

use self::error::Error;

#[derive(Debug, Clone)]
pub struct Migration {
    pub path: String,
    pub name: String,
    pub timestamp: i64,
    pub sql: String,
}

impl From<Migration> for NamedSource {
    fn from(m: Migration) -> Self {
        Self::new(m.path, m.sql)
    }
}

/// Reads migrations in the directory. Only .sql files are considered.
///
/// # Error
/// Returns an error if `dir_path` does not point to a valid directory or a migration has a filename that cannot be parsed.
pub fn read_migrations(dir_path: &str) -> Result<Vec<Migration>, Error> {
    let mut migrations = Vec::new();
    for entry_result in std::fs::read_dir(dir_path).map_err(|err| Error::Io {
        path: dir_path.into(),
        err,
    })? {
        // Directory entry
        let entry = entry_result.map_err(|err| Error::Io {
            path: dir_path.into(),
            err,
        })?;
        let path_buf = entry.path();

        // Check we're dealing with a .sql file
        if path_buf
            .extension()
            .map(|extension| extension == "sql")
            .unwrap_or_default()
        {
            // Create migration
            let (timestamp, name) = parse_migration_filename(&path_buf)?;
            let sql = std::fs::read_to_string(&path_buf).map_err(|err| Error::Io {
                path: dir_path.into(),
                err,
            })?;
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
    let filename = path_buf.file_stem();
    let (timestamp_str, name) = filename
        .unwrap() // ! We already checked we're dealing with a file
        .to_str()
        .ok_or_else(|| Error::InvalidMigrationFilename {
            path: path_buf.to_string_lossy().to_string(),
            name: filename
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| String::from("unknown")),
        })?
        .split_once('_')
        .ok_or_else(|| Error::InvalidMigrationFilename {
            path: path_buf.to_string_lossy().to_string(),
            name: filename
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| String::from("unknown")),
        })?;

    let timestamp = timestamp_str
        .parse::<i64>()
        .map_err(|_| Error::InvalidTimestamp {
            timestamp: timestamp_str.to_string(),
            path: path_buf.to_string_lossy().into_owned(),
        })?;

    Ok((timestamp, name))
}

pub(crate) mod error {
    use miette::Diagnostic;
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError, Diagnostic)]
    pub enum Error {
        #[error("[{path}] {err:#}")]
        #[diagnostic(code(cornucopia::read_migrations::io))]
        Io { path: String, err: std::io::Error },
        #[error("[{path}] `{name}` is not a valid migration name.")]
        #[diagnostic(code(cornucopia::read_migrations::invalid_migration_filename),help("Migrations must be named with this pattern '<timestamp>_<name>' where <timestamp> is a unix timestamp and <name> is a valid identifier"))]
        InvalidMigrationFilename { path: String, name: String },
        #[error("[{path}] timestamp \"{timestamp}\" is not a valid unix timestamp")]
        #[diagnostic(code(cornucopia::read_migrations::invalid_timestamp))]
        InvalidTimestamp { path: String, timestamp: String },
    }
}
