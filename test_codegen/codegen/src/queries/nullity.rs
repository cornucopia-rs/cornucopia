// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct NullityParams<
    'a,
    T1: crate::StringSql,
    T2: crate::ArraySql<Item = Option<T1>>,
    T3: crate::StringSql,
> {
    pub texts: T2,
    pub name: T3,
    pub composite: Option<crate::types::NullityCompositeParams<'a>>,
}
#[derive(serde::Serialize, Debug, Clone, PartialEq)]
pub struct Nullity {
    pub texts: Vec<Option<String>>,
    pub name: String,
    pub composite: Option<crate::types::NullityComposite>,
}
pub struct NullityBorrowed<'a> {
    pub texts: crate::ArrayIterator<'a, Option<&'a str>>,
    pub name: &'a str,
    pub composite: Option<crate::types::NullityCompositeBorrowed<'a>>,
}
impl<'a> From<NullityBorrowed<'a>> for Nullity {
    fn from(
        NullityBorrowed {
            texts,
            name,
            composite,
        }: NullityBorrowed<'a>,
    ) -> Self {
        Self {
            texts: texts.map(|v| v.map(|v| v.into())).collect(),
            name: name.into(),
            composite: composite.map(|v| v.into()),
        }
    }
}
pub mod sync {
    use postgres::{fallible_iterator::FallibleIterator, GenericClient};
    pub struct NullityQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> super::NullityBorrowed,
        mapper: fn(super::NullityBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> NullityQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::NullityBorrowed) -> R) -> NullityQuery<'a, C, R, N> {
            NullityQuery {
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
    pub fn new_nullity() -> NewNullityStmt {
        NewNullityStmt(crate::client::sync::Stmt::new(
            "INSERT INTO nullity(texts, name, composite) VALUES ($1, $2, $3)",
        ))
    }
    pub struct NewNullityStmt(crate::client::sync::Stmt);
    impl NewNullityStmt {
        pub fn bind<
            'a,
            C: GenericClient,
            T1: crate::StringSql,
            T2: crate::ArraySql<Item = Option<T1>>,
            T3: crate::StringSql,
        >(
            &'a mut self,
            client: &'a mut C,
            texts: &'a T2,
            name: &'a T3,
            composite: &'a Option<crate::types::NullityCompositeParams<'a>>,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[texts, name, composite])
        }
    }
    impl<
            'a,
            C: GenericClient,
            T1: crate::StringSql,
            T2: crate::ArraySql<Item = Option<T1>>,
            T3: crate::StringSql,
        >
        crate::client::sync::Params<
            'a,
            super::NullityParams<'a, T1, T2, T3>,
            Result<u64, postgres::Error>,
            C,
        > for NewNullityStmt
    {
        fn params(
            &'a mut self,
            client: &'a mut C,
            params: &'a super::NullityParams<'a, T1, T2, T3>,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.texts, &params.name, &params.composite)
        }
    }
    pub fn nullity() -> NullityStmt {
        NullityStmt(crate::client::sync::Stmt::new("SELECT * FROM nullity"))
    }
    pub struct NullityStmt(crate::client::sync::Stmt);
    impl NullityStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
        ) -> NullityQuery<'a, C, super::Nullity, 0> {
            NullityQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| super::NullityBorrowed {
                    texts: row.get(0),
                    name: row.get(1),
                    composite: row.get(2),
                },
                mapper: |it| <super::Nullity>::from(it),
            }
        }
    }
}
pub mod async_ {
    use crate::client::async_::GenericClient;
    use futures::{self, StreamExt, TryStreamExt};
    pub struct NullityQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::async_::Stmt,
        extractor: fn(&tokio_postgres::Row) -> super::NullityBorrowed,
        mapper: fn(super::NullityBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> NullityQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::NullityBorrowed) -> R) -> NullityQuery<'a, C, R, N> {
            NullityQuery {
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
    pub fn new_nullity() -> NewNullityStmt {
        NewNullityStmt(crate::client::async_::Stmt::new(
            "INSERT INTO nullity(texts, name, composite) VALUES ($1, $2, $3)",
        ))
    }
    pub struct NewNullityStmt(crate::client::async_::Stmt);
    impl NewNullityStmt {
        pub async fn bind<
            'a,
            C: GenericClient,
            T1: crate::StringSql,
            T2: crate::ArraySql<Item = Option<T1>>,
            T3: crate::StringSql,
        >(
            &'a mut self,
            client: &'a C,
            texts: &'a T2,
            name: &'a T3,
            composite: &'a Option<crate::types::NullityCompositeParams<'a>>,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[texts, name, composite]).await
        }
    }
    impl<
            'a,
            C: GenericClient + Send + Sync,
            T1: crate::StringSql,
            T2: crate::ArraySql<Item = Option<T1>>,
            T3: crate::StringSql,
        >
        crate::client::async_::Params<
            'a,
            super::NullityParams<'a, T1, T2, T3>,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for NewNullityStmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::NullityParams<'a, T1, T2, T3>,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.texts, &params.name, &params.composite))
        }
    }
    pub fn nullity() -> NullityStmt {
        NullityStmt(crate::client::async_::Stmt::new("SELECT * FROM nullity"))
    }
    pub struct NullityStmt(crate::client::async_::Stmt);
    impl NullityStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
        ) -> NullityQuery<'a, C, super::Nullity, 0> {
            NullityQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| super::NullityBorrowed {
                    texts: row.get(0),
                    name: row.get(1),
                    composite: row.get(2),
                },
                mapper: |it| <super::Nullity>::from(it),
            }
        }
    }
}
