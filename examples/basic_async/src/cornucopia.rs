// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod types {
    pub mod public {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[allow(non_camel_case_types)]
        pub enum SpongebobCharacter {
            Bob,
            Patrick,
            Squidward,
        }
        impl<'a> postgres_types::ToSql for SpongebobCharacter {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                buf: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                let s = match *self {
                    SpongebobCharacter::Bob => "Bob",
                    SpongebobCharacter::Patrick => "Patrick",
                    SpongebobCharacter::Squidward => "Squidward",
                };
                buf.extend_from_slice(s.as_bytes());
                std::result::Result::Ok(postgres_types::IsNull::No)
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "spongebob_character" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Enum(ref variants) => {
                        if variants.len() != 3usize {
                            return false;
                        }
                        variants.iter().all(|v| match &**v {
                            "Bob" => true,
                            "Patrick" => true,
                            "Squidward" => true,
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }
            fn to_sql_checked(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
        impl<'a> postgres_types::FromSql<'a> for SpongebobCharacter {
            fn from_sql(
                ty: &postgres_types::Type,
                buf: &'a [u8],
            ) -> Result<SpongebobCharacter, Box<dyn std::error::Error + Sync + Send>> {
                match std::str::from_utf8(buf)? {
                    "Bob" => Ok(SpongebobCharacter::Bob),
                    "Patrick" => Ok(SpongebobCharacter::Patrick),
                    "Squidward" => Ok(SpongebobCharacter::Squidward),
                    s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
                }
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "spongebob_character" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Enum(ref variants) => {
                        if variants.len() != 3usize {
                            return false;
                        }
                        variants.iter().all(|v| match &**v {
                            "Bob" => true,
                            "Patrick" => true,
                            "Squidward" => true,
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }
        }
    }
}
#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod queries {
    pub mod module_1 {
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        pub fn insert_book() -> InsertBookStmt {
            InsertBookStmt(cornucopia_async::private::Stmt::new(
                "INSERT INTO Book (title)
  VALUES ($1)",
            ))
        }
        pub struct InsertBookStmt(cornucopia_async::private::Stmt);
        impl InsertBookStmt {
            pub async fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
                &'a mut self,
                client: &'a C,
                title: &'a T1,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[title]).await
            }
        }
    }
    pub mod module_2 {
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug)]
        pub struct AuthorNameStartingWithParams<T1: cornucopia_async::StringSql> {
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
        cornucopia_async::query! {AuthorsQuery, AuthorsBorrowed}
        cornucopia_async::query! {StringQuery, & str}
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
        cornucopia_async::query! {AuthorNameStartingWithQuery, AuthorNameStartingWithBorrowed}
        cornucopia_async::query! {SuperSuperTypesPublicSpongebobCharacterQuery, super::super::types::public::SpongebobCharacter}
        #[derive(Debug, Clone, PartialEq)]
        pub struct SelectTranslations {
            pub title: String,
            pub translations: Vec<String>,
        }
        pub struct SelectTranslationsBorrowed<'a> {
            pub title: &'a str,
            pub translations: cornucopia_async::ArrayIterator<'a, &'a str>,
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
        cornucopia_async::query! {SelectTranslationsQuery, SelectTranslationsBorrowed}
        pub fn authors() -> AuthorsStmt {
            AuthorsStmt(cornucopia_async::private::Stmt::new(
                "SELECT
    *
FROM
    Author",
            ))
        }
        pub struct AuthorsStmt(cornucopia_async::private::Stmt);
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
            BooksStmt(cornucopia_async::private::Stmt::new(
                "SELECT
    Title
FROM
    Book",
            ))
        }
        pub struct BooksStmt(cornucopia_async::private::Stmt);
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
            AuthorNameByIdStmt(cornucopia_async::private::Stmt::new(
                "SELECT
    Author.Name
FROM
    Author
WHERE
    Author.Id = $1",
            ))
        }
        pub struct AuthorNameByIdStmt(cornucopia_async::private::Stmt);
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
            AuthorNameStartingWithStmt(cornucopia_async::private::Stmt::new(
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
        pub struct AuthorNameStartingWithStmt(cornucopia_async::private::Stmt);
        impl AuthorNameStartingWithStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
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
        impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
            cornucopia_async::Params<
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
        pub fn select_where_custom_type() -> SelectWhereCustomTypeStmt {
            SelectWhereCustomTypeStmt(cornucopia_async::private::Stmt::new(
                "SELECT
    col2
FROM
    CustomTable
WHERE (col1).persona = $1",
            ))
        }
        pub struct SelectWhereCustomTypeStmt(cornucopia_async::private::Stmt);
        impl SelectWhereCustomTypeStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
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
            SelectTranslationsStmt(cornucopia_async::private::Stmt::new(
                "SELECT
    Title,
    Translations
FROM
    Book",
            ))
        }
        pub struct SelectTranslationsStmt(cornucopia_async::private::Stmt);
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
    }
}
