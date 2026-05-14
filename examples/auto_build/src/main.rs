use codegen::deadpool_postgres::{Config, Runtime};
use codegen::tokio_postgres::NoTls;

// Take a look at the generated `cornucopia` (codegen) crate if you want to
// see what it looks like under the hood.
use codegen::queries::module_1::example_query;

// Add more schema files and queries, rebuild the crate,
// and observe how your cornucopia modules are regenerated!
#[tokio::main]
async fn main() {
    // Connection pool configuration
    // This has nothing to do with cornucopia, please look at
    // `tokio_postgres` and `deadpool_postgres` for details
    let mut cfg = Config::new();
    cfg.user = Some(String::from("postgres"));
    cfg.password = Some(String::from("postgres"));
    cfg.host = Some(String::from("127.0.0.1"));
    cfg.port = Some(5435);
    cfg.dbname = Some(String::from("postgres"));
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    let client = pool.get().await.unwrap();
    example_query().bind(&client).all().await.unwrap();
}
