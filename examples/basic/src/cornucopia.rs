// This file was generated with `cornucopia`. Do not modify.

pub mod types {
    pub mod public {
        #[derive(Debug, postgres_types::ToSql, postgres_types::FromSql)]
        #[postgres(name = "custom_composite")]
        #[derive(Clone)]
        pub struct CustomComposite {
            pub age: i32,
            pub name: String,
            pub persona: super::public::SpongebobCharacter,
        }

        #[derive(Debug, postgres_types::ToSql, postgres_types::FromSql)]
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
        pub async fn insert_book<T: cornucopia_client::GenericClient>(
            client: &T,
            title: &str,
        ) -> Result<(), tokio_postgres::Error> {
            let stmt = client
                .prepare(
                    "INSERT INTO Book (title)
VALUES ($1);
",
                )
                .await?;
            let _ = client.execute(&stmt, &[&title]).await?;
            Ok(())
        }
    }

    pub mod module_2 {
        pub async fn authors<T: cornucopia_client::GenericClient>(
            client: &T,
        ) -> Result<Vec<(i32, String, String)>, tokio_postgres::Error> {
            use futures::{StreamExt, TryStreamExt};
            let stmt = client
                .prepare(
                    "SELECT
*
FROM
Author;
",
                )
                .await?;
            let res = client
                .query_raw(&stmt, std::iter::empty::<i32>())
                .await?
                .map(|res| {
                    res.map(|res| {
                        let return_value_0: i32 = res.get(0);
                        let return_value_1: String = res.get(1);
                        let return_value_2: String = res.get(2);
                        (return_value_0, return_value_1, return_value_2)
                    })
                })
                .try_collect()
                .await?;
            Ok(res)
        }

        pub async fn authors_stream<T: cornucopia_client::GenericClient>(
            client: &T,
        ) -> Result<
            impl futures::Stream<Item = Result<(i32, String, String), tokio_postgres::Error>>,
            tokio_postgres::Error,
        > {
            use futures::{StreamExt, TryStreamExt};
            let stmt = client
                .prepare(
                    "SELECT
*
FROM
Author;
",
                )
                .await?;
            let row_stream = client
                .query_raw(&stmt, std::iter::empty::<i32>())
                .await?
                .map(|res| {
                    res.map(|res| {
                        let return_value_0: i32 = res.get(0);
                        let return_value_1: String = res.get(1);
                        let return_value_2: String = res.get(2);
                        (return_value_0, return_value_1, return_value_2)
                    })
                });
            Ok(row_stream.into_stream())
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Books {
            pub title: String,
        }
        pub async fn books<T: cornucopia_client::GenericClient>(
            client: &T,
        ) -> Result<Vec<super::super::queries::module_2::Books>, tokio_postgres::Error> {
            use futures::{StreamExt, TryStreamExt};
            let stmt = client
                .prepare(
                    "SELECT
Title
FROM
Book;
",
                )
                .await?;
            let res = client
                .query_raw(&stmt, std::iter::empty::<i32>())
                .await?
                .map(|res| {
                    res.map(|res| {
                        let return_value_0: String = res.get(0);
                        super::super::queries::module_2::Books {
                            title: return_value_0,
                        }
                    })
                })
                .try_collect()
                .await?;
            Ok(res)
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct BooksOptRetParam {
            pub title: Option<String>,
        }
        pub async fn books_opt_ret_param<T: cornucopia_client::GenericClient>(
            client: &T,
        ) -> Result<Vec<super::super::queries::module_2::BooksOptRetParam>, tokio_postgres::Error>
        {
            use futures::{StreamExt, TryStreamExt};
            let stmt = client
                .prepare(
                    "SELECT
Title
FROM
Book;
",
                )
                .await?;
            let res = client
                .query_raw(&stmt, std::iter::empty::<i32>())
                .await?
                .map(|res| {
                    res.map(|res| {
                        let return_value_0: Option<String> = res.get(0);
                        super::super::queries::module_2::BooksOptRetParam {
                            title: return_value_0,
                        }
                    })
                })
                .try_collect()
                .await?;
            Ok(res)
        }

        pub async fn books_from_author_id<T: cornucopia_client::GenericClient>(
            client: &T,
            id: &i32,
        ) -> Result<Vec<String>, tokio_postgres::Error> {
            use futures::{StreamExt, TryStreamExt};
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
            let res = client
                .query_raw(&stmt, &[&id])
                .await?
                .map(|res| {
                    res.map(|row| {
                        let value: String = row.get(0);
                        value
                    })
                })
                .try_collect()
                .await?;
            Ok(res)
        }

        pub async fn author_name_by_id_opt<T: cornucopia_client::GenericClient>(
            client: &T,
            id: &i32,
        ) -> Result<Option<String>, tokio_postgres::Error> {
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

        pub async fn author_name_by_id<T: cornucopia_client::GenericClient>(
            client: &T,
            id: &i32,
        ) -> Result<String, tokio_postgres::Error> {
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

        pub async fn author_name_starting_with<T: cornucopia_client::GenericClient>(
            client: &T,
            s: &str,
        ) -> Result<Vec<(i32, String, i32, String)>, tokio_postgres::Error> {
            use futures::{StreamExt, TryStreamExt};
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
            let res = client
                .query_raw(&stmt, &[&s])
                .await?
                .map(|res| {
                    res.map(|res| {
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
                })
                .try_collect()
                .await?;
            Ok(res)
        }

        pub async fn return_custom_type<T: cornucopia_client::GenericClient>(
            client: &T,
        ) -> Result<super::super::types::public::CustomComposite, tokio_postgres::Error> {
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

        pub async fn select_where_custom_type<T: cornucopia_client::GenericClient>(
            client: &T,
            spongebob_character: &super::super::types::public::SpongebobCharacter,
        ) -> Result<super::super::types::public::SpongebobCharacter, tokio_postgres::Error>
        {
            let stmt = client
                .prepare(
                    "SELECT
col2
FROM
CustomTable
WHERE (col1).persona = $1;
",
                )
                .await?;
            let res = client.query_one(&stmt, &[&spongebob_character]).await?;
            let return_value: super::super::types::public::SpongebobCharacter = res.get(0);
            Ok(return_value)
        }

        pub async fn select_translations<T: cornucopia_client::GenericClient>(
            client: &T,
        ) -> Result<Vec<String>, tokio_postgres::Error> {
            let stmt = client
                .prepare(
                    "SELECT
Translations
FROM
Book;
",
                )
                .await?;
            let res = client.query_one(&stmt, &[]).await?;
            let return_value: Vec<String> = res.get(0);
            Ok(return_value)
        }
    }
}
