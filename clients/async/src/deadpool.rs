use async_trait::async_trait;
use deadpool_postgres::{
    Client as DeadpoolClient, ClientWrapper, Transaction as DeadpoolTransaction,
};
use tokio_postgres::{
    types::BorrowToSql, Client as PgClient, Error, RowStream, Transaction as PgTransaction,
};

use crate::generic_client::GenericClient;

#[async_trait]
impl GenericClient for DeadpoolClient {
    async fn execute(
        &self,
        query: &str,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Error> {
        let stmt = ClientWrapper::prepare_cached(self, query).await?;
        PgClient::execute(self, &stmt, params).await
    }

    async fn query_one(
        &self,
        query: &str,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<tokio_postgres::Row, Error> {
        let stmt = ClientWrapper::prepare_cached(self, query).await?;
        PgClient::query_one(self, &stmt, params).await
    }

    async fn query_opt(
        &self,
        query: &str,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Option<tokio_postgres::Row>, Error> {
        let stmt = ClientWrapper::prepare_cached(self, query).await?;
        PgClient::query_opt(self, &stmt, params).await
    }

    async fn query(
        &self,
        query: &str,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Vec<tokio_postgres::Row>, Error> {
        let stmt = ClientWrapper::prepare_cached(self, query).await?;
        PgClient::query(self, &stmt, params).await
    }

    async fn query_raw<P, I>(&self, query: &str, params: I) -> Result<RowStream, Error>
    where
        P: BorrowToSql,
        I: IntoIterator<Item = P> + Sync + Send,
        I::IntoIter: ExactSizeIterator,
    {
        let stmt = ClientWrapper::prepare_cached(self, query).await?;
        PgClient::query_raw(self, &stmt, params).await
    }
}

#[async_trait]
impl GenericClient for DeadpoolTransaction<'_> {
    async fn execute(
        &self,
        query: &str,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Error> {
        PgTransaction::execute(self, query, params).await
    }

    async fn query_one(
        &self,
        query: &str,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<tokio_postgres::Row, Error> {
        PgTransaction::query_one(self, query, params).await
    }

    async fn query_opt(
        &self,
        query: &str,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Option<tokio_postgres::Row>, Error> {
        PgTransaction::query_opt(self, query, params).await
    }

    async fn query(
        &self,
        query: &str,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Vec<tokio_postgres::Row>, Error> {
        PgTransaction::query(self, query, params).await
    }

    async fn query_raw<P, I>(&self, query: &str, params: I) -> Result<RowStream, Error>
    where
        P: BorrowToSql,
        I: IntoIterator<Item = P> + Sync + Send,
        I::IntoIter: ExactSizeIterator,
    {
        PgTransaction::query_raw(self, query, params).await
    }
}
