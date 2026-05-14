// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
pub struct BookAuthor {
    pub id: i32,
    pub name: Option<String>,
    pub bio: String,
}
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
pub struct BookAuthorBorrowed<'a> {
    pub id: i32,
    pub name: Option<&'a str>,
    pub bio: &'a str,
}
impl<'a> From<BookAuthorBorrowed<'a>> for BookAuthor {
    fn from(BookAuthorBorrowed { id, name, bio }: BookAuthorBorrowed<'a>) -> Self {
        Self {
            id,
            name: name.map(|v| v.into()),
            bio: bio.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[allow(deprecated)]
pub struct BookAuthor2 {
    pub id: i32,
    pub name: Option<String>,
    pub bio: String,
}
pub struct BookAuthor2Borrowed<'a> {
    pub id: i32,
    pub name: Option<&'a str>,
    pub bio: &'a str,
}
impl<'a> From<BookAuthor2Borrowed<'a>> for BookAuthor2 {
    fn from(BookAuthor2Borrowed { id, name, bio }: BookAuthor2Borrowed<'a>) -> Self {
        Self {
            id,
            name: name.map(|v| v.into()),
            bio: bio.into(),
        }
    }
}
pub mod sync {
    use crate::client::sync::GenericClient;
    use postgres::fallible_iterator::FallibleIterator;
    pub struct BookAuthorQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor: fn(&postgres::Row) -> Result<super::BookAuthorBorrowed, postgres::Error>,
        mapper: fn(super::BookAuthorBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> BookAuthorQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::BookAuthorBorrowed) -> R,
        ) -> BookAuthorQuery<'c, 'a, 's, C, R, N> {
            BookAuthorQuery {
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
    pub struct BookAuthor2Query<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor: fn(&postgres::Row) -> Result<super::BookAuthor2Borrowed, postgres::Error>,
        mapper: fn(super::BookAuthor2Borrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> BookAuthor2Query<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::BookAuthor2Borrowed) -> R,
        ) -> BookAuthor2Query<'c, 'a, 's, C, R, N> {
            BookAuthor2Query {
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
    pub struct GetAuthorByIdStmt(&'static str, Option<postgres::Statement>);
    #[deprecated = "Use get_author_v2 instead"]
    #[allow(dead_code)]
    pub fn get_author_by_id() -> GetAuthorByIdStmt {
        GetAuthorByIdStmt("SELECT id, name, bio FROM book_authors WHERE id = $1", None)
    }
    impl GetAuthorByIdStmt {
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
            id: &'a i32,
        ) -> BookAuthorQuery<'c, 'a, 's, C, super::BookAuthor, 1> {
            BookAuthorQuery {
                client,
                params: [id],
                query: self.0,
                cached: self.1.as_ref(),
                extractor:
                    |row: &postgres::Row| -> Result<super::BookAuthorBorrowed, postgres::Error> {
                        Ok(super::BookAuthorBorrowed {
                            id: row.try_get(0)?,
                            name: row.try_get(1)?,
                            bio: row.try_get(2)?,
                        })
                    },
                mapper: |it| super::BookAuthor::from(it),
            }
        }
    }
    pub struct GetAuthorByNameStmt(&'static str, Option<postgres::Statement>);
    pub fn get_author_by_name() -> GetAuthorByNameStmt {
        GetAuthorByNameStmt(
            "SELECT id, name, bio FROM book_authors WHERE name = $1",
            None,
        )
    }
    impl GetAuthorByNameStmt {
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
        ) -> BookAuthor2Query<'c, 'a, 's, C, super::BookAuthor2, 1> {
            BookAuthor2Query {
                client,
                params: [name],
                query: self.0,
                cached: self.1.as_ref(),
                extractor:
                    |row: &postgres::Row| -> Result<super::BookAuthor2Borrowed, postgres::Error> {
                        Ok(super::BookAuthor2Borrowed {
                            id: row.try_get(0)?,
                            name: row.try_get(1)?,
                            bio: row.try_get(2)?,
                        })
                    },
                mapper: |it| super::BookAuthor2::from(it),
            }
        }
    }
}
pub mod async_ {
    use crate::client::async_::GenericClient;
    use futures::{self, StreamExt, TryStreamExt};
    pub struct BookAuthorQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor:
            fn(&tokio_postgres::Row) -> Result<super::BookAuthorBorrowed, tokio_postgres::Error>,
        mapper: fn(super::BookAuthorBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> BookAuthorQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::BookAuthorBorrowed) -> R,
        ) -> BookAuthorQuery<'c, 'a, 's, C, R, N> {
            BookAuthorQuery {
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
    pub struct BookAuthor2Query<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor:
            fn(&tokio_postgres::Row) -> Result<super::BookAuthor2Borrowed, tokio_postgres::Error>,
        mapper: fn(super::BookAuthor2Borrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> BookAuthor2Query<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::BookAuthor2Borrowed) -> R,
        ) -> BookAuthor2Query<'c, 'a, 's, C, R, N> {
            BookAuthor2Query {
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
    pub struct GetAuthorByIdStmt(&'static str, Option<tokio_postgres::Statement>);
    #[deprecated = "Use get_author_v2 instead"]
    #[allow(dead_code)]
    pub fn get_author_by_id() -> GetAuthorByIdStmt {
        GetAuthorByIdStmt("SELECT id, name, bio FROM book_authors WHERE id = $1", None)
    }
    impl GetAuthorByIdStmt {
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
            id: &'a i32,
        ) -> BookAuthorQuery<'c, 'a, 's, C, super::BookAuthor, 1> {
            BookAuthorQuery {
                client,
                params: [id],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |
                    row: &tokio_postgres::Row,
                | -> Result<super::BookAuthorBorrowed, tokio_postgres::Error> {
                    Ok(super::BookAuthorBorrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        bio: row.try_get(2)?,
                    })
                },
                mapper: |it| super::BookAuthor::from(it),
            }
        }
    }
    pub struct GetAuthorByNameStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn get_author_by_name() -> GetAuthorByNameStmt {
        GetAuthorByNameStmt(
            "SELECT id, name, bio FROM book_authors WHERE name = $1",
            None,
        )
    }
    impl GetAuthorByNameStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
            &'s self,
            client: &'c C,
            name: &'a T1,
        ) -> BookAuthor2Query<'c, 'a, 's, C, super::BookAuthor2, 1> {
            BookAuthor2Query {
                client,
                params: [name],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |
                    row: &tokio_postgres::Row,
                | -> Result<super::BookAuthor2Borrowed, tokio_postgres::Error> {
                    Ok(super::BookAuthor2Borrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        bio: row.try_get(2)?,
                    })
                },
                mapper: |it| super::BookAuthor2::from(it),
            }
        }
    }
}
