use postgres::{Client, Config, Error, NoTls};

/// Creates a non-TLS connection from a URL.
pub(crate) fn from_url(url: &str) -> Result<Client, Error> {
    Client::connect(url, NoTls)
}

/// Create a non-TLS connection for usage internal to Cornucopia.
pub(crate) fn cornucopia_conn() -> Result<Client, Error> {
    Config::new()
        .user("postgres")
        .password("postgres")
        .host("127.0.0.1")
        .port(5432)
        .dbname("postgres")
        .connect(NoTls)
}
