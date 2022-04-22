// This file was generated with `cornucopia`. Do not modify.

pub mod types {}

pub mod queries {
    pub mod module_1 {
        use cornucopia_client::GenericClient;
        use tokio_postgres::Error;

        pub async fn example_query1<T: GenericClient>(client: &T) -> Result<String, Error> {
            let stmt = client
                .prepare(
                    "SELECT
*
FROM
example_table;
",
                )
                .await?;
            let res = client.query_one(&stmt, &[]).await?;

            let return_value: String = res.get(0);
            Ok(return_value)
        }
    }
}
