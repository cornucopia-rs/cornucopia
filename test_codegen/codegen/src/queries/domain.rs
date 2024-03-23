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
#[derive(serde::Serialize, Debug, Clone, PartialEq)]
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
#[derive(serde::Serialize, Debug, Clone, PartialEq)]
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
    use postgres::{fallible_iterator::FallibleIterator, GenericClient};
    pub struct SelectNightmareDomainQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> super::SelectNightmareDomainBorrowed,
        mapper: fn(super::SelectNightmareDomainBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> SelectNightmareDomainQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectNightmareDomainBorrowed) -> R,
        ) -> SelectNightmareDomainQuery<'a, C, R, N> {
            SelectNightmareDomainQuery {
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
    pub struct SelectNightmareDomainNullQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> super::SelectNightmareDomainNullBorrowed,
        mapper: fn(super::SelectNightmareDomainNullBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> SelectNightmareDomainNullQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectNightmareDomainNullBorrowed) -> R,
        ) -> SelectNightmareDomainNullQuery<'a, C, R, N> {
            SelectNightmareDomainNullQuery {
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
    pub fn select_nightmare_domain() -> SelectNightmareDomainStmt {
        SelectNightmareDomainStmt(crate::client::sync::Stmt::new(
            "SELECT txt, json, nb, arr FROM nightmare_domain",
        ))
    }
    pub struct SelectNightmareDomainStmt(crate::client::sync::Stmt);
    impl SelectNightmareDomainStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
        ) -> SelectNightmareDomainQuery<'a, C, super::SelectNightmareDomain, 0> {
            SelectNightmareDomainQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| super::SelectNightmareDomainBorrowed {
                    txt: row.get(0),
                    json: row.get(1),
                    nb: row.get(2),
                    arr: row.get(3),
                },
                mapper: |it| <super::SelectNightmareDomain>::from(it),
            }
        }
    }
    pub fn insert_nightmare_domain() -> InsertNightmareDomainStmt {
        InsertNightmareDomainStmt(crate::client::sync::Stmt::new("INSERT INTO nightmare_domain (txt, json, nb, arr, composite) VALUES ($1, $2, $3, $4, $5)"))
    }
    pub struct InsertNightmareDomainStmt(crate::client::sync::Stmt);
    impl InsertNightmareDomainStmt {
        pub fn bind<
            'a,
            C: GenericClient,
            T1: crate::StringSql,
            T2: crate::JsonSql,
            T3: crate::JsonSql,
            T4: crate::ArraySql<Item = T3>,
        >(
            &'a mut self,
            client: &'a mut C,
            txt: &'a T1,
            json: &'a T2,
            nb: &'a i32,
            arr: &'a T4,
            composite: &'a Option<crate::types::DomainCompositeParams<'a>>,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(
                stmt,
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
            'a,
            C: GenericClient,
            T1: crate::StringSql,
            T2: crate::JsonSql,
            T3: crate::JsonSql,
            T4: crate::ArraySql<Item = T3>,
        >
        crate::client::sync::Params<
            'a,
            super::InsertNightmareDomainParams<'a, T1, T2, T3, T4>,
            Result<u64, postgres::Error>,
            C,
        > for InsertNightmareDomainStmt
    {
        fn params(
            &'a mut self,
            client: &'a mut C,
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
    pub fn select_nightmare_domain_null() -> SelectNightmareDomainNullStmt {
        SelectNightmareDomainNullStmt(crate::client::sync::Stmt::new(
            "SELECT * FROM nightmare_domain",
        ))
    }
    pub struct SelectNightmareDomainNullStmt(crate::client::sync::Stmt);
    impl SelectNightmareDomainNullStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
        ) -> SelectNightmareDomainNullQuery<'a, C, super::SelectNightmareDomainNull, 0> {
            SelectNightmareDomainNullQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| super::SelectNightmareDomainNullBorrowed {
                    txt: row.get(0),
                    json: row.get(1),
                    nb: row.get(2),
                    arr: row.get(3),
                    composite: row.get(4),
                },
                mapper: |it| <super::SelectNightmareDomainNull>::from(it),
            }
        }
    }
}
pub mod async_ {
    use crate::client::async_::GenericClient;
    use futures::{self, StreamExt, TryStreamExt};
    pub struct SelectNightmareDomainQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::async_::Stmt,
        extractor: fn(&tokio_postgres::Row) -> super::SelectNightmareDomainBorrowed,
        mapper: fn(super::SelectNightmareDomainBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> SelectNightmareDomainQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectNightmareDomainBorrowed) -> R,
        ) -> SelectNightmareDomainQuery<'a, C, R, N> {
            SelectNightmareDomainQuery {
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
    pub struct SelectNightmareDomainNullQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::async_::Stmt,
        extractor: fn(&tokio_postgres::Row) -> super::SelectNightmareDomainNullBorrowed,
        mapper: fn(super::SelectNightmareDomainNullBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> SelectNightmareDomainNullQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectNightmareDomainNullBorrowed) -> R,
        ) -> SelectNightmareDomainNullQuery<'a, C, R, N> {
            SelectNightmareDomainNullQuery {
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
    pub fn select_nightmare_domain() -> SelectNightmareDomainStmt {
        SelectNightmareDomainStmt(crate::client::async_::Stmt::new(
            "SELECT txt, json, nb, arr FROM nightmare_domain",
        ))
    }
    pub struct SelectNightmareDomainStmt(crate::client::async_::Stmt);
    impl SelectNightmareDomainStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
        ) -> SelectNightmareDomainQuery<'a, C, super::SelectNightmareDomain, 0> {
            SelectNightmareDomainQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| super::SelectNightmareDomainBorrowed {
                    txt: row.get(0),
                    json: row.get(1),
                    nb: row.get(2),
                    arr: row.get(3),
                },
                mapper: |it| <super::SelectNightmareDomain>::from(it),
            }
        }
    }
    pub fn insert_nightmare_domain() -> InsertNightmareDomainStmt {
        InsertNightmareDomainStmt(crate::client::async_::Stmt::new("INSERT INTO nightmare_domain (txt, json, nb, arr, composite) VALUES ($1, $2, $3, $4, $5)"))
    }
    pub struct InsertNightmareDomainStmt(crate::client::async_::Stmt);
    impl InsertNightmareDomainStmt {
        pub async fn bind<
            'a,
            C: GenericClient,
            T1: crate::StringSql,
            T2: crate::JsonSql,
            T3: crate::JsonSql,
            T4: crate::ArraySql<Item = T3>,
        >(
            &'a mut self,
            client: &'a C,
            txt: &'a T1,
            json: &'a T2,
            nb: &'a i32,
            arr: &'a T4,
            composite: &'a Option<crate::types::DomainCompositeParams<'a>>,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client
                .execute(
                    stmt,
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
            super::InsertNightmareDomainParams<'a, T1, T2, T3, T4>,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for InsertNightmareDomainStmt
    {
        fn params(
            &'a mut self,
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
    pub fn select_nightmare_domain_null() -> SelectNightmareDomainNullStmt {
        SelectNightmareDomainNullStmt(crate::client::async_::Stmt::new(
            "SELECT * FROM nightmare_domain",
        ))
    }
    pub struct SelectNightmareDomainNullStmt(crate::client::async_::Stmt);
    impl SelectNightmareDomainNullStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
        ) -> SelectNightmareDomainNullQuery<'a, C, super::SelectNightmareDomainNull, 0> {
            SelectNightmareDomainNullQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| super::SelectNightmareDomainNullBorrowed {
                    txt: row.get(0),
                    json: row.get(1),
                    nb: row.get(2),
                    arr: row.get(3),
                    composite: row.get(4),
                },
                mapper: |it| <super::SelectNightmareDomainNull>::from(it),
            }
        }
    }
}
