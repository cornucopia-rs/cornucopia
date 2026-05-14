// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct InsertNightmareDomainParams<
    'a,
    T1: crate::StringSql,
    T2: crate::JsonSql,
    T3: crate::JsonSql,
    T4: crate::ArraySql<Item = T3>,
> {
    pub txt: T1,
    pub json: T2,
    pub nb: i32,
    pub arr: T4,
    pub composite: Option<crate::types::DomainCompositeParams<'a>>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct SelectNightmareDomain {
    pub txt: String,
    pub json: serde_json::Value,
    pub nb: i32,
    pub arr: Vec<serde_json::Value>,
}
pub struct SelectNightmareDomainBorrowed<'a> {
    pub txt: &'a str,
    pub json: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub nb: i32,
    pub arr: crate::ArrayIterator<'a, postgres_types::Json<&'a serde_json::value::RawValue>>,
}
impl<'a> From<SelectNightmareDomainBorrowed<'a>> for SelectNightmareDomain {
    fn from(
        SelectNightmareDomainBorrowed { txt, json, nb, arr }: SelectNightmareDomainBorrowed<'a>,
    ) -> Self {
        Self {
            txt: txt.into(),
            json: serde_json::from_str(json.0.get()).unwrap(),
            nb,
            arr: arr
                .map(|v| serde_json::from_str(v.0.get()).unwrap())
                .collect(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SelectNightmareDomainNull {
    pub txt: Option<String>,
    pub json: Option<serde_json::Value>,
    pub nb: Option<i32>,
    pub arr: Option<Vec<Option<serde_json::Value>>>,
    pub composite: Option<crate::types::DomainComposite>,
}
pub struct SelectNightmareDomainNullBorrowed<'a> {
    pub txt: Option<&'a str>,
    pub json: Option<postgres_types::Json<&'a serde_json::value::RawValue>>,
    pub nb: Option<i32>,
    pub arr: Option<
        crate::ArrayIterator<'a, Option<postgres_types::Json<&'a serde_json::value::RawValue>>>,
    >,
    pub composite: Option<crate::types::DomainCompositeBorrowed<'a>>,
}
impl<'a> From<SelectNightmareDomainNullBorrowed<'a>> for SelectNightmareDomainNull {
    fn from(
        SelectNightmareDomainNullBorrowed {
            txt,
            json,
            nb,
            arr,
            composite,
        }: SelectNightmareDomainNullBorrowed<'a>,
    ) -> Self {
        Self {
            txt: txt.map(|v| v.into()),
            json: json.map(|v| serde_json::from_str(v.0.get()).unwrap()),
            nb,
            arr: arr.map(|v| {
                v.map(|v| v.map(|v| serde_json::from_str(v.0.get()).unwrap()))
                    .collect()
            }),
            composite: composite.map(|v| v.into()),
        }
    }
}
pub mod sync {
    use crate::client::sync::GenericClient;
    use postgres::fallible_iterator::FallibleIterator;
    pub struct SelectNightmareDomainQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor:
            fn(&postgres::Row) -> Result<super::SelectNightmareDomainBorrowed, postgres::Error>,
        mapper: fn(super::SelectNightmareDomainBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> SelectNightmareDomainQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectNightmareDomainBorrowed) -> R,
        ) -> SelectNightmareDomainQuery<'c, 'a, 's, C, R, N> {
            SelectNightmareDomainQuery {
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
    pub struct SelectNightmareDomainNullQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor:
            fn(&postgres::Row) -> Result<super::SelectNightmareDomainNullBorrowed, postgres::Error>,
        mapper: fn(super::SelectNightmareDomainNullBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> SelectNightmareDomainNullQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectNightmareDomainNullBorrowed) -> R,
        ) -> SelectNightmareDomainNullQuery<'c, 'a, 's, C, R, N> {
            SelectNightmareDomainNullQuery {
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
    pub struct SelectNightmareDomainStmt(&'static str, Option<postgres::Statement>);
    pub fn select_nightmare_domain() -> SelectNightmareDomainStmt {
        SelectNightmareDomainStmt("SELECT txt, json, nb, arr FROM nightmare_domain", None)
    }
    impl SelectNightmareDomainStmt {
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
        ) -> SelectNightmareDomainQuery<'c, 'a, 's, C, super::SelectNightmareDomain, 0> {
            SelectNightmareDomainQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |
                    row: &postgres::Row,
                | -> Result<super::SelectNightmareDomainBorrowed, postgres::Error> {
                    Ok(super::SelectNightmareDomainBorrowed {
                        txt: row.try_get(0)?,
                        json: row.try_get(1)?,
                        nb: row.try_get(2)?,
                        arr: row.try_get(3)?,
                    })
                },
                mapper: |it| super::SelectNightmareDomain::from(it),
            }
        }
    }
    pub struct InsertNightmareDomainStmt(&'static str, Option<postgres::Statement>);
    pub fn insert_nightmare_domain() -> InsertNightmareDomainStmt {
        InsertNightmareDomainStmt(
            "INSERT INTO nightmare_domain (txt, json, nb, arr, composite) VALUES ($1, $2, $3, $4, $5)",
            None,
        )
    }
    impl InsertNightmareDomainStmt {
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
            T2: crate::JsonSql,
            T3: crate::JsonSql,
            T4: crate::ArraySql<Item = T3>,
        >(
            &'s self,
            client: &'c mut C,
            txt: &'a T1,
            json: &'a T2,
            nb: &'a i32,
            arr: &'a T4,
            composite: &'a Option<crate::types::DomainCompositeParams<'a>>,
        ) -> Result<u64, postgres::Error> {
            client.execute(
                self.0,
                &[
                    &crate::Domain(txt),
                    &crate::Domain(json),
                    &crate::Domain(nb),
                    &crate::Domain(&crate::DomainArray(arr)),
                    composite,
                ],
            )
        }
    }
    impl<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::JsonSql,
        T3: crate::JsonSql,
        T4: crate::ArraySql<Item = T3>,
    >
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::InsertNightmareDomainParams<'a, T1, T2, T3, T4>,
            Result<u64, postgres::Error>,
            C,
        > for InsertNightmareDomainStmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::InsertNightmareDomainParams<'a, T1, T2, T3, T4>,
        ) -> Result<u64, postgres::Error> {
            self.bind(
                client,
                &params.txt,
                &params.json,
                &params.nb,
                &params.arr,
                &params.composite,
            )
        }
    }
    pub struct SelectNightmareDomainNullStmt(&'static str, Option<postgres::Statement>);
    pub fn select_nightmare_domain_null() -> SelectNightmareDomainNullStmt {
        SelectNightmareDomainNullStmt("SELECT * FROM nightmare_domain", None)
    }
    impl SelectNightmareDomainNullStmt {
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
        ) -> SelectNightmareDomainNullQuery<'c, 'a, 's, C, super::SelectNightmareDomainNull, 0>
        {
            SelectNightmareDomainNullQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row: &postgres::Row| -> Result<
                    super::SelectNightmareDomainNullBorrowed,
                    postgres::Error,
                > {
                    Ok(super::SelectNightmareDomainNullBorrowed {
                        txt: row.try_get(0)?,
                        json: row.try_get(1)?,
                        nb: row.try_get(2)?,
                        arr: row.try_get(3)?,
                        composite: row.try_get(4)?,
                    })
                },
                mapper: |it| super::SelectNightmareDomainNull::from(it),
            }
        }
    }
}
pub mod async_ {
    use crate::client::async_::GenericClient;
    use futures::{self, StreamExt, TryStreamExt};
    pub struct SelectNightmareDomainQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor: fn(
            &tokio_postgres::Row,
        )
            -> Result<super::SelectNightmareDomainBorrowed, tokio_postgres::Error>,
        mapper: fn(super::SelectNightmareDomainBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> SelectNightmareDomainQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectNightmareDomainBorrowed) -> R,
        ) -> SelectNightmareDomainQuery<'c, 'a, 's, C, R, N> {
            SelectNightmareDomainQuery {
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
    pub struct SelectNightmareDomainNullQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor: fn(
            &tokio_postgres::Row,
        )
            -> Result<super::SelectNightmareDomainNullBorrowed, tokio_postgres::Error>,
        mapper: fn(super::SelectNightmareDomainNullBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> SelectNightmareDomainNullQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectNightmareDomainNullBorrowed) -> R,
        ) -> SelectNightmareDomainNullQuery<'c, 'a, 's, C, R, N> {
            SelectNightmareDomainNullQuery {
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
    pub struct SelectNightmareDomainStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn select_nightmare_domain() -> SelectNightmareDomainStmt {
        SelectNightmareDomainStmt("SELECT txt, json, nb, arr FROM nightmare_domain", None)
    }
    impl SelectNightmareDomainStmt {
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
        ) -> SelectNightmareDomainQuery<'c, 'a, 's, C, super::SelectNightmareDomain, 0> {
            SelectNightmareDomainQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row: &tokio_postgres::Row| -> Result<
                    super::SelectNightmareDomainBorrowed,
                    tokio_postgres::Error,
                > {
                    Ok(super::SelectNightmareDomainBorrowed {
                        txt: row.try_get(0)?,
                        json: row.try_get(1)?,
                        nb: row.try_get(2)?,
                        arr: row.try_get(3)?,
                    })
                },
                mapper: |it| super::SelectNightmareDomain::from(it),
            }
        }
    }
    pub struct InsertNightmareDomainStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn insert_nightmare_domain() -> InsertNightmareDomainStmt {
        InsertNightmareDomainStmt(
            "INSERT INTO nightmare_domain (txt, json, nb, arr, composite) VALUES ($1, $2, $3, $4, $5)",
            None,
        )
    }
    impl InsertNightmareDomainStmt {
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
            T2: crate::JsonSql,
            T3: crate::JsonSql,
            T4: crate::ArraySql<Item = T3>,
        >(
            &'s self,
            client: &'c C,
            txt: &'a T1,
            json: &'a T2,
            nb: &'a i32,
            arr: &'a T4,
            composite: &'a Option<crate::types::DomainCompositeParams<'a>>,
        ) -> Result<u64, tokio_postgres::Error> {
            client
                .execute(
                    self.0,
                    &[
                        &crate::Domain(txt),
                        &crate::Domain(json),
                        &crate::Domain(nb),
                        &crate::Domain(&crate::DomainArray(arr)),
                        composite,
                    ],
                )
                .await
        }
    }
    impl<
        'a,
        C: GenericClient + Send + Sync,
        T1: crate::StringSql,
        T2: crate::JsonSql,
        T3: crate::JsonSql,
        T4: crate::ArraySql<Item = T3>,
    >
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::InsertNightmareDomainParams<'a, T1, T2, T3, T4>,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for InsertNightmareDomainStmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::InsertNightmareDomainParams<'a, T1, T2, T3, T4>,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(
                client,
                &params.txt,
                &params.json,
                &params.nb,
                &params.arr,
                &params.composite,
            ))
        }
    }
    pub struct SelectNightmareDomainNullStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn select_nightmare_domain_null() -> SelectNightmareDomainNullStmt {
        SelectNightmareDomainNullStmt("SELECT * FROM nightmare_domain", None)
    }
    impl SelectNightmareDomainNullStmt {
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
        ) -> SelectNightmareDomainNullQuery<'c, 'a, 's, C, super::SelectNightmareDomainNull, 0>
        {
            SelectNightmareDomainNullQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row: &tokio_postgres::Row| -> Result<
                    super::SelectNightmareDomainNullBorrowed,
                    tokio_postgres::Error,
                > {
                    Ok(super::SelectNightmareDomainNullBorrowed {
                        txt: row.try_get(0)?,
                        json: row.try_get(1)?,
                        nb: row.try_get(2)?,
                        arr: row.try_get(3)?,
                        composite: row.try_get(4)?,
                    })
                },
                mapper: |it| super::SelectNightmareDomainNull::from(it),
            }
        }
    }
}
