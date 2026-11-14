// This file was generated with `cornucopia`. Do not modify.

pub use generic_client::GenericClient;
#[cfg(feature = "deadpool")]
mod deadpool;
mod generic_client;
use tokio_postgres::{
    Error, Row, RowStream, Statement,
    types::{ToSql, Type},
};
/// This trait allows you to bind parameters to a query using a single
/// struct, rather than passing each bind parameter as a function parameter.
pub trait Params<'c, 'a, 's, P, O, C> {
    fn params(&'s self, client: &'c C, params: &'a P) -> O;
}
pub async fn one<C: GenericClient>(
    client: &C,
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    typed_params: Option<&[(&(dyn ToSql + Sync), Type)]>,
    cached: Option<&Statement>,
) -> Result<Row, Error> {
    if let Some(cached) = cached {
        client.query_one(cached, params).await
    } else if C::stmt_cache() {
        let cached = client.prepare(query).await?;
        client.query_one(&cached, params).await
    } else if let Some(typed_params) = typed_params {
        client.query_typed_one(query, typed_params).await
    } else {
        client.query_one(query, params).await
    }
}
pub async fn opt<C: GenericClient>(
    client: &C,
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    typed_params: Option<&[(&(dyn ToSql + Sync), Type)]>,
    cached: Option<&Statement>,
) -> Result<Option<Row>, Error> {
    if let Some(cached) = cached {
        client.query_opt(cached, params).await
    } else if C::stmt_cache() {
        let cached = client.prepare(query).await?;
        client.query_opt(&cached, params).await
    } else if let Some(typed_params) = typed_params {
        client.query_typed_opt(query, typed_params).await
    } else {
        client.query_opt(query, params).await
    }
}
pub async fn raw<C: GenericClient>(
    client: &C,
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    typed_params: Option<&[(&(dyn ToSql + Sync), Type)]>,
    cached: Option<&Statement>,
) -> Result<RowStream, Error> {
    if let Some(cached) = cached {
        client.query_raw(cached, params).await
    } else if C::stmt_cache() {
        let cached = client.prepare(query).await?;
        client.query_raw(&cached, params).await
    } else if let Some(typed_params) = typed_params {
        client.query_typed_raw(query, typed_params).await
    } else {
        client.query_raw(query, params).await
    }
}
pub async fn execute<C: GenericClient>(
    client: &C,
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    typed_params: Option<&[(&(dyn ToSql + Sync), Type)]>,
    cached: Option<&Statement>,
) -> Result<u64, Error> {
    if let Some(cached) = cached {
        client.execute(cached, params).await
    } else if C::stmt_cache() {
        let cached = client.prepare(query).await?;
        client.execute(&cached, params).await
    } else if let Some(typed_params) = typed_params {
        client.execute_typed(query, typed_params).await
    } else {
        client.execute(query, params).await
    }
}
