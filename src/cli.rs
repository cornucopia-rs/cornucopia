use crate::{
    conn, container, error::Error, generate_live, generate_managed, new_migration,
    read_migrations::read_migrations, run_migrations, CodegenSettings,
};
use clap::{Parser, Subcommand};

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
        /// Generate synchronous rust code. Async otherwise.
        #[clap(long)]
        sync: bool,
        /// Derive serde's `Serialize` trait for generated types.
        #[clap(long)]
        serialize: bool,
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
pub fn run() -> Result<(), Error> {
    let args = Args::parse();

    match args.action {
        Action::Migrations {
            action,
            migrations_path,
        } => match action {
            MigrationsAction::New { name } => new_migration(&migrations_path, &name),
            MigrationsAction::Run { url } => {
                let mut client = conn::from_url(&url)?;
                let migrations = read_migrations(&migrations_path)?;
                run_migrations(&mut client, migrations)
            }
        },
        Action::Generate {
            action,
            podman,
            migrations_path,
            queries_path,
            destination,
            sync,
            serialize,
        } => {
            match action {
                Some(GenerateLiveAction::Live { url }) => {
                    let mut client = conn::from_url(&url)?;
                    generate_live(
                        &mut client,
                        &queries_path,
                        Some(&destination),
                        CodegenSettings {
                            is_async: !sync,
                            derive_ser: serialize,
                        },
                    )?;
                }
                None => {
                    // Run the generate command. If the command is unsuccessful, cleanup Cornucopia's container
                    if let Err(e) = generate_managed(
                        &queries_path,
                        &migrations_path,
                        Some(&destination),
                        podman,
                        CodegenSettings {
                            is_async: !sync,
                            derive_ser: serialize,
                        },
                    ) {
                        let _ = container::cleanup(podman);
                        return Err(e);
                    }
                }
            }

            Ok(())
        }
    }
}
