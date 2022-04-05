use deadpool_postgres::{Config, CreatePoolError, Pool, Runtime};
use tokio_postgres::NoTls;

pub fn cli_pool() -> Result<Pool, CreatePoolError> {
    create_pool(
        String::from("postgres"),
        String::from("postgres"),
        String::from("127.0.0.1"),
        5432,
    )
}

pub fn create_pool(
    user: String,
    password: String,
    host: String,
    port: u16,
) -> Result<Pool, CreatePoolError> {
    let mut cfg = Config::new();
    cfg.user = Some(user);
    cfg.password = Some(password);
    cfg.host = Some(host);
    cfg.port = Some(port);
    cfg.dbname = Some(String::from("postgres"));
    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
}
