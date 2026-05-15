// This file was generated with `cornucopia`. Do not modify.

use crate::client::sync::GenericClient;
use postgres::fallible_iterator::FallibleIterator;
pub struct InsertBookStmt(&'static str, Option<postgres::Statement>);
pub fn insert_book() -> InsertBookStmt {
    InsertBookStmt("INSERT INTO books (title) VALUES ($1)", None)
}
impl InsertBookStmt {
    pub fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a mut C,
    ) -> Result<Self, postgres::Error> {
        self.1 = Some(client.prepare(self.0)?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c mut C,
        title: &'a T1,
    ) -> Result<u64, postgres::Error> {
        client.execute(self.0, &[title])
    }
}
