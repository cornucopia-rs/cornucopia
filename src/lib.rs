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

pub use cli::run;
pub use error::Error;

pub mod conn;
pub mod container;

use codegen::generate as generate_internal;
use error::{NewMigrationError, WriteCodeGenFileError};
use parser::parse_query_module;
use postgres::Client;
use prepare_queries::prepare;
use read_queries::read_query_modules;
use run_migrations::run_migrations as run_migrations_internal;
use std::{path::Path, rc::Rc};
use time::OffsetDateTime;
use validation::validate_module;

/// Runs the migrations at `migrations_path`.
pub fn run_migrations(client: &mut Client, migrations_path: &str) -> Result<(), Error> {
    Ok(crate::run_migrations::run_migrations(
        client,
        migrations_path,
    )?)
}

/// Creates a new migration file at the specified `migrations_path`.
/// The full name of the migration will correspond to `timestamp`_`name`.sql
/// where `timestamp is the unix time when the migration was created.`
pub fn new_migration(migrations_path: &str, name: &str) -> Result<(), Error> {
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

/// Generates your cornucopia queries residing in `queries_path`.
/// If some `destination` is given, the generated code will be written at that path.
pub fn generate_live(
    client: &mut Client,
    queries_path: &str,
    destination: Option<&str>,
    is_async: bool,
    serialize: bool,
) -> Result<String, Error> {
    // Read
    let modules_info = read_query_modules(queries_path)?;

    let mut validated_modules = Vec::new();
    for module_info in modules_info {
        let info = Rc::new(module_info);
        // Parse
        let parsed_module = parse_query_module(&info.path, &info.content)?;
        // Validate
        validated_modules.push(validate_module(info, parsed_module)?);
    }

    // Generate
    let prepared_modules = prepare(client, validated_modules)?;
    let generated_code = generate_internal(prepared_modules, is_async, serialize)?;
    // Write
    if let Some(d) = destination {
        write_generated_code(d, &generated_code)?
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
    is_async: bool,
    serialize: bool,
) -> Result<String, Error> {
    let modules_info = read_query_modules(queries_path)?;
    let mut validated_modules = Vec::new();
    for info in modules_info {
        let info = Rc::new(info);
        // Parse
        let parsed_module = parse_query_module(&info.path, &info.content)?;
        // Validate
        validated_modules.push(validate_module(info, parsed_module)?);
    }
    container::setup(podman)?;
    let mut client = conn::cornucopia_conn()?;
    run_migrations_internal(&mut client, migrations_path)?;
    let prepared_modules = prepare(&mut client, validated_modules)?;
    let generated_code = generate_internal(prepared_modules, is_async, serialize)?;
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
