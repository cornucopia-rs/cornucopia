// This file was generated with `cornucopia`. Do not modify.

use super::generic_client::GenericClient;
use async_trait::async_trait;
use deadpool_postgres::{
    Client as DeadpoolClient, ClientWrapper, Transaction as DeadpoolTransaction,
};
use tokio_postgres::{
    types::BorrowToSql, Client as PgClient, Error, RowStream, Statement, ToStatement,
    Transaction as PgTransaction,
};
#[async_trait]
impl GenericClient for DeadpoolClient {
    async fn prepare(&self, query: &str) -> Result<Statement, Error> {
        ClientWrapper::prepare_cached(self, query).await
    }
    async fn execute<T>(
        &self,
        query: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgClient::execute(self, query, params).await
    }
    async fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<tokio_postgres::Row, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgClient::query_one(self, statement, params).await
    }
    async fn query_opt<T>(
        &self,
        statement: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Option<tokio_postgres::Row>, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgClient::query_opt(self, statement, params).await
    }
    async fn query<T>(
        &self,
        query: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Vec<tokio_postgres::Row>, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgClient::query(self, query, params).await
    }
    async fn query_raw<T, P, I>(&self, statement: &T, params: I) -> Result<RowStream, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
        P: BorrowToSql,
        I: IntoIterator<Item = P> + Sync + Send,
        I::IntoIter: ExactSizeIterator,
    {
        PgClient::query_raw(self, statement, params).await
    }
}
#[async_trait]
impl GenericClient for DeadpoolTransaction<'_> {
    async fn prepare(&self, query: &str) -> Result<Statement, Error> {
        DeadpoolTransaction::prepare_cached(self, query).await
    }
    async fn execute<T>(
        &self,
        query: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgTransaction::execute(self, query, params).await
    }
    async fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<tokio_postgres::Row, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgTransaction::query_one(self, statement, params).await
    }
    async fn query_opt<T>(
        &self,
        statement: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Option<tokio_postgres::Row>, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgTransaction::query_opt(self, statement, params).await
    }
    async fn query<T>(
        &self,
        query: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Vec<tokio_postgres::Row>, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgTransaction::query(self, query, params).await
    }
    async fn query_raw<T, P, I>(&self, statement: &T, params: I) -> Result<RowStream, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
        P: BorrowToSql,
        I: IntoIterator<Item = P> + Sync + Send,
        I::IntoIter: ExactSizeIterator,
    {
        PgTransaction::query_raw(self, statement, params).await
    }
}
