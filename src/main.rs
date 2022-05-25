use cornucopia::{cli::run, Error};

fn main() -> Result<(), Error> {
    let result = run();
    if let Err(e) = &result {
        eprintln!("{e}");
    }
    result
}
