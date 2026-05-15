// Take a look at the generated `cornucopia` crate if you want to
// see what it looks like under the hood.
use cornucopia::queries::module_1::{characters, select_character_by_element};

#[tokio::main]
pub async fn main() {
    // You can learn which database connection types are compatible with Cornucopia in the book
    // https://cornucopia-rs.github.io/cornucopia/using_queries/db_connections.html
    let pool = create_pool().await.unwrap();
    let client = pool.get().await.unwrap();

    let characters = characters().bind(&client).all().await.unwrap();
    dbg!(&characters[0].name);

    let character = select_character_by_element()
        .bind(&client, &db_types::element::Element::Hydro)
        .one()
        .await
        .unwrap();

    dbg!(character);
}

/// Connection pool configuration.
///
/// This is just a simple example config, please look at
/// `tokio_postgres` and `deadpool_postgres` for details.
use cornucopia::deadpool_postgres::{Config, CreatePoolError, Pool, Runtime};
use cornucopia::tokio_postgres::NoTls;

async fn create_pool() -> Result<Pool, CreatePoolError> {
    let mut cfg = Config::new();
    cfg.user = Some(String::from("postgres"));
    cfg.password = Some(String::from("postgres"));
    cfg.host = Some(String::from("127.0.0.1"));
    cfg.port = Some(5435);
    cfg.dbname = Some(String::from("postgres"));
    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
}
