use error::Error;

#[derive(Debug)]
pub struct Migration {
    pub name: String,
    pub timestamp: i64,
    pub sql: String,
}
pub fn read_migrations(path: &str) -> Result<Vec<Migration>, Error> {
    let mut migrations = Vec::new();
    for entry_result in std::fs::read_dir(path)? {
        let entry = entry_result?;
        let path = entry.path();

        if path
            .extension()
            .map(|extension| extension == "sql")
            .unwrap_or_default()
        {
            let (timestamp_str, name) = path
                .file_stem()
                .unwrap() // ! We already checked we're dealing with a file
                .to_str()
                .ok_or(Error::InvalidMigrationFilename)?
                .split_once('_')
                .ok_or(Error::InvalidMigrationFilename)?;

            let timestamp = timestamp_str
                .parse::<i64>()
                .map_err(|_| Error::InvalidTimestamp)?;

            let migration = Migration {
                timestamp,
                name: name.to_string(),
                sql: std::fs::read_to_string(&path)?,
            };

            migrations.push(migration);
        } else {
            continue;
        }
    }
    migrations.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    Ok(migrations)
}

pub mod error {
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    #[error("error while reading migrations")]
    pub enum Error {
        #[error("file io error")]
        Io(#[from] std::io::Error),
        #[error("your migration folder must contain only .sql files")]
        InvalidFile,
        #[error("migrations must be named with this pattern '<timestamp>_<name>' where <timestamp> is a unix timestamp and <name> is a valid identifier")]
        InvalidMigrationFilename,
        #[error("timestamp is not a valid unix timestamp")]
        InvalidTimestamp,
    }
}
