// This file was generated with `cornucopia`. Do not modify.
#![allow(clippy::all)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
pub mod types {}
pub mod queries {
    pub mod module_1 {
        use cornucopia_client::async_::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug, Clone, PartialEq)]
        pub struct ExampleQuery {
            pub col1: String,
        }
        pub struct ExampleQueryBorrowed<'a> {
            pub col1: &'a str,
        }
        impl<'a> From<ExampleQueryBorrowed<'a>> for ExampleQuery {
            fn from(ExampleQueryBorrowed { col1 }: ExampleQueryBorrowed<'a>) -> Self {
                Self { col1: col1.into() }
            }
        }
        pub struct ExampleQueryQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::async_::Stmt,
            extractor: fn(&tokio_postgres::Row) -> &str,
            mapper: fn(&str) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> ExampleQueryQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(&str) -> R) -> ExampleQueryQuery<'a, C, R, N> {
                ExampleQueryQuery {
                    client: self.client,
                    params: self.params,
                    stmt: self.stmt,
                    extractor: self.extractor,
                    mapper,
                }
            }

            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                let row = self.client.query_one(stmt, &self.params).await?;
                Ok((self.mapper)((self.extractor)(&row)))
            }

            pub async fn vec(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.stream().await?.try_collect().await
            }

            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)
                    .await?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }

            pub async fn stream(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt.prepare(self.client).await?;
                let stream = self
                    .client
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(stream)
            }
        }
        pub fn example_query() -> ExampleQueryStmt {
            ExampleQueryStmt(cornucopia_client::async_::Stmt::new(
                "SELECT
    *
FROM
    example_table",
            ))
        }
        pub struct ExampleQueryStmt(cornucopia_client::async_::Stmt);
        impl ExampleQueryStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> ExampleQueryQuery<'a, C, String, 0> {
                ExampleQueryQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it.into(),
                }
            }
        }
    }
}
