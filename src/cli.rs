use std::{path::Path, process::Command};

use crate::{
    codegen::generate,
    container,
    error::{Error, FmtError},
    pg_type::TypeRegistrar,
    pool::{self, cornucopia_pool, from_url},
    prepare_queries::prepare_modules,
    read_queries::read_queries,
    run_migrations::run_migrations,
};
use clap::{Parser, Subcommand};
use time::OffsetDateTime;
/// Command line interface to interact with Cornucopia SQL.
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Debug, Subcommand)]
enum Action {
    /// Create and run migrations
    Migrations {
        #[clap(subcommand)]
        action: MigrationsAction,
        /// Folder containing the migrations
        #[clap(short, long, default_value = "migrations/")]
        migrations_path: String,
    },
    /// Generate Rust modules from queries
    Generate {
        /// Folder containing the migrations
        #[clap(short, long)]
        no_formatting: bool,
        /// Use `podman` instead of `docker`
        #[clap(short, long)]
        podman: bool,
        /// Folder containing the migrations (ignored if using the `live` command)
        #[clap(short, long, default_value = "migrations/")]
        migrations_path: String,
        /// Folder containing the queries
        #[clap(short, long, default_value = "queries/")]
        queries_path: String,
        /// Destination folder for generated modules
        #[clap(short, long, default_value = "src/cornucopia.rs")]
        destination: String,
        #[clap(subcommand)]
        action: Option<GenerateLiveAction>,
    },
}

#[derive(Debug, Subcommand)]
enum MigrationsAction {
    /// Create a new migration
    New { name: String },
    /// Run all migrations
    Run {
        /// Postgres url to the database
        #[clap(long)]
        url: String,
    },
}

#[derive(Debug, Subcommand)]
enum GenerateLiveAction {
    /// Generate your modules against your own db
    Live {
        /// Postgres url to the database
        #[clap(short, long)]
        url: String,
    },
}

pub(crate) async fn run() -> Result<(), Error> {
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
            no_formatting,
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
                        &migrations_path,
                        &queries_path,
                        &destination,
                    )
                    .await
                    {
                        container::cleanup(podman)?;
                        return Err(e);
                    }

                    if no_formatting {
                    } else {
                        format_generated_file(&destination)?
                    };
                }
            }

            Ok(())
        }
    }
}

pub(crate) fn format_generated_file(path: &str) -> Result<(), FmtError> {
    if Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .arg(path)
        .spawn()?
        .wait()?
        .success()
    {
        Ok(())
    } else {
        Err(FmtError::RustFmt)
    }
}

pub(crate) async fn generate_action(
    type_registrar: &mut TypeRegistrar,
    podman: bool,
    migrations_path: &str,
    queries_path: &str,
    destination: &str,
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
