// This file was generated with `cornucopia`. Do not modify.

use super::generic_client::GenericClient;
use deadpool_postgres::{
    Client as DeadpoolClient, ClientWrapper, Transaction as DeadpoolTransaction,
};
use tokio_postgres::{
    Client as PgClient, Error, Row, RowStream, Statement, ToStatement,
    Transaction as PgTransaction,
    types::{ToSql, Type},
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
    async fn execute_typed(
        &self,
        statement: &str,
        typed_params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<u64, Error> {
        PgClient::execute_typed(self, statement, typed_params).await
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
    async fn query_typed_one(
        &self,
        statement: &str,
        typed_params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<Row, Error> {
        PgClient::query_typed_one(self, statement, typed_params).await
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
    async fn query_typed_opt(
        &self,
        statement: &str,
        typed_params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<Option<Row>, Error> {
        PgClient::query_typed_opt(self, statement, typed_params).await
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
    async fn query_typed_raw(
        &self,
        statement: &str,
        typed_params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<RowStream, Error> {
        PgClient::query_typed_raw(self, statement, typed_params.iter().cloned()).await
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
    async fn execute_typed(
        &self,
        statement: &str,
        typed_params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<u64, Error> {
        PgTransaction::execute_typed(self, statement, typed_params).await
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
    async fn query_typed_one(
        &self,
        statement: &str,
        typed_params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<Row, Error> {
        PgTransaction::query_typed_one(self, statement, typed_params).await
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
    async fn query_typed_opt(
        &self,
        statement: &str,
        typed_params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<Option<Row>, Error> {
        PgTransaction::query_typed_opt(self, statement, typed_params).await
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
    async fn query_typed_raw(
        &self,
        statement: &str,
        typed_params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<RowStream, Error> {
        PgTransaction::query_typed_raw(self, statement, typed_params.iter().cloned()).await
    }
}
