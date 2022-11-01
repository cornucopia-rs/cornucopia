pub use cornucopia_client_core::{slice_iter, Domain, DomainArray};

use crate::generic_client::GenericClient;
use tokio_postgres::{Error, Statement};

/// Cached statement
pub struct Stmt {
    query: &'static str,
    cached: Option<Statement>,
}

impl Stmt {
    #[must_use]
    pub fn new(query: &'static str) -> Self {
        Self {
            query,
            cached: None,
        }
    }

    pub async fn prepare<'a, C: GenericClient>(
        &'a mut self,
        client: &C,
    ) -> Result<&'a Statement, Error> {
        if self.cached.is_none() {
            let stmt = client.prepare(self.query).await?;
            self.cached = Some(stmt);
        }
        // the statement is always prepared at this point
        Ok(unsafe { self.cached.as_ref().unwrap_unchecked() })
    }
}


#[macro_export]
macro_rules! query {
    ($name:ident, $brw:ty) => {
        pub struct $name<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> $brw,
            mapper: fn($brw) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> $name<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn($brw) -> R) -> $name<'a, C, R, N> {
                $name {
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

            pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.iter().await?.try_collect().await
            }

            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)
                    .await?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }

            pub async fn iter(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt.prepare(self.client).await?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
    };
}