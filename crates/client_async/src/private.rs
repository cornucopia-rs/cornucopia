pub use cornucopia_client_core::{slice_iter, Domain, DomainArray};
use tokio_postgres::{
    types::{BorrowToSql, ToSql},
    Error, Row, RowStream, Statement,
};

use crate::GenericClient;

pub async fn one<C: GenericClient>(
    client: &C,
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    cached: Option<&Statement>,
) -> Result<Row, Error> {
    if let Some(cached) = cached {
        client.query_one(cached, params).await
    } else if C::stmt_cache() {
        let cached = client.prepare(query).await?;
        client.query_one(&cached, params).await
    } else {
        client.query_one(query, params).await
    }
}

pub async fn opt<C: GenericClient>(
    client: &C,
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    cached: Option<&Statement>,
) -> Result<Option<Row>, Error> {
    if let Some(cached) = cached {
        client.query_opt(cached, params).await
    } else if C::stmt_cache() {
        let cached = client.prepare(query).await?;
        client.query_opt(&cached, params).await
    } else {
        client.query_opt(query, params).await
    }
}

pub async fn raw<C: GenericClient, P, I>(
    client: &C,
    query: &str,
    params: I,
    cached: Option<&Statement>,
) -> Result<RowStream, Error>
where
    P: BorrowToSql,
    I: IntoIterator<Item = P> + Sync + Send,
    I::IntoIter: ExactSizeIterator,
{
    if let Some(cached) = cached {
        client.query_raw(cached, params).await
    } else if C::stmt_cache() {
        let cached = client.prepare(query).await?;
        client.query_raw(&cached, params).await
    } else {
        client.query_raw(query, params).await
    }
}
