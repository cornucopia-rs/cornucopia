use std::str::FromStr;
use tokio_postgres::{Client, Config, Error, NoTls};

/// Creates a non-TLS connection from a URL.
pub(crate) async fn from_url(url: &str) -> Result<Client, Error> {
    let config = tokio_postgres::Config::from_str(url)?;
    from_config(&config).await
}

/// Create a non-TLS connection for usage internal to Cornucopia.
pub(crate) async fn cornucopia_conn() -> Result<Client, Error> {
    from_config(
        Config::new()
            .user("postgres")
            .password("postgres")
            .host("127.0.0.1")
            .port(5432)
            .dbname("postgres"),
    )
    .await
}

async fn from_config(config: &Config) -> Result<Client, Error> {
    // Connect to the database.
    let (client, connection) = config.connect(NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(connection);

    Ok(client)
}
