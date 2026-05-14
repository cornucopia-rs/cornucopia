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
#[derive(Clone, Copy, Debug)]
pub struct SemicolonInStringParams {
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct SemicolonInDollarQuoteParams {
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct SemicolonInTaggedDollarQuoteParams {
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct SemicolonInEscapeStringParams {
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Row {
    pub id: i32,
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct RowSpace {
    pub id: i32,
}
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
pub struct SelectComment {
    pub trick_y: String,
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
pub struct SelectCommentBorrowed<'a> {
    pub trick_y: &'a str,
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
impl<'a> From<SelectCommentBorrowed<'a>> for SelectComment {
    fn from(
        SelectCommentBorrowed {
            trick_y,
            r#async,
            r#enum,
        }: SelectCommentBorrowed<'a>,
    ) -> Self {
        Self {
            trick_y: trick_y.into(),
            r#async,
            r#enum,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SelectInlineComment {
    pub c1: String,
    pub c2: String,
    pub c3: String,
    pub c4: String,
    pub c5: String,
}
pub struct SelectInlineCommentBorrowed<'a> {
    pub c1: &'a str,
    pub c2: &'a str,
    pub c3: &'a str,
    pub c4: &'a str,
    pub c5: &'a str,
}
impl<'a> From<SelectInlineCommentBorrowed<'a>> for SelectInlineComment {
    fn from(
        SelectInlineCommentBorrowed { c1, c2, c3, c4, c5 }: SelectInlineCommentBorrowed<'a>,
    ) -> Self {
        Self {
            c1: c1.into(),
            c2: c2.into(),
            c3: c3.into(),
            c4: c4.into(),
            c5: c5.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SemicolonInComment {
    pub trick_y: String,
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
pub struct SemicolonInCommentBorrowed<'a> {
    pub trick_y: &'a str,
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
impl<'a> From<SemicolonInCommentBorrowed<'a>> for SemicolonInComment {
    fn from(
        SemicolonInCommentBorrowed {
            trick_y,
            r#async,
            r#enum,
        }: SemicolonInCommentBorrowed<'a>,
    ) -> Self {
        Self {
            trick_y: trick_y.into(),
            r#async,
            r#enum,
        }
    }
}
pub mod sync {
    use crate::client::sync::GenericClient;
    use postgres::fallible_iterator::FallibleIterator;
    pub struct CloneCompositeQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor:
            fn(&postgres::Row) -> Result<crate::types::CloneCompositeBorrowed, postgres::Error>,
        mapper: fn(crate::types::CloneCompositeBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> CloneCompositeQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(crate::types::CloneCompositeBorrowed) -> R,
        ) -> CloneCompositeQuery<'c, 'a, 's, C, R, N> {
            CloneCompositeQuery {
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
    pub struct Optioni32Query<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor: fn(&postgres::Row) -> Result<Option<i32>, postgres::Error>,
        mapper: fn(Option<i32>) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> Optioni32Query<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(Option<i32>) -> R) -> Optioni32Query<'c, 'a, 's, C, R, N> {
            Optioni32Query {
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
    pub struct RowQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor: fn(&postgres::Row) -> Result<super::Row, postgres::Error>,
        mapper: fn(super::Row) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> RowQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::Row) -> R) -> RowQuery<'c, 'a, 's, C, R, N> {
            RowQuery {
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
    pub struct RowSpaceQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor: fn(&postgres::Row) -> Result<super::RowSpace, postgres::Error>,
        mapper: fn(super::RowSpace) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> RowSpaceQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::RowSpace) -> R,
        ) -> RowSpaceQuery<'c, 'a, 's, C, R, N> {
            RowSpaceQuery {
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
    pub struct TypeofQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor: fn(&postgres::Row) -> Result<super::TypeofBorrowed, postgres::Error>,
        mapper: fn(super::TypeofBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> TypeofQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::TypeofBorrowed) -> R,
        ) -> TypeofQuery<'c, 'a, 's, C, R, N> {
            TypeofQuery {
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
    pub struct SelectCommentQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor: fn(&postgres::Row) -> Result<super::SelectCommentBorrowed, postgres::Error>,
        mapper: fn(super::SelectCommentBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> SelectCommentQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectCommentBorrowed) -> R,
        ) -> SelectCommentQuery<'c, 'a, 's, C, R, N> {
            SelectCommentQuery {
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
    pub struct SelectInlineCommentQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor:
            fn(&postgres::Row) -> Result<super::SelectInlineCommentBorrowed, postgres::Error>,
        mapper: fn(super::SelectInlineCommentBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> SelectInlineCommentQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectInlineCommentBorrowed) -> R,
        ) -> SelectInlineCommentQuery<'c, 'a, 's, C, R, N> {
            SelectInlineCommentQuery {
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
    pub struct SemicolonInCommentQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor: fn(&postgres::Row) -> Result<super::SemicolonInCommentBorrowed, postgres::Error>,
        mapper: fn(super::SemicolonInCommentBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> SemicolonInCommentQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SemicolonInCommentBorrowed) -> R,
        ) -> SemicolonInCommentQuery<'c, 'a, 's, C, R, N> {
            SemicolonInCommentQuery {
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
    pub struct SelectCompactStmt(&'static str, Option<postgres::Statement>);
    pub fn select_compact() -> SelectCompactStmt {
        SelectCompactStmt("SELECT * FROM clone", None)
    }
    impl SelectCompactStmt {
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
        ) -> CloneCompositeQuery<'c, 'a, 's, C, crate::types::CloneComposite, 0> {
            CloneCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it.into(),
            }
        }
    }
    pub struct SelectSpacedStmt(&'static str, Option<postgres::Statement>);
    pub fn select_spaced() -> SelectSpacedStmt {
        SelectSpacedStmt("SELECT * FROM clone", None)
    }
    impl SelectSpacedStmt {
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
        ) -> CloneCompositeQuery<'c, 'a, 's, C, crate::types::CloneComposite, 0> {
            CloneCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it.into(),
            }
        }
    }
    pub struct ImplicitCompactStmt(&'static str, Option<postgres::Statement>);
    pub fn implicit_compact() -> ImplicitCompactStmt {
        ImplicitCompactStmt(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
            None,
        )
    }
    impl ImplicitCompactStmt {
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
            name: &'a Option<T1>,
            price: &'a Option<f64>,
        ) -> Optioni32Query<'c, 'a, 's, C, Option<i32>, 2> {
            Optioni32Query {
                client,
                params: [name, price],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it,
            }
        }
    }
    impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::ImplicitCompactParams<T1>,
            Optioni32Query<'c, 'a, 's, C, Option<i32>, 2>,
            C,
        > for ImplicitCompactStmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::ImplicitCompactParams<T1>,
        ) -> Optioni32Query<'c, 'a, 's, C, Option<i32>, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub struct ImplicitSpacedStmt(&'static str, Option<postgres::Statement>);
    pub fn implicit_spaced() -> ImplicitSpacedStmt {
        ImplicitSpacedStmt(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
            None,
        )
    }
    impl ImplicitSpacedStmt {
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
            name: &'a Option<T1>,
            price: &'a Option<f64>,
        ) -> Optioni32Query<'c, 'a, 's, C, Option<i32>, 2> {
            Optioni32Query {
                client,
                params: [name, price],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it,
            }
        }
    }
    impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::ImplicitSpacedParams<T1>,
            Optioni32Query<'c, 'a, 's, C, Option<i32>, 2>,
            C,
        > for ImplicitSpacedStmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::ImplicitSpacedParams<T1>,
        ) -> Optioni32Query<'c, 'a, 's, C, Option<i32>, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub struct NamedCompactStmt(&'static str, Option<postgres::Statement>);
    pub fn named_compact() -> NamedCompactStmt {
        NamedCompactStmt(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
            None,
        )
    }
    impl NamedCompactStmt {
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
            price: &'a f64,
        ) -> RowQuery<'c, 'a, 's, C, super::Row, 2> {
            RowQuery {
                client,
                params: [name, price],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row: &postgres::Row| -> Result<super::Row, postgres::Error> {
                    Ok(super::Row {
                        id: row.try_get(0)?,
                    })
                },
                mapper: |it| super::Row::from(it),
            }
        }
    }
    impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::Params<T1>,
            RowQuery<'c, 'a, 's, C, super::Row, 2>,
            C,
        > for NamedCompactStmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::Params<T1>,
        ) -> RowQuery<'c, 'a, 's, C, super::Row, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub struct NamedSpacedStmt(&'static str, Option<postgres::Statement>);
    pub fn named_spaced() -> NamedSpacedStmt {
        NamedSpacedStmt(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
            None,
        )
    }
    impl NamedSpacedStmt {
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
            price: &'a f64,
        ) -> RowSpaceQuery<'c, 'a, 's, C, super::RowSpace, 2> {
            RowSpaceQuery {
                client,
                params: [name, price],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row: &postgres::Row| -> Result<super::RowSpace, postgres::Error> {
                    Ok(super::RowSpace {
                        id: row.try_get(0)?,
                    })
                },
                mapper: |it| super::RowSpace::from(it),
            }
        }
    }
    impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::ParamsSpace<T1>,
            RowSpaceQuery<'c, 'a, 's, C, super::RowSpace, 2>,
            C,
        > for NamedSpacedStmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::ParamsSpace<T1>,
        ) -> RowSpaceQuery<'c, 'a, 's, C, super::RowSpace, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub struct TrickySqlStmt(&'static str, Option<postgres::Statement>);
    pub fn tricky_sql() -> TrickySqlStmt {
        TrickySqlStmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a bind_param\\', $1, $2)",
            None,
        )
    }
    impl TrickySqlStmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[r#async, r#enum])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::TrickySqlParams,
            Result<u64, postgres::Error>,
            C,
        > for TrickySqlStmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::TrickySqlParams,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub struct TrickySql1Stmt(&'static str, Option<postgres::Statement>);
    pub fn tricky_sql1() -> TrickySql1Stmt {
        TrickySql1Stmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a :bind_param', $1, $2)",
            None,
        )
    }
    impl TrickySql1Stmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[r#async, r#enum])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::TrickySql1Params,
            Result<u64, postgres::Error>,
            C,
        > for TrickySql1Stmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::TrickySql1Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub struct TrickySql2Stmt(&'static str, Option<postgres::Statement>);
    pub fn tricky_sql2() -> TrickySql2Stmt {
        TrickySql2Stmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a '':bind_param''', $1, $2)",
            None,
        )
    }
    impl TrickySql2Stmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[r#async, r#enum])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::TrickySql2Params,
            Result<u64, postgres::Error>,
            C,
        > for TrickySql2Stmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::TrickySql2Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub struct TrickySql3Stmt(&'static str, Option<postgres::Statement>);
    pub fn tricky_sql3() -> TrickySql3Stmt {
        TrickySql3Stmt(
            "INSERT INTO syntax (\"trick:y\", async, enum)  VALUES ($$this is not a :bind_param$$, $1, $2)",
            None,
        )
    }
    impl TrickySql3Stmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[r#async, r#enum])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::TrickySql3Params,
            Result<u64, postgres::Error>,
            C,
        > for TrickySql3Stmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::TrickySql3Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub struct TrickySql4Stmt(&'static str, Option<postgres::Statement>);
    pub fn tricky_sql4() -> TrickySql4Stmt {
        TrickySql4Stmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ($tag$this is not a :bind_param$tag$, $1, $2)",
            None,
        )
    }
    impl TrickySql4Stmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[r#async, r#enum])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::TrickySql4Params,
            Result<u64, postgres::Error>,
            C,
        > for TrickySql4Stmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::TrickySql4Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub struct TrickySql6Stmt(&'static str, Option<postgres::Statement>);
    pub fn tricky_sql6() -> TrickySql6Stmt {
        TrickySql6Stmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES (e'this is not a '':bind_param''', $1, $2)",
            None,
        )
    }
    impl TrickySql6Stmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[r#async, r#enum])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::TrickySql6Params,
            Result<u64, postgres::Error>,
            C,
        > for TrickySql6Stmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::TrickySql6Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub struct TrickySql7Stmt(&'static str, Option<postgres::Statement>);
    pub fn tricky_sql7() -> TrickySql7Stmt {
        TrickySql7Stmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES (E'this is not a \\':bind_param\\'', $1, $2)",
            None,
        )
    }
    impl TrickySql7Stmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[r#async, r#enum])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::TrickySql7Params,
            Result<u64, postgres::Error>,
            C,
        > for TrickySql7Stmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::TrickySql7Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub struct TrickySql8Stmt(&'static str, Option<postgres::Statement>);
    pub fn tricky_sql8() -> TrickySql8Stmt {
        TrickySql8Stmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES (e'this is ''not'' a \\':bind_param\\'', $1, $2)",
            None,
        )
    }
    impl TrickySql8Stmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[r#async, r#enum])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::TrickySql8Params,
            Result<u64, postgres::Error>,
            C,
        > for TrickySql8Stmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::TrickySql8Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub struct TrickySql9Stmt(&'static str, Option<postgres::Statement>);
    pub fn tricky_sql9() -> TrickySql9Stmt {
        TrickySql9Stmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES (E'this is \\'not\\' a \\':bind_param\\'', $1, $2)",
            None,
        )
    }
    impl TrickySql9Stmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[r#async, r#enum])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::TrickySql9Params,
            Result<u64, postgres::Error>,
            C,
        > for TrickySql9Stmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::TrickySql9Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub struct TrickySql10Stmt(&'static str, Option<postgres::Statement>);
    pub fn tricky_sql10() -> TrickySql10Stmt {
        TrickySql10Stmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is just a cast'::text, $1, $2)",
            None,
        )
    }
    impl TrickySql10Stmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[r#async, r#enum])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::TrickySql10Params,
            Result<u64, postgres::Error>,
            C,
        > for TrickySql10Stmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::TrickySql10Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub struct RTypeofStmt(&'static str, Option<postgres::Statement>);
    pub fn r#typeof() -> RTypeofStmt {
        RTypeofStmt("SELECT * FROM syntax", None)
    }
    impl RTypeofStmt {
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
        ) -> TypeofQuery<'c, 'a, 's, C, super::Typeof, 0> {
            TypeofQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row: &postgres::Row| -> Result<super::TypeofBorrowed, postgres::Error> {
                    Ok(super::TypeofBorrowed {
                        trick_y: row.try_get(0)?,
                        r#async: row.try_get(1)?,
                        r#enum: row.try_get(2)?,
                    })
                },
                mapper: |it| super::Typeof::from(it),
            }
        }
    }
    pub struct SelectCommentStmt(&'static str, Option<postgres::Statement>);
    /// Multi line
    ///
    /// Doc string comment
    pub fn select_comment() -> SelectCommentStmt {
        SelectCommentStmt("SELECT * FROM syntax", None)
    }
    impl SelectCommentStmt {
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
        ) -> SelectCommentQuery<'c, 'a, 's, C, super::SelectComment, 0> {
            SelectCommentQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor:
                    |row: &postgres::Row| -> Result<super::SelectCommentBorrowed, postgres::Error> {
                        Ok(super::SelectCommentBorrowed {
                            trick_y: row.try_get(0)?,
                            r#async: row.try_get(1)?,
                            r#enum: row.try_get(2)?,
                        })
                    },
                mapper: |it| super::SelectComment::from(it),
            }
        }
    }
    pub struct SelectInlineCommentStmt(&'static str, Option<postgres::Statement>);
    pub fn select_inline_comment() -> SelectInlineCommentStmt {
        SelectInlineCommentStmt(
            "SELECT '-- dont remove this\\n' as c1, $$-- or this$$ as c2, E'-- escape string here' as c3, e'-- another escape string' as c4, $tag$-- dollar quoted here$tag$ as c5",
            None,
        )
    }
    impl SelectInlineCommentStmt {
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
        ) -> SelectInlineCommentQuery<'c, 'a, 's, C, super::SelectInlineComment, 0> {
            SelectInlineCommentQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |
                    row: &postgres::Row,
                | -> Result<super::SelectInlineCommentBorrowed, postgres::Error> {
                    Ok(super::SelectInlineCommentBorrowed {
                        c1: row.try_get(0)?,
                        c2: row.try_get(1)?,
                        c3: row.try_get(2)?,
                        c4: row.try_get(3)?,
                        c5: row.try_get(4)?,
                    })
                },
                mapper: |it| super::SelectInlineComment::from(it),
            }
        }
    }
    pub struct SemicolonInStringStmt(&'static str, Option<postgres::Statement>);
    pub fn semicolon_in_string() -> SemicolonInStringStmt {
        SemicolonInStringStmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('a; b', $1, $2)",
            None,
        )
    }
    impl SemicolonInStringStmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[r#async, r#enum])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::SemicolonInStringParams,
            Result<u64, postgres::Error>,
            C,
        > for SemicolonInStringStmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::SemicolonInStringParams,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub struct SemicolonInDollarQuoteStmt(&'static str, Option<postgres::Statement>);
    pub fn semicolon_in_dollar_quote() -> SemicolonInDollarQuoteStmt {
        SemicolonInDollarQuoteStmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ($$a; b$$, $1, $2)",
            None,
        )
    }
    impl SemicolonInDollarQuoteStmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[r#async, r#enum])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::SemicolonInDollarQuoteParams,
            Result<u64, postgres::Error>,
            C,
        > for SemicolonInDollarQuoteStmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::SemicolonInDollarQuoteParams,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub struct SemicolonInTaggedDollarQuoteStmt(&'static str, Option<postgres::Statement>);
    pub fn semicolon_in_tagged_dollar_quote() -> SemicolonInTaggedDollarQuoteStmt {
        SemicolonInTaggedDollarQuoteStmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ($tag$a; b$tag$, $1, $2)",
            None,
        )
    }
    impl SemicolonInTaggedDollarQuoteStmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[r#async, r#enum])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::SemicolonInTaggedDollarQuoteParams,
            Result<u64, postgres::Error>,
            C,
        > for SemicolonInTaggedDollarQuoteStmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::SemicolonInTaggedDollarQuoteParams,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub struct SemicolonInEscapeStringStmt(&'static str, Option<postgres::Statement>);
    pub fn semicolon_in_escape_string() -> SemicolonInEscapeStringStmt {
        SemicolonInEscapeStringStmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES (E'a\\; b', $1, $2)",
            None,
        )
    }
    impl SemicolonInEscapeStringStmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[r#async, r#enum])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::SemicolonInEscapeStringParams,
            Result<u64, postgres::Error>,
            C,
        > for SemicolonInEscapeStringStmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::SemicolonInEscapeStringParams,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub struct SemicolonInCommentStmt(&'static str, Option<postgres::Statement>);
    pub fn semicolon_in_comment() -> SemicolonInCommentStmt {
        SemicolonInCommentStmt("SELECT * FROM syntax", None)
    }
    impl SemicolonInCommentStmt {
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
        ) -> SemicolonInCommentQuery<'c, 'a, 's, C, super::SemicolonInComment, 0> {
            SemicolonInCommentQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |
                    row: &postgres::Row,
                | -> Result<super::SemicolonInCommentBorrowed, postgres::Error> {
                    Ok(super::SemicolonInCommentBorrowed {
                        trick_y: row.try_get(0)?,
                        r#async: row.try_get(1)?,
                        r#enum: row.try_get(2)?,
                    })
                },
                mapper: |it| super::SemicolonInComment::from(it),
            }
        }
    }
}
pub mod async_ {
    use crate::client::async_::GenericClient;
    use futures::{self, StreamExt, TryStreamExt};
    pub struct CloneCompositeQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor: fn(
            &tokio_postgres::Row,
        )
            -> Result<crate::types::CloneCompositeBorrowed, tokio_postgres::Error>,
        mapper: fn(crate::types::CloneCompositeBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> CloneCompositeQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(crate::types::CloneCompositeBorrowed) -> R,
        ) -> CloneCompositeQuery<'c, 'a, 's, C, R, N> {
            CloneCompositeQuery {
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
    pub struct Optioni32Query<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor: fn(&tokio_postgres::Row) -> Result<Option<i32>, tokio_postgres::Error>,
        mapper: fn(Option<i32>) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> Optioni32Query<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(Option<i32>) -> R) -> Optioni32Query<'c, 'a, 's, C, R, N> {
            Optioni32Query {
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
    pub struct RowQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor: fn(&tokio_postgres::Row) -> Result<super::Row, tokio_postgres::Error>,
        mapper: fn(super::Row) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> RowQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::Row) -> R) -> RowQuery<'c, 'a, 's, C, R, N> {
            RowQuery {
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
    pub struct RowSpaceQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor: fn(&tokio_postgres::Row) -> Result<super::RowSpace, tokio_postgres::Error>,
        mapper: fn(super::RowSpace) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> RowSpaceQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::RowSpace) -> R,
        ) -> RowSpaceQuery<'c, 'a, 's, C, R, N> {
            RowSpaceQuery {
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
    pub struct TypeofQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor: fn(&tokio_postgres::Row) -> Result<super::TypeofBorrowed, tokio_postgres::Error>,
        mapper: fn(super::TypeofBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> TypeofQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::TypeofBorrowed) -> R,
        ) -> TypeofQuery<'c, 'a, 's, C, R, N> {
            TypeofQuery {
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
    pub struct SelectCommentQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor:
            fn(&tokio_postgres::Row) -> Result<super::SelectCommentBorrowed, tokio_postgres::Error>,
        mapper: fn(super::SelectCommentBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> SelectCommentQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectCommentBorrowed) -> R,
        ) -> SelectCommentQuery<'c, 'a, 's, C, R, N> {
            SelectCommentQuery {
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
    pub struct SelectInlineCommentQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor: fn(
            &tokio_postgres::Row,
        ) -> Result<super::SelectInlineCommentBorrowed, tokio_postgres::Error>,
        mapper: fn(super::SelectInlineCommentBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> SelectInlineCommentQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectInlineCommentBorrowed) -> R,
        ) -> SelectInlineCommentQuery<'c, 'a, 's, C, R, N> {
            SelectInlineCommentQuery {
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
    pub struct SemicolonInCommentQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor: fn(
            &tokio_postgres::Row,
        ) -> Result<super::SemicolonInCommentBorrowed, tokio_postgres::Error>,
        mapper: fn(super::SemicolonInCommentBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> SemicolonInCommentQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SemicolonInCommentBorrowed) -> R,
        ) -> SemicolonInCommentQuery<'c, 'a, 's, C, R, N> {
            SemicolonInCommentQuery {
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
    pub struct SelectCompactStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn select_compact() -> SelectCompactStmt {
        SelectCompactStmt("SELECT * FROM clone", None)
    }
    impl SelectCompactStmt {
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
        ) -> CloneCompositeQuery<'c, 'a, 's, C, crate::types::CloneComposite, 0> {
            CloneCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it.into(),
            }
        }
    }
    pub struct SelectSpacedStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn select_spaced() -> SelectSpacedStmt {
        SelectSpacedStmt("SELECT * FROM clone", None)
    }
    impl SelectSpacedStmt {
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
        ) -> CloneCompositeQuery<'c, 'a, 's, C, crate::types::CloneComposite, 0> {
            CloneCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it.into(),
            }
        }
    }
    pub struct ImplicitCompactStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn implicit_compact() -> ImplicitCompactStmt {
        ImplicitCompactStmt(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
            None,
        )
    }
    impl ImplicitCompactStmt {
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
            name: &'a Option<T1>,
            price: &'a Option<f64>,
        ) -> Optioni32Query<'c, 'a, 's, C, Option<i32>, 2> {
            Optioni32Query {
                client,
                params: [name, price],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it,
            }
        }
    }
    impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
        crate::client::async_::Params<
            'c,
            'a,
            's,
            super::ImplicitCompactParams<T1>,
            Optioni32Query<'c, 'a, 's, C, Option<i32>, 2>,
            C,
        > for ImplicitCompactStmt
    {
        fn params(
            &'s self,
            client: &'c C,
            params: &'a super::ImplicitCompactParams<T1>,
        ) -> Optioni32Query<'c, 'a, 's, C, Option<i32>, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub struct ImplicitSpacedStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn implicit_spaced() -> ImplicitSpacedStmt {
        ImplicitSpacedStmt(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
            None,
        )
    }
    impl ImplicitSpacedStmt {
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
            name: &'a Option<T1>,
            price: &'a Option<f64>,
        ) -> Optioni32Query<'c, 'a, 's, C, Option<i32>, 2> {
            Optioni32Query {
                client,
                params: [name, price],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it,
            }
        }
    }
    impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
        crate::client::async_::Params<
            'c,
            'a,
            's,
            super::ImplicitSpacedParams<T1>,
            Optioni32Query<'c, 'a, 's, C, Option<i32>, 2>,
            C,
        > for ImplicitSpacedStmt
    {
        fn params(
            &'s self,
            client: &'c C,
            params: &'a super::ImplicitSpacedParams<T1>,
        ) -> Optioni32Query<'c, 'a, 's, C, Option<i32>, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub struct NamedCompactStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn named_compact() -> NamedCompactStmt {
        NamedCompactStmt(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
            None,
        )
    }
    impl NamedCompactStmt {
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
            price: &'a f64,
        ) -> RowQuery<'c, 'a, 's, C, super::Row, 2> {
            RowQuery {
                client,
                params: [name, price],
                query: self.0,
                cached: self.1.as_ref(),
                extractor:
                    |row: &tokio_postgres::Row| -> Result<super::Row, tokio_postgres::Error> {
                        Ok(super::Row {
                            id: row.try_get(0)?,
                        })
                    },
                mapper: |it| super::Row::from(it),
            }
        }
    }
    impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
        crate::client::async_::Params<
            'c,
            'a,
            's,
            super::Params<T1>,
            RowQuery<'c, 'a, 's, C, super::Row, 2>,
            C,
        > for NamedCompactStmt
    {
        fn params(
            &'s self,
            client: &'c C,
            params: &'a super::Params<T1>,
        ) -> RowQuery<'c, 'a, 's, C, super::Row, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub struct NamedSpacedStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn named_spaced() -> NamedSpacedStmt {
        NamedSpacedStmt(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
            None,
        )
    }
    impl NamedSpacedStmt {
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
            price: &'a f64,
        ) -> RowSpaceQuery<'c, 'a, 's, C, super::RowSpace, 2> {
            RowSpaceQuery {
                client,
                params: [name, price],
                query: self.0,
                cached: self.1.as_ref(),
                extractor:
                    |row: &tokio_postgres::Row| -> Result<super::RowSpace, tokio_postgres::Error> {
                        Ok(super::RowSpace {
                            id: row.try_get(0)?,
                        })
                    },
                mapper: |it| super::RowSpace::from(it),
            }
        }
    }
    impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
        crate::client::async_::Params<
            'c,
            'a,
            's,
            super::ParamsSpace<T1>,
            RowSpaceQuery<'c, 'a, 's, C, super::RowSpace, 2>,
            C,
        > for NamedSpacedStmt
    {
        fn params(
            &'s self,
            client: &'c C,
            params: &'a super::ParamsSpace<T1>,
        ) -> RowSpaceQuery<'c, 'a, 's, C, super::RowSpace, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub struct TrickySqlStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn tricky_sql() -> TrickySqlStmt {
        TrickySqlStmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a bind_param\\', $1, $2)",
            None,
        )
    }
    impl TrickySqlStmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::TrickySqlParams,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySqlStmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::TrickySqlParams,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub struct TrickySql1Stmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn tricky_sql1() -> TrickySql1Stmt {
        TrickySql1Stmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a :bind_param', $1, $2)",
            None,
        )
    }
    impl TrickySql1Stmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::TrickySql1Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql1Stmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::TrickySql1Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub struct TrickySql2Stmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn tricky_sql2() -> TrickySql2Stmt {
        TrickySql2Stmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a '':bind_param''', $1, $2)",
            None,
        )
    }
    impl TrickySql2Stmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::TrickySql2Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql2Stmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::TrickySql2Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub struct TrickySql3Stmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn tricky_sql3() -> TrickySql3Stmt {
        TrickySql3Stmt(
            "INSERT INTO syntax (\"trick:y\", async, enum)  VALUES ($$this is not a :bind_param$$, $1, $2)",
            None,
        )
    }
    impl TrickySql3Stmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::TrickySql3Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql3Stmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::TrickySql3Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub struct TrickySql4Stmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn tricky_sql4() -> TrickySql4Stmt {
        TrickySql4Stmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ($tag$this is not a :bind_param$tag$, $1, $2)",
            None,
        )
    }
    impl TrickySql4Stmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::TrickySql4Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql4Stmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::TrickySql4Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub struct TrickySql6Stmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn tricky_sql6() -> TrickySql6Stmt {
        TrickySql6Stmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES (e'this is not a '':bind_param''', $1, $2)",
            None,
        )
    }
    impl TrickySql6Stmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::TrickySql6Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql6Stmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::TrickySql6Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub struct TrickySql7Stmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn tricky_sql7() -> TrickySql7Stmt {
        TrickySql7Stmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES (E'this is not a \\':bind_param\\'', $1, $2)",
            None,
        )
    }
    impl TrickySql7Stmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::TrickySql7Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql7Stmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::TrickySql7Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub struct TrickySql8Stmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn tricky_sql8() -> TrickySql8Stmt {
        TrickySql8Stmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES (e'this is ''not'' a \\':bind_param\\'', $1, $2)",
            None,
        )
    }
    impl TrickySql8Stmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::TrickySql8Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql8Stmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::TrickySql8Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub struct TrickySql9Stmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn tricky_sql9() -> TrickySql9Stmt {
        TrickySql9Stmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES (E'this is \\'not\\' a \\':bind_param\\'', $1, $2)",
            None,
        )
    }
    impl TrickySql9Stmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::TrickySql9Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql9Stmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::TrickySql9Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub struct TrickySql10Stmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn tricky_sql10() -> TrickySql10Stmt {
        TrickySql10Stmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is just a cast'::text, $1, $2)",
            None,
        )
    }
    impl TrickySql10Stmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::TrickySql10Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql10Stmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::TrickySql10Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub struct RTypeofStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn r#typeof() -> RTypeofStmt {
        RTypeofStmt("SELECT * FROM syntax", None)
    }
    impl RTypeofStmt {
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
        ) -> TypeofQuery<'c, 'a, 's, C, super::Typeof, 0> {
            TypeofQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |
                    row: &tokio_postgres::Row,
                | -> Result<super::TypeofBorrowed, tokio_postgres::Error> {
                    Ok(super::TypeofBorrowed {
                        trick_y: row.try_get(0)?,
                        r#async: row.try_get(1)?,
                        r#enum: row.try_get(2)?,
                    })
                },
                mapper: |it| super::Typeof::from(it),
            }
        }
    }
    pub struct SelectCommentStmt(&'static str, Option<tokio_postgres::Statement>);
    /// Multi line
    ///
    /// Doc string comment
    pub fn select_comment() -> SelectCommentStmt {
        SelectCommentStmt("SELECT * FROM syntax", None)
    }
    impl SelectCommentStmt {
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
        ) -> SelectCommentQuery<'c, 'a, 's, C, super::SelectComment, 0> {
            SelectCommentQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |
                    row: &tokio_postgres::Row,
                | -> Result<super::SelectCommentBorrowed, tokio_postgres::Error> {
                    Ok(super::SelectCommentBorrowed {
                        trick_y: row.try_get(0)?,
                        r#async: row.try_get(1)?,
                        r#enum: row.try_get(2)?,
                    })
                },
                mapper: |it| super::SelectComment::from(it),
            }
        }
    }
    pub struct SelectInlineCommentStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn select_inline_comment() -> SelectInlineCommentStmt {
        SelectInlineCommentStmt(
            "SELECT '-- dont remove this\\n' as c1, $$-- or this$$ as c2, E'-- escape string here' as c3, e'-- another escape string' as c4, $tag$-- dollar quoted here$tag$ as c5",
            None,
        )
    }
    impl SelectInlineCommentStmt {
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
        ) -> SelectInlineCommentQuery<'c, 'a, 's, C, super::SelectInlineComment, 0> {
            SelectInlineCommentQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row: &tokio_postgres::Row| -> Result<
                    super::SelectInlineCommentBorrowed,
                    tokio_postgres::Error,
                > {
                    Ok(super::SelectInlineCommentBorrowed {
                        c1: row.try_get(0)?,
                        c2: row.try_get(1)?,
                        c3: row.try_get(2)?,
                        c4: row.try_get(3)?,
                        c5: row.try_get(4)?,
                    })
                },
                mapper: |it| super::SelectInlineComment::from(it),
            }
        }
    }
    pub struct SemicolonInStringStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn semicolon_in_string() -> SemicolonInStringStmt {
        SemicolonInStringStmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('a; b', $1, $2)",
            None,
        )
    }
    impl SemicolonInStringStmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::SemicolonInStringParams,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for SemicolonInStringStmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::SemicolonInStringParams,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub struct SemicolonInDollarQuoteStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn semicolon_in_dollar_quote() -> SemicolonInDollarQuoteStmt {
        SemicolonInDollarQuoteStmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ($$a; b$$, $1, $2)",
            None,
        )
    }
    impl SemicolonInDollarQuoteStmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::SemicolonInDollarQuoteParams,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for SemicolonInDollarQuoteStmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::SemicolonInDollarQuoteParams,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub struct SemicolonInTaggedDollarQuoteStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn semicolon_in_tagged_dollar_quote() -> SemicolonInTaggedDollarQuoteStmt {
        SemicolonInTaggedDollarQuoteStmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ($tag$a; b$tag$, $1, $2)",
            None,
        )
    }
    impl SemicolonInTaggedDollarQuoteStmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::SemicolonInTaggedDollarQuoteParams,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for SemicolonInTaggedDollarQuoteStmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::SemicolonInTaggedDollarQuoteParams,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub struct SemicolonInEscapeStringStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn semicolon_in_escape_string() -> SemicolonInEscapeStringStmt {
        SemicolonInEscapeStringStmt(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES (E'a\\; b', $1, $2)",
            None,
        )
    }
    impl SemicolonInEscapeStringStmt {
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
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::SemicolonInEscapeStringParams,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for SemicolonInEscapeStringStmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::SemicolonInEscapeStringParams,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub struct SemicolonInCommentStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn semicolon_in_comment() -> SemicolonInCommentStmt {
        SemicolonInCommentStmt("SELECT * FROM syntax", None)
    }
    impl SemicolonInCommentStmt {
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
        ) -> SemicolonInCommentQuery<'c, 'a, 's, C, super::SemicolonInComment, 0> {
            SemicolonInCommentQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |
                    row: &tokio_postgres::Row,
                | -> Result<super::SemicolonInCommentBorrowed, tokio_postgres::Error> {
                    Ok(super::SemicolonInCommentBorrowed {
                        trick_y: row.try_get(0)?,
                        r#async: row.try_get(1)?,
                        r#enum: row.try_get(2)?,
                    })
                },
                mapper: |it| super::SemicolonInComment::from(it),
            }
        }
    }
}
