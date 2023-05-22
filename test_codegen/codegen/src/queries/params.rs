// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct InsertBookParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub author: Option<T1>,
    pub name: T2,
}
#[derive(Clone, Copy, Debug)]
pub struct ParamsOrderParams {
    pub c: i32,
    pub a: i32,
}
#[derive(serde::Serialize, Debug, Clone, PartialEq)]
pub struct SelectBook {
    pub name: String,
    pub author: Option<String>,
}
pub struct SelectBookBorrowed<'a> {
    pub name: &'a str,
    pub author: Option<&'a str>,
}
impl<'a> From<SelectBookBorrowed<'a>> for SelectBook {
    fn from(SelectBookBorrowed { name, author }: SelectBookBorrowed<'a>) -> Self {
        Self {
            name: name.into(),
            author: author.map(|v| v.into()),
        }
    }
}
#[derive(serde::Serialize, Debug, Clone, PartialEq)]
pub struct FindBooks {
    pub name: String,
    pub author: Option<String>,
}
pub struct FindBooksBorrowed<'a> {
    pub name: &'a str,
    pub author: Option<&'a str>,
}
impl<'a> From<FindBooksBorrowed<'a>> for FindBooks {
    fn from(FindBooksBorrowed { name, author }: FindBooksBorrowed<'a>) -> Self {
        Self {
            name: name.into(),
            author: author.map(|v| v.into()),
        }
    }
}
pub mod sync {
    use postgres::{fallible_iterator::FallibleIterator, GenericClient};
    pub struct SelectBookQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> super::SelectBookBorrowed,
        mapper: fn(super::SelectBookBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> SelectBookQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectBookBorrowed) -> R,
        ) -> SelectBookQuery<'a, C, R, N> {
            SelectBookQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            let row = self.client.query_one(stmt, &self.params)?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)?
                .map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
        {
            let stmt = self.stmt.prepare(self.client)?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))?
                .iterator()
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
            Ok(it)
        }
    }
    pub struct FindBooksQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> super::FindBooksBorrowed,
        mapper: fn(super::FindBooksBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> FindBooksQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::FindBooksBorrowed) -> R,
        ) -> FindBooksQuery<'a, C, R, N> {
            FindBooksQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            let row = self.client.query_one(stmt, &self.params)?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)?
                .map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
        {
            let stmt = self.stmt.prepare(self.client)?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))?
                .iterator()
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
            Ok(it)
        }
    }
    pub fn insert_book() -> InsertBookStmt {
        InsertBookStmt(crate::client::sync::Stmt::new(
            "INSERT INTO book (author, name) VALUES ($1, $2)",
        ))
    }
    pub struct InsertBookStmt(crate::client::sync::Stmt);
    impl InsertBookStmt {
        pub fn bind<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
            &'a mut self,
            client: &'a mut C,
            author: &'a Option<T1>,
            name: &'a T2,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[author, name])
        }
    }
    impl<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
        crate::client::sync::Params<
            'a,
            super::InsertBookParams<T1, T2>,
            Result<u64, postgres::Error>,
            C,
        > for InsertBookStmt
    {
        fn params(
            &'a mut self,
            client: &'a mut C,
            params: &'a super::InsertBookParams<T1, T2>,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.author, &params.name)
        }
    }
    pub fn select_book() -> SelectBookStmt {
        SelectBookStmt(crate::client::sync::Stmt::new("SELECT * FROM book"))
    }
    pub struct SelectBookStmt(crate::client::sync::Stmt);
    impl SelectBookStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
        ) -> SelectBookQuery<'a, C, super::SelectBook, 0> {
            SelectBookQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| super::SelectBookBorrowed {
                    name: row.get(0),
                    author: row.get(1),
                },
                mapper: |it| <super::SelectBook>::from(it),
            }
        }
    }
    pub fn find_books() -> FindBooksStmt {
        FindBooksStmt(crate::client::sync::Stmt::new(
            "SELECT * FROM book WHERE name = ANY ($1)",
        ))
    }
    pub struct FindBooksStmt(crate::client::sync::Stmt);
    impl FindBooksStmt {
        pub fn bind<'a, C: GenericClient, T1: crate::StringSql, T2: crate::ArraySql<Item = T1>>(
            &'a mut self,
            client: &'a mut C,
            title: &'a T2,
        ) -> FindBooksQuery<'a, C, super::FindBooks, 1> {
            FindBooksQuery {
                client,
                params: [title],
                stmt: &mut self.0,
                extractor: |row| super::FindBooksBorrowed {
                    name: row.get(0),
                    author: row.get(1),
                },
                mapper: |it| <super::FindBooks>::from(it),
            }
        }
    }
    pub fn params_use_twice() -> ParamsUseTwiceStmt {
        ParamsUseTwiceStmt(crate::client::sync::Stmt::new(
            "UPDATE book SET name = $1 WHERE length(name) > 42 AND length($1) < 42",
        ))
    }
    pub struct ParamsUseTwiceStmt(crate::client::sync::Stmt);
    impl ParamsUseTwiceStmt {
        pub fn bind<'a, C: GenericClient, T1: crate::StringSql>(
            &'a mut self,
            client: &'a mut C,
            name: &'a T1,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[name])
        }
    }
    pub fn params_order() -> ParamsOrderStmt {
        ParamsOrderStmt(crate::client::sync::Stmt::new(
            "UPDATE imaginary SET c=$1, a=$2, z=$2, r=$1",
        ))
    }
    pub struct ParamsOrderStmt(crate::client::sync::Stmt);
    impl ParamsOrderStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
            c: &'a i32,
            a: &'a i32,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[c, a])
        }
    }
    impl<'a, C: GenericClient>
        crate::client::sync::Params<'a, super::ParamsOrderParams, Result<u64, postgres::Error>, C>
        for ParamsOrderStmt
    {
        fn params(
            &'a mut self,
            client: &'a mut C,
            params: &'a super::ParamsOrderParams,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.c, &params.a)
        }
    }
}
pub mod async_ {
    use crate::client::async_::GenericClient;
    use futures::{self, StreamExt, TryStreamExt};
    pub struct SelectBookQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::async_::Stmt,
        extractor: fn(&tokio_postgres::Row) -> super::SelectBookBorrowed,
        mapper: fn(super::SelectBookBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> SelectBookQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectBookBorrowed) -> R,
        ) -> SelectBookQuery<'a, C, R, N> {
            SelectBookQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            let row = self.client.query_one(stmt, &self.params).await?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)
                .await?
                .map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
            tokio_postgres::Error,
        > {
            let stmt = self.stmt.prepare(self.client).await?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))
                .await?
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                .into_stream();
            Ok(it)
        }
    }
    pub struct FindBooksQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::async_::Stmt,
        extractor: fn(&tokio_postgres::Row) -> super::FindBooksBorrowed,
        mapper: fn(super::FindBooksBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> FindBooksQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::FindBooksBorrowed) -> R,
        ) -> FindBooksQuery<'a, C, R, N> {
            FindBooksQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            let row = self.client.query_one(stmt, &self.params).await?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)
                .await?
                .map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
            tokio_postgres::Error,
        > {
            let stmt = self.stmt.prepare(self.client).await?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))
                .await?
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                .into_stream();
            Ok(it)
        }
    }
    pub fn insert_book() -> InsertBookStmt {
        InsertBookStmt(crate::client::async_::Stmt::new(
            "INSERT INTO book (author, name) VALUES ($1, $2)",
        ))
    }
    pub struct InsertBookStmt(crate::client::async_::Stmt);
    impl InsertBookStmt {
        pub async fn bind<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
            &'a mut self,
            client: &'a C,
            author: &'a Option<T1>,
            name: &'a T2,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[author, name]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql, T2: crate::StringSql>
        crate::client::async_::Params<
            'a,
            super::InsertBookParams<T1, T2>,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for InsertBookStmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::InsertBookParams<T1, T2>,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.author, &params.name))
        }
    }
    pub fn select_book() -> SelectBookStmt {
        SelectBookStmt(crate::client::async_::Stmt::new("SELECT * FROM book"))
    }
    pub struct SelectBookStmt(crate::client::async_::Stmt);
    impl SelectBookStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
        ) -> SelectBookQuery<'a, C, super::SelectBook, 0> {
            SelectBookQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| super::SelectBookBorrowed {
                    name: row.get(0),
                    author: row.get(1),
                },
                mapper: |it| <super::SelectBook>::from(it),
            }
        }
    }
    pub fn find_books() -> FindBooksStmt {
        FindBooksStmt(crate::client::async_::Stmt::new(
            "SELECT * FROM book WHERE name = ANY ($1)",
        ))
    }
    pub struct FindBooksStmt(crate::client::async_::Stmt);
    impl FindBooksStmt {
        pub fn bind<'a, C: GenericClient, T1: crate::StringSql, T2: crate::ArraySql<Item = T1>>(
            &'a mut self,
            client: &'a C,
            title: &'a T2,
        ) -> FindBooksQuery<'a, C, super::FindBooks, 1> {
            FindBooksQuery {
                client,
                params: [title],
                stmt: &mut self.0,
                extractor: |row| super::FindBooksBorrowed {
                    name: row.get(0),
                    author: row.get(1),
                },
                mapper: |it| <super::FindBooks>::from(it),
            }
        }
    }
    pub fn params_use_twice() -> ParamsUseTwiceStmt {
        ParamsUseTwiceStmt(crate::client::async_::Stmt::new(
            "UPDATE book SET name = $1 WHERE length(name) > 42 AND length($1) < 42",
        ))
    }
    pub struct ParamsUseTwiceStmt(crate::client::async_::Stmt);
    impl ParamsUseTwiceStmt {
        pub async fn bind<'a, C: GenericClient, T1: crate::StringSql>(
            &'a mut self,
            client: &'a C,
            name: &'a T1,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[name]).await
        }
    }
    pub fn params_order() -> ParamsOrderStmt {
        ParamsOrderStmt(crate::client::async_::Stmt::new(
            "UPDATE imaginary SET c=$1, a=$2, z=$2, r=$1",
        ))
    }
    pub struct ParamsOrderStmt(crate::client::async_::Stmt);
    impl ParamsOrderStmt {
        pub async fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
            c: &'a i32,
            a: &'a i32,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[c, a]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            super::ParamsOrderParams,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for ParamsOrderStmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::ParamsOrderParams,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.c, &params.a))
        }
    }
}
