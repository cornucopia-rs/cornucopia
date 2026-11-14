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
        let typed_params: Option<[(&(dyn postgres_types::ToSql + Sync), postgres_types::Type); 1]> =
            Some([(title, postgres_types::Type::VARCHAR)]);
        crate::client::async_::execute(
            client,
            self.0,
            &[title],
            typed_params.as_ref().map(|params| params.as_slice()),
            self.1.as_ref(),
        )
        .await
    }
}
