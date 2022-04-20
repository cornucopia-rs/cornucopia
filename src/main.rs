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

mod integration;
mod parse;
mod parse_file;
mod sanitize;

use crate::cli::run;
use crate::error::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run().await
}
