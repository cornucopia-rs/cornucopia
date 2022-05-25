use error::Error;
use std::process::{Command, Stdio};

use self::error::{RemoveContainerError, RunContainerError, StopContainerError};

/// Starts Cornucopia's database container and wait until it reports healthy.
pub fn setup(podman: bool) -> Result<(), Error> {
    spawn_container(podman)?;
    healthcheck(podman, 120, 1000)?;
    Ok(())
}

/// Stop and remove a container and its volume.
pub fn cleanup(podman: bool) -> Result<(), Error> {
    stop_container(podman)?;
    remove_container(podman)?;
    Ok(())
}

/// Starts Cornucopia's database container.
fn spawn_container(podman: bool) -> Result<(), RunContainerError> {
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
        Err(RunContainerError::Status)
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
        .map_err(Error::HealthCheck)?
        .wait()
        .map_err(Error::HealthCheck)?
        .success())
}

/// This function controls how the healthcheck retries are handled.
fn healthcheck(podman: bool, max_retries: u64, ms_per_retry: u64) -> Result<(), Error> {
    let mut nb_retries = 0;
    while !is_postgres_healthy(podman)? {
        if nb_retries >= max_retries {
            return Err(Error::MaxNbRetries);
        };
        std::thread::sleep(std::time::Duration::from_millis(ms_per_retry));
        nb_retries += 1;

        if nb_retries % 10 == 0 {
            println!("Container startup slower than expected ({nb_retries} retries out of {max_retries})")
        }
    }
    // Just for extra safety...
    std::thread::sleep(std::time::Duration::from_millis(250));
    Ok(())
}

/// Stops Cornucopia's container.
fn stop_container(podman: bool) -> Result<(), StopContainerError> {
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
        Err(StopContainerError::Status)
    }
}

/// Removes Cornucopia's container and its volume.
fn remove_container(podman: bool) -> Result<(), RemoveContainerError> {
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
        Err(RemoveContainerError::Status)
    }
}
pub(crate) mod error {
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    pub enum Error {
        #[error("Couldn't start database container ({0}). If you are using `docker`, please check that the daemon is up-and-running.")]
        RunContainer(#[from] RunContainerError),
        #[error("Encountered error while probing database container health. If you are using `docker`, please check that the daemon is up-and-running.")]
        HealthCheck(std::io::Error),
        #[error("Couldn't stop database container ({0}). If you are using `docker`, please check that the daemon is up-and-running.")]
        StopContainer(#[from] StopContainerError),
        #[error("Couldn't clean up database container ({0}). If you are using `docker`, please check that the daemon is up-and-running.")]
        RemoveContainer(#[from] RemoveContainerError),
        #[error("Max number of retries reached while waiting for database container to start. If you are using `docker`, please check that the daemon is up-and-running.")]
        MaxNbRetries,
    }

    #[derive(Debug, ThisError)]
    pub enum RunContainerError {
        #[error("{0}")]
        Io(#[from] std::io::Error),
        #[error("command returned with an error status")]
        Status,
    }

    #[derive(Debug, ThisError)]
    pub enum StopContainerError {
        #[error("{0}")]
        Io(#[from] std::io::Error),
        #[error("command returned with an error status")]
        Status,
    }

    #[derive(Debug, ThisError)]
    pub enum RemoveContainerError {
        #[error("{0}")]
        Io(#[from] std::io::Error),
        #[error("command returned with an error status")]
        Status,
    }
}
