use crate::read_migrations::read_migrations;
use deadpool_postgres::Object;
use error::Error;

/// Runs all migrations in the specified directory. Only `.sql` files are considered.
///
/// # Errors
/// Returns an error if a migration can't be read or installed.
pub(crate) async fn run_migrations(client: &Object, dir_path: &str) -> Result<(), Error> {
    // Create the table holding Cornucopia migrations
    create_migration_table(client)
        .await
        .map_err(|err| Error::new(err.into(), None, dir_path))?;

    // Install each migration that is not already installed.
    for migration in
        read_migrations(dir_path).map_err(|err| Error::new(err.into(), None, dir_path))?
    {
        let migration_not_installed = !is_installed(client, &migration.timestamp, &migration.name)
            .await
            .map_err(|err| Error::new(err.into(), None, &migration.path))?;
        if migration_not_installed {
            install_migration(
                client,
                &migration.timestamp,
                &migration.name,
                &migration.sql,
            )
            .await
            .map_err(|err| Error::new(err.into(), Some(&migration.sql), &migration.path))?;
        }
    }
    Ok(())
}

async fn create_migration_table(client: &Object) -> Result<(), tokio_postgres::Error> {
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS _cornucopia_migrations (
    unix_timestamp BIGINT NOT NULL,
    name TEXT NOT NULL,
    installed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (unix_timestamp, name)
)",
            &[],
        )
        .await?;
    Ok(())
}

async fn is_installed(
    client: &Object,
    timestamp: &i64,
    name: &str,
) -> Result<bool, tokio_postgres::Error> {
    let is_installed: bool = client
        .query_one(
            "select EXISTS(
    SELECT 1 from _cornucopia_migrations 
    WHERE (unix_timestamp, name) = ($1, $2))",
            &[&timestamp, &name],
        )
        .await?
        .get(0);
    Ok(is_installed)
}

async fn install_migration(
    client: &Object,
    timestamp: &i64,
    name: &str,
    sql: &str,
) -> Result<(), tokio_postgres::Error> {
    client.batch_execute(sql).await?;
    client
        .execute(
            "INSERT INTO _cornucopia_migrations VALUES ($1, $2)",
            &[&timestamp, &name],
        )
        .await?;
    Ok(())
}

pub(crate) mod error {
    use std::{error::Error as ErrorTrait, fmt::Display};

    use super::super::read_migrations::error::Error as MigrationError;
    use thiserror::Error as ThisError;
    use tokio_postgres::error::ErrorPosition;

    #[derive(Debug, ThisError)]
    #[error("{0}")]
    pub(crate) enum ErrorVariant {
        ReadMigration(#[from] MigrationError),
        Db(#[from] tokio_postgres::Error),
    }

    #[derive(Debug)]
    pub(crate) struct Error {
        path: String,
        line: Option<usize>,
        err: ErrorVariant,
    }

    impl Error {
        pub fn new(err: ErrorVariant, sql: Option<&str>, path: &str) -> Self {
            let path = path.to_string();
            let mut line = None;
            if let Some(sql) = sql {
                if let ErrorVariant::Db(e) = &err {
                    if let Some(db_err) = e.as_db_error() {
                        if let Some(ErrorPosition::Original(position)) = db_err.position() {
                            // Count new lines up to the position where to error occured.
                            line = Some(
                                sql[..*position as usize]
                                    .chars()
                                    .filter(|&c| c == '\n')
                                    .count()
                                    + 1,
                            );
                        }
                    }
                };
            }

            Error { err, path, line }
        }
    }

    impl Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &self.err {
                ErrorVariant::ReadMigration(_) => {
                    write!(f, "{}", self.err)
                }
                ErrorVariant::Db(_) => match self.line {
                    Some(line) => write!(
                        f,
                        "Error while running migration [\"{}\", line: {}] ({})",
                        self.path,
                        line,
                        self.err.source().unwrap().source().unwrap()
                    ),
                    None => write!(
                        f,
                        "Error while running migration [\"{}\"] ({})",
                        self.path,
                        self.err.source().unwrap().source().unwrap()
                    ),
                },
            }
        }
    }

    impl std::error::Error for Error {}
}
