use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::{
    conn,
    container::{self, ContainerOpts},
    error::Error,
    generate_live, generate_managed, CodegenSettings,
};

/// Command line interface to interact with Cornucopia SQL.
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// Folder containing the queries
    #[clap(short, long, default_value = "queries/")]
    queries_path: PathBuf,
    /// Destination folder for generated modules
    #[clap(short, long, default_value = "src/cornucopia.rs")]
    destination: PathBuf,
    #[clap(subcommand)]
    action: Action,
    /// Generate synchronous rust code
    #[clap(long)]
    sync: bool,
    /// Generate asynchronous rust code
    #[clap(long)]
    r#async: bool,
    /// Derive serde's `Serialize` trait for generated types.
    #[clap(long)]
    serialize: bool,
}

#[derive(Debug, Subcommand)]
enum Action {
    /// Generate your modules against your own db
    Live {
        /// Postgres url to the database
        url: String,
    },
    /// Generate your modules against schema files
    Schema {
        /// SQL files containing the database schema
        schema_files: Vec<PathBuf>,
        /// Container options
        #[command(flatten)]
        container_opts: ContainerOpts,
    },
}

// Main entrypoint of the CLI. Parses the args and calls the appropriate routines.
pub fn run() -> Result<(), Error> {
    let Args {
        queries_path,
        destination,
        action,
        sync,
        r#async,
        serialize,
    } = Args::parse();

    let settings = CodegenSettings {
        gen_async: r#async || !sync,
        gen_sync: sync,
        derive_ser: serialize,
    };

    match action {
        Action::Live { url } => {
            let mut client = conn::from_url(&url)?;
            generate_live(&mut client, &queries_path, Some(&destination), settings)?;
        }
        Action::Schema {
            schema_files,
            container_opts,
        } => {
            // Run the generate command. If the command is unsuccessful, cleanup Cornucopia's container
            if let Err(e) = generate_managed(
                queries_path,
                &schema_files,
                Some(destination),
                &container_opts,
                settings,
            ) {
                container::cleanup(&container_opts).ok();
                return Err(e);
            }
        }
    };
    Ok(())
}
