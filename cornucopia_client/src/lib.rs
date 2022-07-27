mod array_iterator;

#[cfg(feature = "async")]
#[doc(hidden)]
pub mod async_;

#[cfg(feature = "deadpool")]
#[doc(hidden)]
mod deadpool;

#[cfg(feature = "sync")]
#[doc(hidden)]
pub mod sync;

#[doc(hidden)]
pub mod private;

pub use array_iterator::ArrayIterator;

#[cfg(feature = "async")]
pub use async_::GenericClient;
