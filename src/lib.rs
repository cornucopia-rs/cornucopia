#![allow(unused_assignments)] // TODO: remove when rustc fixed (thiserror)

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

pub mod config;
/// Helpers to establish connections to database instances.
pub mod conn;
/// High-level interfaces to work with Cornucopia's container manager.
pub mod container;

/// Re-export of cargo_toml for working with manifest configurations
pub use cargo_toml;
use std::path::Path;

use tokio_postgres::Client;

use config::Config;
use parser::parse_query_module;
use prepare_queries::prepare;
use read_queries::read_query_modules;

#[doc(hidden)]
pub use cli::run;

pub use error::Error;
pub use load_schema::load_schema;

/// Generates Rust queries from PostgreSQL queries located at `queries_path`,
/// using a live database managed by you. Code generation settings are
/// set using the `config` parameter.
pub fn gen_live(client: &Client, config: Config) -> Result<(), Error> {
    // Read
    let modules = read_query_modules(config.queries.as_ref(), &config)?
        .into_iter()
        .map(parse_query_module)
        .collect::<Result<_, parser::error::Error>>()
        .map_err(Box::new)?;

    // Generate
    let prepared_modules = prepare(client, modules, &config).map_err(Box::new)?;
    let generated = codegen::gen(prepared_modules, &config);

    // Write
    generated.persist(config.destination, config.static_files)?;

    Ok(())
}

/// Generates Rust queries from PostgreSQL queries located at `queries_path`, using
/// a container managed by cornucopia. The database schema is created using `schema_files`.
/// Code generation settings are set using the `config` parameter.
///
/// By default, the container manager is Docker, but Podman can be used by setting the
/// `podman` parameter to `true`.
pub fn gen_managed<P: AsRef<Path>>(schema_files: &[P], config: Config) -> Result<(), Error> {
    // Read
    let modules = read_query_modules(config.queries.as_ref(), &config)?
        .into_iter()
        .map(parse_query_module)
        .collect::<Result<_, parser::error::Error>>()
        .map_err(Box::new)?;

    container::setup(
        config.podman,
        &config.container_image,
        config.container_wait,
    )?;

    let client = conn::cornucopia_conn()?;
    load_schema(&client, schema_files).map_err(Box::new)?;
    let prepared_modules = prepare(&client, modules, &config).map_err(Box::new)?;
    let generated = codegen::gen(prepared_modules, &config);
    container::cleanup(config.podman)?;

    // Write
    generated.persist(config.destination, config.static_files)?;

    Ok(())
}

/// Generates Rust queries from PostgreSQL queries located at `queries_path`, using
/// a temporary database created on an existing PostgreSQL server. The database schema
/// is created using `schema_files`. Code generation settings are set using the `config` parameter.
///
/// This function creates a temporary database on the specified server, loads the schema,
/// generates the code, and optionally drops the temporary database based on the `keep_db` parameter.
pub fn gen_fresh<P: AsRef<Path>>(
    url: &str,
    db_name: &str,
    schema_files: &[P],
    search_path: Option<&str>,
    keep_db: bool,
    config: Config,
) -> Result<(), Error> {
    let modules = read_query_modules(config.queries.as_ref(), &config)?
        .into_iter()
        .map(parse_query_module)
        .collect::<Result<_, parser::error::Error>>()
        .map_err(Box::new)?;

    let server_client = conn::from_url(url)?;

    let create_db_query = format!("CREATE DATABASE \"{db_name}\"");
    futures::executor::block_on(server_client.execute(&create_db_query, &[]))
        .map_err(conn::error::Error)?;

    let db_url = if url.contains('?') {
        format!("{url}&dbname={db_name}")
    } else if url.ends_with('/') {
        format!("{url}{db_name}?")
    } else {
        format!("{url}/{db_name}?")
    };

    let generation_result = (|| -> Result<(), Error> {
        let db_client = conn::from_url(&db_url)?;

        if let Some(search_path) = search_path {
            conn::set_search_path(&db_client, search_path)?;
        }

        load_schema(&db_client, schema_files).map_err(Box::new)?;

        let prepared_modules = prepare(&db_client, modules, &config).map_err(Box::new)?;
        let generated = codegen::gen(prepared_modules, &config);

        generated.persist(config.destination, config.static_files)?;

        Ok(())
    })();

    if !keep_db {
        let drop_db_query = format!("DROP DATABASE \"{db_name}\"");
        if let Err(e) = futures::executor::block_on(server_client.execute(&drop_db_query, &[])) {
            eprintln!("Warning: Failed to drop temporary database '{db_name}': {e}");
        }
    }

    generation_result
}
