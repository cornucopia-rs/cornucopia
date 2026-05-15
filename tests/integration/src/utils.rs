/// Reset the current database
pub(crate) fn reset_db(client: &tokio_postgres::Client) -> Result<(), tokio_postgres::Error> {
    futures::executor::block_on(
        client.batch_execute("DROP SCHEMA public CASCADE;CREATE SCHEMA public;"),
    )
}
