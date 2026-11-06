// This file was generated with `cornucopia`. Do not modify.

use super::generic_client::GenericClient;
use deadpool_postgres::{
    Client as DeadpoolClient, ClientWrapper, Transaction as DeadpoolTransaction,
};
use tokio_postgres::{
    Client as PgClient, Error, Row, RowStream, Statement, ToStatement,
    Transaction as PgTransaction, types::ToSql,
};
impl GenericClient for DeadpoolClient {
    fn stmt_cache() -> bool {
        true
    }
    async fn prepare(&self, query: &str) -> Result<Statement, Error> {
        ClientWrapper::prepare_cached(self, query).await
    }
    async fn execute<T>(&self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<u64, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        PgClient::execute(self, query, params).await
    }
    async fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Row, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        PgClient::query_one(self, statement, params).await
    }
    async fn query_opt<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        PgClient::query_opt(self, statement, params).await
    }
    async fn query<T>(&self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        PgClient::query(self, query, params).await
    }
    async fn query_raw<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<RowStream, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        PgClient::query_raw(self, statement, params.iter().copied()).await
    }
}
impl GenericClient for DeadpoolTransaction<'_> {
    fn stmt_cache() -> bool {
        false
    }
    async fn prepare(&self, query: &str) -> Result<Statement, Error> {
        DeadpoolTransaction::prepare_cached(self, query).await
    }
    async fn execute<T>(&self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<u64, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        PgTransaction::execute(self, query, params).await
    }
    async fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Row, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        PgTransaction::query_one(self, statement, params).await
    }
    async fn query_opt<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        PgTransaction::query_opt(self, statement, params).await
    }
    async fn query<T>(&self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        PgTransaction::query(self, query, params).await
    }
    async fn query_raw<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<RowStream, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        PgTransaction::query_raw(self, statement, params.iter().copied()).await
    }
}
