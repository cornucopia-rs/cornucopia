use cornucopia::{run, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let result = run().await;
    if let Err(e) = &result {
        eprintln!("{e}");
    }
    result
}
