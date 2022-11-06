#[doc(hidden)]
pub mod private;

pub use crate::generic_client::GenericClient;
pub use cornucopia_client_core::{ArrayIterator, ArraySql, BytesSql, IterSql, StringSql};

#[cfg(feature = "with-serde_json-1")]
pub use cornucopia_client_core::JsonSql;

#[cfg(feature = "deadpool")]
mod deadpool;
mod generic_client;

/// This trait allows you to bind parameters to a query using a single
/// struct, rather than passing each bind parameter as a function parameter.
pub trait Params<'a, P, O, C> {
    fn params(&'a mut self, client: &'a C, params: &'a P) -> O;
}
