#![feature(generic_associated_types)]
mod array_iterator;
#[cfg(feature = "async")]
pub mod async_;
#[cfg(feature = "deadpool")]
mod deadpool;
#[doc(hidden)]
pub mod private;
pub mod sync;

pub use array_iterator::ArrayIterator;

pub trait Borrow {
    type Borrow<'r>: 'r;
}

macro_rules! borrow {
    ($ty:ty) => {
        borrow!($ty, $ty);
    };
    ($own:ty, $brw:ty) => {
        impl Borrow for $own {
            type Borrow<'r> = $brw;
        }
        impl Borrow for Vec<$own> {
            type Borrow<'r> = ArrayIterator<'r, $brw>;
        }
        impl Borrow for Option<$own> {
            type Borrow<'r> = Option<$brw>;
        }
    };
}
borrow!(bool);
borrow!(i16);
borrow!(i32);
borrow!(i64);
borrow!(f32);
borrow!(f64);
borrow!(String, &'r str);

// TODO borrow for all supported types
