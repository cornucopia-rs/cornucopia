// Take a look at the generated `cornucopia.rs` file if you want to
// see what it looks like under the hood.
#[rustfmt::skip]
mod cornucopia;

use deadpool_postgres::{Config, Runtime};
use tokio_postgres::NoTls;

use crate::cornucopia::{
    queries::{
        module_1::insert_book,
        module_2::{
            author_name_by_id, authors, books, select_where_custom_type,
            AuthorNameStartingWithParams,
        },
    },
    types::public::SpongebobCharacter,
};

#[tokio::main]
pub async fn main() {
    // Connection pool configuration
    // Please look at `tokio_postgres` and `deadpool_postgres` for details.
    let mut cfg = Config::new();
    cfg.user = Some(String::from("postgres"));
    cfg.password = Some(String::from("postgres"));
    cfg.host = Some(String::from("127.0.0.1"));
    cfg.port = Some(5432);
    cfg.dbname = Some(String::from("postgres"));
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    let mut client = pool.get().await.unwrap();

    // Queries accept regular clients.
    println!("{:?}", authors(&client).vec().await.unwrap());

    // Queries also accept transactions
    // Don't forget to `.commit()` when you're done!
    {
        let transaction = client.transaction().await.unwrap();
        // Insert a book
        insert_book(&transaction, &"The Great Gatsby")
            .await
            .unwrap();
        // Use a map if needed
        let books = books(&transaction).vec().await.unwrap();
        println!("{books:?}");
        transaction.commit().await.unwrap();
    }

    // Using opt returns an optional row (zero or one).
    println!("{:?}", author_name_by_id(&client, &0).opt().await.unwrap());

    // The param struct can be more convenient
    // and less error-prone in some cases
    println!(
        "{:?}",
        AuthorNameStartingWithParams { start_str: "Jo" }
            .author_name_starting_with(&client)
            .vec()
            .await
            .unwrap()
    );

    // Custom types from your queries also work!
    println!(
        "{:?}",
        select_where_custom_type(&client, &SpongebobCharacter::Patrick)
            .one()
            .await
            .unwrap()
    );
}
