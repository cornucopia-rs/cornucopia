// This file was generated with `cornucopia`. Do not modify.
#![allow(clippy::all, clippy::pedantic)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
pub mod types {
    pub mod public {
        #[derive(
            Debug, postgres_types::ToSql, postgres_types::FromSql, Clone, Copy, PartialEq, Eq,
        )]
        #[postgres(name = "spongebob_character")]
        pub enum SpongebobCharacter {
            Bob,
            Patrick,
            Squidward,
        }
    }
}
pub mod queries {
    pub mod module_1 {
        use postgres::{fallible_iterator::FallibleIterator, GenericClient};
        pub fn insert_book() -> InsertBookStmt {
            InsertBookStmt(cornucopia_sync::private::Stmt::new(
                "INSERT INTO Book (title)
  VALUES ($1)",
            ))
        }
        pub struct InsertBookStmt(cornucopia_sync::private::Stmt);
        impl InsertBookStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                title: &'a impl cornucopia_sync::StringSql,
            ) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(stmt, &[title])
            }
        }
    }
    pub mod module_2 {
        use postgres::{fallible_iterator::FallibleIterator, GenericClient};
        #[derive(Debug)]
        pub struct AuthorNameStartingWithParams<'a> {
            pub start_str: &'a str,
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Authors {
            pub id: i32,
            pub name: String,
            pub country: String,
        }
        pub struct AuthorsBorrowed<'a> {
            pub id: i32,
            pub name: &'a str,
            pub country: &'a str,
        }
        impl<'a> From<AuthorsBorrowed<'a>> for Authors {
            fn from(AuthorsBorrowed { id, name, country }: AuthorsBorrowed<'a>) -> Self {
                Self {
                    id,
                    name: name.into(),
                    country: country.into(),
                }
            }
        }
        pub struct AuthorsQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor: fn(&postgres::Row) -> AuthorsBorrowed,
            mapper: fn(AuthorsBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> AuthorsQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(AuthorsBorrowed) -> R) -> AuthorsQuery<'a, C, R, N> {
                AuthorsQuery {
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
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }

        pub struct StringQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor: fn(&postgres::Row) -> &str,
            mapper: fn(&str) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> StringQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(&str) -> R) -> StringQuery<'a, C, R, N> {
                StringQuery {
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
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct AuthorNameStartingWith {
            pub authorid: i32,
            pub name: String,
            pub bookid: i32,
            pub title: String,
        }
        pub struct AuthorNameStartingWithBorrowed<'a> {
            pub authorid: i32,
            pub name: &'a str,
            pub bookid: i32,
            pub title: &'a str,
        }
        impl<'a> From<AuthorNameStartingWithBorrowed<'a>> for AuthorNameStartingWith {
            fn from(
                AuthorNameStartingWithBorrowed {
                    authorid,
                    name,
                    bookid,
                    title,
                }: AuthorNameStartingWithBorrowed<'a>,
            ) -> Self {
                Self {
                    authorid,
                    name: name.into(),
                    bookid,
                    title: title.into(),
                }
            }
        }
        pub struct AuthorNameStartingWithQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor: fn(&postgres::Row) -> AuthorNameStartingWithBorrowed,
            mapper: fn(AuthorNameStartingWithBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> AuthorNameStartingWithQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(AuthorNameStartingWithBorrowed) -> R,
            ) -> AuthorNameStartingWithQuery<'a, C, R, N> {
                AuthorNameStartingWithQuery {
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
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }

        pub struct SuperSuperTypesPublicSpongebobCharacterQuery<
            'a,
            C: GenericClient,
            T,
            const N: usize,
        > {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor: fn(&postgres::Row) -> super::super::types::public::SpongebobCharacter,
            mapper: fn(super::super::types::public::SpongebobCharacter) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SuperSuperTypesPublicSpongebobCharacterQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(super::super::types::public::SpongebobCharacter) -> R,
            ) -> SuperSuperTypesPublicSpongebobCharacterQuery<'a, C, R, N> {
                SuperSuperTypesPublicSpongebobCharacterQuery {
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
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SelectTranslations {
            pub title: String,
            pub translations: Vec<String>,
        }
        pub struct SelectTranslationsBorrowed<'a> {
            pub title: &'a str,
            pub translations: cornucopia_sync::ArrayIterator<'a, &'a str>,
        }
        impl<'a> From<SelectTranslationsBorrowed<'a>> for SelectTranslations {
            fn from(
                SelectTranslationsBorrowed {
                    title,
                    translations,
                }: SelectTranslationsBorrowed<'a>,
            ) -> Self {
                Self {
                    title: title.into(),
                    translations: translations.map(|v| v.into()).collect(),
                }
            }
        }
        pub struct SelectTranslationsQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor: fn(&postgres::Row) -> SelectTranslationsBorrowed,
            mapper: fn(SelectTranslationsBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SelectTranslationsQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(SelectTranslationsBorrowed) -> R,
            ) -> SelectTranslationsQuery<'a, C, R, N> {
                SelectTranslationsQuery {
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
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }
        pub fn authors() -> AuthorsStmt {
            AuthorsStmt(cornucopia_sync::private::Stmt::new(
                "SELECT
    *
FROM
    Author",
            ))
        }
        pub struct AuthorsStmt(cornucopia_sync::private::Stmt);
        impl AuthorsStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
            ) -> AuthorsQuery<'a, C, Authors, 0> {
                AuthorsQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| AuthorsBorrowed {
                        id: row.get(0),
                        name: row.get(1),
                        country: row.get(2),
                    },
                    mapper: |it| <Authors>::from(it),
                }
            }
        }
        pub fn books() -> BooksStmt {
            BooksStmt(cornucopia_sync::private::Stmt::new(
                "SELECT
    Title
FROM
    Book",
            ))
        }
        pub struct BooksStmt(cornucopia_sync::private::Stmt);
        impl BooksStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
            ) -> StringQuery<'a, C, String, 0> {
                StringQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it.into(),
                }
            }
        }
        pub fn author_name_by_id() -> AuthorNameByIdStmt {
            AuthorNameByIdStmt(cornucopia_sync::private::Stmt::new(
                "SELECT
    Author.Name
FROM
    Author
WHERE
    Author.Id = $1",
            ))
        }
        pub struct AuthorNameByIdStmt(cornucopia_sync::private::Stmt);
        impl AuthorNameByIdStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                id: &'a i32,
            ) -> StringQuery<'a, C, String, 1> {
                StringQuery {
                    client,
                    params: [id],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it.into(),
                }
            }
        }
        pub fn author_name_starting_with() -> AuthorNameStartingWithStmt {
            AuthorNameStartingWithStmt(cornucopia_sync::private::Stmt::new(
                "SELECT
    BookAuthor.AuthorId,
    Author.Name,
    BookAuthor.BookId,
    Book.Title
FROM
    BookAuthor
    INNER JOIN Author ON Author.id = BookAuthor.AuthorId
    INNER JOIN Book ON Book.Id = BookAuthor.BookId
WHERE
    Author.Name LIKE CONCAT($1::text, '%')",
            ))
        }
        pub struct AuthorNameStartingWithStmt(cornucopia_sync::private::Stmt);
        impl AuthorNameStartingWithStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                start_str: &'a impl cornucopia_sync::StringSql,
            ) -> AuthorNameStartingWithQuery<'a, C, AuthorNameStartingWith, 1> {
                AuthorNameStartingWithQuery {
                    client,
                    params: [start_str],
                    stmt: &mut self.0,
                    extractor: |row| AuthorNameStartingWithBorrowed {
                        authorid: row.get(0),
                        name: row.get(1),
                        bookid: row.get(2),
                        title: row.get(3),
                    },
                    mapper: |it| <AuthorNameStartingWith>::from(it),
                }
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_sync::Params<
                'a,
                AuthorNameStartingWithParams<'a>,
                AuthorNameStartingWithQuery<'a, C, AuthorNameStartingWith, 1>,
                C,
            > for AuthorNameStartingWithStmt
        {
            fn params(
                &'a mut self,
                client: &'a mut C,
                params: &'a AuthorNameStartingWithParams<'a>,
            ) -> AuthorNameStartingWithQuery<'a, C, AuthorNameStartingWith, 1> {
                self.bind(client, &params.start_str)
            }
        }
        pub fn select_where_custom_type() -> SelectWhereCustomTypeStmt {
            SelectWhereCustomTypeStmt(cornucopia_sync::private::Stmt::new(
                "SELECT
    col2
FROM
    CustomTable
WHERE (col1).persona = $1",
            ))
        }
        pub struct SelectWhereCustomTypeStmt(cornucopia_sync::private::Stmt);
        impl SelectWhereCustomTypeStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                spongebob_character: &'a super::super::types::public::SpongebobCharacter,
            ) -> SuperSuperTypesPublicSpongebobCharacterQuery<
                'a,
                C,
                super::super::types::public::SpongebobCharacter,
                1,
            > {
                SuperSuperTypesPublicSpongebobCharacterQuery {
                    client,
                    params: [spongebob_character],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it,
                }
            }
        }
        pub fn select_translations() -> SelectTranslationsStmt {
            SelectTranslationsStmt(cornucopia_sync::private::Stmt::new(
                "SELECT
    Title,
    Translations
FROM
    Book",
            ))
        }
        pub struct SelectTranslationsStmt(cornucopia_sync::private::Stmt);
        impl SelectTranslationsStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
            ) -> SelectTranslationsQuery<'a, C, SelectTranslations, 0> {
                SelectTranslationsQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| SelectTranslationsBorrowed {
                        title: row.get(0),
                        translations: row.get(1),
                    },
                    mapper: |it| <SelectTranslations>::from(it),
                }
            }
        }
    }
}
