// This file was generated with `cornucopia`. Do not modify.

use postgres::{fallible_iterator::FallibleIterator, GenericClient};
pub fn insert_book() -> InsertBookStmt {
    InsertBookStmt(crate::client::sync::Stmt::new(
        "INSERT INTO Book (title)
  VALUES ($1)",
    ))
}
pub struct InsertBookStmt(crate::client::sync::Stmt);
impl InsertBookStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::StringSql>(
        &'a mut self,
        client: &'a mut C,
        title: &'a T1,
    ) -> Result<u64, postgres::Error> {
        let stmt = self.0.prepare(client)?;
        client.execute(stmt, &[title])
    }
}
