// This file was generated with `cornucopia`. Do not modify.

pub mod sync {
    use postgres::{fallible_iterator::FallibleIterator, GenericClient};
    pub struct CloneCompositeQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> crate::types::CloneCompositeBorrowed,
        mapper: fn(crate::types::CloneCompositeBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> CloneCompositeQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(crate::types::CloneCompositeBorrowed) -> R,
        ) -> CloneCompositeQuery<'a, C, R, N> {
            CloneCompositeQuery {
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
    pub struct CopyCompositeQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> crate::types::CopyComposite,
        mapper: fn(crate::types::CopyComposite) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> CopyCompositeQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(crate::types::CopyComposite) -> R,
        ) -> CopyCompositeQuery<'a, C, R, N> {
            CopyCompositeQuery {
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
    pub fn insert_clone() -> InsertCloneStmt {
        InsertCloneStmt(crate::client::sync::Stmt::new(
            "INSERT INTO clone (composite) VALUES ($1)",
        ))
    }
    pub struct InsertCloneStmt(crate::client::sync::Stmt);
    impl InsertCloneStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
            composite: &'a crate::types::CloneCompositeBorrowed<'a>,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[composite])
        }
    }
    pub fn select_clone() -> SelectCloneStmt {
        SelectCloneStmt(crate::client::sync::Stmt::new("SELECT * FROM clone"))
    }
    pub struct SelectCloneStmt(crate::client::sync::Stmt);
    impl SelectCloneStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
        ) -> CloneCompositeQuery<'a, C, crate::types::CloneComposite, 0> {
            CloneCompositeQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| row.get(0),
                mapper: |it| it.into(),
            }
        }
    }
    pub fn insert_copy() -> InsertCopyStmt {
        InsertCopyStmt(crate::client::sync::Stmt::new(
            "INSERT INTO copy (composite) VALUES ($1)",
        ))
    }
    pub struct InsertCopyStmt(crate::client::sync::Stmt);
    impl InsertCopyStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
            composite: &'a crate::types::CopyComposite,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[composite])
        }
    }
    pub fn select_copy() -> SelectCopyStmt {
        SelectCopyStmt(crate::client::sync::Stmt::new("SELECT * FROM copy"))
    }
    pub struct SelectCopyStmt(crate::client::sync::Stmt);
    impl SelectCopyStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
        ) -> CopyCompositeQuery<'a, C, crate::types::CopyComposite, 0> {
            CopyCompositeQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| row.get(0),
                mapper: |it| it,
            }
        }
    }
}
pub mod async_ {
    use crate::client::async_::GenericClient;
    use futures::{self, StreamExt, TryStreamExt};
    pub struct CloneCompositeQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::async_::Stmt,
        extractor: fn(&tokio_postgres::Row) -> crate::types::CloneCompositeBorrowed,
        mapper: fn(crate::types::CloneCompositeBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> CloneCompositeQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(crate::types::CloneCompositeBorrowed) -> R,
        ) -> CloneCompositeQuery<'a, C, R, N> {
            CloneCompositeQuery {
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
    pub struct CopyCompositeQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::async_::Stmt,
        extractor: fn(&tokio_postgres::Row) -> crate::types::CopyComposite,
        mapper: fn(crate::types::CopyComposite) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> CopyCompositeQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(crate::types::CopyComposite) -> R,
        ) -> CopyCompositeQuery<'a, C, R, N> {
            CopyCompositeQuery {
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
    pub fn insert_clone() -> InsertCloneStmt {
        InsertCloneStmt(crate::client::async_::Stmt::new(
            "INSERT INTO clone (composite) VALUES ($1)",
        ))
    }
    pub struct InsertCloneStmt(crate::client::async_::Stmt);
    impl InsertCloneStmt {
        pub async fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
            composite: &'a crate::types::CloneCompositeBorrowed<'a>,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[composite]).await
        }
    }
    pub fn select_clone() -> SelectCloneStmt {
        SelectCloneStmt(crate::client::async_::Stmt::new("SELECT * FROM clone"))
    }
    pub struct SelectCloneStmt(crate::client::async_::Stmt);
    impl SelectCloneStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
        ) -> CloneCompositeQuery<'a, C, crate::types::CloneComposite, 0> {
            CloneCompositeQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| row.get(0),
                mapper: |it| it.into(),
            }
        }
    }
    pub fn insert_copy() -> InsertCopyStmt {
        InsertCopyStmt(crate::client::async_::Stmt::new(
            "INSERT INTO copy (composite) VALUES ($1)",
        ))
    }
    pub struct InsertCopyStmt(crate::client::async_::Stmt);
    impl InsertCopyStmt {
        pub async fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
            composite: &'a crate::types::CopyComposite,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[composite]).await
        }
    }
    pub fn select_copy() -> SelectCopyStmt {
        SelectCopyStmt(crate::client::async_::Stmt::new("SELECT * FROM copy"))
    }
    pub struct SelectCopyStmt(crate::client::async_::Stmt);
    impl SelectCopyStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
        ) -> CopyCompositeQuery<'a, C, crate::types::CopyComposite, 0> {
            CopyCompositeQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| row.get(0),
                mapper: |it| it,
            }
        }
    }
}
