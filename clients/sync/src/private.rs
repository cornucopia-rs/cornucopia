use cornucopia_client_core::Borrow;
pub use cornucopia_client_core::{slice_iter, Domain, DomainArray};

use postgres::{
    fallible_iterator::FallibleIterator, types::ToSql, Error, GenericClient, Row, Statement,
};

/// Cached statement
pub struct Stmt {
    query: &'static str,
    cached: Option<Statement>,
}

impl Stmt {
    #[must_use]
    pub fn new(query: &'static str) -> Self {
        Self {
            query,
            cached: None,
        }
    }

    pub fn prepare<'a, C: postgres::GenericClient>(
        &'a mut self,
        client: &mut C,
    ) -> Result<&'a Statement, postgres::Error> {
        if self.cached.is_none() {
            let stmt = client.prepare(self.query)?;
            self.cached = Some(stmt);
        }
        // the statement is always prepared at this point
        Ok(unsafe { self.cached.as_ref().unwrap_unchecked() })
    }
}

pub struct Query<'a, C: GenericClient, T, B, const N: usize>
where
    B: Borrow,
{
    pub client: &'a mut C,
    pub params: [&'a (dyn ToSql + Sync); N],
    pub stmt: &'a mut Stmt,
    pub extractor: for<'r> fn(&'r Row) -> B::Borrow<'r>,
    pub mapper: for<'r> fn(B::Borrow<'r>) -> T,
}

impl<'a, C, T: 'a, B: Borrow, const N: usize> Query<'a, C, T, B, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: for<'b> fn(B::Borrow<'b>) -> R) -> Query<'a, C, R, B, N> {
        Query {
            client: self.client,
            params: self.params,
            stmt: self.stmt,
            extractor: self.extractor,
            mapper,
        }
    }

    pub fn one(&mut self) -> Result<T, Error> {
        let stmt = self.stmt.prepare(self.client)?;
        let row = self.client.query_one(stmt, &self.params)?;
        Ok((self.mapper)((self.extractor)(&row)))
    }

    pub fn opt(&mut self) -> Result<Option<T>, Error> {
        let stmt = self.stmt.prepare(self.client)?;
        Ok(self
            .client
            .query_opt(stmt, &self.params)?
            .map(|row| (self.mapper)((self.extractor)(&row))))
    }

    pub fn all(&'a mut self) -> Result<Vec<T>, Error> {
        self.iter()?.collect()
    }

    pub fn iter(&'a mut self) -> Result<impl Iterator<Item = Result<T, Error>> + 'a, Error> {
        let stmt = self.stmt.prepare(self.client)?;
        let stream = self
            .client
            .query_raw(stmt, slice_iter(&self.params))?
            .iterator()
            .map(|res| res.map(|row| (self.mapper)((self.extractor)(&row))));
        Ok(stream)
    }
}
