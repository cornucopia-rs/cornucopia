use std::net::{IpAddr, Ipv4Addr};

use error::Error;
use eui48::MacAddress;
use serde_json::Map;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

use crate::integration::cornucopia_gen::{
    queries::module_2::{books_opt_ret_param, BooksOptRetParam},
    types::public::SpongebobCharacter,
};

use super::cornucopia_gen::{self, queries::module_2::select_everything};
use deadpool_postgres::Client;

async fn setup() -> Result<Client, crate::error::Error> {
    use crate::run_migrations::run_migrations;
    use crate::{container, pool::cli_pool};

    container::setup()?;
    let pool = cli_pool()?;
    let client = pool.get().await?;
    run_migrations(&client, "src/integration/migrations").await?;

    Ok(client)
}

async fn teardown() -> Result<(), crate::error::Error> {
    use crate::container;
    container::cleanup()?;
    Ok(())
}

async fn integration() -> Result<(), Error> {
    let client = setup().await?;

    authors_test(&client).await?;
    books_test(&client).await?;
    books_from_author_id_test(&client).await?;
    author_name_by_id_test(&client).await?;
    author_name_by_id_opt_test(&client).await?;
    author_name_starting_with_test(&client).await?;
    return_custom_type_test(&client).await?;
    select_where_custom_type_test(&client).await?;
    select_everything_test(&client).await?;

    // ! This test must be last because it has side-effects
    insert_books_test(&client).await?;

    teardown().await?;

    Ok(())
}

#[tokio::test]

async fn integration_test() {
    if let Err(e) = integration().await {
        let _ = teardown().await;
        panic!("{:?}", e)
    }
}

/// This test monitors the behaviour of queries
/// that don't return anything. In this case,
/// the quantifier should be ignored.
async fn insert_books_test(client: &Client) -> Result<(), Error> {
    use cornucopia_gen::queries::module_1::{
        insert_book_one, insert_book_zero_or_more, insert_book_zero_or_one,
    };

    let _ = insert_book_one(client).await?;
    let _ = insert_book_zero_or_more(client).await?;
    let _ = insert_book_zero_or_one(client).await?;

    Ok(())
}

async fn authors_test(client: &Client) -> Result<(), Error> {
    use cornucopia_gen::queries::module_2::authors;

    let expected = vec![
        (
            1,
            String::from("Agatha Christie"),
            String::from("United Kingdom"),
        ),
        (
            2,
            String::from("John Ronald Reuel Tolkien"),
            String::from("United Kingdom"),
        ),
    ];
    let actual = authors(client).await?;

    if !actual.iter().all(|item| expected.contains(item)) {
        return Err(Error::Integration {
            expected: format!("{:?}", expected),
            actual: format!("{:?}", actual),
        });
    };

    Ok(())
}

async fn books_test(client: &Client) -> Result<(), Error> {
    use cornucopia_gen::queries::module_2::{books, Books};

    let expected = vec![
        Books {
            title: String::from("The Silmarillion"),
        },
        Books {
            title: String::from("The Hobbit"),
        },
        Books {
            title: String::from("Murder on the Orient Express"),
        },
        Books {
            title: String::from("Death on the Nile"),
        },
    ];

    let actual = books(client).await?;

    if !actual.iter().all(|item| expected.contains(item)) {
        return Err(Error::Integration {
            expected: format!("{:?}", expected),
            actual: format!("{:?}", actual),
        });
    };

    Ok(())
}

async fn books_from_author_id_test(client: &Client) -> Result<(), Error> {
    use cornucopia_gen::queries::module_2::books_from_author_id;

    let expected = vec![
        String::from("Death on the Nile"),
        String::from("Murder on the Orient Express"),
    ];
    let actual = books_from_author_id(client, &0).await?;

    if !actual.iter().all(|item| expected.contains(item)) {
        return Err(Error::Integration {
            expected: format!("{:?}", expected),
            actual: format!("{:?}", actual),
        });
    };

    Ok(())
}

async fn books_opt_ret_param_test(client: &Client) -> Result<(), Error> {
    let expected = vec![
        BooksOptRetParam {
            title: Some(String::from("The Silmarillion")),
        },
        BooksOptRetParam {
            title: Some(String::from("The Hobbit")),
        },
        BooksOptRetParam {
            title: Some(String::from("Murder on the Orient Express")),
        },
        BooksOptRetParam {
            title: Some(String::from("Death on the Nile")),
        },
    ];

    let actual = books_opt_ret_param(client).await?;

    if !actual.iter().all(|item| expected.contains(item)) {
        return Err(Error::Integration {
            expected: format!("{:?}", expected),
            actual: format!("{:?}", actual),
        });
    };

    Ok(())
}

async fn author_name_by_id_test(client: &Client) -> Result<(), Error> {
    use cornucopia_gen::queries::module_2::author_name_by_id;
    let expected = String::from("Agatha Christie");
    let actual = author_name_by_id(client, &1).await?;

    if expected != actual {
        return Err(Error::Integration {
            expected: format!("{:?}", expected),
            actual: format!("{:?}", actual),
        });
    };
    Ok(())
}

async fn author_name_by_id_opt_test(client: &Client) -> Result<(), Error> {
    use cornucopia_gen::queries::module_2::author_name_by_id_opt;
    let expected = None;
    let actual = author_name_by_id_opt(client, &-1).await?;

    if expected != actual {
        return Err(Error::Integration {
            expected: format!("{:?}", expected),
            actual: format!("{:?}", actual),
        });
    };
    Ok(())
}

async fn author_name_starting_with_test(client: &Client) -> Result<(), Error> {
    use cornucopia_gen::queries::module_2::author_name_starting_with;
    let expected = vec![
        (
            2,
            String::from("John Ronald Reuel Tolkien"),
            3,
            String::from("The Hobbit"),
        ),
        (
            2,
            String::from("John Ronald Reuel Tolkien"),
            4,
            String::from("The Silmarillion"),
        ),
    ];
    let actual = author_name_starting_with(client, "Jo").await?;

    if !actual.iter().all(|item| expected.contains(item)) {
        return Err(Error::Integration {
            expected: format!("{:?}", expected),
            actual: format!("{:?}", actual),
        });
    }

    Ok(())
}

async fn return_custom_type_test(client: &Client) -> Result<(), Error> {
    use cornucopia_gen::queries::module_2::return_custom_type;
    use cornucopia_gen::types::public::CustomComposite;

    let expected = CustomComposite {
        wow: String::from("Impressive"),
        such_cool: 42,
        nice: SpongebobCharacter::Patrick,
    };
    let actual = return_custom_type(client).await?;

    if expected.wow == actual.wow
        && expected.such_cool == actual.such_cool
        && expected.nice == actual.nice
    {
        return Err(Error::Integration {
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        });
    }

    Ok(())
}

async fn select_where_custom_type_test(client: &Client) -> Result<(), Error> {
    use cornucopia_gen::queries::module_2::select_where_custom_type;

    let actual = select_where_custom_type(client, &SpongebobCharacter::Patrick).await?;
    let expected = SpongebobCharacter::Bob;
    if expected != actual {
        return Err(Error::Integration {
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        });
    }

    Ok(())
}

async fn select_everything_test(client: &Client) -> Result<(), Error> {
    //'1999-01-08', '04:05:06.789', '{}', '{}', 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', '127.0.0.1', '08:00:2b:01:02:03'

    let primitive_datetime_format =
        time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
    let primitive_datetime =
        PrimitiveDateTime::parse("2020-01-02 03:04:05", &primitive_datetime_format).unwrap();
    let offset_datetime = OffsetDateTime::parse(
        "1985-04-12T23:20:50.52Z",
        &time::format_description::well_known::Rfc3339,
    )
    .unwrap();

    let expected = (
        true,
        true,
        42i8,
        300i16,
        300i16,
        300i16,
        300i16,
        100000i32,
        100000i32,
        100000i32,
        100000i32,
        10000000000i64,
        10000000000i64,
        10000000000i64,
        10000000000i64,
        1.12f32,
        1.12f32,
        1.1231231231f64,
        1.1231231231f64,
        String::from("hello"),
        String::from("hello"),
        vec![222u8, 173u8, 190u8, 239u8],
        primitive_datetime.clone(),
        primitive_datetime,
        offset_datetime.clone(),
        offset_datetime,
        time::Date::from_calendar_date(1999, time::Month::January, 8).unwrap(),
        time::Time::from_hms_milli(4, 5, 6, 789).unwrap(),
        serde_json::Value::Object(Map::new()),
        serde_json::Value::Object(Map::new()),
        Uuid::parse_str("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11").unwrap(),
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        MacAddress::new([8, 0, 43, 1, 2, 3]),
    );
    let actual = select_everything(client).await?;

    assert_eq(expected.0, actual.0)?;
    assert_eq(expected.1, actual.1)?;
    assert_eq(expected.2, actual.2)?;
    assert_eq(expected.3, actual.3)?;
    assert_eq(expected.4, actual.4)?;
    assert_eq(expected.5, actual.5)?;
    assert_eq(expected.6, actual.6)?;
    assert_eq(expected.7, actual.7)?;
    assert_eq(expected.8, actual.8)?;
    assert_eq(expected.9, actual.9)?;
    assert_eq(expected.10, actual.10)?;
    assert_eq(expected.11, actual.11)?;
    assert_eq(expected.12, actual.12)?;
    assert_eq(expected.13, actual.13)?;
    assert_eq(expected.14, actual.14)?;
    assert_eq(expected.15, actual.15)?;
    assert_eq(expected.16, actual.16)?;
    assert_eq(expected.17, actual.17)?;
    assert_eq(expected.18, actual.18)?;
    assert_eq(expected.19, actual.19)?;
    assert_eq(expected.20, actual.20)?;
    assert_eq(expected.21, actual.21)?;
    assert_eq(expected.22, actual.22)?;
    assert_eq(expected.23, actual.23)?;
    assert_eq(expected.24, actual.24)?;
    assert_eq(expected.25, actual.25)?;
    assert_eq(expected.26, actual.26)?;
    assert_eq(expected.27, actual.27)?;
    assert_eq(expected.28, actual.28)?;
    assert_eq(expected.29, actual.29)?;
    assert_eq(expected.30, actual.30)?;
    assert_eq(expected.31, actual.31)?;

    Ok(())
}

fn assert_eq<T: std::fmt::Debug + PartialEq>(expected: T, actual: T) -> Result<(), Error> {
    if actual != expected {
        Err(Error::Integration {
            expected: format!("{:?}", expected),
            actual: format!("{:?}", actual),
        })
    } else {
        Ok(())
    }
}

mod error {
    use crate::error::Error as CornucopiaError;
    use thiserror::Error as ThisError;
    use tokio_postgres::Error as DbError;
    #[derive(Debug, ThisError)]
    #[error("error occured during integration testing")]
    pub enum Error {
        #[error("expected {expected}, got {actual}")]
        Integration {
            expected: String,
            actual: String,
        },
        Db(#[from] DbError),
        Cornucopia(#[from] CornucopiaError),
    }
}
