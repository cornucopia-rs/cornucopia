// This file was generated with `cornucopia`. Do not modify.

mod array_iterator;
pub mod client;
mod domain;
#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod queries;
mod type_traits;
#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod types;
mod utils;
pub use array_iterator::ArrayIterator;
pub use domain::{Domain, DomainArray};
pub use type_traits::JsonSql;
pub use type_traits::{ArraySql, BytesSql, IterSql, StringSql};
pub(crate) use utils::slice_iter;
