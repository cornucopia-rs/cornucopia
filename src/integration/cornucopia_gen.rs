// This file was generated with `cornucopia`. Do not modify.

pub mod types {
    pub mod public {
        use postgres_types::{FromSql, ToSql};
        #[derive(Debug, ToSql, FromSql)]
        #[postgres(name = "custom_composite")]
        #[derive(Clone)]
        pub struct CustomComposite {
            pub such_cool: i32,
            pub wow: String,
            pub nice: super::public::SpongebobCharacter,
        }

        #[derive(Debug, ToSql, FromSql)]
        #[postgres(name = "spongebob_character")]
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub enum SpongebobCharacter {
            Bob,
            Patrick,
            Squidward,
        }
    }
}

pub mod queries {
    pub mod module_1 {
        use cornucopia_client::GenericClient;
        use tokio_postgres::Error;

        pub async fn insert_book_one<T: GenericClient>(client: &T) -> Result<(), Error> {
            let stmt = client
                .prepare(
                    "INSERT INTO Book (title)
VALUES ('bob');
",
                )
                .await?;
            let _ = client.execute(&stmt, &[]).await?;

            Ok(())
        }

        pub async fn insert_book_zero_or_one<T: GenericClient>(client: &T) -> Result<(), Error> {
            let stmt = client
                .prepare(
                    "INSERT INTO Book (title)
VALUES ('alice');
",
                )
                .await?;
            let _ = client.execute(&stmt, &[]).await?;

            Ok(())
        }

        pub async fn insert_book_zero_or_more<T: GenericClient>(client: &T) -> Result<(), Error> {
            let stmt = client
                .prepare(
                    "INSERT INTO Book (title)
VALUES ('carl');
",
                )
                .await?;
            let _ = client.execute(&stmt, &[]).await?;

            Ok(())
        }
    }

    pub mod module_2 {
        use cornucopia_client::GenericClient;
        use tokio_postgres::Error;

        pub async fn authors<T: GenericClient>(
            client: &T,
        ) -> Result<Vec<(i32, String, String)>, Error> {
            let stmt = client
                .prepare(
                    "SELECT
*
FROM
Author;
",
                )
                .await?;
            let res = client.query(&stmt, &[]).await?;

            let return_value = res
                .iter()
                .map(|res| {
                    let return_value_0: i32 = res.get(0);
                    let return_value_1: String = res.get(1);
                    let return_value_2: String = res.get(2);
                    (return_value_0, return_value_1, return_value_2)
                })
                .collect::<Vec<(i32, String, String)>>();
            Ok(return_value)
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Books {
            pub title: String,
        }
        pub async fn books<T: GenericClient>(
            client: &T,
        ) -> Result<Vec<super::super::queries::module_2::Books>, Error> {
            let stmt = client
                .prepare(
                    "SELECT
Title
FROM
Book;
",
                )
                .await?;
            let res = client.query(&stmt, &[]).await?;

            let return_value = res
                .iter()
                .map(|res| {
                    let return_value_0: String = res.get(0);
                    super::super::queries::module_2::Books {
                        title: return_value_0,
                    }
                })
                .collect::<Vec<super::super::queries::module_2::Books>>();
            Ok(return_value)
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct BooksOptRetParam {
            pub title: Option<String>,
        }
        pub async fn books_opt_ret_param<T: GenericClient>(
            client: &T,
        ) -> Result<Vec<super::super::queries::module_2::BooksOptRetParam>, Error> {
            let stmt = client
                .prepare(
                    "SELECT
Title
FROM
Book;
",
                )
                .await?;
            let res = client.query(&stmt, &[]).await?;

            let return_value = res
                .iter()
                .map(|res| {
                    let return_value_0: Option<String> = res.get(0);
                    super::super::queries::module_2::BooksOptRetParam {
                        title: return_value_0,
                    }
                })
                .collect::<Vec<super::super::queries::module_2::BooksOptRetParam>>();
            Ok(return_value)
        }

        pub async fn books_from_author_id<T: GenericClient>(
            client: &T,
            id: &i32,
        ) -> Result<Vec<String>, Error> {
            let stmt = client
                .prepare(
                    "SELECT
Book.Title
FROM
BookAuthor
INNER JOIN Author ON Author.Id = BookAuthor.AuthorId
INNER JOIN Book ON Book.Id = BookAuthor.BookId
WHERE
Author.Id = $1;
",
                )
                .await?;
            let res = client.query(&stmt, &[&id]).await?;

            let return_value = res
                .iter()
                .map(|row| {
                    let value: String = row.get(0);
                    value
                })
                .collect::<Vec<String>>();
            Ok(return_value)
        }

        pub async fn author_name_by_id_opt<T: GenericClient>(
            client: &T,
            id: &i32,
        ) -> Result<Option<String>, Error> {
            let stmt = client
                .prepare(
                    "SELECT
Author.Name
FROM
Author
WHERE
Author.Id = $1;
",
                )
                .await?;
            let res = client.query_opt(&stmt, &[&id]).await?;

            let return_value = res.map(|row| {
                let value: String = row.get(0);
                value
            });
            Ok(return_value)
        }

        pub async fn author_name_by_id<T: GenericClient>(
            client: &T,
            id: &i32,
        ) -> Result<String, Error> {
            let stmt = client
                .prepare(
                    "SELECT
Author.Name
FROM
Author
WHERE
Author.Id = $1;
",
                )
                .await?;
            let res = client.query_one(&stmt, &[&id]).await?;

            let return_value: String = res.get(0);
            Ok(return_value)
        }

        pub async fn author_name_starting_with<T: GenericClient>(
            client: &T,
            s: &str,
        ) -> Result<Vec<(i32, String, i32, String)>, Error> {
            let stmt = client
                .prepare(
                    "SELECT
BookAuthor.AuthorId,
Author.Name,
BookAuthor.BookId,
Book.Title
FROM
BookAuthor
INNER JOIN Author ON Author.id = BookAuthor.AuthorId
INNER JOIN Book ON Book.Id = BookAuthor.BookId
WHERE
Author.Name LIKE CONCAT($1::text, '%');
",
                )
                .await?;
            let res = client.query(&stmt, &[&s]).await?;

            let return_value = res
                .iter()
                .map(|res| {
                    let return_value_0: i32 = res.get(0);
                    let return_value_1: String = res.get(1);
                    let return_value_2: i32 = res.get(2);
                    let return_value_3: String = res.get(3);
                    (
                        return_value_0,
                        return_value_1,
                        return_value_2,
                        return_value_3,
                    )
                })
                .collect::<Vec<(i32, String, i32, String)>>();
            Ok(return_value)
        }

        pub async fn return_custom_type<T: GenericClient>(
            client: &T,
        ) -> Result<super::super::types::public::CustomComposite, Error> {
            let stmt = client
                .prepare(
                    "SELECT
col1
FROM
CustomTable;
",
                )
                .await?;
            let res = client.query_one(&stmt, &[]).await?;

            let return_value: super::super::types::public::CustomComposite = res.get(0);
            Ok(return_value)
        }

        pub async fn select_where_custom_type<T: GenericClient>(
            client: &T,
            spongebob_character: &super::super::types::public::SpongebobCharacter,
        ) -> Result<super::super::types::public::SpongebobCharacter, Error> {
            let stmt = client
                .prepare(
                    "SELECT
col2
FROM
CustomTable
WHERE (col1).nice = $1;
",
                )
                .await?;
            let res = client.query_one(&stmt, &[&spongebob_character]).await?;

            let return_value: super::super::types::public::SpongebobCharacter = res.get(0);
            Ok(return_value)
        }

        pub async fn select_everything<T: GenericClient>(
            client: &T,
        ) -> Result<
            (
                bool,
                bool,
                i8,
                i16,
                i16,
                i16,
                i16,
                i32,
                i32,
                i32,
                i32,
                i64,
                i64,
                i64,
                i64,
                f32,
                f32,
                f64,
                f64,
                String,
                String,
                Vec<u8>,
                time::PrimitiveDateTime,
                time::PrimitiveDateTime,
                time::OffsetDateTime,
                time::OffsetDateTime,
                time::Date,
                time::Time,
                serde_json::Value,
                serde_json::Value,
                uuid::Uuid,
                std::net::IpAddr,
                eui48::MacAddress,
            ),
            Error,
        > {
            let stmt = client
                .prepare(
                    "SELECT
*
FROM
Everything;
",
                )
                .await?;
            let res = client.query_one(&stmt, &[]).await?;

            let return_value = {
                let return_value_0: bool = res.get(0);
                let return_value_1: bool = res.get(1);
                let return_value_2: i8 = res.get(2);
                let return_value_3: i16 = res.get(3);
                let return_value_4: i16 = res.get(4);
                let return_value_5: i16 = res.get(5);
                let return_value_6: i16 = res.get(6);
                let return_value_7: i32 = res.get(7);
                let return_value_8: i32 = res.get(8);
                let return_value_9: i32 = res.get(9);
                let return_value_10: i32 = res.get(10);
                let return_value_11: i64 = res.get(11);
                let return_value_12: i64 = res.get(12);
                let return_value_13: i64 = res.get(13);
                let return_value_14: i64 = res.get(14);
                let return_value_15: f32 = res.get(15);
                let return_value_16: f32 = res.get(16);
                let return_value_17: f64 = res.get(17);
                let return_value_18: f64 = res.get(18);
                let return_value_19: String = res.get(19);
                let return_value_20: String = res.get(20);
                let return_value_21: Vec<u8> = res.get(21);
                let return_value_22: time::PrimitiveDateTime = res.get(22);
                let return_value_23: time::PrimitiveDateTime = res.get(23);
                let return_value_24: time::OffsetDateTime = res.get(24);
                let return_value_25: time::OffsetDateTime = res.get(25);
                let return_value_26: time::Date = res.get(26);
                let return_value_27: time::Time = res.get(27);
                let return_value_28: serde_json::Value = res.get(28);
                let return_value_29: serde_json::Value = res.get(29);
                let return_value_30: uuid::Uuid = res.get(30);
                let return_value_31: std::net::IpAddr = res.get(31);
                let return_value_32: eui48::MacAddress = res.get(32);
                (
                    return_value_0,
                    return_value_1,
                    return_value_2,
                    return_value_3,
                    return_value_4,
                    return_value_5,
                    return_value_6,
                    return_value_7,
                    return_value_8,
                    return_value_9,
                    return_value_10,
                    return_value_11,
                    return_value_12,
                    return_value_13,
                    return_value_14,
                    return_value_15,
                    return_value_16,
                    return_value_17,
                    return_value_18,
                    return_value_19,
                    return_value_20,
                    return_value_21,
                    return_value_22,
                    return_value_23,
                    return_value_24,
                    return_value_25,
                    return_value_26,
                    return_value_27,
                    return_value_28,
                    return_value_29,
                    return_value_30,
                    return_value_31,
                    return_value_32,
                )
            };
            Ok(return_value)
        }
    }
}
