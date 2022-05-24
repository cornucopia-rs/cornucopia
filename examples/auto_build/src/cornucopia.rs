// This file was generated with `cornucopia`. Do not modify.

pub mod types {}

pub mod queries {
    pub mod module_1 {
        use futures::{StreamExt, TryStreamExt};

        pub struct ExampleQueryBorrowed<'a> {
            pub col1: &'a str,
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ExampleQuery {
            pub col1: String,
        }
        impl<'a> From<ExampleQueryBorrowed<'a>> for ExampleQuery {
            fn from(ExampleQueryBorrowed { col1 }: ExampleQueryBorrowed<'a>) -> Self {
                Self { col1: col1.into() }
            }
        }
        pub struct ExampleQueryQuery<'a, C: cornucopia_client::GenericClient, T> {
            client: &'a C,
            params: [&'a (dyn tokio_postgres::types::ToSql + Sync); 0],
            mapper: fn(ExampleQueryBorrowed) -> T,
        }

        impl<'a, C, T> ExampleQueryQuery<'a, C, T>
        where
            C: cornucopia_client::GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(ExampleQueryBorrowed) -> R,
            ) -> ExampleQueryQuery<'a, C, R> {
                ExampleQueryQuery {
                    client: self.client,
                    params: self.params,
                    mapper,
                }
            }

            pub fn extractor(row: &tokio_postgres::row::Row) -> ExampleQueryBorrowed {
                ExampleQueryBorrowed { col1: row.get(0) }
            }

            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client
                    .prepare(
                        "SELECT
    *
FROM
    example_table;

",
                    )
                    .await
            }

            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok((self.mapper)(Self::extractor(&row)))
            }

            pub async fn vec(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.stream().await?.try_collect().await
            }

            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)
                    .await?
                    .map(|row| (self.mapper)(Self::extractor(&row))))
            }

            pub async fn stream(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>>,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt().await?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)(Self::extractor(&row))));
                Ok(stream.into_stream())
            }
        }
        pub fn example_query<'a, C: cornucopia_client::GenericClient>(
            client: &'a C,
        ) -> ExampleQueryQuery<'a, C, ExampleQuery> {
            ExampleQueryQuery {
                client,
                params: [],
                mapper: |it| ExampleQuery::from(it),
            }
        }
    }
}
