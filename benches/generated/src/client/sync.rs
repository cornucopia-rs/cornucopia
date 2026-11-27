// This file was generated with `cornucopia`. Do not modify.

pub use generic_client::GenericClient;
mod generic_client;
use postgres::{
    Error, Row, RowIter, Statement,
    types::{ToSql, Type},
};
/// This trait allows you to bind parameters to a query using a single
/// struct, rather than passing each bind parameter as a function parameter.
pub trait Params<'c, 'a, 's, P, O, C> {
    fn params(&'s self, client: &'c mut C, params: &'a P) -> O;
}
pub fn one<C: GenericClient>(
    client: &mut C,
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    typed_params: Option<&[(&(dyn ToSql + Sync), Type)]>,
    cached: Option<&Statement>,
) -> Result<Row, Error> {
    if let Some(cached) = cached {
        client.query_one(cached, params)
    } else if C::stmt_cache() {
        let cached = client.prepare(query)?;
        client.query_one(&cached, params)
    } else if let Some(typed_params) = typed_params {
        client.query_typed_one(query, typed_params)
    } else {
        client.query_one(query, params)
    }
}
pub fn opt<C: GenericClient>(
    client: &mut C,
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    typed_params: Option<&[(&(dyn ToSql + Sync), Type)]>,
    cached: Option<&Statement>,
) -> Result<Option<Row>, Error> {
    if let Some(cached) = cached {
        client.query_opt(cached, params)
    } else if C::stmt_cache() {
        let cached = client.prepare(query)?;
        client.query_opt(&cached, params)
    } else if let Some(typed_params) = typed_params {
        client.query_typed_opt(query, typed_params)
    } else {
        client.query_opt(query, params)
    }
}
pub fn raw<'a, C: GenericClient>(
    client: &'a mut C,
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    typed_params: Option<&[(&(dyn ToSql + Sync), Type)]>,
    cached: Option<&Statement>,
) -> Result<RowIter<'a>, Error> {
    if let Some(cached) = cached {
        client.query_raw(cached, params)
    } else if C::stmt_cache() {
        let cached = client.prepare(query)?;
        client.query_raw(&cached, params)
    } else if let Some(typed_params) = typed_params {
        client.query_typed_raw(query, typed_params)
    } else {
        client.query_raw(query, params)
    }
}
pub fn execute<C: GenericClient>(
    client: &mut C,
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    typed_params: Option<&[(&(dyn ToSql + Sync), Type)]>,
    cached: Option<&Statement>,
) -> Result<u64, Error> {
    if let Some(cached) = cached {
        client.execute(cached, params)
    } else if C::stmt_cache() {
        let cached = client.prepare(query)?;
        client.execute(&cached, params)
    } else if let Some(typed_params) = typed_params {
        client.execute_typed(query, typed_params)
    } else {
        client.execute(query, params)
    }
}
