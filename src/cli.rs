use clap::{Parser, Subcommand};

/// Command line interface to interact with Cornucopia SQL.
#[derive(Parser, Debug)]
#[clap(version)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
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
pub enum MigrationsAction {
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
pub enum GenerateLiveAction {
    /// Generate your modules against your own db
    Live {
        /// Postgres url to the database
        #[clap(short, long)]
        url: String,
    },
}
