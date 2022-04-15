use cornucopia::{run, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run().await
}
