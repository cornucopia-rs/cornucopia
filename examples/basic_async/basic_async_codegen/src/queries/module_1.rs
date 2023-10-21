// This file was generated with `cornucopia`. Do not modify.

use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub fn insert_book() -> InsertBookStmt {
    InsertBookStmt(crate::client::async_::Stmt::new(
        "INSERT INTO Book (title)
  VALUES ($1)",
    ))
}
pub struct InsertBookStmt(crate::client::async_::Stmt);
impl InsertBookStmt {
    pub async fn bind<'a, C: GenericClient, T1: crate::StringSql>(
        &'a mut self,
        client: &'a C,
        title: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[title]).await
    }
}
