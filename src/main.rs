use std::fmt::Display;

use cornucopia::{run, Error};

struct ErrorWrapper(Error);

impl std::fmt::Debug for ErrorWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Error as Display>::fmt(&self.0, f)
    }
}

fn main() -> Result<(), ErrorWrapper> {
    run().map_err(ErrorWrapper)
}
