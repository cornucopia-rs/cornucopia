use error::Error;
use std::process::{Command, Stdio};

use self::error::{RemoveContainerError, RunContainerError, StopContainerError};

pub fn setup(podman: bool) -> Result<(), Error> {
    spawn_container(podman)?;
    healthcheck(podman, 120, 1000)?;
    Ok(())
}

pub fn cleanup(podman: bool) -> Result<(), Error> {
    stop_container(podman)?;
    remove_container(podman)?;
    Ok(())
}

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
        .status()?
        .success();

    if success {
        Ok(())
    } else {
        Err(RunContainerError::Status)
    }
}

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
    Ok(())
}

fn stop_container(podman: bool) -> Result<(), StopContainerError> {
    let command = if podman { "podman" } else { "docker" };
    let success = Command::new(&command)
        .arg("stop")
        .arg("cornucopia_postgres")
        .status()?
        .success();

    if success {
        Ok(())
    } else {
        Err(StopContainerError::Status)
    }
}

fn remove_container(podman: bool) -> Result<(), RemoveContainerError> {
    let command = if podman { "podman" } else { "docker" };
    let success = Command::new(&command)
        .arg("rm")
        .arg("-v")
        .arg("cornucopia_postgres")
        .status()?
        .success();

    if success {
        Ok(())
    } else {
        Err(RemoveContainerError::Status)
    }
}

pub mod error {
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    #[error("Error encountered while running docker command. Please check that docker is installed, and that the daemon is running. If you are a Linux user, please check that you are in the `docker` group")]
    pub enum Error {
        #[error("couldn't start database container")]
        RunContainer(#[from] RunContainerError),
        #[error("encountered error while probing database container health")]
        HealthCheck(std::io::Error),
        #[error("couldn't stop database container")]
        StopContainer(#[from] StopContainerError),
        #[error("couldn't clean up database container")]
        RemoveContainer(#[from] RemoveContainerError),
        #[error("max number of retries reached while waiting for database container to start")]
        MaxNbRetries,
    }

    #[derive(Debug, ThisError)]
    #[error("couldn't start database container")]
    pub enum RunContainerError {
        Io(#[from] std::io::Error),
        #[error("command returned with an error status")]
        Status,
    }

    #[derive(Debug, ThisError)]
    #[error("couldn't stop database container")]
    pub enum StopContainerError {
        Io(#[from] std::io::Error),
        #[error("command returned with an error status")]
        Status,
    }

    #[derive(Debug, ThisError)]
    #[error("couldn't clean up database container")]
    pub enum RemoveContainerError {
        Io(#[from] std::io::Error),
        #[error("command returned with an error status")]
        Status,
    }
}
