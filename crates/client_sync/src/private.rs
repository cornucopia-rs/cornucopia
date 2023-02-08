pub use cornucopia_client_core::{slice_iter, Domain, DomainArray};
use postgres::{
    types::{BorrowToSql, ToSql},
    Error, Row, RowIter, Statement,
};

use crate::GenericClient;

pub fn one<C: GenericClient>(
    client: &mut C,
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    cached: Option<&Statement>,
) -> Result<Row, Error> {
    if let Some(cached) = cached {
        client.query_one(cached, params)
    } else if C::stmt_cache() {
        let cached = client.prepare(query)?;
        client.query_one(&cached, params)
    } else {
        client.query_one(query, params)
    }
}

pub fn opt<C: GenericClient>(
    client: &mut C,
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    cached: Option<&Statement>,
) -> Result<Option<Row>, Error> {
    if let Some(cached) = cached {
        client.query_opt(cached, params)
    } else if C::stmt_cache() {
        let cached = client.prepare(query)?;
        client.query_opt(&cached, params)
    } else {
        client.query_opt(query, params)
    }
}

pub fn raw<'a, C: GenericClient, P, I>(
    client: &'a mut C,
    query: &str,
    params: I,
    cached: Option<&Statement>,
) -> Result<RowIter<'a>, Error>
where
    P: BorrowToSql,
    I: IntoIterator<Item = P>,
    I::IntoIter: ExactSizeIterator,
{
    if let Some(cached) = cached {
        client.query_raw(cached, params)
    } else if C::stmt_cache() {
        let cached = client.prepare(query)?;
        client.query_raw(&cached, params)
    } else {
        client.query_raw(query, params)
    }
}
