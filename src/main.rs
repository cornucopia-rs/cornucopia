mod cli;
mod codegen;
mod container;
mod parse;
mod parse_file;
mod pg_type;
mod pool;
mod prepare_queries;
mod read_migrations;
mod read_queries;
mod run_migrations;
mod sanitize;

use crate::codegen::generate;
use crate::prepare_queries::prepare_modules;
use clap::{Result, StructOpt};
use cli::{Action, Args, MigrationAction};
use error::Error;
use pool::{cli_pool, create_pool};
use run_migrations::run_migrations;
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
                return Err(e)
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

pub mod error {
    use crate::codegen::error::Error as CodegenError;
    use crate::container::error::Error as ContainerError;
    use crate::prepare_queries::error::Error as PrepareQueriesError;
    use crate::run_migrations::error::Error as MigrationError;
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    #[error("the program encountered an unexpected error")]
    pub enum Error {
        ContainerError(#[from] ContainerError),
        Codegen(#[from] CodegenError),
        PrepareQueries(#[from] PrepareQueriesError),
        NewMigration(#[from] std::io::Error),
        Migration(#[from] MigrationError),
        PoolCreation(#[from] deadpool_postgres::CreatePoolError),
        Pool(#[from] deadpool_postgres::PoolError),
    }
}
