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
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
pub struct TestNestedComposite {
    pub composite: crate::types::NullityComposite,
}
pub struct TestNestedCompositeBorrowed<'a> {
    pub composite: crate::types::NullityCompositeBorrowed<'a>,
}
impl<'a> From<TestNestedCompositeBorrowed<'a>> for TestNestedComposite {
    fn from(TestNestedCompositeBorrowed { composite }: TestNestedCompositeBorrowed<'a>) -> Self {
        Self {
            composite: composite.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct TestDirectComposite {
    pub composite: Option<crate::types::NullityComposite>,
}
pub struct TestDirectCompositeBorrowed<'a> {
    pub composite: Option<crate::types::NullityCompositeBorrowed<'a>>,
}
impl<'a> From<TestDirectCompositeBorrowed<'a>> for TestDirectComposite {
    fn from(TestDirectCompositeBorrowed { composite }: TestDirectCompositeBorrowed<'a>) -> Self {
        Self {
            composite: composite.map(|v| v.into()),
        }
    }
}
pub mod sync {
    use crate::client::sync::GenericClient;
    use postgres::fallible_iterator::FallibleIterator;
    pub struct NullityQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor: fn(&postgres::Row) -> Result<super::NullityBorrowed, postgres::Error>,
        mapper: fn(super::NullityBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> NullityQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::NullityBorrowed) -> R,
        ) -> NullityQuery<'c, 'a, 's, C, R, N> {
            NullityQuery {
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
    pub struct NullityCompositeQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor:
            fn(&postgres::Row) -> Result<crate::types::NullityCompositeBorrowed, postgres::Error>,
        mapper: fn(crate::types::NullityCompositeBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> NullityCompositeQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(crate::types::NullityCompositeBorrowed) -> R,
        ) -> NullityCompositeQuery<'c, 'a, 's, C, R, N> {
            NullityCompositeQuery {
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
    pub struct VecNullityCompositeQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor: fn(
            &postgres::Row,
        ) -> Result<
            crate::ArrayIterator<'_, crate::types::NullityCompositeBorrowed>,
            postgres::Error,
        >,
        mapper: fn(crate::ArrayIterator<'_, crate::types::NullityCompositeBorrowed>) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> VecNullityCompositeQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(crate::ArrayIterator<'_, crate::types::NullityCompositeBorrowed>) -> R,
        ) -> VecNullityCompositeQuery<'c, 'a, 's, C, R, N> {
            VecNullityCompositeQuery {
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
    pub struct TestNestedCompositeQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor:
            fn(&postgres::Row) -> Result<super::TestNestedCompositeBorrowed, postgres::Error>,
        mapper: fn(super::TestNestedCompositeBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> TestNestedCompositeQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::TestNestedCompositeBorrowed) -> R,
        ) -> TestNestedCompositeQuery<'c, 'a, 's, C, R, N> {
            TestNestedCompositeQuery {
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
    pub struct OptionNullityCompositeQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor: fn(
            &postgres::Row,
        )
            -> Result<Option<crate::types::NullityCompositeBorrowed>, postgres::Error>,
        mapper: fn(Option<crate::types::NullityCompositeBorrowed>) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> OptionNullityCompositeQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(Option<crate::types::NullityCompositeBorrowed>) -> R,
        ) -> OptionNullityCompositeQuery<'c, 'a, 's, C, R, N> {
            OptionNullityCompositeQuery {
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
    pub struct TestDirectCompositeQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor:
            fn(&postgres::Row) -> Result<super::TestDirectCompositeBorrowed, postgres::Error>,
        mapper: fn(super::TestDirectCompositeBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> TestDirectCompositeQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::TestDirectCompositeBorrowed) -> R,
        ) -> TestDirectCompositeQuery<'c, 'a, 's, C, R, N> {
            TestDirectCompositeQuery {
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
    pub struct NewNullityStmt(&'static str, Option<postgres::Statement>);
    pub fn new_nullity() -> NewNullityStmt {
        NewNullityStmt(
            "INSERT INTO nullity(texts, name, composite) VALUES ($1, $2, $3)",
            None,
        )
    }
    impl NewNullityStmt {
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
            T2: crate::ArraySql<Item = Option<T1>>,
            T3: crate::StringSql,
        >(
            &'s self,
            client: &'c mut C,
            texts: &'a T2,
            name: &'a T3,
            composite: &'a Option<crate::types::NullityCompositeParams<'a>>,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[texts, name, composite])
        }
    }
    impl<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::ArraySql<Item = Option<T1>>,
        T3: crate::StringSql,
    >
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::NullityParams<'a, T1, T2, T3>,
            Result<u64, postgres::Error>,
            C,
        > for NewNullityStmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::NullityParams<'a, T1, T2, T3>,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.texts, &params.name, &params.composite)
        }
    }
    pub struct NullityStmt(&'static str, Option<postgres::Statement>);
    pub fn nullity() -> NullityStmt {
        NullityStmt("SELECT * FROM nullity", None)
    }
    impl NullityStmt {
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
        ) -> NullityQuery<'c, 'a, 's, C, super::Nullity, 0> {
            NullityQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor:
                    |row: &postgres::Row| -> Result<super::NullityBorrowed, postgres::Error> {
                        Ok(super::NullityBorrowed {
                            texts: row.try_get(0)?,
                            name: row.try_get(1)?,
                            composite: row.try_get(2)?,
                        })
                    },
                mapper: |it| super::Nullity::from(it),
            }
        }
    }
    pub struct TestNestedNullityStmt(&'static str, Option<postgres::Statement>);
    pub fn test_nested_nullity() -> TestNestedNullityStmt {
        TestNestedNullityStmt(
            "SELECT composite FROM nullity WHERE composite IS NOT NULL",
            None,
        )
    }
    impl TestNestedNullityStmt {
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
        ) -> NullityCompositeQuery<'c, 'a, 's, C, crate::types::NullityComposite, 0> {
            NullityCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it.into(),
            }
        }
    }
    pub struct TestSingleNestedStmt(&'static str, Option<postgres::Statement>);
    pub fn test_single_nested() -> TestSingleNestedStmt {
        TestSingleNestedStmt(
            "SELECT composite FROM nullity WHERE composite IS NOT NULL LIMIT 1",
            None,
        )
    }
    impl TestSingleNestedStmt {
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
        ) -> NullityCompositeQuery<'c, 'a, 's, C, crate::types::NullityComposite, 0> {
            NullityCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it.into(),
            }
        }
    }
    pub struct TestNestedArrayStmt(&'static str, Option<postgres::Statement>);
    pub fn test_nested_array() -> TestNestedArrayStmt {
        TestNestedArrayStmt(
            "SELECT ARRAY[composite, composite] as composite FROM nullity WHERE composite IS NOT NULL LIMIT 1",
            None,
        )
    }
    impl TestNestedArrayStmt {
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
        ) -> VecNullityCompositeQuery<'c, 'a, 's, C, Vec<crate::types::NullityComposite>, 0>
        {
            VecNullityCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it.map(|v| v.into()).collect(),
            }
        }
    }
    pub struct TestNamedNestedStmt(&'static str, Option<postgres::Statement>);
    pub fn test_named_nested() -> TestNamedNestedStmt {
        TestNamedNestedStmt(
            "SELECT composite FROM nullity WHERE composite IS NOT NULL LIMIT 1",
            None,
        )
    }
    impl TestNamedNestedStmt {
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
        ) -> TestNestedCompositeQuery<'c, 'a, 's, C, super::TestNestedComposite, 0> {
            TestNestedCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |
                    row: &postgres::Row,
                | -> Result<super::TestNestedCompositeBorrowed, postgres::Error> {
                    Ok(super::TestNestedCompositeBorrowed {
                        composite: row.try_get(0)?,
                    })
                },
                mapper: |it| super::TestNestedComposite::from(it),
            }
        }
    }
    pub struct TestDirectNullityStmt(&'static str, Option<postgres::Statement>);
    pub fn test_direct_nullity() -> TestDirectNullityStmt {
        TestDirectNullityStmt(
            "SELECT composite FROM nullity WHERE composite IS NOT NULL LIMIT 1",
            None,
        )
    }
    impl TestDirectNullityStmt {
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
        ) -> OptionNullityCompositeQuery<'c, 'a, 's, C, Option<crate::types::NullityComposite>, 0>
        {
            OptionNullityCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it.map(|v| v.into()),
            }
        }
    }
    pub struct TestSingleDirectStmt(&'static str, Option<postgres::Statement>);
    pub fn test_single_direct() -> TestSingleDirectStmt {
        TestSingleDirectStmt(
            "SELECT composite FROM nullity WHERE composite IS NOT NULL LIMIT 1",
            None,
        )
    }
    impl TestSingleDirectStmt {
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
        ) -> OptionNullityCompositeQuery<'c, 'a, 's, C, Option<crate::types::NullityComposite>, 0>
        {
            OptionNullityCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it.map(|v| v.into()),
            }
        }
    }
    pub struct TestNamedDirectStmt(&'static str, Option<postgres::Statement>);
    pub fn test_named_direct() -> TestNamedDirectStmt {
        TestNamedDirectStmt(
            "SELECT composite FROM nullity WHERE composite IS NOT NULL LIMIT 1",
            None,
        )
    }
    impl TestNamedDirectStmt {
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
        ) -> TestDirectCompositeQuery<'c, 'a, 's, C, super::TestDirectComposite, 0> {
            TestDirectCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |
                    row: &postgres::Row,
                | -> Result<super::TestDirectCompositeBorrowed, postgres::Error> {
                    Ok(super::TestDirectCompositeBorrowed {
                        composite: row.try_get(0)?,
                    })
                },
                mapper: |it| super::TestDirectComposite::from(it),
            }
        }
    }
}
pub mod async_ {
    use crate::client::async_::GenericClient;
    use futures::{self, StreamExt, TryStreamExt};
    pub struct NullityQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor:
            fn(&tokio_postgres::Row) -> Result<super::NullityBorrowed, tokio_postgres::Error>,
        mapper: fn(super::NullityBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> NullityQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::NullityBorrowed) -> R,
        ) -> NullityQuery<'c, 'a, 's, C, R, N> {
            NullityQuery {
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
    pub struct NullityCompositeQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor: fn(
            &tokio_postgres::Row,
        )
            -> Result<crate::types::NullityCompositeBorrowed, tokio_postgres::Error>,
        mapper: fn(crate::types::NullityCompositeBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> NullityCompositeQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(crate::types::NullityCompositeBorrowed) -> R,
        ) -> NullityCompositeQuery<'c, 'a, 's, C, R, N> {
            NullityCompositeQuery {
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
    pub struct VecNullityCompositeQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor: fn(
            &tokio_postgres::Row,
        ) -> Result<
            crate::ArrayIterator<'_, crate::types::NullityCompositeBorrowed>,
            tokio_postgres::Error,
        >,
        mapper: fn(crate::ArrayIterator<'_, crate::types::NullityCompositeBorrowed>) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> VecNullityCompositeQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(crate::ArrayIterator<'_, crate::types::NullityCompositeBorrowed>) -> R,
        ) -> VecNullityCompositeQuery<'c, 'a, 's, C, R, N> {
            VecNullityCompositeQuery {
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
    pub struct TestNestedCompositeQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor: fn(
            &tokio_postgres::Row,
        ) -> Result<super::TestNestedCompositeBorrowed, tokio_postgres::Error>,
        mapper: fn(super::TestNestedCompositeBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> TestNestedCompositeQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::TestNestedCompositeBorrowed) -> R,
        ) -> TestNestedCompositeQuery<'c, 'a, 's, C, R, N> {
            TestNestedCompositeQuery {
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
    pub struct OptionNullityCompositeQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor:
            fn(
                &tokio_postgres::Row,
            )
                -> Result<Option<crate::types::NullityCompositeBorrowed>, tokio_postgres::Error>,
        mapper: fn(Option<crate::types::NullityCompositeBorrowed>) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> OptionNullityCompositeQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(Option<crate::types::NullityCompositeBorrowed>) -> R,
        ) -> OptionNullityCompositeQuery<'c, 'a, 's, C, R, N> {
            OptionNullityCompositeQuery {
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
    pub struct TestDirectCompositeQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor: fn(
            &tokio_postgres::Row,
        ) -> Result<super::TestDirectCompositeBorrowed, tokio_postgres::Error>,
        mapper: fn(super::TestDirectCompositeBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> TestDirectCompositeQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::TestDirectCompositeBorrowed) -> R,
        ) -> TestDirectCompositeQuery<'c, 'a, 's, C, R, N> {
            TestDirectCompositeQuery {
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
    pub struct NewNullityStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn new_nullity() -> NewNullityStmt {
        NewNullityStmt(
            "INSERT INTO nullity(texts, name, composite) VALUES ($1, $2, $3)",
            None,
        )
    }
    impl NewNullityStmt {
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
            T2: crate::ArraySql<Item = Option<T1>>,
            T3: crate::StringSql,
        >(
            &'s self,
            client: &'c C,
            texts: &'a T2,
            name: &'a T3,
            composite: &'a Option<crate::types::NullityCompositeParams<'a>>,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[texts, name, composite]).await
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
            'a,
            'a,
            super::NullityParams<'a, T1, T2, T3>,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for NewNullityStmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::NullityParams<'a, T1, T2, T3>,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.texts, &params.name, &params.composite))
        }
    }
    pub struct NullityStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn nullity() -> NullityStmt {
        NullityStmt("SELECT * FROM nullity", None)
    }
    impl NullityStmt {
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
        ) -> NullityQuery<'c, 'a, 's, C, super::Nullity, 0> {
            NullityQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |
                    row: &tokio_postgres::Row,
                | -> Result<super::NullityBorrowed, tokio_postgres::Error> {
                    Ok(super::NullityBorrowed {
                        texts: row.try_get(0)?,
                        name: row.try_get(1)?,
                        composite: row.try_get(2)?,
                    })
                },
                mapper: |it| super::Nullity::from(it),
            }
        }
    }
    pub struct TestNestedNullityStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn test_nested_nullity() -> TestNestedNullityStmt {
        TestNestedNullityStmt(
            "SELECT composite FROM nullity WHERE composite IS NOT NULL",
            None,
        )
    }
    impl TestNestedNullityStmt {
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
        ) -> NullityCompositeQuery<'c, 'a, 's, C, crate::types::NullityComposite, 0> {
            NullityCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it.into(),
            }
        }
    }
    pub struct TestSingleNestedStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn test_single_nested() -> TestSingleNestedStmt {
        TestSingleNestedStmt(
            "SELECT composite FROM nullity WHERE composite IS NOT NULL LIMIT 1",
            None,
        )
    }
    impl TestSingleNestedStmt {
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
        ) -> NullityCompositeQuery<'c, 'a, 's, C, crate::types::NullityComposite, 0> {
            NullityCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it.into(),
            }
        }
    }
    pub struct TestNestedArrayStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn test_nested_array() -> TestNestedArrayStmt {
        TestNestedArrayStmt(
            "SELECT ARRAY[composite, composite] as composite FROM nullity WHERE composite IS NOT NULL LIMIT 1",
            None,
        )
    }
    impl TestNestedArrayStmt {
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
        ) -> VecNullityCompositeQuery<'c, 'a, 's, C, Vec<crate::types::NullityComposite>, 0>
        {
            VecNullityCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it.map(|v| v.into()).collect(),
            }
        }
    }
    pub struct TestNamedNestedStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn test_named_nested() -> TestNamedNestedStmt {
        TestNamedNestedStmt(
            "SELECT composite FROM nullity WHERE composite IS NOT NULL LIMIT 1",
            None,
        )
    }
    impl TestNamedNestedStmt {
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
        ) -> TestNestedCompositeQuery<'c, 'a, 's, C, super::TestNestedComposite, 0> {
            TestNestedCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row: &tokio_postgres::Row| -> Result<
                    super::TestNestedCompositeBorrowed,
                    tokio_postgres::Error,
                > {
                    Ok(super::TestNestedCompositeBorrowed {
                        composite: row.try_get(0)?,
                    })
                },
                mapper: |it| super::TestNestedComposite::from(it),
            }
        }
    }
    pub struct TestDirectNullityStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn test_direct_nullity() -> TestDirectNullityStmt {
        TestDirectNullityStmt(
            "SELECT composite FROM nullity WHERE composite IS NOT NULL LIMIT 1",
            None,
        )
    }
    impl TestDirectNullityStmt {
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
        ) -> OptionNullityCompositeQuery<'c, 'a, 's, C, Option<crate::types::NullityComposite>, 0>
        {
            OptionNullityCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it.map(|v| v.into()),
            }
        }
    }
    pub struct TestSingleDirectStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn test_single_direct() -> TestSingleDirectStmt {
        TestSingleDirectStmt(
            "SELECT composite FROM nullity WHERE composite IS NOT NULL LIMIT 1",
            None,
        )
    }
    impl TestSingleDirectStmt {
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
        ) -> OptionNullityCompositeQuery<'c, 'a, 's, C, Option<crate::types::NullityComposite>, 0>
        {
            OptionNullityCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it.map(|v| v.into()),
            }
        }
    }
    pub struct TestNamedDirectStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn test_named_direct() -> TestNamedDirectStmt {
        TestNamedDirectStmt(
            "SELECT composite FROM nullity WHERE composite IS NOT NULL LIMIT 1",
            None,
        )
    }
    impl TestNamedDirectStmt {
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
        ) -> TestDirectCompositeQuery<'c, 'a, 's, C, super::TestDirectComposite, 0> {
            TestDirectCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row: &tokio_postgres::Row| -> Result<
                    super::TestDirectCompositeBorrowed,
                    tokio_postgres::Error,
                > {
                    Ok(super::TestDirectCompositeBorrowed {
                        composite: row.try_get(0)?,
                    })
                },
                mapper: |it| super::TestDirectComposite::from(it),
            }
        }
    }
}
