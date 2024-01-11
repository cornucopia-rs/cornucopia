use postgres::{Client, Config, NoTls};

use crate::container::ContainerOpts;

use self::error::Error;

/// Creates a non-TLS connection from a URL.
pub(crate) fn from_url(url: &str) -> Result<Client, Error> {
    Ok(Client::connect(url, NoTls)?)
}

/// Create a non-TLS connection to the container managed by Cornucopia.
pub fn cornucopia_conn(opts: &ContainerOpts) -> Result<Client, Error> {
    Ok(Config::new()
        .user("postgres")
        .password("postgres")
        .host("127.0.0.1")
        .port(opts.port)
        .dbname("postgres")
        .connect(NoTls)?)
}

pub(crate) mod error {
    use miette::Diagnostic;

    #[derive(Debug, thiserror::Error, Diagnostic)]
    #[error("Couldn't establish a connection with the database.")]
    pub struct Error(#[from] pub postgres::Error);
}
