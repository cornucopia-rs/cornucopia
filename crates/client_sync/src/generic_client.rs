use postgres::{
    types::{BorrowToSql, ToSql},
    Client, Error, Row, RowIter, Statement, ToStatement, Transaction,
};

/// Abstraction over multiple types of synchronous clients.
/// This allows you to use postgres clients and transactions interchangeably.
pub trait GenericClient {
    fn prepare(&mut self, query: &str) -> Result<Statement, Error>;
    fn stmt_cache() -> bool {
        false
    }
    fn execute<T>(&mut self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<u64, Error>
    where
        T: ?Sized + ToStatement;
    fn query_one<T>(&mut self, statement: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Row, Error>
    where
        T: ?Sized + ToStatement;
    fn query_opt<T>(
        &mut self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>, Error>
    where
        T: ?Sized + ToStatement;
    fn query<T>(&mut self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error>
    where
        T: ?Sized + ToStatement;

    fn query_raw<T, P, I>(&mut self, statement: &T, params: I) -> Result<RowIter<'_>, Error>
    where
        T: ?Sized + ToStatement,
        P: BorrowToSql,
        I: IntoIterator<Item = P>,
        I::IntoIter: ExactSizeIterator;
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

    fn query_one<T>(&mut self, statement: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Row, Error>
    where
        T: ?Sized + ToStatement,
    {
        Transaction::query_one(self, statement, params)
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

    fn query<T>(&mut self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error>
    where
        T: ?Sized + ToStatement,
    {
        Transaction::query(self, query, params)
    }

    fn query_raw<T, P, I>(&mut self, statement: &T, params: I) -> Result<RowIter<'_>, Error>
    where
        T: ?Sized + ToStatement,
        P: BorrowToSql,
        I: IntoIterator<Item = P>,
        I::IntoIter: ExactSizeIterator,
    {
        Transaction::query_raw(self, statement, params)
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

    fn query_one<T>(&mut self, statement: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Row, Error>
    where
        T: ?Sized + ToStatement,
    {
        Client::query_one(self, statement, params)
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

    fn query<T>(&mut self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error>
    where
        T: ?Sized + ToStatement,
    {
        Client::query(self, query, params)
    }

    fn query_raw<T, P, I>(&mut self, statement: &T, params: I) -> Result<RowIter<'_>, Error>
    where
        T: ?Sized + ToStatement,
        P: BorrowToSql,
        I: IntoIterator<Item = P>,
        I::IntoIter: ExactSizeIterator,
    {
        Client::query_raw(self, statement, params)
    }
}
