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
#[derive(serde::Serialize, Debug, Clone, PartialEq, Copy)]
pub struct Id {
    pub id: i32,
}
#[derive(serde::Serialize, Debug, Clone, PartialEq)]
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
#[derive(serde::Serialize, Debug, Clone, PartialEq)]
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
    use postgres::{fallible_iterator::FallibleIterator, GenericClient};
    pub struct IdQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> super::Id,
        mapper: fn(super::Id) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> IdQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::Id) -> R) -> IdQuery<'a, C, R, N> {
            IdQuery {
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
    pub struct NamedQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> super::NamedBorrowed,
        mapper: fn(super::NamedBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> NamedQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::NamedBorrowed) -> R) -> NamedQuery<'a, C, R, N> {
            NamedQuery {
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
    pub struct NamedComplexQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> super::NamedComplexBorrowed,
        mapper: fn(super::NamedComplexBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> NamedComplexQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::NamedComplexBorrowed) -> R,
        ) -> NamedComplexQuery<'a, C, R, N> {
            NamedComplexQuery {
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
    pub fn new_named_visible() -> NewNamedVisibleStmt {
        NewNamedVisibleStmt(crate::client::sync::Stmt::new(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, true) RETURNING id ",
        ))
    }
    pub struct NewNamedVisibleStmt(crate::client::sync::Stmt);
    impl NewNamedVisibleStmt {
        pub fn bind<'a, C: GenericClient, T1: crate::StringSql>(
            &'a mut self,
            client: &'a mut C,
            name: &'a T1,
            price: &'a Option<f64>,
        ) -> IdQuery<'a, C, super::Id, 2> {
            IdQuery {
                client,
                params: [name, price],
                stmt: &mut self.0,
                extractor: |row| super::Id { id: row.get(0) },
                mapper: |it| <super::Id>::from(it),
            }
        }
    }
    impl<'a, C: GenericClient, T1: crate::StringSql>
        crate::client::sync::Params<'a, super::NamedParams<T1>, IdQuery<'a, C, super::Id, 2>, C>
        for NewNamedVisibleStmt
    {
        fn params(
            &'a mut self,
            client: &'a mut C,
            params: &'a super::NamedParams<T1>,
        ) -> IdQuery<'a, C, super::Id, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn new_named_hidden() -> NewNamedHiddenStmt {
        NewNamedHiddenStmt(crate::client::sync::Stmt::new(
            "INSERT INTO named (price, name, show) VALUES ($1, $2, false) RETURNING id",
        ))
    }
    pub struct NewNamedHiddenStmt(crate::client::sync::Stmt);
    impl NewNamedHiddenStmt {
        pub fn bind<'a, C: GenericClient, T1: crate::StringSql>(
            &'a mut self,
            client: &'a mut C,
            price: &'a Option<f64>,
            name: &'a T1,
        ) -> IdQuery<'a, C, super::Id, 2> {
            IdQuery {
                client,
                params: [price, name],
                stmt: &mut self.0,
                extractor: |row| super::Id { id: row.get(0) },
                mapper: |it| <super::Id>::from(it),
            }
        }
    }
    impl<'a, C: GenericClient, T1: crate::StringSql>
        crate::client::sync::Params<'a, super::NamedParams<T1>, IdQuery<'a, C, super::Id, 2>, C>
        for NewNamedHiddenStmt
    {
        fn params(
            &'a mut self,
            client: &'a mut C,
            params: &'a super::NamedParams<T1>,
        ) -> IdQuery<'a, C, super::Id, 2> {
            self.bind(client, &params.price, &params.name)
        }
    }
    pub fn named() -> NamedStmt {
        NamedStmt(crate::client::sync::Stmt::new("SELECT * FROM named"))
    }
    pub struct NamedStmt(crate::client::sync::Stmt);
    impl NamedStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
        ) -> NamedQuery<'a, C, super::Named, 0> {
            NamedQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| super::NamedBorrowed {
                    id: row.get(0),
                    name: row.get(1),
                    price: row.get(2),
                    show: row.get(3),
                },
                mapper: |it| <super::Named>::from(it),
            }
        }
    }
    pub fn named_by_id() -> NamedByIdStmt {
        NamedByIdStmt(crate::client::sync::Stmt::new(
            "SELECT * FROM named WHERE id = $1",
        ))
    }
    pub struct NamedByIdStmt(crate::client::sync::Stmt);
    impl NamedByIdStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
            id: &'a i32,
        ) -> NamedQuery<'a, C, super::Named, 1> {
            NamedQuery {
                client,
                params: [id],
                stmt: &mut self.0,
                extractor: |row| super::NamedBorrowed {
                    id: row.get(0),
                    name: row.get(1),
                    price: row.get(2),
                    show: row.get(3),
                },
                mapper: |it| <super::Named>::from(it),
            }
        }
    }
    pub fn new_named_complex() -> NewNamedComplexStmt {
        NewNamedComplexStmt(crate::client::sync::Stmt::new(
            "INSERT INTO named_complex (named, \"named.with_dot\") VALUES ($1, $2)",
        ))
    }
    pub struct NewNamedComplexStmt(crate::client::sync::Stmt);
    impl NewNamedComplexStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
            named: &'a crate::types::NamedCompositeBorrowed<'a>,
            named_with_dot: &'a Option<crate::types::NamedCompositeWithDot>,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[named, named_with_dot])
        }
    }
    impl<'a, C: GenericClient>
        crate::client::sync::Params<
            'a,
            super::NamedComplexParams<'a>,
            Result<u64, postgres::Error>,
            C,
        > for NewNamedComplexStmt
    {
        fn params(
            &'a mut self,
            client: &'a mut C,
            params: &'a super::NamedComplexParams<'a>,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.named, &params.named_with_dot)
        }
    }
    pub fn named_complex() -> NamedComplexStmt {
        NamedComplexStmt(crate::client::sync::Stmt::new(
            "SELECT * FROM named_complex",
        ))
    }
    pub struct NamedComplexStmt(crate::client::sync::Stmt);
    impl NamedComplexStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
        ) -> NamedComplexQuery<'a, C, super::NamedComplex, 0> {
            NamedComplexQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| super::NamedComplexBorrowed {
                    named: row.get(0),
                    named_with_dot: row.get(1),
                },
                mapper: |it| <super::NamedComplex>::from(it),
            }
        }
    }
}
pub mod async_ {
    use crate::client::async_::GenericClient;
    use futures::{self, StreamExt, TryStreamExt};
    pub struct IdQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::async_::Stmt,
        extractor: fn(&tokio_postgres::Row) -> super::Id,
        mapper: fn(super::Id) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> IdQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::Id) -> R) -> IdQuery<'a, C, R, N> {
            IdQuery {
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
    pub struct NamedQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::async_::Stmt,
        extractor: fn(&tokio_postgres::Row) -> super::NamedBorrowed,
        mapper: fn(super::NamedBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> NamedQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::NamedBorrowed) -> R) -> NamedQuery<'a, C, R, N> {
            NamedQuery {
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
    pub struct NamedComplexQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::async_::Stmt,
        extractor: fn(&tokio_postgres::Row) -> super::NamedComplexBorrowed,
        mapper: fn(super::NamedComplexBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> NamedComplexQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::NamedComplexBorrowed) -> R,
        ) -> NamedComplexQuery<'a, C, R, N> {
            NamedComplexQuery {
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
    pub fn new_named_visible() -> NewNamedVisibleStmt {
        NewNamedVisibleStmt(crate::client::async_::Stmt::new(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, true) RETURNING id ",
        ))
    }
    pub struct NewNamedVisibleStmt(crate::client::async_::Stmt);
    impl NewNamedVisibleStmt {
        pub fn bind<'a, C: GenericClient, T1: crate::StringSql>(
            &'a mut self,
            client: &'a C,
            name: &'a T1,
            price: &'a Option<f64>,
        ) -> IdQuery<'a, C, super::Id, 2> {
            IdQuery {
                client,
                params: [name, price],
                stmt: &mut self.0,
                extractor: |row| super::Id { id: row.get(0) },
                mapper: |it| <super::Id>::from(it),
            }
        }
    }
    impl<'a, C: GenericClient, T1: crate::StringSql>
        crate::client::async_::Params<'a, super::NamedParams<T1>, IdQuery<'a, C, super::Id, 2>, C>
        for NewNamedVisibleStmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::NamedParams<T1>,
        ) -> IdQuery<'a, C, super::Id, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn new_named_hidden() -> NewNamedHiddenStmt {
        NewNamedHiddenStmt(crate::client::async_::Stmt::new(
            "INSERT INTO named (price, name, show) VALUES ($1, $2, false) RETURNING id",
        ))
    }
    pub struct NewNamedHiddenStmt(crate::client::async_::Stmt);
    impl NewNamedHiddenStmt {
        pub fn bind<'a, C: GenericClient, T1: crate::StringSql>(
            &'a mut self,
            client: &'a C,
            price: &'a Option<f64>,
            name: &'a T1,
        ) -> IdQuery<'a, C, super::Id, 2> {
            IdQuery {
                client,
                params: [price, name],
                stmt: &mut self.0,
                extractor: |row| super::Id { id: row.get(0) },
                mapper: |it| <super::Id>::from(it),
            }
        }
    }
    impl<'a, C: GenericClient, T1: crate::StringSql>
        crate::client::async_::Params<'a, super::NamedParams<T1>, IdQuery<'a, C, super::Id, 2>, C>
        for NewNamedHiddenStmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::NamedParams<T1>,
        ) -> IdQuery<'a, C, super::Id, 2> {
            self.bind(client, &params.price, &params.name)
        }
    }
    pub fn named() -> NamedStmt {
        NamedStmt(crate::client::async_::Stmt::new("SELECT * FROM named"))
    }
    pub struct NamedStmt(crate::client::async_::Stmt);
    impl NamedStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
        ) -> NamedQuery<'a, C, super::Named, 0> {
            NamedQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| super::NamedBorrowed {
                    id: row.get(0),
                    name: row.get(1),
                    price: row.get(2),
                    show: row.get(3),
                },
                mapper: |it| <super::Named>::from(it),
            }
        }
    }
    pub fn named_by_id() -> NamedByIdStmt {
        NamedByIdStmt(crate::client::async_::Stmt::new(
            "SELECT * FROM named WHERE id = $1",
        ))
    }
    pub struct NamedByIdStmt(crate::client::async_::Stmt);
    impl NamedByIdStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
            id: &'a i32,
        ) -> NamedQuery<'a, C, super::Named, 1> {
            NamedQuery {
                client,
                params: [id],
                stmt: &mut self.0,
                extractor: |row| super::NamedBorrowed {
                    id: row.get(0),
                    name: row.get(1),
                    price: row.get(2),
                    show: row.get(3),
                },
                mapper: |it| <super::Named>::from(it),
            }
        }
    }
    pub fn new_named_complex() -> NewNamedComplexStmt {
        NewNamedComplexStmt(crate::client::async_::Stmt::new(
            "INSERT INTO named_complex (named, \"named.with_dot\") VALUES ($1, $2)",
        ))
    }
    pub struct NewNamedComplexStmt(crate::client::async_::Stmt);
    impl NewNamedComplexStmt {
        pub async fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
            named: &'a crate::types::NamedCompositeBorrowed<'a>,
            named_with_dot: &'a Option<crate::types::NamedCompositeWithDot>,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[named, named_with_dot]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            super::NamedComplexParams<'a>,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for NewNamedComplexStmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::NamedComplexParams<'a>,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.named, &params.named_with_dot))
        }
    }
    pub fn named_complex() -> NamedComplexStmt {
        NamedComplexStmt(crate::client::async_::Stmt::new(
            "SELECT * FROM named_complex",
        ))
    }
    pub struct NamedComplexStmt(crate::client::async_::Stmt);
    impl NamedComplexStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
        ) -> NamedComplexQuery<'a, C, super::NamedComplex, 0> {
            NamedComplexQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| super::NamedComplexBorrowed {
                    named: row.get(0),
                    named_with_dot: row.get(1),
                },
                mapper: |it| <super::NamedComplex>::from(it),
            }
        }
    }
}
