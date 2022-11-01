pub use cornucopia_client_core::{slice_iter, Domain, DomainArray};

use postgres::Statement;

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

#[macro_export]
macro_rules! query {
    ($name:ident, $brw:ty) => {
        pub struct $name<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor: fn(&postgres::Row) -> $brw,
            mapper: fn($brw) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> $name<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn($brw) -> R) -> $name<'a, C, R, N> {
                $name {
                    client: self.client,
                    params: self.params,
                    stmt: self.stmt,
                    extractor: self.extractor,
                    mapper,
                }
            }

            pub fn one(self) -> Result<T, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                let row = self.client.query_one(stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }

            pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                self.iter()?.collect()
            }

            pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }

            pub fn iter(
                self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt.prepare(self.client)?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }
    };
}
