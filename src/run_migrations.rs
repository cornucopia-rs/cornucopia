use crate::read_migrations::read_migrations;
use deadpool_postgres::Object;
use error::Error;

pub(crate) async fn run_migrations(client: &Object, path: &str) -> Result<(), Error> {
    create_migration_table(client).await?;
    for migration in read_migrations(path)? {
        let migration_not_installed =
            !migration_is_installed(client, &migration.timestamp, &migration.name).await?;
        if migration_not_installed {
            install_migration(
                client,
                &migration.timestamp,
                &migration.name,
                &migration.sql,
            )
            .await?;
        }
    }
    Ok(())
}

async fn create_migration_table(client: &Object) -> Result<(), Error> {
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

async fn migration_is_installed(
    client: &Object,
    timestamp: &i64,
    name: &str,
) -> Result<bool, Error> {
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
) -> Result<(), Error> {
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
    use super::super::read_migrations::error::Error as MigrationError;
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    #[error("{0}")]
    pub(crate) enum Error {
        ReadMigration(#[from] MigrationError),
        Db(#[from] tokio_postgres::Error),
    }
}
