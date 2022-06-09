// Take a look at the generated `cornucopia.rs` file if you want to
// see what it looks like under the hood.
#[rustfmt::skip]
mod cornucopia;

use postgres::{Config, NoTls};

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

pub fn main() {
    // Connection pool configuration
    // Please look at `tokio_postgres` and `deadpool_postgres` for details.

    let mut client = Config::new()
        .user("postgres")
        .password("postgres")
        .host("127.0.0.1")
        .port(5432)
        .dbname("postgres")
        .connect(NoTls)
        .unwrap();

    // Queries accept regular clients.
    println!("{:?}", authors(&mut client).vec().unwrap());

    // Queries also accept transactions
    // Don't forget to `.commit()` when you're done!
    {
        let mut transaction = client.transaction().unwrap();
        // Insert a book
        insert_book(&mut transaction, &"The Great Gatsby").unwrap();
        // Use a map if needed
        let books = books(&mut transaction).vec().unwrap();
        println!("{books:?}");
        transaction.commit().unwrap();
    }

    // Using opt returns an optional row (zero or one).
    println!("{:?}", author_name_by_id(&mut client, &0).opt().unwrap());

    // The param struct can be more convenient
    // and less error-prone in some cases
    println!(
        "{:?}",
        AuthorNameStartingWithParams { start_str: "Jo" }
            .author_name_starting_with(&mut client)
            .vec()
            .unwrap()
    );

    // Custom types from your queries also work!
    println!(
        "{:?}",
        select_where_custom_type(&mut client, &SpongebobCharacter::Patrick)
            .one()
            .unwrap()
    );
}
