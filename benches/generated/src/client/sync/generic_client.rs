// This file was generated with `cornucopia`. Do not modify.

use postgres::{
    Client, Error, Row, RowIter, Statement, ToStatement, Transaction,
    types::{ToSql, Type},
};
/// Abstraction over multiple types of synchronous clients.
/// This allows you to use postgres clients and transactions interchangeably.
pub trait GenericClient {
    fn stmt_cache() -> bool {
        false
    }
    fn prepare(&mut self, query: &str) -> Result<Statement, Error>;
    fn execute<T>(&mut self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<u64, Error>
    where
        T: ?Sized + ToStatement;
    fn execute_typed(
        &mut self,
        statement: &str,
        typed_params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<u64, Error>;
    fn query_one<T>(&mut self, statement: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Row, Error>
    where
        T: ?Sized + ToStatement;
    fn query_typed_one(
        &mut self,
        statement: &str,
        typed_params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<Row, Error>;
    fn query_opt<T>(
        &mut self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>, Error>
    where
        T: ?Sized + ToStatement;
    fn query_typed_opt(
        &mut self,
        statement: &str,
        typed_params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<Option<Row>, Error>;
    fn query<T>(&mut self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error>
    where
        T: ?Sized + ToStatement;
    fn query_raw<T>(
        &mut self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<RowIter<'_>, Error>
    where
        T: ?Sized + ToStatement;
    fn query_typed_raw(
        &mut self,
        statement: &str,
        typed_params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<RowIter<'_>, Error>;
}
impl GenericClient for Transaction<'_> {
    fn prepare(&mut self, query: &str) -> Result<Statement, Error> {
        Transaction::prepare(self, query)
    }
    fn execute<T>(&mut self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<u64, Error>
    where
        T: ?Sized + ToStatement,
    {
        Transaction::execute(self, query, params)
    }
    fn execute_typed(
        &mut self,
        statement: &str,
        typed_params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<u64, Error> {
        Transaction::execute_typed(self, statement, typed_params)
    }
    fn query_one<T>(&mut self, statement: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Row, Error>
    where
        T: ?Sized + ToStatement,
    {
        Transaction::query_one(self, statement, params)
    }
    fn query_typed_one(
        &mut self,
        statement: &str,
        typed_params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<Row, Error> {
        Transaction::query_typed_one(self, statement, typed_params)
    }
    fn query_opt<T>(
        &mut self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>, Error>
    where
        T: ?Sized + ToStatement,
    {
        Transaction::query_opt(self, statement, params)
    }
    fn query_typed_opt(
        &mut self,
        statement: &str,
        typed_params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<Option<Row>, Error> {
        Transaction::query_typed_opt(self, statement, typed_params)
    }
    fn query<T>(&mut self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error>
    where
        T: ?Sized + ToStatement,
    {
        Transaction::query(self, query, params)
    }
    fn query_raw<T>(
        &mut self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<RowIter<'_>, Error>
    where
        T: ?Sized + ToStatement,
    {
        Transaction::query_raw(self, statement, params.iter().copied())
    }
    fn query_typed_raw(
        &mut self,
        statement: &str,
        typed_params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<RowIter<'_>, Error> {
        Transaction::query_typed_raw(self, statement, typed_params.iter().cloned())
    }
}
impl GenericClient for Client {
    fn prepare(&mut self, query: &str) -> Result<Statement, Error> {
        Client::prepare(self, query)
    }
    fn execute<T>(&mut self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<u64, Error>
    where
        T: ?Sized + ToStatement,
    {
        Client::execute(self, query, params)
    }
    fn execute_typed(
        &mut self,
        statement: &str,
        typed_params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<u64, Error> {
        Client::execute_typed(self, statement, typed_params)
    }
    fn query_one<T>(&mut self, statement: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Row, Error>
    where
        T: ?Sized + ToStatement,
    {
        Client::query_one(self, statement, params)
    }
    fn query_typed_one(
        &mut self,
        statement: &str,
        typed_params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<Row, Error> {
        Client::query_typed_one(self, statement, typed_params)
    }
    fn query_opt<T>(
        &mut self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>, Error>
    where
        T: ?Sized + ToStatement,
    {
        Client::query_opt(self, statement, params)
    }
    fn query_typed_opt(
        &mut self,
        statement: &str,
        typed_params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<Option<Row>, Error> {
        Client::query_typed_opt(self, statement, typed_params)
    }
    fn query<T>(&mut self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error>
    where
        T: ?Sized + ToStatement,
    {
        Client::query(self, query, params)
    }
    fn query_raw<T>(
        &mut self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<RowIter<'_>, Error>
    where
        T: ?Sized + ToStatement,
    {
        Client::query_raw(self, statement, params.iter().copied())
    }
    fn query_typed_raw(
        &mut self,
        statement: &str,
        typed_params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<RowIter<'_>, Error> {
        Client::query_typed_raw(self, statement, typed_params.iter().cloned())
    }
}
