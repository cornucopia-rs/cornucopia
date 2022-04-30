use deadpool_postgres::{Config, Runtime};
use tokio_postgres::NoTls;

pub mod cornucopia;

#[tokio::main]
pub async fn main() {
    // Take a look at the generated `cornucopia.rs` file if you want to
    // see what it looks like under the hood.
    use crate::cornucopia::queries::module_1::*;
    use crate::cornucopia::queries::module_2::*;
    use crate::cornucopia::types::public::SpongebobCharacter;

    // Connection pool configuration
    // This has nothing to do with cornucopia, please look at
    // `tokio_postgres` and `deadpool_postgres` for details
    let mut cfg = Config::new();
    cfg.user = Some(String::from("postgres"));
    cfg.password = Some(String::from("postgres"));
    cfg.host = Some(String::from("127.0.0.1"));
    cfg.port = Some(5432);
    cfg.dbname = Some(String::from("postgres"));
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    let mut client = pool.get().await.unwrap();

    // An example of how transactions work. Pretty easy :)
    // Just don't forget to `.commit()` when you're done.
    {
        let transaction = client.transaction().await.unwrap();
        println!(
            "{:?}",
            insert_book(&transaction, "The Great Gatsby").await.unwrap()
        );
        println!("{:?}", books(&transaction).await.unwrap());
        transaction.commit().await.unwrap();
    }

    // Regular queries. These queries have been chosen to showcase the
    // features of cornucopia, including custom types, nullable return columns,
    // quantifiers, etc. You can compare with the SQL queries in the `queries` folder.
    println!("{:?}", authors(&client).await.unwrap());
    println!("{:?}", books_opt_ret_param(&client).await.unwrap());
    println!("{:?}", books_from_author_id(&client, &0).await.unwrap());
    println!("{:?}", author_name_by_id(&client, &0).await.unwrap());
    println!("{:?}", author_name_by_id_opt(&client, &0).await.unwrap());
    println!(
        "{:?}",
        author_name_starting_with(&client, "Jo").await.unwrap()
    );
    println!("{:?}", return_custom_type(&client).await.unwrap());
    println!(
        "{:?}",
        select_where_custom_type(&client, &SpongebobCharacter::Bob).await
    );
}
