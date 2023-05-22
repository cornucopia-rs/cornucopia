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
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct AuthorsQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> AuthorsBorrowed,
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
pub struct StringQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> &str,
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
pub struct AuthorNameStartingWithQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> AuthorNameStartingWithBorrowed,
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
pub struct VoiceactorQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> crate::types::VoiceactorBorrowed,
    mapper: fn(crate::types::VoiceactorBorrowed) -> T,
}
impl<'a, C, T: 'a, const N: usize> VoiceactorQuery<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(crate::types::VoiceactorBorrowed) -> R,
    ) -> VoiceactorQuery<'a, C, R, N> {
        VoiceactorQuery {
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
pub struct SelectTranslationsQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> SelectTranslationsBorrowed,
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
pub fn authors() -> AuthorsStmt {
    AuthorsStmt(crate::client::async_::Stmt::new(
        "SELECT
    *
FROM
    Author",
    ))
}
pub struct AuthorsStmt(crate::client::async_::Stmt);
impl AuthorsStmt {
    pub fn bind<'a, C: GenericClient>(
        &'a mut self,
        client: &'a C,
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
    BooksStmt(crate::client::async_::Stmt::new(
        "SELECT
    Title
FROM
    Book",
    ))
}
pub struct BooksStmt(crate::client::async_::Stmt);
impl BooksStmt {
    pub fn bind<'a, C: GenericClient>(
        &'a mut self,
        client: &'a C,
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
    AuthorNameByIdStmt(crate::client::async_::Stmt::new(
        "SELECT
    Author.Name
FROM
    Author
WHERE
    Author.Id = $1",
    ))
}
pub struct AuthorNameByIdStmt(crate::client::async_::Stmt);
impl AuthorNameByIdStmt {
    pub fn bind<'a, C: GenericClient>(
        &'a mut self,
        client: &'a C,
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
    AuthorNameStartingWithStmt(crate::client::async_::Stmt::new(
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
pub struct AuthorNameStartingWithStmt(crate::client::async_::Stmt);
impl AuthorNameStartingWithStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::StringSql>(
        &'a mut self,
        client: &'a C,
        start_str: &'a T1,
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
impl<'a, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'a,
        AuthorNameStartingWithParams<T1>,
        AuthorNameStartingWithQuery<'a, C, AuthorNameStartingWith, 1>,
        C,
    > for AuthorNameStartingWithStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a AuthorNameStartingWithParams<T1>,
    ) -> AuthorNameStartingWithQuery<'a, C, AuthorNameStartingWith, 1> {
        self.bind(client, &params.start_str)
    }
}
pub fn select_voice_actor_with_character() -> SelectVoiceActorWithCharacterStmt {
    SelectVoiceActorWithCharacterStmt(crate::client::async_::Stmt::new(
        "SELECT
    voice_actor
FROM
    SpongeBobVoiceActor
WHERE
    character = $1",
    ))
}
pub struct SelectVoiceActorWithCharacterStmt(crate::client::async_::Stmt);
impl SelectVoiceActorWithCharacterStmt {
    pub fn bind<'a, C: GenericClient>(
        &'a mut self,
        client: &'a C,
        spongebob_character: &'a crate::types::SpongeBobCharacter,
    ) -> VoiceactorQuery<'a, C, crate::types::Voiceactor, 1> {
        VoiceactorQuery {
            client,
            params: [spongebob_character],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it.into(),
        }
    }
}
pub fn select_translations() -> SelectTranslationsStmt {
    SelectTranslationsStmt(crate::client::async_::Stmt::new(
        "SELECT
    Title,
    Translations
FROM
    Book",
    ))
}
pub struct SelectTranslationsStmt(crate::client::async_::Stmt);
impl SelectTranslationsStmt {
    pub fn bind<'a, C: GenericClient>(
        &'a mut self,
        client: &'a C,
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
