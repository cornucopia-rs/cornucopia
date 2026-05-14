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
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
    use crate::client::sync::GenericClient;
    use postgres::fallible_iterator::FallibleIterator;
    pub struct SelectBookQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor: fn(&postgres::Row) -> Result<super::SelectBookBorrowed, postgres::Error>,
        mapper: fn(super::SelectBookBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> SelectBookQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectBookBorrowed) -> R,
        ) -> SelectBookQuery<'c, 'a, 's, C, R, N> {
            SelectBookQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let row = crate::client::sync::one(self.client, self.query, &self.params, self.cached)?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let opt_row =
                crate::client::sync::opt(self.client, self.query, &self.params, self.cached)?;
            Ok(opt_row
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'c, postgres::Error>
        {
            let stream = crate::client::sync::raw(
                self.client,
                self.query,
                crate::slice_iter(&self.params),
                self.cached,
            )?;
            let mapped = stream.iterator().map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            });
            Ok(mapped)
        }
    }
    pub struct FindBooksQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor: fn(&postgres::Row) -> Result<super::FindBooksBorrowed, postgres::Error>,
        mapper: fn(super::FindBooksBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> FindBooksQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::FindBooksBorrowed) -> R,
        ) -> FindBooksQuery<'c, 'a, 's, C, R, N> {
            FindBooksQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let row = crate::client::sync::one(self.client, self.query, &self.params, self.cached)?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let opt_row =
                crate::client::sync::opt(self.client, self.query, &self.params, self.cached)?;
            Ok(opt_row
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'c, postgres::Error>
        {
            let stream = crate::client::sync::raw(
                self.client,
                self.query,
                crate::slice_iter(&self.params),
                self.cached,
            )?;
            let mapped = stream.iterator().map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            });
            Ok(mapped)
        }
    }
    pub struct InsertBookStmt(&'static str, Option<postgres::Statement>);
    pub fn insert_book() -> InsertBookStmt {
        InsertBookStmt("INSERT INTO book (author, name) VALUES ($1, $2)", None)
    }
    impl InsertBookStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
            &'s self,
            client: &'c mut C,
            author: &'a Option<T1>,
            name: &'a T2,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[author, name])
        }
    }
    impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::InsertBookParams<T1, T2>,
            Result<u64, postgres::Error>,
            C,
        > for InsertBookStmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::InsertBookParams<T1, T2>,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.author, &params.name)
        }
    }
    pub struct SelectBookStmt(&'static str, Option<postgres::Statement>);
    pub fn select_book() -> SelectBookStmt {
        SelectBookStmt("SELECT * FROM book", None)
    }
    impl SelectBookStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s self,
            client: &'c mut C,
        ) -> SelectBookQuery<'c, 'a, 's, C, super::SelectBook, 0> {
            SelectBookQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor:
                    |row: &postgres::Row| -> Result<super::SelectBookBorrowed, postgres::Error> {
                        Ok(super::SelectBookBorrowed {
                            name: row.try_get(0)?,
                            author: row.try_get(1)?,
                        })
                    },
                mapper: |it| super::SelectBook::from(it),
            }
        }
    }
    pub struct FindBooksStmt(&'static str, Option<postgres::Statement>);
    pub fn find_books() -> FindBooksStmt {
        FindBooksStmt("SELECT * FROM book WHERE name = ANY ($1)", None)
    }
    impl FindBooksStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<
            'c,
            'a,
            's,
            C: GenericClient,
            T1: crate::StringSql,
            T2: crate::ArraySql<Item = T1>,
        >(
            &'s self,
            client: &'c mut C,
            title: &'a T2,
        ) -> FindBooksQuery<'c, 'a, 's, C, super::FindBooks, 1> {
            FindBooksQuery {
                client,
                params: [title],
                query: self.0,
                cached: self.1.as_ref(),
                extractor:
                    |row: &postgres::Row| -> Result<super::FindBooksBorrowed, postgres::Error> {
                        Ok(super::FindBooksBorrowed {
                            name: row.try_get(0)?,
                            author: row.try_get(1)?,
                        })
                    },
                mapper: |it| super::FindBooks::from(it),
            }
        }
    }
    pub struct ParamsUseTwiceStmt(&'static str, Option<postgres::Statement>);
    pub fn params_use_twice() -> ParamsUseTwiceStmt {
        ParamsUseTwiceStmt(
            "UPDATE book SET name = $1 WHERE length(name) > 42 AND length($1) < 42",
            None,
        )
    }
    impl ParamsUseTwiceStmt {
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
            name: &'a T1,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[name])
        }
    }
    pub struct ParamsOrderStmt(&'static str, Option<postgres::Statement>);
    pub fn params_order() -> ParamsOrderStmt {
        ParamsOrderStmt("UPDATE imaginary SET c=$1, a=$2, z=$2, r=$1", None)
    }
    impl ParamsOrderStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s self,
            client: &'c mut C,
            c: &'a i32,
            a: &'a i32,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[c, a])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::ParamsOrderParams,
            Result<u64, postgres::Error>,
            C,
        > for ParamsOrderStmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::ParamsOrderParams,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.c, &params.a)
        }
    }
}
pub mod async_ {
    use crate::client::async_::GenericClient;
    use futures::{self, StreamExt, TryStreamExt};
    pub struct SelectBookQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor:
            fn(&tokio_postgres::Row) -> Result<super::SelectBookBorrowed, tokio_postgres::Error>,
        mapper: fn(super::SelectBookBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> SelectBookQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectBookBorrowed) -> R,
        ) -> SelectBookQuery<'c, 'a, 's, C, R, N> {
            SelectBookQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let row =
                crate::client::async_::one(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let opt_row =
                crate::client::async_::opt(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok(opt_row
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
            tokio_postgres::Error,
        > {
            let stream = crate::client::async_::raw(
                self.client,
                self.query,
                crate::slice_iter(&self.params),
                self.cached,
            )
            .await?;
            let mapped = stream
                .map(move |res| {
                    res.and_then(|row| {
                        let extracted = (self.extractor)(&row)?;
                        Ok((self.mapper)(extracted))
                    })
                })
                .into_stream();
            Ok(mapped)
        }
    }
    pub struct FindBooksQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor:
            fn(&tokio_postgres::Row) -> Result<super::FindBooksBorrowed, tokio_postgres::Error>,
        mapper: fn(super::FindBooksBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> FindBooksQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::FindBooksBorrowed) -> R,
        ) -> FindBooksQuery<'c, 'a, 's, C, R, N> {
            FindBooksQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let row =
                crate::client::async_::one(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let opt_row =
                crate::client::async_::opt(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok(opt_row
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
            tokio_postgres::Error,
        > {
            let stream = crate::client::async_::raw(
                self.client,
                self.query,
                crate::slice_iter(&self.params),
                self.cached,
            )
            .await?;
            let mapped = stream
                .map(move |res| {
                    res.and_then(|row| {
                        let extracted = (self.extractor)(&row)?;
                        Ok((self.mapper)(extracted))
                    })
                })
                .into_stream();
            Ok(mapped)
        }
    }
    pub struct InsertBookStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn insert_book() -> InsertBookStmt {
        InsertBookStmt("INSERT INTO book (author, name) VALUES ($1, $2)", None)
    }
    impl InsertBookStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub async fn bind<
            'c,
            'a,
            's,
            C: GenericClient,
            T1: crate::StringSql,
            T2: crate::StringSql,
        >(
            &'s self,
            client: &'c C,
            author: &'a Option<T1>,
            name: &'a T2,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[author, name]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql, T2: crate::StringSql>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::InsertBookParams<T1, T2>,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for InsertBookStmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::InsertBookParams<T1, T2>,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.author, &params.name))
        }
    }
    pub struct SelectBookStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn select_book() -> SelectBookStmt {
        SelectBookStmt("SELECT * FROM book", None)
    }
    impl SelectBookStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s self,
            client: &'c C,
        ) -> SelectBookQuery<'c, 'a, 's, C, super::SelectBook, 0> {
            SelectBookQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |
                    row: &tokio_postgres::Row,
                | -> Result<super::SelectBookBorrowed, tokio_postgres::Error> {
                    Ok(super::SelectBookBorrowed {
                        name: row.try_get(0)?,
                        author: row.try_get(1)?,
                    })
                },
                mapper: |it| super::SelectBook::from(it),
            }
        }
    }
    pub struct FindBooksStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn find_books() -> FindBooksStmt {
        FindBooksStmt("SELECT * FROM book WHERE name = ANY ($1)", None)
    }
    impl FindBooksStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub fn bind<
            'c,
            'a,
            's,
            C: GenericClient,
            T1: crate::StringSql,
            T2: crate::ArraySql<Item = T1>,
        >(
            &'s self,
            client: &'c C,
            title: &'a T2,
        ) -> FindBooksQuery<'c, 'a, 's, C, super::FindBooks, 1> {
            FindBooksQuery {
                client,
                params: [title],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |
                    row: &tokio_postgres::Row,
                | -> Result<super::FindBooksBorrowed, tokio_postgres::Error> {
                    Ok(super::FindBooksBorrowed {
                        name: row.try_get(0)?,
                        author: row.try_get(1)?,
                    })
                },
                mapper: |it| super::FindBooks::from(it),
            }
        }
    }
    pub struct ParamsUseTwiceStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn params_use_twice() -> ParamsUseTwiceStmt {
        ParamsUseTwiceStmt(
            "UPDATE book SET name = $1 WHERE length(name) > 42 AND length($1) < 42",
            None,
        )
    }
    impl ParamsUseTwiceStmt {
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
            name: &'a T1,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[name]).await
        }
    }
    pub struct ParamsOrderStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn params_order() -> ParamsOrderStmt {
        ParamsOrderStmt("UPDATE imaginary SET c=$1, a=$2, z=$2, r=$1", None)
    }
    impl ParamsOrderStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub async fn bind<'c, 'a, 's, C: GenericClient>(
            &'s self,
            client: &'c C,
            c: &'a i32,
            a: &'a i32,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[c, a]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::ParamsOrderParams,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for ParamsOrderStmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::ParamsOrderParams,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.c, &params.a))
        }
    }
}
