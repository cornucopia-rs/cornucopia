use std::path::Path;

use crate::{
    codegen::generate,
    conn::{self, cornucopia_conn, from_url},
    container,
    error::Error,
    prepare_queries::prepare,
    read_queries::{read_query_modules, Module},
    run_migrations::run_migrations,
    type_registrar::TypeRegistrar,
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

// Main entrypoint of the CLI. Parses the args and calls the appropriate routines.
pub(crate) async fn run() -> Result<(), Error> {
    let args = Args::parse();

    match args.action {
        Action::Migrations {
            action,
            migrations_path,
        } => match action {
            MigrationsAction::New { name } => {
                // Create a timestamp of the current time.
                let unix_ts = OffsetDateTime::now_utc().unix_timestamp();
                // Format the target file name
                let file_path =
                    Path::new(&migrations_path).join(format!("{}_{}.sql", unix_ts, name));
                // Write file with header
                std::fs::write(file_path, "-- Write your migration SQL here\n")?;
                Ok(())
            }
            MigrationsAction::Run { url } => {
                // Runs all migrations at the target url
                let client = conn::from_url(&url).await?;
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
                    let modules = read_query_modules(&queries_path)?;
                    let client = from_url(&url).await?;
                    let modules = prepare(&client, &mut type_registrar, modules).await?;
                    generate(&type_registrar, modules, &destination)?;
                }
                None => {
                    let modules = read_query_modules(&queries_path)?;
                    container::setup(podman)?;
                    // Run the generate command. If the command is unsuccessful, cleanup Cornucopia's container
                    if let Err(e) = generate_action(
                        &mut type_registrar,
                        modules,
                        podman,
                        &migrations_path,
                        &destination,
                    )
                    .await
                    {
                        container::cleanup(podman)?;
                        return Err(e);
                    }
                }
            }

            Ok(())
        }
    }
}

/// Performs the `generate` CLI command
async fn generate_action(
    type_registrar: &mut TypeRegistrar,
    modules: Vec<Module>,
    podman: bool,
    migrations_path: &str,
    destination: &str,
) -> Result<(), Error> {
    let client = cornucopia_conn().await?;
    run_migrations(&client, migrations_path).await?;
    let prepared_modules = prepare(&client, type_registrar, modules).await?;
    generate(type_registrar, prepared_modules, destination)?;
    container::cleanup(podman)?;

    Ok(())
}
