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
        impl cornucopia_sync::Borrow for SpongebobCharacter {
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
        use postgres::{fallible_iterator::FallibleIterator, GenericClient};
        pub fn insert_book() -> InsertBookStmt {
            InsertBookStmt(cornucopia_sync::private::Stmt::new(
                "INSERT INTO Book (title)
  VALUES ($1)",
            ))
        }
        pub struct InsertBookStmt(cornucopia_sync::private::Stmt);
        impl InsertBookStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_sync::StringSql>(
                &'a mut self,
                client: &'a mut C,
                title: &'a T1,
            ) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(stmt, &[title])
            }
        }
    }
    pub mod module_2 {
        use postgres::{fallible_iterator::FallibleIterator, GenericClient};
        #[derive(Debug)]
        pub struct AuthorNameStartingWithParams<T1: cornucopia_sync::StringSql> {
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
        impl cornucopia_sync::Borrow for Authors {
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
        impl cornucopia_sync::Borrow for AuthorNameStartingWith {
            type Borrow<'r> = AuthorNameStartingWithBorrowed<'r>;
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
        impl cornucopia_sync::Borrow for SelectTranslations {
            type Borrow<'r> = SelectTranslationsBorrowed<'r>;
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
            ) -> cornucopia_sync::private::Query<'a, C, Authors, Authors, 0> {
                cornucopia_sync::private::Query {
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
            ) -> cornucopia_sync::private::Query<'a, C, String, String, 0> {
                cornucopia_sync::private::Query {
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
            ) -> cornucopia_sync::private::Query<'a, C, String, String, 1> {
                cornucopia_sync::private::Query {
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
            pub fn bind<'a, C: GenericClient, T1: cornucopia_sync::StringSql>(
                &'a mut self,
                client: &'a mut C,
                start_str: &'a T1,
            ) -> cornucopia_sync::private::Query<
                'a,
                C,
                AuthorNameStartingWith,
                AuthorNameStartingWith,
                1,
            > {
                cornucopia_sync::private::Query {
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
        impl<'a, C: GenericClient, T1: cornucopia_sync::StringSql>
            cornucopia_sync::Params<
                'a,
                AuthorNameStartingWithParams<T1>,
                cornucopia_sync::private::Query<
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
                client: &'a mut C,
                params: &'a AuthorNameStartingWithParams<T1>,
            ) -> cornucopia_sync::private::Query<
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
            ) -> cornucopia_sync::private::Query<
                'a,
                C,
                super::super::types::public::SpongebobCharacter,
                super::super::types::public::SpongebobCharacter,
                1,
            > {
                cornucopia_sync::private::Query {
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
            ) -> cornucopia_sync::private::Query<'a, C, SelectTranslations, SelectTranslations, 0>
            {
                cornucopia_sync::private::Query {
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
