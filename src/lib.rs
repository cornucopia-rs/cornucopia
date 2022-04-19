pub(crate) mod cli;
pub(crate) mod codegen;
pub(crate) mod container;
pub(crate) mod error;
pub(crate) mod pg_type;
pub(crate) mod pool;
pub(crate) mod prepare_queries;
pub(crate) mod read_migrations;
pub(crate) mod read_queries;
pub(crate) mod run_migrations;

mod generic_client;
mod integration;
mod parse;
mod parse_file;
mod sanitize;

use clap::{Result, StructOpt};
use cli::{Action, Args, MigrationAction};
use codegen::generate;
use error::FmtError;
use pg_type::TypeRegistrar;
use pool::{cli_pool, create_pool};
use prepare_queries::prepare_modules;
use read_queries::read_queries;
use run_migrations::run_migrations;
use std::path::Path;
use std::process::Command;
use time::OffsetDateTime;

pub use error::Error;
pub use generic_client::GenericClient;

pub async fn run() -> Result<(), Error> {
    let args = Args::parse();

    match args.action {
        Action::Migration {
            action,
            migrations_path,
        } => match action {
            MigrationAction::New { name } => {
                let unix_ts = OffsetDateTime::now_utc().unix_timestamp();
                let file_path =
                    Path::new(&migrations_path).join(format!("{}_{}.sql", unix_ts, name));
                std::fs::write(file_path, "-- Write your migration SQL here\n")?;
                Ok(())
            }
            MigrationAction::Run {
                user,
                password,
                host,
                port,
            } => {
                let client = create_pool(user, password, host, port)?.get().await?;
                run_migrations(&client, &migrations_path).await?;

                Ok(())
            }
        },
        Action::Generation {
            migrations_path,
            queries_path,
            destination,
        } => {
            let mut type_registrar = TypeRegistrar::default();
            if let Err(e) = generation(
                &mut type_registrar,
                migrations_path,
                queries_path,
                destination,
            )
            .await
            {
                container::cleanup()?;
                return Err(e);
            }

            format_project()?;

            Ok(())
        }
    }
}

pub(crate) fn format_project() -> Result<(), FmtError> {
    if Command::new("cargo").arg("fmt").spawn()?.wait()?.success() {
        Ok(())
    } else {
        Err(FmtError::RustFmt)
    }
}

pub(crate) async fn generation(
    type_registrar: &mut TypeRegistrar,
    migrations_path: String,
    queries_path: String,
    destination: String,
) -> Result<(), Error> {
    let modules = read_queries(&queries_path)?;
    container::setup()?;
    let client = cli_pool()?.get().await?;
    run_migrations(&client, &migrations_path).await?;
    let modules = prepare_modules(&client, type_registrar, modules).await?;
    generate(type_registrar, modules, &destination)?;
    container::cleanup()?;

    Ok(())
}
