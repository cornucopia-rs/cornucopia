use async_trait::async_trait;
use deadpool_postgres::{Client, ClientWrapper, Transaction};
use tokio_postgres::{Client as PgClient, Error, Statement, Transaction as PgTransaction};

#[async_trait]
pub trait GenericClient {
    async fn prepare(&self, query: &str) -> Result<Statement, Error>;
    async fn execute<T>(
        &self,
        query: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send;
    async fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<tokio_postgres::Row, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send;
    async fn query_opt<T>(
        &self,
        statement: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Option<tokio_postgres::Row>, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send;
    async fn query<T>(
        &self,
        query: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Vec<tokio_postgres::Row>, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send;
}

#[async_trait]
impl GenericClient for Transaction<'_> {
    async fn prepare(&self, query: &str) -> Result<Statement, Error> {
        Transaction::prepare_cached(self, query).await
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
}

#[async_trait]
impl GenericClient for PgTransaction<'_> {
    async fn prepare(&self, query: &str) -> Result<Statement, Error> {
        PgTransaction::prepare(self, query).await
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
}

#[async_trait]
impl GenericClient for Client {
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
}

#[async_trait]
impl GenericClient for PgClient {
    async fn prepare(&self, query: &str) -> Result<Statement, Error> {
        PgClient::prepare(self, query).await
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
}
