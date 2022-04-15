use deadpool_postgres::{Config, Runtime};
use tokio_postgres::NoTls;

pub mod cornucopia;

#[tokio::main]
pub async fn main() {
    // Take a look at the generated `cornucopia.rs` file if you want to
    // see what it looks like under the hood.
    use cornucopia::queries::module_2::*;
    use cornucopia::transactions::module_1::insert_book_one;
    use cornucopia::transactions::module_2::books;
    use cornucopia::types::public::SpongebobCharacter;

    let mut cfg = Config::new();
    cfg.user = Some(String::from("postgres"));
    cfg.password = Some(String::from("postgres"));
    cfg.host = Some(String::from("127.0.0.1"));
    cfg.port = Some(5432);
    cfg.dbname = Some(String::from("postgres"));
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    let mut client = pool.get().await.unwrap();

    {
        let transaction = client.transaction().await.unwrap();
        println!("{:?}", insert_book_one(&transaction).await.unwrap());
        println!("{:?}", books(&transaction).await.unwrap());
        transaction.commit().await.unwrap();
    }

    println!("{:?}", authors(&client).await.unwrap());
    println!("{:?}", books_from_author_id(&client, &0).await.unwrap());
    println!("{:?}", author_name_by_id(&client, &1).await.unwrap());
    println!("{:?}", author_name_by_id_opt(&client, &-1).await.unwrap());
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
