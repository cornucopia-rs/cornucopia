use postgres::Client;

use crate::read_migrations::Migration;

use self::error::Error;

/// Runs all migrations in the specified directory. Only `.sql` files are considered.
///
/// # Errors
/// Returns an error if a migration can't be read or installed.
pub(crate) fn run_migrations(client: &mut Client, migrations: Vec<Migration>) -> Result<(), Error> {
    // Create the table holding Cornucopia migrations
    create_migration_table(client).map_err(Error::new_db)?;

    // Install each migration that is not already installed.
    for migration in migrations {
        let migration_not_installed = !is_installed(client, &migration.timestamp, &migration.name)
            .map_err(|err| Error::new_migration(err, migration.clone()))?;
        if migration_not_installed {
            install_migration(client, &migration)
                .map_err(|err| Error::new_migration(err, migration))?;
        }
    }
    Ok(())
}

fn create_migration_table(client: &mut Client) -> Result<(), postgres::Error> {
    client.execute(
        "CREATE TABLE IF NOT EXISTS _cornucopia_migrations (
    unix_timestamp BIGINT NOT NULL,
    name TEXT NOT NULL,
    installed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (unix_timestamp, name)
)",
        &[],
    )?;
    Ok(())
}

fn is_installed(client: &mut Client, timestamp: &i64, name: &str) -> Result<bool, postgres::Error> {
    let is_installed: bool = client
        .query_one(
            "select EXISTS(
    SELECT 1 from _cornucopia_migrations 
    WHERE (unix_timestamp, name) = ($1, $2))",
            &[&timestamp, &name],
        )?
        .get(0);
    Ok(is_installed)
}

fn install_migration(
    client: &mut Client,
    Migration {
        timestamp,
        name,
        sql,
        ..
    }: &Migration,
) -> Result<(), postgres::Error> {
    client.batch_execute(sql)?;
    client.execute(
        "INSERT INTO _cornucopia_migrations VALUES ($1, $2)",
        &[&timestamp, &name],
    )?;
    Ok(())
}

pub(crate) mod error {
    use miette::{Diagnostic, NamedSource, SourceSpan};
    use thiserror::Error as ThisError;

    use crate::{read_migrations::Migration, utils::db_err};

    #[derive(Debug, ThisError, Diagnostic)]
    #[error("Couldn't run migration: {msg}.")]
    #[diagnostic(code(cornucopia::run_migrations))]
    pub struct Error {
        pub msg: String,
        #[help]
        pub help: Option<String>,
        #[source_code]
        pub src: NamedSource,
        #[label("error occurs near this location")]
        pub err_span: Option<SourceSpan>,
    }

    impl Error {
        pub(crate) fn new_migration(err: postgres::Error, migration: Migration) -> Self {
            let msg = format!("{:#}", err);
            if let Some((position, msg, help)) = db_err(err) {
                Self {
                    msg,
                    help,
                    src: migration.into(),
                    err_span: Some((position as usize..position as usize).into()),
                }
            } else {
                Self {
                    msg,
                    help: None,
                    src: migration.into(),
                    err_span: None,
                }
            }
        }

        pub(crate) fn new_db(err: postgres::Error) -> Self {
            Self {
                msg: format!("{:#}", err),
                help: None,
                src: NamedSource::new("", ""),
                err_span: None,
            }
        }
    }
}
