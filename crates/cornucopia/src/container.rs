use std::process::{Command, Stdio};

use self::error::Error;

/// Starts Cornucopia's database container and wait until it reports healthy.
pub fn setup(podman: bool) -> Result<(), Error> {
    spawn_container(podman)?;
    healthcheck(podman, 120, 500)?;
    Ok(())
}

/// Stop and remove a container and its volume.
pub fn cleanup(podman: bool) -> Result<(), Error> {
    stop_container(podman)?;
    remove_container(podman)?;
    Ok(())
}

/// Starts Cornucopia's database container.
fn spawn_container(podman: bool) -> Result<(), Error> {
    cmd(
        podman,
        &[
            "run",
            "-d",
            "--name",
            "cornucopia_postgres",
            "-p",
            "5435:5432",
            "-e",
            "POSTGRES_PASSWORD=postgres",
            "docker.io/library/postgres:latest",
        ],
        "spawn container",
    )
}

/// Checks if Cornucopia's container reports healthy
fn is_postgres_healthy(podman: bool) -> Result<bool, Error> {
    Ok(cmd(
        podman,
        &["exec", "cornucopia_postgres", "pg_isready"],
        "check container health",
    )
    .is_ok())
}

/// This function controls how the healthcheck retries are handled.
fn healthcheck(podman: bool, max_retries: u64, ms_per_retry: u64) -> Result<(), Error> {
    let slow_threshold = 10 + max_retries / 10;
    let mut nb_retries = 0;
    while !is_postgres_healthy(podman)? {
        if nb_retries >= max_retries {
            return Err(Error::new(
                String::from("Cornucopia reached the max number of connection retries"),
                podman,
            ));
        };
        std::thread::sleep(std::time::Duration::from_millis(ms_per_retry));
        nb_retries += 1;

        if nb_retries % slow_threshold == 0 {
            println!("Container startup slower than expected ({nb_retries} retries out of {max_retries})");
        }
    }
    // Just for extra safety...
    std::thread::sleep(std::time::Duration::from_millis(250));
    Ok(())
}

/// Stops Cornucopia's container.
fn stop_container(podman: bool) -> Result<(), Error> {
    cmd(podman, &["stop", "cornucopia_postgres"], "stop container")
}

/// Removes Cornucopia's container and its volume.
fn remove_container(podman: bool) -> Result<(), Error> {
    cmd(
        podman,
        &["rm", "-v", "cornucopia_postgres"],
        "remove container",
    )
}

fn cmd(podman: bool, args: &[&'static str], action: &'static str) -> Result<(), Error> {
    let command = if podman { "podman" } else { "docker" };
    let output = Command::new(command)
        .args(args)
        .stderr(Stdio::piped())
        .stdout(Stdio::null())
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        Err(Error::new(
            format!("`{command}` couldn't {action}: {err}"),
            podman,
        ))
    }
}

pub(crate) mod error {
    use std::fmt::Debug;

    use miette::Diagnostic;
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError, Diagnostic)]
    #[error("{msg}")]
    pub struct Error {
        msg: String,
        #[help]
        pub help: Option<String>,
    }

    impl Error {
        pub fn new(msg: String, podman: bool) -> Self {
            let help = if podman {
                "Make sure that port 5435 is usable and that no container named `cornucopia_postgres` already exists."
            } else {
                "First, check that the docker daemon is up-and-running. Then, make sure that port 5435 is usable and that no container named `cornucopia_postgres` already exists."
            };
            Error {
                msg,
                help: Some(String::from(help)),
            }
        }
    }

    impl From<std::io::Error> for Error {
        fn from(e: std::io::Error) -> Self {
            Self {
                msg: format!("{e:#}"),
                help: None,
            }
        }
    }
}
