use std::{fmt::Display, process::ExitCode};

use crate::{codegen::run_codegen_test, errors::run_errors_test};
use clap::Parser;
use cornucopia::container;

mod codegen;
mod errors;
mod fixtures;
mod utils;

/// Integration test CLI arguments
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// Format test descriptors and update error msg
    #[clap(long)]
    apply_errors: bool,
    /// Update the project's generated code
    #[clap(long)]
    apply_codegen: bool,
    /// Use `podman` instead of `docker`
    #[clap(short, long)]
    podman: bool,
}

/// Print error to stderr
fn display<T, E: Display>(result: Result<T, E>) -> Result<T, E> {
    if let Err(err) = &result {
        eprintln!("{err}");
    }
    result
}

// Run test, return true if all test are successful
fn test(
    Args {
        apply_errors,
        apply_codegen,
        podman,
    }: Args,
) -> bool {
    // Start by removing previous container if it was left open
    container::cleanup(podman).ok();
    container::setup(podman).unwrap();
    let successful = std::panic::catch_unwind(|| {
        let mut client = cornucopia::conn::cornucopia_conn().unwrap();
        display(run_errors_test(&mut client, apply_errors)).unwrap()
            && display(run_codegen_test(&mut client, apply_codegen)).unwrap()
    });
    container::cleanup(podman).unwrap();
    successful.unwrap()
}

/// Main entry point
fn main() -> ExitCode {
    let args = Args::parse();
    if test(args) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

#[cfg(test)]
mod test {
    use crate::test;

    #[test]
    fn run() {
        assert!(test(crate::Args {
            apply_errors: false,
            apply_codegen: false,
            podman: false
        }))
    }
}
