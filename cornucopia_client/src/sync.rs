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

/// This trait allows you to bind parameters to a query using a single
/// struct, rather than passing each bind parameter as a function parameter.
pub trait Params<'a, P, O, C> {
    fn params(&'a mut self, client: &'a mut C, params: &'a P) -> O;
}
