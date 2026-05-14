// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct NamedParams<T1: crate::StringSql> {
    pub name: T1,
    pub price: Option<f64>,
}
#[derive(Debug)]
pub struct NamedComplexParams<'a> {
    pub named: crate::types::NamedCompositeBorrowed<'a>,
    pub named_with_dot: Option<crate::types::NamedCompositeWithDot>,
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Id {
    pub id: i32,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Named {
    pub id: i32,
    pub name: String,
    pub price: Option<f64>,
    pub show: bool,
}
pub struct NamedBorrowed<'a> {
    pub id: i32,
    pub name: &'a str,
    pub price: Option<f64>,
    pub show: bool,
}
impl<'a> From<NamedBorrowed<'a>> for Named {
    fn from(
        NamedBorrowed {
            id,
            name,
            price,
            show,
        }: NamedBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            price,
            show,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct NamedComplex {
    pub named: crate::types::NamedComposite,
    pub named_with_dot: Option<crate::types::NamedCompositeWithDot>,
}
pub struct NamedComplexBorrowed<'a> {
    pub named: crate::types::NamedCompositeBorrowed<'a>,
    pub named_with_dot: Option<crate::types::NamedCompositeWithDot>,
}
impl<'a> From<NamedComplexBorrowed<'a>> for NamedComplex {
    fn from(
        NamedComplexBorrowed {
            named,
            named_with_dot,
        }: NamedComplexBorrowed<'a>,
    ) -> Self {
        Self {
            named: named.into(),
            named_with_dot,
        }
    }
}
pub mod sync {
    use crate::client::sync::GenericClient;
    use postgres::fallible_iterator::FallibleIterator;
    pub struct IdQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor: fn(&postgres::Row) -> Result<super::Id, postgres::Error>,
        mapper: fn(super::Id) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> IdQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::Id) -> R) -> IdQuery<'c, 'a, 's, C, R, N> {
            IdQuery {
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
    pub struct NamedQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor: fn(&postgres::Row) -> Result<super::NamedBorrowed, postgres::Error>,
        mapper: fn(super::NamedBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> NamedQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::NamedBorrowed) -> R,
        ) -> NamedQuery<'c, 'a, 's, C, R, N> {
            NamedQuery {
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
    pub struct NamedComplexQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor: fn(&postgres::Row) -> Result<super::NamedComplexBorrowed, postgres::Error>,
        mapper: fn(super::NamedComplexBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> NamedComplexQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::NamedComplexBorrowed) -> R,
        ) -> NamedComplexQuery<'c, 'a, 's, C, R, N> {
            NamedComplexQuery {
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
    pub struct NewNamedVisibleStmt(&'static str, Option<postgres::Statement>);
    pub fn new_named_visible() -> NewNamedVisibleStmt {
        NewNamedVisibleStmt(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, true) RETURNING id",
            None,
        )
    }
    impl NewNamedVisibleStmt {
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
            price: &'a Option<f64>,
        ) -> IdQuery<'c, 'a, 's, C, super::Id, 2> {
            IdQuery {
                client,
                params: [name, price],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row: &postgres::Row| -> Result<super::Id, postgres::Error> {
                    Ok(super::Id {
                        id: row.try_get(0)?,
                    })
                },
                mapper: |it| super::Id::from(it),
            }
        }
    }
    impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::NamedParams<T1>,
            IdQuery<'c, 'a, 's, C, super::Id, 2>,
            C,
        > for NewNamedVisibleStmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::NamedParams<T1>,
        ) -> IdQuery<'c, 'a, 's, C, super::Id, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub struct NewNamedHiddenStmt(&'static str, Option<postgres::Statement>);
    pub fn new_named_hidden() -> NewNamedHiddenStmt {
        NewNamedHiddenStmt(
            "INSERT INTO named (price, name, show) VALUES ($1, $2, false) RETURNING id",
            None,
        )
    }
    impl NewNamedHiddenStmt {
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
            price: &'a Option<f64>,
            name: &'a T1,
        ) -> IdQuery<'c, 'a, 's, C, super::Id, 2> {
            IdQuery {
                client,
                params: [price, name],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row: &postgres::Row| -> Result<super::Id, postgres::Error> {
                    Ok(super::Id {
                        id: row.try_get(0)?,
                    })
                },
                mapper: |it| super::Id::from(it),
            }
        }
    }
    impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::NamedParams<T1>,
            IdQuery<'c, 'a, 's, C, super::Id, 2>,
            C,
        > for NewNamedHiddenStmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::NamedParams<T1>,
        ) -> IdQuery<'c, 'a, 's, C, super::Id, 2> {
            self.bind(client, &params.price, &params.name)
        }
    }
    pub struct NamedStmt(&'static str, Option<postgres::Statement>);
    pub fn named() -> NamedStmt {
        NamedStmt("SELECT * FROM named", None)
    }
    impl NamedStmt {
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
        ) -> NamedQuery<'c, 'a, 's, C, super::Named, 0> {
            NamedQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row: &postgres::Row| -> Result<super::NamedBorrowed, postgres::Error> {
                    Ok(super::NamedBorrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        price: row.try_get(2)?,
                        show: row.try_get(3)?,
                    })
                },
                mapper: |it| super::Named::from(it),
            }
        }
    }
    pub struct NamedByIdStmt(&'static str, Option<postgres::Statement>);
    pub fn named_by_id() -> NamedByIdStmt {
        NamedByIdStmt("SELECT * FROM named WHERE id = $1", None)
    }
    impl NamedByIdStmt {
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
        ) -> NamedQuery<'c, 'a, 's, C, super::Named, 1> {
            NamedQuery {
                client,
                params: [id],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row: &postgres::Row| -> Result<super::NamedBorrowed, postgres::Error> {
                    Ok(super::NamedBorrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        price: row.try_get(2)?,
                        show: row.try_get(3)?,
                    })
                },
                mapper: |it| super::Named::from(it),
            }
        }
    }
    pub struct NewNamedComplexStmt(&'static str, Option<postgres::Statement>);
    pub fn new_named_complex() -> NewNamedComplexStmt {
        NewNamedComplexStmt(
            "INSERT INTO named_complex (named, \"named.with_dot\") VALUES ($1, $2)",
            None,
        )
    }
    impl NewNamedComplexStmt {
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
            named: &'a crate::types::NamedCompositeBorrowed<'a>,
            named_with_dot: &'a Option<crate::types::NamedCompositeWithDot>,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[named, named_with_dot])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::NamedComplexParams<'a>,
            Result<u64, postgres::Error>,
            C,
        > for NewNamedComplexStmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::NamedComplexParams<'a>,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.named, &params.named_with_dot)
        }
    }
    pub struct NamedComplexStmt(&'static str, Option<postgres::Statement>);
    pub fn named_complex() -> NamedComplexStmt {
        NamedComplexStmt("SELECT * FROM named_complex", None)
    }
    impl NamedComplexStmt {
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
        ) -> NamedComplexQuery<'c, 'a, 's, C, super::NamedComplex, 0> {
            NamedComplexQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor:
                    |row: &postgres::Row| -> Result<super::NamedComplexBorrowed, postgres::Error> {
                        Ok(super::NamedComplexBorrowed {
                            named: row.try_get(0)?,
                            named_with_dot: row.try_get(1)?,
                        })
                    },
                mapper: |it| super::NamedComplex::from(it),
            }
        }
    }
}
pub mod async_ {
    use crate::client::async_::GenericClient;
    use futures::{self, StreamExt, TryStreamExt};
    pub struct IdQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor: fn(&tokio_postgres::Row) -> Result<super::Id, tokio_postgres::Error>,
        mapper: fn(super::Id) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> IdQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::Id) -> R) -> IdQuery<'c, 'a, 's, C, R, N> {
            IdQuery {
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
    pub struct NamedQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor: fn(&tokio_postgres::Row) -> Result<super::NamedBorrowed, tokio_postgres::Error>,
        mapper: fn(super::NamedBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> NamedQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::NamedBorrowed) -> R,
        ) -> NamedQuery<'c, 'a, 's, C, R, N> {
            NamedQuery {
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
    pub struct NamedComplexQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor:
            fn(&tokio_postgres::Row) -> Result<super::NamedComplexBorrowed, tokio_postgres::Error>,
        mapper: fn(super::NamedComplexBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> NamedComplexQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::NamedComplexBorrowed) -> R,
        ) -> NamedComplexQuery<'c, 'a, 's, C, R, N> {
            NamedComplexQuery {
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
    pub struct NewNamedVisibleStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn new_named_visible() -> NewNamedVisibleStmt {
        NewNamedVisibleStmt(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, true) RETURNING id",
            None,
        )
    }
    impl NewNamedVisibleStmt {
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
            price: &'a Option<f64>,
        ) -> IdQuery<'c, 'a, 's, C, super::Id, 2> {
            IdQuery {
                client,
                params: [name, price],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row: &tokio_postgres::Row| -> Result<super::Id, tokio_postgres::Error> {
                    Ok(super::Id {
                        id: row.try_get(0)?,
                    })
                },
                mapper: |it| super::Id::from(it),
            }
        }
    }
    impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
        crate::client::async_::Params<
            'c,
            'a,
            's,
            super::NamedParams<T1>,
            IdQuery<'c, 'a, 's, C, super::Id, 2>,
            C,
        > for NewNamedVisibleStmt
    {
        fn params(
            &'s self,
            client: &'c C,
            params: &'a super::NamedParams<T1>,
        ) -> IdQuery<'c, 'a, 's, C, super::Id, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub struct NewNamedHiddenStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn new_named_hidden() -> NewNamedHiddenStmt {
        NewNamedHiddenStmt(
            "INSERT INTO named (price, name, show) VALUES ($1, $2, false) RETURNING id",
            None,
        )
    }
    impl NewNamedHiddenStmt {
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
            price: &'a Option<f64>,
            name: &'a T1,
        ) -> IdQuery<'c, 'a, 's, C, super::Id, 2> {
            IdQuery {
                client,
                params: [price, name],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row: &tokio_postgres::Row| -> Result<super::Id, tokio_postgres::Error> {
                    Ok(super::Id {
                        id: row.try_get(0)?,
                    })
                },
                mapper: |it| super::Id::from(it),
            }
        }
    }
    impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
        crate::client::async_::Params<
            'c,
            'a,
            's,
            super::NamedParams<T1>,
            IdQuery<'c, 'a, 's, C, super::Id, 2>,
            C,
        > for NewNamedHiddenStmt
    {
        fn params(
            &'s self,
            client: &'c C,
            params: &'a super::NamedParams<T1>,
        ) -> IdQuery<'c, 'a, 's, C, super::Id, 2> {
            self.bind(client, &params.price, &params.name)
        }
    }
    pub struct NamedStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn named() -> NamedStmt {
        NamedStmt("SELECT * FROM named", None)
    }
    impl NamedStmt {
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
        ) -> NamedQuery<'c, 'a, 's, C, super::Named, 0> {
            NamedQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |
                    row: &tokio_postgres::Row,
                | -> Result<super::NamedBorrowed, tokio_postgres::Error> {
                    Ok(super::NamedBorrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        price: row.try_get(2)?,
                        show: row.try_get(3)?,
                    })
                },
                mapper: |it| super::Named::from(it),
            }
        }
    }
    pub struct NamedByIdStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn named_by_id() -> NamedByIdStmt {
        NamedByIdStmt("SELECT * FROM named WHERE id = $1", None)
    }
    impl NamedByIdStmt {
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
        ) -> NamedQuery<'c, 'a, 's, C, super::Named, 1> {
            NamedQuery {
                client,
                params: [id],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |
                    row: &tokio_postgres::Row,
                | -> Result<super::NamedBorrowed, tokio_postgres::Error> {
                    Ok(super::NamedBorrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        price: row.try_get(2)?,
                        show: row.try_get(3)?,
                    })
                },
                mapper: |it| super::Named::from(it),
            }
        }
    }
    pub struct NewNamedComplexStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn new_named_complex() -> NewNamedComplexStmt {
        NewNamedComplexStmt(
            "INSERT INTO named_complex (named, \"named.with_dot\") VALUES ($1, $2)",
            None,
        )
    }
    impl NewNamedComplexStmt {
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
            named: &'a crate::types::NamedCompositeBorrowed<'a>,
            named_with_dot: &'a Option<crate::types::NamedCompositeWithDot>,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[named, named_with_dot]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::NamedComplexParams<'a>,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for NewNamedComplexStmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::NamedComplexParams<'a>,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.named, &params.named_with_dot))
        }
    }
    pub struct NamedComplexStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn named_complex() -> NamedComplexStmt {
        NamedComplexStmt("SELECT * FROM named_complex", None)
    }
    impl NamedComplexStmt {
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
        ) -> NamedComplexQuery<'c, 'a, 's, C, super::NamedComplex, 0> {
            NamedComplexQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |
                    row: &tokio_postgres::Row,
                | -> Result<super::NamedComplexBorrowed, tokio_postgres::Error> {
                    Ok(super::NamedComplexBorrowed {
                        named: row.try_get(0)?,
                        named_with_dot: row.try_get(1)?,
                    })
                },
                mapper: |it| super::NamedComplex::from(it),
            }
        }
    }
}
