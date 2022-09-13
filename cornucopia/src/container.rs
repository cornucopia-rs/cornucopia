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
        .arg("5435:5432")
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
        Err(Error::new(
            format!("`{command}` couldn't spawn container."),
            podman,
        ))
    }
}

/// Checks if Cornucopia's container reports healthy
fn is_postgres_healthy(podman: bool) -> Result<bool, Error> {
    let command = if podman { "podman" } else { "docker" };
    Ok(Command::new(command)
        .arg("exec")
        .arg("cornucopia_postgres")
        .arg("pg_isready")
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .map_err(|_| {
            Error::new(
                format!("`{command}` couldn't check container health."),
                podman,
            )
        })?
        .wait()
        .map_err(|_| {
            Error::new(
                format!("`{command}` couldn't check contaienr health."),
                podman,
            )
        })?
        .success())
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
    let command = if podman { "podman" } else { "docker" };
    let success = Command::new(command)
        .arg("stop")
        .arg("cornucopia_postgres")
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .status()?
        .success();

    if success {
        Ok(())
    } else {
        Err(Error::new(
            format!("`{command}` couldn't stop container."),
            podman,
        ))
    }
}

/// Removes Cornucopia's container and its volume.
fn remove_container(podman: bool) -> Result<(), Error> {
    let command = if podman { "podman" } else { "docker" };
    let success = Command::new(command)
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
        Err(Error::new(
            format!("`{command}` couldn't remove container."),
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
