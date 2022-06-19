use std::process::{Command, Stdio};

use self::error::Error;

/// Starts Cornucopia's database container and wait until it reports healthy.
pub fn setup(podman: bool) -> Result<(), Error> {
    spawn_container(podman)?;
    healthcheck(podman, 120, 50)?;
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
    let command = if podman { "podman" } else { "docker" };
    let success = Command::new(&command)
        .arg("run")
        .arg("-d")
        .arg("--name")
        .arg("cornucopia_postgres")
        .arg("-p")
        .arg("5432:5432")
        .arg("-e")
        .arg("POSTGRES_PASSWORD=postgres")
        .arg("postgres")
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .status()?
        .success();

    if success {
        Ok(())
    } else {
        Err(Error::from("Couldn't spawn container."))
    }
}

/// Checks if Cornucopia's container reports healthy
fn is_postgres_healthy(podman: bool) -> Result<bool, Error> {
    let command = if podman { "podman" } else { "docker" };
    Ok(Command::new(&command)
        .arg("exec")
        .arg("cornucopia_postgres")
        .arg("pg_isready")
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .map_err(|_| Error::from("Couldn't check container health."))?
        .wait()
        .map_err(|_| Error::from("Couldn't check contaienr health."))?
        .success())
}

/// This function controls how the healthcheck retries are handled.
fn healthcheck(podman: bool, max_retries: u64, ms_per_retry: u64) -> Result<(), Error> {
    let slow_threshold = 10 + max_retries / 10;
    let mut nb_retries = 0;
    while !is_postgres_healthy(podman)? {
        if nb_retries >= max_retries {
            return Err(Error::from("reached max number of connection retries"));
        };
        std::thread::sleep(std::time::Duration::from_millis(ms_per_retry));
        nb_retries += 1;

        if nb_retries % slow_threshold == 0 {
            println!("Container startup slower than expected ({nb_retries} retries out of {max_retries})")
        }
    }
    // Just for extra safety...
    std::thread::sleep(std::time::Duration::from_millis(250));
    Ok(())
}

/// Stops Cornucopia's container.
fn stop_container(podman: bool) -> Result<(), Error> {
    let command = if podman { "podman" } else { "docker" };
    let success = Command::new(&command)
        .arg("stop")
        .arg("cornucopia_postgres")
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .status()?
        .success();

    if success {
        Ok(())
    } else {
        Err(Error::from("Couldn't stop container."))
    }
}

/// Removes Cornucopia's container and its volume.
fn remove_container(podman: bool) -> Result<(), Error> {
    let command = if podman { "podman" } else { "docker" };
    let success = Command::new(&command)
        .arg("rm")
        .arg("-v")
        .arg("cornucopia_postgres")
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .status()?
        .success();

    if success {
        Ok(())
    } else {
        Err(Error::from("Couldn't remove container."))
    }
}
pub(crate) mod error {
    use std::fmt::Debug;

    use miette::Diagnostic;
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError, Diagnostic)]
    #[error("{msg}")]
    #[diagnostic(
        help("if you are using `docker`, please ensure that the daemon is up-and-running. You must also ensure that no container named `cornucopia_postgres` already exists.")
    )]
    pub struct Error {
        pub msg: String,
    }

    impl From<std::io::Error> for Error {
        fn from(e: std::io::Error) -> Self {
            Self {
                msg: format!("{e:#}"),
            }
        }
    }

    impl From<&str> for Error {
        fn from(s: &str) -> Self {
            Self { msg: s.into() }
        }
    }
}
