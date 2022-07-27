mod cli;
mod codegen;
mod error;
mod load_schema;
mod parser;
mod prepare_queries;
mod read_queries;
mod type_registrar;
mod utils;
mod validation;

/// Helpers to establish connections to database instances.
pub mod conn;
/// High-level interfaces to work with Cornucopia's container manager.
pub mod container;

use postgres::Client;

use codegen::generate as generate_internal;
use error::WriteOutputError;
use parser::parse_query_module;
use prepare_queries::prepare;
use read_queries::read_query_modules;

pub use cli::run;
pub use error::Error;
pub use load_schema::load_schema;

/// Struct containing the settings for code generation.
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
    let modules = read_query_modules(queries_path)?
        .into_iter()
        .map(parse_query_module)
        .collect::<Result<_, parser::error::Error>>()?;
    // Generate
    let prepared_modules = prepare(client, modules)?;
    let generated_code = generate_internal(prepared_modules, settings);
    // Write
    if let Some(d) = destination {
        write_generated_code(d, &generated_code)?;
    };

    Ok(generated_code)
}

/// Generates your cornucopia queries residing in `queries_path` against a container
/// managed by cornucopia. The database is created using`schema_files`.
/// If some `destination` is given, the generated code will be written at that path.
pub fn generate_managed(
    queries_path: &str,
    schema_files: Vec<String>,
    destination: Option<&str>,
    podman: bool,
    settings: CodegenSettings,
) -> Result<String, Error> {
    // Read
    let modules = read_query_modules(queries_path)?
        .into_iter()
        .map(parse_query_module)
        .collect::<Result<_, parser::error::Error>>()?;
    container::setup(podman)?;
    let mut client = conn::cornucopia_conn()?;
    load_schema(&mut client, schema_files)?;
    let prepared_modules = prepare(&mut client, modules)?;
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
