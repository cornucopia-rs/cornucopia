use error::Error;
use std::str::FromStr;

use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;

pub(crate) fn from_url(url: &str) -> Result<Pool, Error> {
    let config = tokio_postgres::Config::from_str(url)?;
    let manager = deadpool_postgres::Manager::new(config, tokio_postgres::NoTls);
    let pool = deadpool_postgres::Pool::builder(manager).build()?;
    Ok(pool)
}

pub(crate) fn cornucopia_pool() -> Result<Pool, Error> {
    let mut cfg = Config::new();
    cfg.user = Some(String::from("postgres"));
    cfg.password = Some(String::from("postgres"));
    cfg.host = Some(String::from("127.0.0.1"));
    cfg.port = Some(5432);
    cfg.dbname = Some(String::from("postgres"));
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;
    Ok(pool)
}

pub(crate) mod error {
    use deadpool_postgres::BuildError as PoolBuilderError;
    use deadpool_postgres::CreatePoolError;
    use thiserror::Error as ThisError;
    #[derive(Debug, ThisError)]
    #[error("An error happened when trying to acquire a connection")]
    pub(crate) enum Error {
        #[error("Invalid database URL")]
        PoolBuilder(#[from] PoolBuilderError),
        #[error("Invalid database URL")]
        DbUrl(#[from] CreatePoolError),
        #[error("Invalid database URL")]
        Db(#[from] tokio_postgres::Error),
    }
}
