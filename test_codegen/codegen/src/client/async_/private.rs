pub use super::super::{slice_iter, Domain, DomainArray};

use super::generic_client::GenericClient;
use tokio_postgres::{Error, Statement};

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

    pub async fn prepare<'a, C: GenericClient>(
        &'a mut self,
        client: &C,
    ) -> Result<&'a Statement, Error> {
        if self.cached.is_none() {
            let stmt = client.prepare(self.query).await?;
            self.cached = Some(stmt);
        }
        // the statement is always prepared at this point
        Ok(unsafe { self.cached.as_ref().unwrap_unchecked() })
    }
}
