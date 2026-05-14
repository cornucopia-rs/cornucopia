// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct AuthorNameStartingWithParams<T1: crate::StringSql> {
    pub start_str: T1,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Authors {
    pub id: i32,
    pub name: String,
    pub country: String,
    pub dob: chrono::NaiveDate,
}
pub struct AuthorsBorrowed<'a> {
    pub id: i32,
    pub name: &'a str,
    pub country: &'a str,
    pub dob: chrono::NaiveDate,
}
impl<'a> From<AuthorsBorrowed<'a>> for Authors {
    fn from(
        AuthorsBorrowed {
            id,
            name,
            country,
            dob,
        }: AuthorsBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            country: country.into(),
            dob,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct AuthorNameStartingWith {
    pub author_id: i32,
    pub name: String,
    pub book_id: i32,
    pub title: String,
}
pub struct AuthorNameStartingWithBorrowed<'a> {
    pub author_id: i32,
    pub name: &'a str,
    pub book_id: i32,
    pub title: &'a str,
}
impl<'a> From<AuthorNameStartingWithBorrowed<'a>> for AuthorNameStartingWith {
    fn from(
        AuthorNameStartingWithBorrowed {
            author_id,
            name,
            book_id,
            title,
        }: AuthorNameStartingWithBorrowed<'a>,
    ) -> Self {
        Self {
            author_id,
            name: name.into(),
            book_id,
            title: title.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SelectTranslations {
    pub title: String,
    pub translations: Vec<String>,
}
pub struct SelectTranslationsBorrowed<'a> {
    pub title: &'a str,
    pub translations: crate::ArrayIterator<'a, &'a str>,
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
use crate::client::sync::GenericClient;
use postgres::fallible_iterator::FallibleIterator;
pub struct AuthorsQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c mut C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s postgres::Statement>,
    extractor: fn(&postgres::Row) -> Result<AuthorsBorrowed, postgres::Error>,
    mapper: fn(AuthorsBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> AuthorsQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(AuthorsBorrowed) -> R) -> AuthorsQuery<'c, 'a, 's, C, R, N> {
        AuthorsQuery {
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
        let opt_row = crate::client::sync::opt(self.client, self.query, &self.params, self.cached)?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub fn iter(
        self,
    ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'c, postgres::Error> {
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
pub struct StringQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c mut C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s postgres::Statement>,
    extractor: fn(&postgres::Row) -> Result<&str, postgres::Error>,
    mapper: fn(&str) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> StringQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(&str) -> R) -> StringQuery<'c, 'a, 's, C, R, N> {
        StringQuery {
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
        let opt_row = crate::client::sync::opt(self.client, self.query, &self.params, self.cached)?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub fn iter(
        self,
    ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'c, postgres::Error> {
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
pub struct AuthorNameStartingWithQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c mut C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s postgres::Statement>,
    extractor: fn(&postgres::Row) -> Result<AuthorNameStartingWithBorrowed, postgres::Error>,
    mapper: fn(AuthorNameStartingWithBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> AuthorNameStartingWithQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(AuthorNameStartingWithBorrowed) -> R,
    ) -> AuthorNameStartingWithQuery<'c, 'a, 's, C, R, N> {
        AuthorNameStartingWithQuery {
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
        let opt_row = crate::client::sync::opt(self.client, self.query, &self.params, self.cached)?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub fn iter(
        self,
    ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'c, postgres::Error> {
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
pub struct VoiceActorQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c mut C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s postgres::Statement>,
    extractor: fn(&postgres::Row) -> Result<crate::types::VoiceActorBorrowed, postgres::Error>,
    mapper: fn(crate::types::VoiceActorBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> VoiceActorQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(crate::types::VoiceActorBorrowed) -> R,
    ) -> VoiceActorQuery<'c, 'a, 's, C, R, N> {
        VoiceActorQuery {
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
        let opt_row = crate::client::sync::opt(self.client, self.query, &self.params, self.cached)?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub fn iter(
        self,
    ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'c, postgres::Error> {
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
pub struct SelectTranslationsQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c mut C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s postgres::Statement>,
    extractor: fn(&postgres::Row) -> Result<SelectTranslationsBorrowed, postgres::Error>,
    mapper: fn(SelectTranslationsBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> SelectTranslationsQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(SelectTranslationsBorrowed) -> R,
    ) -> SelectTranslationsQuery<'c, 'a, 's, C, R, N> {
        SelectTranslationsQuery {
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
        let opt_row = crate::client::sync::opt(self.client, self.query, &self.params, self.cached)?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub fn iter(
        self,
    ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'c, postgres::Error> {
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
pub struct AuthorsStmt(&'static str, Option<postgres::Statement>);
pub fn authors() -> AuthorsStmt {
    AuthorsStmt("SELECT * FROM authors", None)
}
impl AuthorsStmt {
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
    ) -> AuthorsQuery<'c, 'a, 's, C, Authors, 0> {
        AuthorsQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &postgres::Row| -> Result<AuthorsBorrowed, postgres::Error> {
                Ok(AuthorsBorrowed {
                    id: row.try_get(0)?,
                    name: row.try_get(1)?,
                    country: row.try_get(2)?,
                    dob: row.try_get(3)?,
                })
            },
            mapper: |it| Authors::from(it),
        }
    }
}
pub struct BooksStmt(&'static str, Option<postgres::Statement>);
pub fn books() -> BooksStmt {
    BooksStmt("SELECT title FROM books", None)
}
impl BooksStmt {
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
    ) -> StringQuery<'c, 'a, 's, C, String, 0> {
        StringQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.into(),
        }
    }
}
pub struct AuthorNameByIdStmt(&'static str, Option<postgres::Statement>);
pub fn author_name_by_id() -> AuthorNameByIdStmt {
    AuthorNameByIdStmt(
        "SELECT authors.name FROM authors WHERE authors.id = $1",
        None,
    )
}
impl AuthorNameByIdStmt {
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
    ) -> StringQuery<'c, 'a, 's, C, String, 1> {
        StringQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.into(),
        }
    }
}
pub struct AuthorNameStartingWithStmt(&'static str, Option<postgres::Statement>);
pub fn author_name_starting_with() -> AuthorNameStartingWithStmt {
    AuthorNameStartingWithStmt(
        "SELECT book_authors.author_id, authors.name, book_authors.book_id, books.title FROM book_authors INNER JOIN authors ON authors.id = book_authors.author_id INNER JOIN books ON books.id = book_authors.book_id WHERE authors.name LIKE CONCAT($1::text, '%')",
        None,
    )
}
impl AuthorNameStartingWithStmt {
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
        start_str: &'a T1,
    ) -> AuthorNameStartingWithQuery<'c, 'a, 's, C, AuthorNameStartingWith, 1> {
        AuthorNameStartingWithQuery {
            client,
            params: [start_str],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &postgres::Row| -> Result<AuthorNameStartingWithBorrowed, postgres::Error> {
                    Ok(AuthorNameStartingWithBorrowed {
                        author_id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        book_id: row.try_get(2)?,
                        title: row.try_get(3)?,
                    })
                },
            mapper: |it| AuthorNameStartingWith::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::sync::Params<
        'c,
        'a,
        's,
        AuthorNameStartingWithParams<T1>,
        AuthorNameStartingWithQuery<'c, 'a, 's, C, AuthorNameStartingWith, 1>,
        C,
    > for AuthorNameStartingWithStmt
{
    fn params(
        &'s self,
        client: &'c mut C,
        params: &'a AuthorNameStartingWithParams<T1>,
    ) -> AuthorNameStartingWithQuery<'c, 'a, 's, C, AuthorNameStartingWith, 1> {
        self.bind(client, &params.start_str)
    }
}
pub struct SelectVoiceActorWithCharacterStmt(&'static str, Option<postgres::Statement>);
pub fn select_voice_actor_with_character() -> SelectVoiceActorWithCharacterStmt {
    SelectVoiceActorWithCharacterStmt(
        "SELECT voice_actor FROM spongebob_voice_actors WHERE character = $1",
        None,
    )
}
impl SelectVoiceActorWithCharacterStmt {
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
        spongebob_character: &'a crate::types::SpongebobCharacter,
    ) -> VoiceActorQuery<'c, 'a, 's, C, crate::types::VoiceActor, 1> {
        VoiceActorQuery {
            client,
            params: [spongebob_character],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.into(),
        }
    }
}
pub struct SelectTranslationsStmt(&'static str, Option<postgres::Statement>);
pub fn select_translations() -> SelectTranslationsStmt {
    SelectTranslationsStmt("SELECT title, translations FROM books", None)
}
impl SelectTranslationsStmt {
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
    ) -> SelectTranslationsQuery<'c, 'a, 's, C, SelectTranslations, 0> {
        SelectTranslationsQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &postgres::Row| -> Result<SelectTranslationsBorrowed, postgres::Error> {
                    Ok(SelectTranslationsBorrowed {
                        title: row.try_get(0)?,
                        translations: row.try_get(1)?,
                    })
                },
            mapper: |it| SelectTranslations::from(it),
        }
    }
}
