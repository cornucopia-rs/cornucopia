mod array_iterator;
#[cfg(feature = "async")]
pub mod async_;
#[cfg(feature = "deadpool")]
mod deadpool;
#[doc(hidden)]
pub mod private;

pub use array_iterator::ArrayIterator;

pub mod sync {
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

    pub trait Params<'a, S, O, C> {
        fn bind(&'a self, client: &'a mut C, stmt: &'a mut S) -> O;
    }
}
