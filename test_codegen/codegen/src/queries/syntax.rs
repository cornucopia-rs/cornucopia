// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct ImplicitCompactParams<T1: crate::StringSql> {
    pub name: Option<T1>,
    pub price: Option<f64>,
}
#[derive(Debug)]
pub struct ImplicitSpacedParams<T1: crate::StringSql> {
    pub name: Option<T1>,
    pub price: Option<f64>,
}
#[derive(Debug)]
pub struct Params<T1: crate::StringSql> {
    pub name: T1,
    pub price: f64,
}
#[derive(Debug)]
pub struct ParamsSpace<T1: crate::StringSql> {
    pub name: T1,
    pub price: f64,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySqlParams {
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql1Params {
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql2Params {
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql3Params {
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql4Params {
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql6Params {
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql7Params {
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql8Params {
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql9Params {
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql10Params {
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
#[derive(serde::Serialize, Debug, Clone, PartialEq, Copy)]
pub struct Row {
    pub id: i32,
}
#[derive(serde::Serialize, Debug, Clone, PartialEq, Copy)]
pub struct RowSpace {
    pub id: i32,
}
#[derive(serde::Serialize, Debug, Clone, PartialEq)]
pub struct Typeof {
    pub trick_y: String,
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
pub struct TypeofBorrowed<'a> {
    pub trick_y: &'a str,
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
impl<'a> From<TypeofBorrowed<'a>> for Typeof {
    fn from(
        TypeofBorrowed {
            trick_y,
            r#async,
            r#enum,
        }: TypeofBorrowed<'a>,
    ) -> Self {
        Self {
            trick_y: trick_y.into(),
            r#async,
            r#enum,
        }
    }
}
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
    pub struct Optioni32Query<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> Option<i32>,
        mapper: fn(Option<i32>) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> Optioni32Query<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(Option<i32>) -> R) -> Optioni32Query<'a, C, R, N> {
            Optioni32Query {
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
    pub struct RowQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> super::Row,
        mapper: fn(super::Row) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> RowQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::Row) -> R) -> RowQuery<'a, C, R, N> {
            RowQuery {
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
    pub struct RowSpaceQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> super::RowSpace,
        mapper: fn(super::RowSpace) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> RowSpaceQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::RowSpace) -> R) -> RowSpaceQuery<'a, C, R, N> {
            RowSpaceQuery {
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
    pub struct TypeofQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> super::TypeofBorrowed,
        mapper: fn(super::TypeofBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> TypeofQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::TypeofBorrowed) -> R) -> TypeofQuery<'a, C, R, N> {
            TypeofQuery {
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
    pub fn select_compact() -> SelectCompactStmt {
        SelectCompactStmt(crate::client::sync::Stmt::new("SELECT * FROM clone"))
    }
    pub struct SelectCompactStmt(crate::client::sync::Stmt);
    impl SelectCompactStmt {
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
    pub fn select_spaced() -> SelectSpacedStmt {
        SelectSpacedStmt(crate::client::sync::Stmt::new("      SELECT * FROM clone "))
    }
    pub struct SelectSpacedStmt(crate::client::sync::Stmt);
    impl SelectSpacedStmt {
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
    pub fn implicit_compact() -> ImplicitCompactStmt {
        ImplicitCompactStmt(crate::client::sync::Stmt::new(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
        ))
    }
    pub struct ImplicitCompactStmt(crate::client::sync::Stmt);
    impl ImplicitCompactStmt {
        pub fn bind<'a, C: GenericClient, T1: crate::StringSql>(
            &'a mut self,
            client: &'a mut C,
            name: &'a Option<T1>,
            price: &'a Option<f64>,
        ) -> Optioni32Query<'a, C, Option<i32>, 2> {
            Optioni32Query {
                client,
                params: [name, price],
                stmt: &mut self.0,
                extractor: |row| row.get(0),
                mapper: |it| it,
            }
        }
    }
    impl<'a, C: GenericClient, T1: crate::StringSql>
        crate::client::sync::Params<
            'a,
            super::ImplicitCompactParams<T1>,
            Optioni32Query<'a, C, Option<i32>, 2>,
            C,
        > for ImplicitCompactStmt
    {
        fn params(
            &'a mut self,
            client: &'a mut C,
            params: &'a super::ImplicitCompactParams<T1>,
        ) -> Optioni32Query<'a, C, Option<i32>, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn implicit_spaced() -> ImplicitSpacedStmt {
        ImplicitSpacedStmt(crate::client::sync::Stmt::new(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
        ))
    }
    pub struct ImplicitSpacedStmt(crate::client::sync::Stmt);
    impl ImplicitSpacedStmt {
        pub fn bind<'a, C: GenericClient, T1: crate::StringSql>(
            &'a mut self,
            client: &'a mut C,
            name: &'a Option<T1>,
            price: &'a Option<f64>,
        ) -> Optioni32Query<'a, C, Option<i32>, 2> {
            Optioni32Query {
                client,
                params: [name, price],
                stmt: &mut self.0,
                extractor: |row| row.get(0),
                mapper: |it| it,
            }
        }
    }
    impl<'a, C: GenericClient, T1: crate::StringSql>
        crate::client::sync::Params<
            'a,
            super::ImplicitSpacedParams<T1>,
            Optioni32Query<'a, C, Option<i32>, 2>,
            C,
        > for ImplicitSpacedStmt
    {
        fn params(
            &'a mut self,
            client: &'a mut C,
            params: &'a super::ImplicitSpacedParams<T1>,
        ) -> Optioni32Query<'a, C, Option<i32>, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn named_compact() -> NamedCompactStmt {
        NamedCompactStmt(crate::client::sync::Stmt::new(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
        ))
    }
    pub struct NamedCompactStmt(crate::client::sync::Stmt);
    impl NamedCompactStmt {
        pub fn bind<'a, C: GenericClient, T1: crate::StringSql>(
            &'a mut self,
            client: &'a mut C,
            name: &'a T1,
            price: &'a f64,
        ) -> RowQuery<'a, C, super::Row, 2> {
            RowQuery {
                client,
                params: [name, price],
                stmt: &mut self.0,
                extractor: |row| super::Row { id: row.get(0) },
                mapper: |it| <super::Row>::from(it),
            }
        }
    }
    impl<'a, C: GenericClient, T1: crate::StringSql>
        crate::client::sync::Params<'a, super::Params<T1>, RowQuery<'a, C, super::Row, 2>, C>
        for NamedCompactStmt
    {
        fn params(
            &'a mut self,
            client: &'a mut C,
            params: &'a super::Params<T1>,
        ) -> RowQuery<'a, C, super::Row, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn named_spaced() -> NamedSpacedStmt {
        NamedSpacedStmt(crate::client::sync::Stmt::new(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
        ))
    }
    pub struct NamedSpacedStmt(crate::client::sync::Stmt);
    impl NamedSpacedStmt {
        pub fn bind<'a, C: GenericClient, T1: crate::StringSql>(
            &'a mut self,
            client: &'a mut C,
            name: &'a T1,
            price: &'a f64,
        ) -> RowSpaceQuery<'a, C, super::RowSpace, 2> {
            RowSpaceQuery {
                client,
                params: [name, price],
                stmt: &mut self.0,
                extractor: |row| super::RowSpace { id: row.get(0) },
                mapper: |it| <super::RowSpace>::from(it),
            }
        }
    }
    impl<'a, C: GenericClient, T1: crate::StringSql>
        crate::client::sync::Params<
            'a,
            super::ParamsSpace<T1>,
            RowSpaceQuery<'a, C, super::RowSpace, 2>,
            C,
        > for NamedSpacedStmt
    {
        fn params(
            &'a mut self,
            client: &'a mut C,
            params: &'a super::ParamsSpace<T1>,
        ) -> RowSpaceQuery<'a, C, super::RowSpace, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn tricky_sql() -> TrickySqlStmt {
        TrickySqlStmt(crate::client::sync::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a bind_param\', $1, $2)"))
    }
    pub struct TrickySqlStmt(crate::client::sync::Stmt);
    impl TrickySqlStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[r#async, r#enum])
        }
    }
    impl<'a, C: GenericClient>
        crate::client::sync::Params<'a, super::TrickySqlParams, Result<u64, postgres::Error>, C>
        for TrickySqlStmt
    {
        fn params(
            &'a mut self,
            client: &'a mut C,
            params: &'a super::TrickySqlParams,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql1() -> TrickySql1Stmt {
        TrickySql1Stmt(crate::client::sync::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a :bind_param', $1, $2)"))
    }
    pub struct TrickySql1Stmt(crate::client::sync::Stmt);
    impl TrickySql1Stmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[r#async, r#enum])
        }
    }
    impl<'a, C: GenericClient>
        crate::client::sync::Params<'a, super::TrickySql1Params, Result<u64, postgres::Error>, C>
        for TrickySql1Stmt
    {
        fn params(
            &'a mut self,
            client: &'a mut C,
            params: &'a super::TrickySql1Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql2() -> TrickySql2Stmt {
        TrickySql2Stmt(crate::client::sync::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a '':bind_param''', $1, $2)"))
    }
    pub struct TrickySql2Stmt(crate::client::sync::Stmt);
    impl TrickySql2Stmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[r#async, r#enum])
        }
    }
    impl<'a, C: GenericClient>
        crate::client::sync::Params<'a, super::TrickySql2Params, Result<u64, postgres::Error>, C>
        for TrickySql2Stmt
    {
        fn params(
            &'a mut self,
            client: &'a mut C,
            params: &'a super::TrickySql2Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql3() -> TrickySql3Stmt {
        TrickySql3Stmt(crate::client::sync::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum)  VALUES ($$this is not a :bind_param$$, $1, $2)"))
    }
    pub struct TrickySql3Stmt(crate::client::sync::Stmt);
    impl TrickySql3Stmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[r#async, r#enum])
        }
    }
    impl<'a, C: GenericClient>
        crate::client::sync::Params<'a, super::TrickySql3Params, Result<u64, postgres::Error>, C>
        for TrickySql3Stmt
    {
        fn params(
            &'a mut self,
            client: &'a mut C,
            params: &'a super::TrickySql3Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql4() -> TrickySql4Stmt {
        TrickySql4Stmt(crate::client::sync::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ($tag$this is not a :bind_param$tag$, $1, $2)"))
    }
    pub struct TrickySql4Stmt(crate::client::sync::Stmt);
    impl TrickySql4Stmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[r#async, r#enum])
        }
    }
    impl<'a, C: GenericClient>
        crate::client::sync::Params<'a, super::TrickySql4Params, Result<u64, postgres::Error>, C>
        for TrickySql4Stmt
    {
        fn params(
            &'a mut self,
            client: &'a mut C,
            params: &'a super::TrickySql4Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql6() -> TrickySql6Stmt {
        TrickySql6Stmt(crate::client::sync::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES (e'this is not a '':bind_param''', $1, $2)"))
    }
    pub struct TrickySql6Stmt(crate::client::sync::Stmt);
    impl TrickySql6Stmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[r#async, r#enum])
        }
    }
    impl<'a, C: GenericClient>
        crate::client::sync::Params<'a, super::TrickySql6Params, Result<u64, postgres::Error>, C>
        for TrickySql6Stmt
    {
        fn params(
            &'a mut self,
            client: &'a mut C,
            params: &'a super::TrickySql6Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql7() -> TrickySql7Stmt {
        TrickySql7Stmt(crate::client::sync::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES (E'this is not a \':bind_param\'', $1, $2)"))
    }
    pub struct TrickySql7Stmt(crate::client::sync::Stmt);
    impl TrickySql7Stmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[r#async, r#enum])
        }
    }
    impl<'a, C: GenericClient>
        crate::client::sync::Params<'a, super::TrickySql7Params, Result<u64, postgres::Error>, C>
        for TrickySql7Stmt
    {
        fn params(
            &'a mut self,
            client: &'a mut C,
            params: &'a super::TrickySql7Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql8() -> TrickySql8Stmt {
        TrickySql8Stmt(crate::client::sync::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES (e'this is ''not'' a \':bind_param\'', $1, $2)"))
    }
    pub struct TrickySql8Stmt(crate::client::sync::Stmt);
    impl TrickySql8Stmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[r#async, r#enum])
        }
    }
    impl<'a, C: GenericClient>
        crate::client::sync::Params<'a, super::TrickySql8Params, Result<u64, postgres::Error>, C>
        for TrickySql8Stmt
    {
        fn params(
            &'a mut self,
            client: &'a mut C,
            params: &'a super::TrickySql8Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql9() -> TrickySql9Stmt {
        TrickySql9Stmt(crate::client::sync::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES (E'this is \'not\' a \':bind_param\'', $1, $2)"))
    }
    pub struct TrickySql9Stmt(crate::client::sync::Stmt);
    impl TrickySql9Stmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[r#async, r#enum])
        }
    }
    impl<'a, C: GenericClient>
        crate::client::sync::Params<'a, super::TrickySql9Params, Result<u64, postgres::Error>, C>
        for TrickySql9Stmt
    {
        fn params(
            &'a mut self,
            client: &'a mut C,
            params: &'a super::TrickySql9Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql10() -> TrickySql10Stmt {
        TrickySql10Stmt(crate::client::sync::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is just a cast'::text, $1, $2)"))
    }
    pub struct TrickySql10Stmt(crate::client::sync::Stmt);
    impl TrickySql10Stmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[r#async, r#enum])
        }
    }
    impl<'a, C: GenericClient>
        crate::client::sync::Params<'a, super::TrickySql10Params, Result<u64, postgres::Error>, C>
        for TrickySql10Stmt
    {
        fn params(
            &'a mut self,
            client: &'a mut C,
            params: &'a super::TrickySql10Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn r#typeof() -> RTypeofStmt {
        RTypeofStmt(crate::client::sync::Stmt::new("SELECT * FROM syntax"))
    }
    pub struct RTypeofStmt(crate::client::sync::Stmt);
    impl RTypeofStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
        ) -> TypeofQuery<'a, C, super::Typeof, 0> {
            TypeofQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| super::TypeofBorrowed {
                    trick_y: row.get(0),
                    r#async: row.get(1),
                    r#enum: row.get(2),
                },
                mapper: |it| <super::Typeof>::from(it),
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
    pub struct Optioni32Query<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::async_::Stmt,
        extractor: fn(&tokio_postgres::Row) -> Option<i32>,
        mapper: fn(Option<i32>) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> Optioni32Query<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(Option<i32>) -> R) -> Optioni32Query<'a, C, R, N> {
            Optioni32Query {
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
    pub struct RowQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::async_::Stmt,
        extractor: fn(&tokio_postgres::Row) -> super::Row,
        mapper: fn(super::Row) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> RowQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::Row) -> R) -> RowQuery<'a, C, R, N> {
            RowQuery {
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
    pub struct RowSpaceQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::async_::Stmt,
        extractor: fn(&tokio_postgres::Row) -> super::RowSpace,
        mapper: fn(super::RowSpace) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> RowSpaceQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::RowSpace) -> R) -> RowSpaceQuery<'a, C, R, N> {
            RowSpaceQuery {
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
    pub struct TypeofQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::async_::Stmt,
        extractor: fn(&tokio_postgres::Row) -> super::TypeofBorrowed,
        mapper: fn(super::TypeofBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> TypeofQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::TypeofBorrowed) -> R) -> TypeofQuery<'a, C, R, N> {
            TypeofQuery {
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
    pub fn select_compact() -> SelectCompactStmt {
        SelectCompactStmt(crate::client::async_::Stmt::new("SELECT * FROM clone"))
    }
    pub struct SelectCompactStmt(crate::client::async_::Stmt);
    impl SelectCompactStmt {
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
    pub fn select_spaced() -> SelectSpacedStmt {
        SelectSpacedStmt(crate::client::async_::Stmt::new(
            "      SELECT * FROM clone ",
        ))
    }
    pub struct SelectSpacedStmt(crate::client::async_::Stmt);
    impl SelectSpacedStmt {
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
    pub fn implicit_compact() -> ImplicitCompactStmt {
        ImplicitCompactStmt(crate::client::async_::Stmt::new(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
        ))
    }
    pub struct ImplicitCompactStmt(crate::client::async_::Stmt);
    impl ImplicitCompactStmt {
        pub fn bind<'a, C: GenericClient, T1: crate::StringSql>(
            &'a mut self,
            client: &'a C,
            name: &'a Option<T1>,
            price: &'a Option<f64>,
        ) -> Optioni32Query<'a, C, Option<i32>, 2> {
            Optioni32Query {
                client,
                params: [name, price],
                stmt: &mut self.0,
                extractor: |row| row.get(0),
                mapper: |it| it,
            }
        }
    }
    impl<'a, C: GenericClient, T1: crate::StringSql>
        crate::client::async_::Params<
            'a,
            super::ImplicitCompactParams<T1>,
            Optioni32Query<'a, C, Option<i32>, 2>,
            C,
        > for ImplicitCompactStmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::ImplicitCompactParams<T1>,
        ) -> Optioni32Query<'a, C, Option<i32>, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn implicit_spaced() -> ImplicitSpacedStmt {
        ImplicitSpacedStmt(crate::client::async_::Stmt::new(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
        ))
    }
    pub struct ImplicitSpacedStmt(crate::client::async_::Stmt);
    impl ImplicitSpacedStmt {
        pub fn bind<'a, C: GenericClient, T1: crate::StringSql>(
            &'a mut self,
            client: &'a C,
            name: &'a Option<T1>,
            price: &'a Option<f64>,
        ) -> Optioni32Query<'a, C, Option<i32>, 2> {
            Optioni32Query {
                client,
                params: [name, price],
                stmt: &mut self.0,
                extractor: |row| row.get(0),
                mapper: |it| it,
            }
        }
    }
    impl<'a, C: GenericClient, T1: crate::StringSql>
        crate::client::async_::Params<
            'a,
            super::ImplicitSpacedParams<T1>,
            Optioni32Query<'a, C, Option<i32>, 2>,
            C,
        > for ImplicitSpacedStmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::ImplicitSpacedParams<T1>,
        ) -> Optioni32Query<'a, C, Option<i32>, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn named_compact() -> NamedCompactStmt {
        NamedCompactStmt(crate::client::async_::Stmt::new(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
        ))
    }
    pub struct NamedCompactStmt(crate::client::async_::Stmt);
    impl NamedCompactStmt {
        pub fn bind<'a, C: GenericClient, T1: crate::StringSql>(
            &'a mut self,
            client: &'a C,
            name: &'a T1,
            price: &'a f64,
        ) -> RowQuery<'a, C, super::Row, 2> {
            RowQuery {
                client,
                params: [name, price],
                stmt: &mut self.0,
                extractor: |row| super::Row { id: row.get(0) },
                mapper: |it| <super::Row>::from(it),
            }
        }
    }
    impl<'a, C: GenericClient, T1: crate::StringSql>
        crate::client::async_::Params<'a, super::Params<T1>, RowQuery<'a, C, super::Row, 2>, C>
        for NamedCompactStmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::Params<T1>,
        ) -> RowQuery<'a, C, super::Row, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn named_spaced() -> NamedSpacedStmt {
        NamedSpacedStmt(crate::client::async_::Stmt::new(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
        ))
    }
    pub struct NamedSpacedStmt(crate::client::async_::Stmt);
    impl NamedSpacedStmt {
        pub fn bind<'a, C: GenericClient, T1: crate::StringSql>(
            &'a mut self,
            client: &'a C,
            name: &'a T1,
            price: &'a f64,
        ) -> RowSpaceQuery<'a, C, super::RowSpace, 2> {
            RowSpaceQuery {
                client,
                params: [name, price],
                stmt: &mut self.0,
                extractor: |row| super::RowSpace { id: row.get(0) },
                mapper: |it| <super::RowSpace>::from(it),
            }
        }
    }
    impl<'a, C: GenericClient, T1: crate::StringSql>
        crate::client::async_::Params<
            'a,
            super::ParamsSpace<T1>,
            RowSpaceQuery<'a, C, super::RowSpace, 2>,
            C,
        > for NamedSpacedStmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::ParamsSpace<T1>,
        ) -> RowSpaceQuery<'a, C, super::RowSpace, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn tricky_sql() -> TrickySqlStmt {
        TrickySqlStmt(crate::client::async_::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a bind_param\', $1, $2)"))
    }
    pub struct TrickySqlStmt(crate::client::async_::Stmt);
    impl TrickySqlStmt {
        pub async fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            super::TrickySqlParams,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySqlStmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::TrickySqlParams,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql1() -> TrickySql1Stmt {
        TrickySql1Stmt(crate::client::async_::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a :bind_param', $1, $2)"))
    }
    pub struct TrickySql1Stmt(crate::client::async_::Stmt);
    impl TrickySql1Stmt {
        pub async fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            super::TrickySql1Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql1Stmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::TrickySql1Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql2() -> TrickySql2Stmt {
        TrickySql2Stmt(crate::client::async_::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a '':bind_param''', $1, $2)"))
    }
    pub struct TrickySql2Stmt(crate::client::async_::Stmt);
    impl TrickySql2Stmt {
        pub async fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            super::TrickySql2Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql2Stmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::TrickySql2Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql3() -> TrickySql3Stmt {
        TrickySql3Stmt(crate::client::async_::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum)  VALUES ($$this is not a :bind_param$$, $1, $2)"))
    }
    pub struct TrickySql3Stmt(crate::client::async_::Stmt);
    impl TrickySql3Stmt {
        pub async fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            super::TrickySql3Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql3Stmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::TrickySql3Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql4() -> TrickySql4Stmt {
        TrickySql4Stmt(crate::client::async_::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ($tag$this is not a :bind_param$tag$, $1, $2)"))
    }
    pub struct TrickySql4Stmt(crate::client::async_::Stmt);
    impl TrickySql4Stmt {
        pub async fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            super::TrickySql4Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql4Stmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::TrickySql4Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql6() -> TrickySql6Stmt {
        TrickySql6Stmt(crate::client::async_::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES (e'this is not a '':bind_param''', $1, $2)"))
    }
    pub struct TrickySql6Stmt(crate::client::async_::Stmt);
    impl TrickySql6Stmt {
        pub async fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            super::TrickySql6Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql6Stmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::TrickySql6Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql7() -> TrickySql7Stmt {
        TrickySql7Stmt(crate::client::async_::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES (E'this is not a \':bind_param\'', $1, $2)"))
    }
    pub struct TrickySql7Stmt(crate::client::async_::Stmt);
    impl TrickySql7Stmt {
        pub async fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            super::TrickySql7Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql7Stmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::TrickySql7Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql8() -> TrickySql8Stmt {
        TrickySql8Stmt(crate::client::async_::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES (e'this is ''not'' a \':bind_param\'', $1, $2)"))
    }
    pub struct TrickySql8Stmt(crate::client::async_::Stmt);
    impl TrickySql8Stmt {
        pub async fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            super::TrickySql8Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql8Stmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::TrickySql8Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql9() -> TrickySql9Stmt {
        TrickySql9Stmt(crate::client::async_::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES (E'this is \'not\' a \':bind_param\'', $1, $2)"))
    }
    pub struct TrickySql9Stmt(crate::client::async_::Stmt);
    impl TrickySql9Stmt {
        pub async fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            super::TrickySql9Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql9Stmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::TrickySql9Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql10() -> TrickySql10Stmt {
        TrickySql10Stmt(crate::client::async_::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is just a cast'::text, $1, $2)"))
    }
    pub struct TrickySql10Stmt(crate::client::async_::Stmt);
    impl TrickySql10Stmt {
        pub async fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            super::TrickySql10Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql10Stmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::TrickySql10Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn r#typeof() -> RTypeofStmt {
        RTypeofStmt(crate::client::async_::Stmt::new("SELECT * FROM syntax"))
    }
    pub struct RTypeofStmt(crate::client::async_::Stmt);
    impl RTypeofStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
        ) -> TypeofQuery<'a, C, super::Typeof, 0> {
            TypeofQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| super::TypeofBorrowed {
                    trick_y: row.get(0),
                    r#async: row.get(1),
                    r#enum: row.get(2),
                },
                mapper: |it| <super::Typeof>::from(it),
            }
        }
    }
}
