// This file was generated with `cornucopia`. Do not modify.

 #[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { } #[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{  pub mod module_1
{   use futures::{StreamExt, TryStreamExt};use futures; use cornucopia_async::GenericClient; pub struct StringQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> & str,
    mapper: fn(& str) -> T,
} impl<'a, C, T:'a, const N: usize> StringQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(& str) -> R) ->
    StringQuery<'a,C,R,N>
    {
        StringQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
} pub fn example_query() -> ExampleQueryStmt
{ ExampleQueryStmt(cornucopia_async::private::Stmt::new("SELECT
    *
FROM
    example_table")) } pub struct
ExampleQueryStmt(cornucopia_async::private::Stmt); impl ExampleQueryStmt
{  pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> StringQuery<'a,C,
String, 0>
{
    StringQuery
    {
        client, params: [], stmt: &mut self.0, extractor:
        |row| {  row.get(0) }, mapper: |it| { it.into() },
    }
} } }}