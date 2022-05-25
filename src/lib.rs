pub(crate) mod cli;
pub(crate) mod codegen;
pub(crate) mod conn;
pub(crate) mod container;
pub(crate) mod error;
pub(crate) mod integration;
pub(crate) mod parser;
pub(crate) mod prepare_queries;
pub(crate) mod read_migrations;
pub(crate) mod read_queries;
pub(crate) mod run_migrations;
pub(crate) mod type_registrar;

use codegen::generate as generate_internal;
use error::{NewMigrationError, WriteCodeGenFileError};
use prepare_queries::prepare;
use read_queries::read_query_modules;
use run_migrations::run_migrations as run_migrations_internal;
use std::path::Path;
use time::OffsetDateTime;
use tokio_postgres::Client;
use type_registrar::TypeRegistrar;

pub use cli::run;
pub use error::Error;

/// Runs the migrations given at `migrations_path` at the specified `url`.
pub async fn run_migrations(client: &Client, migrations_path: &str) -> Result<(), Error> {
    Ok(crate::run_migrations::run_migrations(client, migrations_path).await?)
}

/// Creates a new migration file at the specified `migrations_path`.
/// The full name of the migration will correspond to `timestamp`_`name`.sql
/// where `timestamp is the unix time when the migration was created.`
pub async fn new_migration(migrations_path: &str, name: &str) -> Result<(), Error> {
    // Create a timestamp of the current time.
    let unix_ts = OffsetDateTime::now_utc().unix_timestamp();
    // Format the target file name
    let file_path = Path::new(&migrations_path).join(format!("{}_{}.sql", unix_ts, name));
    // Write file with header
    Ok(
        std::fs::write(&file_path, "-- Write your migration SQL here\n").map_err(|err| {
            NewMigrationError {
                err,
                file_path: file_path.to_string_lossy().to_string(),
            }
        })?,
    )
}

/// Generates your cornucopia queries residing in `queries_path` against the database
/// at `url`. If some `destination` is given, the generated code will be written at that path.
pub async fn generate_live(
    client: &Client,
    queries_path: &str,
    destination: Option<&str>,
) -> Result<String, Error> {
    let mut type_registrar = TypeRegistrar::default();

    let modules = read_query_modules(queries_path)?;
    let prepared_modules = prepare(client, &mut type_registrar, modules).await?;
    let generated_code = generate_internal(&type_registrar, prepared_modules)?;

    if let Some(d) = destination {
        write_generated_code(d, &generated_code)?
    };

    Ok(generated_code)
}

/// Generates your cornucopia queries residing in `queries_path` against a container
/// managed by cornucopia. The database is created using the migrations in the given
/// `migrations_path` folder.
/// If some `destination` is given, the generated code will be written at that path.
pub async fn generate_managed(
    client: &Client,
    queries_path: &str,
    migrations_path: &str,
    destination: Option<&str>,
    podman: bool,
) -> Result<String, Error> {
    let mut type_registrar = TypeRegistrar::default();

    let modules = read_query_modules(queries_path)?;
    container::setup(podman)?;
    run_migrations_internal(client, migrations_path).await?;
    let prepared_modules = prepare(client, &mut type_registrar, modules).await?;
    let generated_code = generate_internal(&type_registrar, prepared_modules)?;
    container::cleanup(podman)?;

    if let Some(destination) = destination {
        write_generated_code(destination, &generated_code)?
    };

    Ok(generated_code)
}

fn write_generated_code(destination: &str, generated_code: &str) -> Result<(), Error> {
    Ok(
        std::fs::write(destination, generated_code).map_err(|err| WriteCodeGenFileError {
            err,
            file_path: String::from(destination),
        })?,
    )
}
