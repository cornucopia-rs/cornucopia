use clap::{Parser, Subcommand};

/// Command line interface to interact with Cornucopia SQL.
#[derive(Parser, Debug)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    /// Create and run migrations
    Migration {
        #[clap(subcommand)]
        action: MigrationAction,
        /// Folder containing the migrations
        #[clap(short, long, default_value = "migrations")]
        migrations_path: String,
    },
    /// Generate Rust modules from queries
    Generation {
        /// Folder containing the migrations
        #[clap(short, long)]
        podman: bool,
        /// Folder containing the migrations
        #[clap(short, long, default_value = "migrations/")]
        migrations_path: String,
        /// Folder containing the queries
        #[clap(short, long, default_value = "queries/")]
        queries_path: String,
        /// Destination folder for generated modules
        #[clap(short, long, default_value = "src/cornucopia.rs")]
        destination: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum MigrationAction {
    /// Create a new migration
    New { name: String },
    /// Run all migrations
    Run {
        /// Postgres url to the database
        #[clap(long)]
        user: String,
        #[clap(long)]
        password: String,
        #[clap(long)]
        host: String,
        #[clap(long)]
        port: u16,
    },
}
