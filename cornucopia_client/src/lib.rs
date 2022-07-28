mod array_iterator;

/// Hidden modules
///
/// The following modules are public because the generated code needs
/// to access them, but users should not depend on them.

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

/// Public items
///
/// These items are the real public part of the API.
pub use array_iterator::ArrayIterator;

#[cfg(feature = "async")]
pub use async_::GenericClient;

#[cfg(all(feature = "async", not(feature = "sync")))]
pub use async_::Params;
#[cfg(all(feature = "sync", feature = "async"))]
pub use async_::Params as AsyncParams;
#[cfg(all(feature = "sync", not(feature = "async")))]
pub use sync::Params;
#[cfg(all(feature = "sync", feature = "async"))]
pub use sync::Params as SyncParams;
