use tokio::runtime::Runtime;
use tokio_postgres::{Client, Config, NoTls};

use self::error::Error;

/// Creates a non-TLS connection from a URL.
pub(crate) fn from_url(url: &str) -> Result<Client, Error> {
    connect(url.parse()?)
}

/// Create a non-TLS connection to the container managed by Cornucopia.
pub fn cornucopia_conn() -> Result<Client, Error> {
    connect(
        Config::new()
            .user("postgres")
            .password("postgres")
            .host("127.0.0.1")
            .port(5435)
            .dbname("postgres")
            .clone(),
    )
}

fn connect(config: Config) -> Result<Client, Error> {
    let rt: &'static Runtime = Box::leak(Box::new(
        Runtime::new().expect("Failed to start async Runtime"),
    ));
    let client = rt.block_on(async {
        let (client, conn) = config.connect(NoTls).await.unwrap();
        rt.spawn(conn);
        client
    });
    Ok(client)
}

pub(crate) mod error {
    use miette::Diagnostic;

    #[derive(Debug, thiserror::Error, Diagnostic)]
    #[error("Couldn't establish a connection with the database.")]
    pub struct Error(#[from] pub tokio_postgres::Error);
}
