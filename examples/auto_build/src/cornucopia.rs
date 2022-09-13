// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod types {}
#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod queries {
    pub mod module_1 {
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        pub fn example_query() -> ExampleQueryStmt {
            ExampleQueryStmt(cornucopia_async::private::Stmt::new(
                "SELECT
    *
FROM
    example_table",
            ))
        }
        pub struct ExampleQueryStmt(cornucopia_async::private::Stmt);
        impl ExampleQueryStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> cornucopia_async::private::Query<'a, C, String, String, 0> {
                cornucopia_async::private::Query {
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
