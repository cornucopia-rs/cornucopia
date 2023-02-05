// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod types {}
#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod queries {
    pub mod module_1 {
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        pub struct StringQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            cached: Option<&'a tokio_postgres::Statement>,
            extractor: fn(&tokio_postgres::Row) -> &str,
            mapper: fn(&str) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> StringQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(&str) -> R) -> StringQuery<'a, C, R, N> {
                StringQuery {
                    client: self.client,
                    params: self.params,
                    query: self.query,
                    cached: self.cached,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let row = cornucopia_async::private::one(
                    self.client,
                    self.query,
                    &self.params,
                    self.cached,
                )
                .await?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.iter().await?.try_collect().await
            }
            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let opt_row = cornucopia_async::private::opt(
                    self.client,
                    self.query,
                    &self.params,
                    self.cached,
                )
                .await?;
                Ok(opt_row.map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub async fn iter(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stream = cornucopia_async::private::raw(
                    self.client,
                    self.query,
                    cornucopia_async::private::slice_iter(&self.params),
                    self.cached,
                )
                .await?;
                let mapped = stream
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(mapped)
            }
        }
        pub fn example_query() -> ExampleQueryStmt {
            ExampleQueryStmt(
                "SELECT
    *
FROM
    example_table",
                None,
            )
        }
        pub struct ExampleQueryStmt(&'static str, Option<tokio_postgres::Statement>);
        impl ExampleQueryStmt {
            pub async fn prepare<'a, C: GenericClient>(
                mut self,
                client: &'a C,
            ) -> Result<Self, tokio_postgres::Error> {
                if self.1.is_none() {
                    self.1 = Some(client.prepare(self.0).await?)
                }
                Ok(self)
            }
            pub fn bind<'a, C: GenericClient>(
                &'a self,
                client: &'a C,
            ) -> StringQuery<'a, C, String, 0> {
                StringQuery {
                    client,
                    params: [],
                    query: self.0,
                    cached: self.1.as_ref(),
                    extractor: |row| row.get(0),
                    mapper: |it| it.into(),
                }
            }
        }
    }
}
