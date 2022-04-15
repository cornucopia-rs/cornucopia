use error::Error;
use std::process::{Command, Stdio};

pub fn setup() -> Result<(), Error> {
    spawn_container()?;
    wait_until_postgres_started(120, 1000)?;
    Ok(())
}

pub fn cleanup() -> Result<(), Error> {
    stop_container()?;
    remove_container()?;
    Ok(())
}

fn spawn_container() -> Result<(), Error> {
    Command::new("docker")
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
        .spawn()
        .map_err(Error::RunContainer)?
        .wait()
        .map_err(Error::RunContainer)?;
    Ok(())
}

fn is_postgres_healthy() -> Result<bool, Error> {
    Ok(Command::new("docker")
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

fn wait_until_postgres_started(max_retries: u64, ms_per_retry: u64) -> Result<(), Error> {
    let mut nb_retries = 0;
    while !is_postgres_healthy()? {
        if nb_retries >= max_retries {
            return Err(Error::MaxNbRetries);
        };
        std::thread::sleep(std::time::Duration::from_millis(ms_per_retry));
        nb_retries += 1;
    }
    Ok(())
}

fn stop_container() -> Result<(), Error> {
    Command::new("docker")
        .arg("stop")
        .arg("cornucopia_postgres")
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .map_err(Error::StopContainer)?
        .wait()
        .map_err(Error::StopContainer)?;

    Ok(())
}

fn remove_container() -> Result<(), Error> {
    Command::new("docker")
        .arg("rm")
        .arg("-v")
        .arg("cornucopia_postgres")
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .map_err(Error::RemoveContainer)?;

    Ok(())
}

pub mod error {
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    #[error("Error encountered while running docker command. Please check that docker is installed, and that the daemon is running. If you are a Linux user, please check that you are in the `docker` group")]
    pub enum Error {
        #[error("couldn't start database container")]
        RunContainer(std::io::Error),
        #[error("encountered error while probing database container health")]
        HealthCheck(std::io::Error),
        #[error("couldn't stop database container")]
        StopContainer(std::io::Error),
        #[error("couldn't clean up database container")]
        RemoveContainer(std::io::Error),
        #[error("max number of retries reached while waiting for database container to start")]
        MaxNbRetries,
    }
}
