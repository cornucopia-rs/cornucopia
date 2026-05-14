use std::sync::OnceLock;
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

// Global runtime for connection handling
static RUNTIME: OnceLock<Runtime> = OnceLock::new();

fn get_runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| Runtime::new().expect("Failed to create Tokio runtime"))
}

fn connect(config: Config) -> Result<Client, Error> {
    // Use futures::executor::block_on which works from any context
    let (tx, rx) = std::sync::mpsc::channel();

    get_runtime().spawn(async move {
        let result = async {
            let (client, conn) = config.connect(NoTls).await?;
            tokio::spawn(conn);
            Ok::<Client, Error>(client)
        }
        .await;
        tx.send(result).unwrap();
    });

    rx.recv().unwrap()
}

// Sets the search path for the given client.
pub fn set_search_path(client: &Client, search_path: &str) -> Result<(), Error> {
    futures::executor::block_on(client.execute(&format!("SET search_path TO {search_path}"), &[]))
        .map_err(Error::from)?;
    Ok(())
}

pub(crate) mod error {
    use miette::Diagnostic;

    #[derive(Debug, thiserror::Error, Diagnostic)]
    #[error("Couldn't establish a connection with the database.")]
    pub struct Error(#[from] pub tokio_postgres::Error);
}
