use clap::{Result, StructOpt};
use cornucopia::cli::{Action, Args, MigrationAction};
use cornucopia::codegen::generate;
use cornucopia::container;
use cornucopia::error::Error;
use cornucopia::pool::{cli_pool, create_pool};
use cornucopia::prepare_queries::prepare_modules;
use cornucopia::run_migrations::run_migrations;
use std::path::Path;
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
            if let Err(e) = generation(migrations_path, queries_path, destination).await {
                container::cleanup()?;
                return Err(e);
            }
            Ok(())
        }
    }
}

pub async fn generation(
    migrations_path: String,
    queries_path: String,
    destination: String,
) -> Result<(), Error> {
    container::setup()?;
    let client = cli_pool()?.get().await?;
    run_migrations(&client, &migrations_path).await?;
    let modules = prepare_modules(&client, &queries_path).await?;
    generate(modules, &destination)?;
    container::cleanup()?;

    Ok(())
}
