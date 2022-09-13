// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
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
        impl cornucopia_async::Borrow for SpongebobCharacter {
            type Borrow<'r> = SpongebobCharacter;
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
        impl cornucopia_async::Borrow for Authors {
            type Borrow<'r> = AuthorsBorrowed<'r>;
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
        impl cornucopia_async::Borrow for AuthorNameStartingWith {
            type Borrow<'r> = AuthorNameStartingWithBorrowed<'r>;
        }

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
        impl cornucopia_async::Borrow for SelectTranslations {
            type Borrow<'r> = SelectTranslationsBorrowed<'r>;
        }
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
            ) -> cornucopia_async::private::Query<'a, C, Authors, Authors, 0> {
                cornucopia_async::private::Query {
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
            ) -> cornucopia_async::private::Query<'a, C, String, String, 0> {
                cornucopia_async::private::Query {
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
            ) -> cornucopia_async::private::Query<'a, C, String, String, 1> {
                cornucopia_async::private::Query {
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
            ) -> cornucopia_async::private::Query<
                'a,
                C,
                AuthorNameStartingWith,
                AuthorNameStartingWith,
                1,
            > {
                cornucopia_async::private::Query {
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
                cornucopia_async::private::Query<
                    'a,
                    C,
                    AuthorNameStartingWith,
                    AuthorNameStartingWith,
                    1,
                >,
                C,
            > for AuthorNameStartingWithStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a AuthorNameStartingWithParams<T1>,
            ) -> cornucopia_async::private::Query<
                'a,
                C,
                AuthorNameStartingWith,
                AuthorNameStartingWith,
                1,
            > {
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
            ) -> cornucopia_async::private::Query<
                'a,
                C,
                super::super::types::public::SpongebobCharacter,
                super::super::types::public::SpongebobCharacter,
                1,
            > {
                cornucopia_async::private::Query {
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
            ) -> cornucopia_async::private::Query<'a, C, SelectTranslations, SelectTranslations, 0>
            {
                cornucopia_async::private::Query {
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
