use clap::{Result, StructOpt};
use cornucopia::cli::{Action, Args, MigrationAction};
use cornucopia::codegen::generate;
use cornucopia::container;
use cornucopia::error::{Error, FmtError};
use cornucopia::pg_type::TypeRegistrar;
use cornucopia::pool::{cli_pool, create_pool};
use cornucopia::prepare_queries::prepare_modules;
use cornucopia::run_migrations::run_migrations;
use std::path::Path;
use std::process::Command;
use time::OffsetDateTime;

#[tokio::main]
async fn main() -> Result<(), Error> {
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

pub fn format_project() -> Result<(), FmtError> {
    if Command::new("cargo").arg("fmt").spawn()?.wait()?.success() {
        Ok(())
    } else {
        Err(FmtError::RustFmt)
    }
}

pub async fn generation(
    type_registrar: &mut TypeRegistrar,
    migrations_path: String,
    queries_path: String,
    destination: String,
) -> Result<(), Error> {
    container::setup()?;
    let client = cli_pool()?.get().await?;
    run_migrations(&client, &migrations_path).await?;
    let modules = prepare_modules(type_registrar, &client, &queries_path).await?;
    generate(type_registrar, modules, &destination)?;
    container::cleanup()?;

    Ok(())
}
