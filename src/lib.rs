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
use cli::{Action, Args, GenerateLiveAction, MigrationsAction};
use codegen::generate;
use error::FmtError;
use pg_type::TypeRegistrar;
use pool::{cornucopia_pool, from_url};
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
        Action::Migrations {
            action,
            migrations_path,
        } => match action {
            MigrationsAction::New { name } => {
                let unix_ts = OffsetDateTime::now_utc().unix_timestamp();
                let file_path =
                    Path::new(&migrations_path).join(format!("{}_{}.sql", unix_ts, name));
                std::fs::write(file_path, "-- Write your migration SQL here\n")?;
                Ok(())
            }
            MigrationsAction::Run { url } => {
                let client = pool::from_url(&url)?.get().await?;
                run_migrations(&client, &migrations_path).await?;

                Ok(())
            }
        },
        Action::Generate {
            action,
            podman,
            migrations_path,
            queries_path,
            destination,
        } => {
            let mut type_registrar = TypeRegistrar::default();
            match action {
                Some(GenerateLiveAction::Live { url }) => {
                    let modules = read_queries(&queries_path)?;
                    let client = from_url(&url)?.get().await?;
                    let modules = prepare_modules(&client, &mut type_registrar, modules).await?;
                    generate(&type_registrar, modules, &destination)?;
                }
                None => {
                    if let Err(e) = generate_action(
                        &mut type_registrar,
                        podman,
                        migrations_path,
                        queries_path,
                        destination,
                    )
                    .await
                    {
                        container::cleanup(podman)?;
                        return Err(e);
                    }

                    format_project()?;
                }
            }

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

pub(crate) async fn generate_action(
    type_registrar: &mut TypeRegistrar,
    podman: bool,
    migrations_path: String,
    queries_path: String,
    destination: String,
) -> Result<(), Error> {
    let modules = read_queries(&queries_path)?;
    container::setup(podman)?;
    let client = cornucopia_pool()?.get().await?;
    run_migrations(&client, &migrations_path).await?;
    let prepared_modules = prepare_modules(&client, type_registrar, modules).await?;
    generate(type_registrar, prepared_modules, &destination)?;
    container::cleanup(podman)?;

    Ok(())
}
