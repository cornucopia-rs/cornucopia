mod cli;
mod codegen;
mod error;
mod parser;
mod prepare_queries;
mod read_migrations;
mod read_queries;
mod run_migrations;
mod type_registrar;
mod utils;
mod validation;

pub mod conn;
pub mod container;

use std::{
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use postgres::Client;

use codegen::generate as generate_internal;
use error::{NewMigrationError, WriteOutputError};
use parser::parse_query_module;
use prepare_queries::prepare;
use read_queries::read_query_modules;
use run_migrations::run_migrations as run_migrations_internal;
use validation::validate_module;

pub use cli::run;
pub use error::Error;
pub use read_migrations::{read_migrations, Migration};

/// Runs the migrations at `migrations_path`.
pub fn run_migrations(client: &mut Client, migrations: Vec<Migration>) -> Result<(), Error> {
    Ok(crate::run_migrations::run_migrations(client, migrations)?)
}

/// Creates a new migration file at the specified `migrations_path`.
/// The full name of the migration will correspond to `timestamp`_`name`.sql
/// where `timestamp is the unix time when the migration was created.`
pub fn new_migration(migrations_path: &str, name: &str) -> Result<(), Error> {
    // Create a timestamp of the current time.
    let unix_ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
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

#[derive(Clone, Copy)]
pub struct CodegenSettings {
    pub is_async: bool,
    pub derive_ser: bool,
}

/// Generates your cornucopia queries residing in `queries_path`.
/// If some `destination` is given, the generated code will be written at that path.
pub fn generate_live(
    client: &mut Client,
    queries_path: &str,
    destination: Option<&str>,
    settings: CodegenSettings,
) -> Result<String, Error> {
    // Read
    let modules_info = read_query_modules(queries_path)?;

    let mut validated_modules = Vec::new();
    for info in modules_info {
        // Parse
        let parsed_module = parse_query_module(&info)?;
        // Validate
        validated_modules.push(validate_module(info, parsed_module)?);
    }

    // Generate
    let prepared_modules = prepare(client, validated_modules)?;
    let generated_code = generate_internal(prepared_modules, settings);
    // Write
    if let Some(d) = destination {
        write_generated_code(d, &generated_code)?;
    };

    Ok(generated_code)
}

/// Generates your cornucopia queries residing in `queries_path` against a container
/// managed by cornucopia. The database is created using the migrations in the given
/// `migrations_path` folder.
/// If some `destination` is given, the generated code will be written at that path.
pub fn generate_managed(
    queries_path: &str,
    migrations_path: &str,
    destination: Option<&str>,
    podman: bool,
    settings: CodegenSettings,
) -> Result<String, Error> {
    let modules_info = read_query_modules(queries_path)?;
    let mut validated_modules = Vec::new();
    for info in modules_info {
        // Parse
        let parsed_module = parse_query_module(&info)?;
        // Validate
        validated_modules.push(validate_module(info, parsed_module)?);
    }
    container::setup(podman)?;
    let mut client = conn::cornucopia_conn()?;
    let migrations = read_migrations(migrations_path)?;
    run_migrations_internal(&mut client, migrations)?;
    let prepared_modules = prepare(&mut client, validated_modules)?;
    let generated_code = generate_internal(prepared_modules, settings);
    container::cleanup(podman)?;

    if let Some(destination) = destination {
        write_generated_code(destination, &generated_code)?;
    };

    Ok(generated_code)
}

fn write_generated_code(destination: &str, generated_code: &str) -> Result<(), Error> {
    Ok(
        std::fs::write(destination, generated_code).map_err(|err| WriteOutputError {
            err,
            file_path: String::from(destination),
        })?,
    )
}
