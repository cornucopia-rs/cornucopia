use std::process::{Command, Stdio};

use self::error::Error;

static DEFAULT_CONTAINER_NAME: &str = "cornucopia_postgres";
static DEFAULT_DOCKER_IMAGE: &str = "docker.io/library/postgres:latest";
static DEFAULT_PORT: u16 = 5435;

/// Container related options
#[derive(Clone, Debug, clap::Parser)]
pub struct ContainerOpts {
    /// Run using podman instead of docker
    #[clap(short = 'p', long = "podman")]
    pub podman: bool,
    /// Port under which to expose the cornucopia specific PostgreSQL container
    #[clap(long = "port", default_value_t = DEFAULT_PORT)]
    pub port: u16,
    /// Name of the container to start
    #[clap(long = "container-name", default_value = DEFAULT_CONTAINER_NAME)]
    pub container_name: String,
    #[clap(long = "docker-image", default_value = DEFAULT_DOCKER_IMAGE)]
    pub docker_image: String,
}

impl Default for ContainerOpts {
    fn default() -> Self {
        Self {
            podman: false,
            port: DEFAULT_PORT,
            container_name: DEFAULT_CONTAINER_NAME.into(),
            docker_image: DEFAULT_DOCKER_IMAGE.into(),
            // startup_delay: STARTUP_DELAY,
        }
    }
}

/// Starts Cornucopia's database container and wait until it reports healthy.
pub fn setup(opts: &ContainerOpts) -> Result<(), Error> {
    spawn_container(opts)?;
    healthcheck(opts, 120, 50)?;
    Ok(())
}

/// Stop and remove a container and its volume.
pub fn cleanup(opts: &ContainerOpts) -> Result<(), Error> {
    stop_container(opts)?;
    remove_container(opts)?;
    Ok(())
}

/// Starts Cornucopia's database container.
fn spawn_container(opts: &ContainerOpts) -> Result<(), Error> {
    let args = [
        "run",
        "-d",
        "--name",
        &opts.container_name,
        "-p",
        &format!("{}:5432", opts.port),
        "-e",
        "POSTGRES_PASSWORD=postgres",
        &opts.docker_image,
    ];
    cmd(opts, &args, "spawn container")
}

/// Checks if Cornucopia's container reports healthy
fn is_postgres_healthy(opts: &ContainerOpts) -> Result<bool, Error> {
    Ok(cmd(
        opts,
        &["exec", &opts.container_name, "pg_isready"],
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
fn stop_container(opts: &ContainerOpts) -> Result<(), Error> {
    cmd(opts, &["stop", &opts.container_name], "stop container")
}

/// Removes Cornucopia's container and its volume.
fn remove_container(opts: &ContainerOpts) -> Result<(), Error> {
    cmd(
        opts,
        &["rm", "-v", &opts.container_name],
        "remove container",
    )
}

fn cmd(opts: &ContainerOpts, args: &[&str], action: &'static str) -> Result<(), Error> {
    let command = if opts.podman { "podman" } else { "docker" };
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
            opts,
        ))
    }
}

pub(crate) mod error {
    use std::fmt::Debug;

    use super::ContainerOpts;
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
        pub fn new(msg: String, opts: &ContainerOpts) -> Self {
            let help = if opts.podman {
                format!("Make sure that port {} is usable and that no container named `{}` already exists.", opts.port, opts.container_name)
            } else {
                format!("First, check that the docker daemon is up-and-running. Then, make sure that port {} is usable and that no container named `{}` already exists.", opts.port, opts.container_name)
            };
            Error {
                msg,
                help: Some(help),
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
