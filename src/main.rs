pub(crate) mod cli;
pub(crate) mod codegen;
pub(crate) mod conn;
pub(crate) mod container;
pub(crate) mod error;
pub(crate) mod integration;
pub(crate) mod parser;
pub(crate) mod prepare_queries;
pub(crate) mod read_migrations;
pub(crate) mod read_queries;
pub(crate) mod run_migrations;
pub(crate) mod type_registrar;

use crate::cli::run;
use crate::error::Error;

fn main() -> Result<(), Error> {
    let result = run();
    if let Err(e) = &result {
        eprintln!("{e}");
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}
