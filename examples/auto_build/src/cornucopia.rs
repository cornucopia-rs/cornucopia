// This file was generated with `cornucopia`. Do not modify.
#![allow(clippy::all, clippy::pedantic)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
pub mod types {}
pub mod queries {
    pub mod module_1 {
        use cornucopia_client::async_::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        pub fn example_query() -> ExampleQueryStmt {
            ExampleQueryStmt(cornucopia_client::async_::Stmt::new(
                "SELECT
    *
FROM
    example_table",
            ))
        }
        pub struct ExampleQueryStmt(cornucopia_client::async_::Stmt);
        impl ExampleQueryStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> cornucopia_client::async_::Query<'a, C, String, String, 0> {
                cornucopia_client::async_::Query {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it.into(),
                }
            }
        }
    }
}
