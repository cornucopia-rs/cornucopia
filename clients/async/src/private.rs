use cornucopia_client_core::Borrow;
pub use cornucopia_client_core::{slice_iter, Domain, DomainArray};

use crate::generic_client::GenericClient;
use futures::{Stream, StreamExt, TryStreamExt};
use tokio_postgres::{types::ToSql, Error, Row, Statement};

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

pub struct Query<'a, C: GenericClient, T, B: Borrow, const N: usize> {
    pub client: &'a C,
    pub params: [&'a (dyn ToSql + Sync); N],
    pub stmt: &'a mut Stmt,
    pub extractor: for<'r> fn(&'r Row) -> B::Borrow<'r>,
    pub mapper: for<'r> fn(B::Borrow<'r>) -> T,
}

impl<'a, C: GenericClient, T: 'a, B: Borrow, const N: usize> Query<'a, C, T, B, N> {
    pub fn map<R>(self, mapper: for<'b> fn(B::Borrow<'b>) -> R) -> Query<'a, C, R, B, N> {
        Query {
            client: self.client,
            params: self.params,
            stmt: self.stmt,
            extractor: self.extractor,
            mapper,
        }
    }

    pub async fn one(&mut self) -> Result<T, Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        let row = self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    }

    pub async fn opt(&mut self) -> Result<Option<T>, Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self
            .client
            .query_opt(stmt, &self.params)
            .await?
            .map(|row| (self.mapper)((self.extractor)(&row))))
    }

    pub async fn all(&'a mut self) -> Result<Vec<T>, Error> {
        self.iter().await?.try_collect().await
    }

    pub async fn iter(&'a mut self) -> Result<impl Stream<Item = Result<T, Error>> + 'a, Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        let stream = self
            .client
            .query_raw(stmt, slice_iter(&self.params))
            .await?
            .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
            .into_stream();
        Ok(stream)
    }
}
