// This file was generated with `cornucopia`. Do not modify.

use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct InsertBookStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn insert_book() -> InsertBookStmt {
    InsertBookStmt("INSERT INTO books (title) VALUES ($1)", None)
}
impl InsertBookStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        title: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[title]).await
    }
}
