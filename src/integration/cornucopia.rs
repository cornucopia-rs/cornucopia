// This file was generated with `cornucopia`. Do not modify.

pub mod types {
    pub mod public {
        #[derive(Debug)]
        #[postgres(name = "spongebob_character")]
        #[derive(postgres_types::ToSql, postgres_types::FromSql, Clone, Copy, PartialEq, Eq)]
        pub enum SpongebobCharacter {
            Bob,
            Patrick,
            Squidward,
        }

        #[derive(Debug)]
        #[postgres(name = "custom_composite")]
        #[derive(postgres_types::ToSql, Clone, PartialEq)]
        pub struct CustomComposite {
            pub wow: String,
            pub such_cool: i32,
            pub nice: super::public::SpongebobCharacter,
        }

        impl<'a> postgres_types::FromSql<'a> for CustomComposite {
            fn from_sql(
                _type: &postgres_types::Type,
                buf: &'a [u8],
            ) -> std::result::Result<
                CustomComposite,
                std::boxed::Box<dyn std::error::Error + std::marker::Sync + std::marker::Send>,
            > {
                let fields = match *_type.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut buf = buf;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let wow = postgres_types::private::read_value(fields[0].type_(), &mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let such_cool = postgres_types::private::read_value(fields[0].type_(), &mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let nice = postgres_types::private::read_value(fields[0].type_(), &mut buf)?;
                std::result::Result::Ok(CustomComposite {
                    wow,
                    such_cool,
                    nice,
                })
            }

            fn accepts(type_: &postgres_types::Type) -> bool {
                type_.name() == "custom_composite" && type_.schema() == "public"
            }
        }
        pub struct CustomCompositeBorrowed<'a> {
            pub wow: &'a str,
            pub such_cool: i32,
            pub nice: super::super::types::public::SpongebobCharacter,
        }

        impl<'a> postgres_types::FromSql<'a> for CustomCompositeBorrowed<'a> {
            fn from_sql(
                _type: &postgres_types::Type,
                buf: &'a [u8],
            ) -> std::result::Result<
                CustomCompositeBorrowed<'a>,
                std::boxed::Box<dyn std::error::Error + std::marker::Sync + std::marker::Send>,
            > {
                let fields = match *_type.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut buf = buf;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let wow = postgres_types::private::read_value(fields[0].type_(), &mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let such_cool = postgres_types::private::read_value(fields[0].type_(), &mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let nice = postgres_types::private::read_value(fields[0].type_(), &mut buf)?;
                std::result::Result::Ok(CustomCompositeBorrowed {
                    wow,
                    such_cool,
                    nice,
                })
            }

            fn accepts(type_: &postgres_types::Type) -> bool {
                type_.name() == "custom_composite" && type_.schema() == "public"
            }
        }

        impl<'a> From<CustomCompositeBorrowed<'a>> for CustomComposite {
            fn from(
                CustomCompositeBorrowed {
                    wow,
                    such_cool,
                    nice,
                }: CustomCompositeBorrowed<'a>,
            ) -> Self {
                Self {
                    wow: wow.to_owned(),
                    such_cool,
                    nice,
                }
            }
        }
    }
}

pub mod queries {
    pub mod module_1 {
        use futures::{StreamExt, TryStreamExt};

        pub struct InsertBookOneQuery<'a, C: cornucopia_client::GenericClient> {
            client: &'a C,
            params: [&'a (dyn tokio_postgres::types::ToSql + Sync); 0],
        }

        impl<'a, C> InsertBookOneQuery<'a, C>
        where
            C: cornucopia_client::GenericClient,
        {
            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client
                    .prepare(
                        "INSERT INTO Book (title)
  VALUES ('bob');

",
                    )
                    .await
            }

            pub async fn exec(self) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                self.client.execute(&stmt, &self.params).await
            }
        }
        pub fn insert_book_one<'a, C: cornucopia_client::GenericClient>(
            client: &'a C,
        ) -> InsertBookOneQuery<'a, C> {
            InsertBookOneQuery { client, params: [] }
        }
    }

    pub mod module_2 {
        use futures::{StreamExt, TryStreamExt};

        pub struct AuthorsBorrowed<'a> {
            pub id: i32,
            pub name: &'a str,
            pub country: &'a str,
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Authors {
            pub id: i32,
            pub name: String,
            pub country: String,
        }
        impl<'a> From<AuthorsBorrowed<'a>> for Authors {
            fn from(AuthorsBorrowed { id, name, country }: AuthorsBorrowed<'a>) -> Self {
                Self {
                    id,
                    name: name.to_owned(),
                    country: country.to_owned(),
                }
            }
        }
        pub struct AuthorsQuery<'a, C: cornucopia_client::GenericClient> {
            client: &'a C,
            params: [&'a (dyn tokio_postgres::types::ToSql + Sync); 0],
        }

        impl<'a, C> AuthorsQuery<'a, C>
        where
            C: cornucopia_client::GenericClient,
        {
            pub fn mapper<'b, R: From<AuthorsBorrowed<'b>>>(
                row: &'b tokio_postgres::row::Row,
            ) -> R {
                let borrow = AuthorsBorrowed {
                    id: row.get(0),
                    name: row.get(1),
                    country: row.get(2),
                };
                R::from(borrow)
            }

            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client
                    .prepare(
                        "SELECT
    *
FROM
    Author;",
                    )
                    .await
            }

            pub async fn one<T: for<'b> From<AuthorsBorrowed<'b>>>(
                self,
            ) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok(Self::mapper(&row))
            }

            pub async fn list<T: for<'b> From<AuthorsBorrowed<'b>>>(
                self,
            ) -> Result<Vec<T>, tokio_postgres::Error> {
                self.raw().await?.try_collect().await
            }

            pub async fn opt<T: for<'b> From<AuthorsBorrowed<'b>>>(
                self,
            ) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)
                    .await?
                    .map(|r| Self::mapper(&r)))
            }

            pub async fn raw<T: for<'b> From<AuthorsBorrowed<'b>>>(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>>,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt().await?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|r| Self::mapper(&r)));
                Ok(stream.into_stream())
            }
        }
        pub fn authors<'a, C: cornucopia_client::GenericClient>(
            client: &'a C,
        ) -> AuthorsQuery<'a, C> {
            AuthorsQuery { client, params: [] }
        }

        pub struct AuthorsStreamBorrowed<'a> {
            pub id: i32,
            pub name: &'a str,
            pub country: &'a str,
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct AuthorsStream {
            pub id: i32,
            pub name: String,
            pub country: String,
        }
        impl<'a> From<AuthorsStreamBorrowed<'a>> for AuthorsStream {
            fn from(
                AuthorsStreamBorrowed { id, name, country }: AuthorsStreamBorrowed<'a>,
            ) -> Self {
                Self {
                    id,
                    name: name.to_owned(),
                    country: country.to_owned(),
                }
            }
        }
        pub struct AuthorsStreamQuery<'a, C: cornucopia_client::GenericClient> {
            client: &'a C,
            params: [&'a (dyn tokio_postgres::types::ToSql + Sync); 0],
        }

        impl<'a, C> AuthorsStreamQuery<'a, C>
        where
            C: cornucopia_client::GenericClient,
        {
            pub fn mapper<'b, R: From<AuthorsStreamBorrowed<'b>>>(
                row: &'b tokio_postgres::row::Row,
            ) -> R {
                let borrow = AuthorsStreamBorrowed {
                    id: row.get(0),
                    name: row.get(1),
                    country: row.get(2),
                };
                R::from(borrow)
            }

            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client
                    .prepare(
                        "SELECT
    *
FROM
    Author;",
                    )
                    .await
            }

            pub async fn one<T: for<'b> From<AuthorsStreamBorrowed<'b>>>(
                self,
            ) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok(Self::mapper(&row))
            }

            pub async fn list<T: for<'b> From<AuthorsStreamBorrowed<'b>>>(
                self,
            ) -> Result<Vec<T>, tokio_postgres::Error> {
                self.raw().await?.try_collect().await
            }

            pub async fn opt<T: for<'b> From<AuthorsStreamBorrowed<'b>>>(
                self,
            ) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)
                    .await?
                    .map(|r| Self::mapper(&r)))
            }

            pub async fn raw<T: for<'b> From<AuthorsStreamBorrowed<'b>>>(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>>,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt().await?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|r| Self::mapper(&r)));
                Ok(stream.into_stream())
            }
        }
        pub fn authors_stream<'a, C: cornucopia_client::GenericClient>(
            client: &'a C,
        ) -> AuthorsStreamQuery<'a, C> {
            AuthorsStreamQuery { client, params: [] }
        }

        pub struct BooksBorrowed<'a> {
            pub title: &'a str,
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Books {
            pub title: String,
        }
        impl<'a> From<BooksBorrowed<'a>> for Books {
            fn from(BooksBorrowed { title }: BooksBorrowed<'a>) -> Self {
                Self {
                    title: title.to_owned(),
                }
            }
        }
        pub struct BooksQuery<'a, C: cornucopia_client::GenericClient> {
            client: &'a C,
            params: [&'a (dyn tokio_postgres::types::ToSql + Sync); 0],
        }

        impl<'a, C> BooksQuery<'a, C>
        where
            C: cornucopia_client::GenericClient,
        {
            pub fn mapper<'b, R: From<BooksBorrowed<'b>>>(row: &'b tokio_postgres::row::Row) -> R {
                let borrow = BooksBorrowed { title: row.get(0) };
                R::from(borrow)
            }

            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client
                    .prepare(
                        "SELECT
    Title
FROM
    Book;",
                    )
                    .await
            }

            pub async fn one<T: for<'b> From<BooksBorrowed<'b>>>(
                self,
            ) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok(Self::mapper(&row))
            }

            pub async fn list<T: for<'b> From<BooksBorrowed<'b>>>(
                self,
            ) -> Result<Vec<T>, tokio_postgres::Error> {
                self.raw().await?.try_collect().await
            }

            pub async fn opt<T: for<'b> From<BooksBorrowed<'b>>>(
                self,
            ) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)
                    .await?
                    .map(|r| Self::mapper(&r)))
            }

            pub async fn raw<T: for<'b> From<BooksBorrowed<'b>>>(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>>,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt().await?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|r| Self::mapper(&r)));
                Ok(stream.into_stream())
            }
        }
        pub fn books<'a, C: cornucopia_client::GenericClient>(client: &'a C) -> BooksQuery<'a, C> {
            BooksQuery { client, params: [] }
        }

        pub struct BooksOptRetParamBorrowed<'a> {
            pub title: &'a str,
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct BooksOptRetParam {
            pub title: String,
        }
        impl<'a> From<BooksOptRetParamBorrowed<'a>> for BooksOptRetParam {
            fn from(BooksOptRetParamBorrowed { title }: BooksOptRetParamBorrowed<'a>) -> Self {
                Self {
                    title: title.to_owned(),
                }
            }
        }
        pub struct BooksOptRetParamQuery<'a, C: cornucopia_client::GenericClient> {
            client: &'a C,
            params: [&'a (dyn tokio_postgres::types::ToSql + Sync); 0],
        }

        impl<'a, C> BooksOptRetParamQuery<'a, C>
        where
            C: cornucopia_client::GenericClient,
        {
            pub fn mapper<'b, R: From<BooksOptRetParamBorrowed<'b>>>(
                row: &'b tokio_postgres::row::Row,
            ) -> R {
                let borrow = BooksOptRetParamBorrowed { title: row.get(0) };
                R::from(borrow)
            }

            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client
                    .prepare(
                        "SELECT
    Title
FROM
    Book;",
                    )
                    .await
            }

            pub async fn one<T: for<'b> From<BooksOptRetParamBorrowed<'b>>>(
                self,
            ) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok(Self::mapper(&row))
            }

            pub async fn list<T: for<'b> From<BooksOptRetParamBorrowed<'b>>>(
                self,
            ) -> Result<Vec<T>, tokio_postgres::Error> {
                self.raw().await?.try_collect().await
            }

            pub async fn opt<T: for<'b> From<BooksOptRetParamBorrowed<'b>>>(
                self,
            ) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)
                    .await?
                    .map(|r| Self::mapper(&r)))
            }

            pub async fn raw<T: for<'b> From<BooksOptRetParamBorrowed<'b>>>(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>>,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt().await?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|r| Self::mapper(&r)));
                Ok(stream.into_stream())
            }
        }
        pub fn books_opt_ret_param<'a, C: cornucopia_client::GenericClient>(
            client: &'a C,
        ) -> BooksOptRetParamQuery<'a, C> {
            BooksOptRetParamQuery { client, params: [] }
        }

        #[derive(Debug, Clone)]
        pub struct BooksFromAuthorIdParams {
            pub id: i32,
        }
        impl BooksFromAuthorIdParams {
            fn books_from_author_id<'a, C: cornucopia_client::GenericClient>(
                &'a self,
                client: &'a C,
            ) -> BooksFromAuthorIdQuery<'a, C> {
                books_from_author_id(client, &self.id)
            }
        }
        pub struct BooksFromAuthorIdBorrowed<'a> {
            pub title: &'a str,
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct BooksFromAuthorId {
            pub title: String,
        }
        impl<'a> From<BooksFromAuthorIdBorrowed<'a>> for BooksFromAuthorId {
            fn from(BooksFromAuthorIdBorrowed { title }: BooksFromAuthorIdBorrowed<'a>) -> Self {
                Self {
                    title: title.to_owned(),
                }
            }
        }
        pub struct BooksFromAuthorIdQuery<'a, C: cornucopia_client::GenericClient> {
            client: &'a C,
            params: [&'a (dyn tokio_postgres::types::ToSql + Sync); 1],
        }

        impl<'a, C> BooksFromAuthorIdQuery<'a, C>
        where
            C: cornucopia_client::GenericClient,
        {
            pub fn mapper<'b, R: From<BooksFromAuthorIdBorrowed<'b>>>(
                row: &'b tokio_postgres::row::Row,
            ) -> R {
                let borrow = BooksFromAuthorIdBorrowed { title: row.get(0) };
                R::from(borrow)
            }

            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client
                    .prepare(
                        "SELECT
    Book.Title
FROM
    BookAuthor
    INNER JOIN Author ON Author.Id = BookAuthor.AuthorId
    INNER JOIN Book ON Book.Id = BookAuthor.BookId
WHERE
    Author.Id = $1;",
                    )
                    .await
            }

            pub async fn one<T: for<'b> From<BooksFromAuthorIdBorrowed<'b>>>(
                self,
            ) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok(Self::mapper(&row))
            }

            pub async fn list<T: for<'b> From<BooksFromAuthorIdBorrowed<'b>>>(
                self,
            ) -> Result<Vec<T>, tokio_postgres::Error> {
                self.raw().await?.try_collect().await
            }

            pub async fn opt<T: for<'b> From<BooksFromAuthorIdBorrowed<'b>>>(
                self,
            ) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)
                    .await?
                    .map(|r| Self::mapper(&r)))
            }

            pub async fn raw<T: for<'b> From<BooksFromAuthorIdBorrowed<'b>>>(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>>,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt().await?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|r| Self::mapper(&r)));
                Ok(stream.into_stream())
            }
        }
        pub fn books_from_author_id<'a, C: cornucopia_client::GenericClient>(
            client: &'a C,
            id: &'a i32,
        ) -> BooksFromAuthorIdQuery<'a, C> {
            BooksFromAuthorIdQuery {
                client,
                params: [id],
            }
        }

        #[derive(Debug, Clone)]
        pub struct AuthorNameByIdOptParams {
            pub id: i32,
        }
        impl AuthorNameByIdOptParams {
            fn author_name_by_id_opt<'a, C: cornucopia_client::GenericClient>(
                &'a self,
                client: &'a C,
            ) -> AuthorNameByIdOptQuery<'a, C> {
                author_name_by_id_opt(client, &self.id)
            }
        }
        pub struct AuthorNameByIdOptBorrowed<'a> {
            pub name: &'a str,
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct AuthorNameByIdOpt {
            pub name: String,
        }
        impl<'a> From<AuthorNameByIdOptBorrowed<'a>> for AuthorNameByIdOpt {
            fn from(AuthorNameByIdOptBorrowed { name }: AuthorNameByIdOptBorrowed<'a>) -> Self {
                Self {
                    name: name.to_owned(),
                }
            }
        }
        pub struct AuthorNameByIdOptQuery<'a, C: cornucopia_client::GenericClient> {
            client: &'a C,
            params: [&'a (dyn tokio_postgres::types::ToSql + Sync); 1],
        }

        impl<'a, C> AuthorNameByIdOptQuery<'a, C>
        where
            C: cornucopia_client::GenericClient,
        {
            pub fn mapper<'b, R: From<AuthorNameByIdOptBorrowed<'b>>>(
                row: &'b tokio_postgres::row::Row,
            ) -> R {
                let borrow = AuthorNameByIdOptBorrowed { name: row.get(0) };
                R::from(borrow)
            }

            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client
                    .prepare(
                        "SELECT
    Author.Name
FROM
    Author
WHERE
    Author.Id = $1;",
                    )
                    .await
            }

            pub async fn one<T: for<'b> From<AuthorNameByIdOptBorrowed<'b>>>(
                self,
            ) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok(Self::mapper(&row))
            }

            pub async fn list<T: for<'b> From<AuthorNameByIdOptBorrowed<'b>>>(
                self,
            ) -> Result<Vec<T>, tokio_postgres::Error> {
                self.raw().await?.try_collect().await
            }

            pub async fn opt<T: for<'b> From<AuthorNameByIdOptBorrowed<'b>>>(
                self,
            ) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)
                    .await?
                    .map(|r| Self::mapper(&r)))
            }

            pub async fn raw<T: for<'b> From<AuthorNameByIdOptBorrowed<'b>>>(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>>,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt().await?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|r| Self::mapper(&r)));
                Ok(stream.into_stream())
            }
        }
        pub fn author_name_by_id_opt<'a, C: cornucopia_client::GenericClient>(
            client: &'a C,
            id: &'a i32,
        ) -> AuthorNameByIdOptQuery<'a, C> {
            AuthorNameByIdOptQuery {
                client,
                params: [id],
            }
        }

        #[derive(Debug, Clone)]
        pub struct AuthorNameByIdParams {
            pub id: i32,
        }
        impl AuthorNameByIdParams {
            fn author_name_by_id<'a, C: cornucopia_client::GenericClient>(
                &'a self,
                client: &'a C,
            ) -> AuthorNameByIdQuery<'a, C> {
                author_name_by_id(client, &self.id)
            }
        }
        pub struct AuthorNameByIdBorrowed<'a> {
            pub name: &'a str,
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct AuthorNameById {
            pub name: String,
        }
        impl<'a> From<AuthorNameByIdBorrowed<'a>> for AuthorNameById {
            fn from(AuthorNameByIdBorrowed { name }: AuthorNameByIdBorrowed<'a>) -> Self {
                Self {
                    name: name.to_owned(),
                }
            }
        }
        pub struct AuthorNameByIdQuery<'a, C: cornucopia_client::GenericClient> {
            client: &'a C,
            params: [&'a (dyn tokio_postgres::types::ToSql + Sync); 1],
        }

        impl<'a, C> AuthorNameByIdQuery<'a, C>
        where
            C: cornucopia_client::GenericClient,
        {
            pub fn mapper<'b, R: From<AuthorNameByIdBorrowed<'b>>>(
                row: &'b tokio_postgres::row::Row,
            ) -> R {
                let borrow = AuthorNameByIdBorrowed { name: row.get(0) };
                R::from(borrow)
            }

            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client
                    .prepare(
                        "SELECT
    Author.Name
FROM
    Author
WHERE
    Author.Id = $1;",
                    )
                    .await
            }

            pub async fn one<T: for<'b> From<AuthorNameByIdBorrowed<'b>>>(
                self,
            ) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok(Self::mapper(&row))
            }

            pub async fn list<T: for<'b> From<AuthorNameByIdBorrowed<'b>>>(
                self,
            ) -> Result<Vec<T>, tokio_postgres::Error> {
                self.raw().await?.try_collect().await
            }

            pub async fn opt<T: for<'b> From<AuthorNameByIdBorrowed<'b>>>(
                self,
            ) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)
                    .await?
                    .map(|r| Self::mapper(&r)))
            }

            pub async fn raw<T: for<'b> From<AuthorNameByIdBorrowed<'b>>>(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>>,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt().await?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|r| Self::mapper(&r)));
                Ok(stream.into_stream())
            }
        }
        pub fn author_name_by_id<'a, C: cornucopia_client::GenericClient>(
            client: &'a C,
            id: &'a i32,
        ) -> AuthorNameByIdQuery<'a, C> {
            AuthorNameByIdQuery {
                client,
                params: [id],
            }
        }

        #[derive(Debug, Clone)]
        pub struct AuthorNameStartingWithParams<'a> {
            pub s: &'a str,
        }
        impl<'a> AuthorNameStartingWithParams<'a> {
            fn author_name_starting_with<C: cornucopia_client::GenericClient>(
                &'a self,
                client: &'a C,
            ) -> AuthorNameStartingWithQuery<'a, C> {
                author_name_starting_with(client, &self.s)
            }
        }
        pub struct AuthorNameStartingWithBorrowed<'a> {
            pub authorid: i32,
            pub name: &'a str,
            pub bookid: i32,
            pub title: &'a str,
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct AuthorNameStartingWith {
            pub authorid: i32,
            pub name: String,
            pub bookid: i32,
            pub title: String,
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
                    name: name.to_owned(),
                    bookid,
                    title: title.to_owned(),
                }
            }
        }
        pub struct AuthorNameStartingWithQuery<'a, C: cornucopia_client::GenericClient> {
            client: &'a C,
            params: [&'a (dyn tokio_postgres::types::ToSql + Sync); 1],
        }

        impl<'a, C> AuthorNameStartingWithQuery<'a, C>
        where
            C: cornucopia_client::GenericClient,
        {
            pub fn mapper<'b, R: From<AuthorNameStartingWithBorrowed<'b>>>(
                row: &'b tokio_postgres::row::Row,
            ) -> R {
                let borrow = AuthorNameStartingWithBorrowed {
                    authorid: row.get(0),
                    name: row.get(1),
                    bookid: row.get(2),
                    title: row.get(3),
                };
                R::from(borrow)
            }

            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client
                    .prepare(
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
    Author.Name LIKE CONCAT(($1)::text, '%');",
                    )
                    .await
            }

            pub async fn one<T: for<'b> From<AuthorNameStartingWithBorrowed<'b>>>(
                self,
            ) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok(Self::mapper(&row))
            }

            pub async fn list<T: for<'b> From<AuthorNameStartingWithBorrowed<'b>>>(
                self,
            ) -> Result<Vec<T>, tokio_postgres::Error> {
                self.raw().await?.try_collect().await
            }

            pub async fn opt<T: for<'b> From<AuthorNameStartingWithBorrowed<'b>>>(
                self,
            ) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)
                    .await?
                    .map(|r| Self::mapper(&r)))
            }

            pub async fn raw<T: for<'b> From<AuthorNameStartingWithBorrowed<'b>>>(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>>,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt().await?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|r| Self::mapper(&r)));
                Ok(stream.into_stream())
            }
        }
        pub fn author_name_starting_with<'a, C: cornucopia_client::GenericClient>(
            client: &'a C,
            s: &'a &str,
        ) -> AuthorNameStartingWithQuery<'a, C> {
            AuthorNameStartingWithQuery {
                client,
                params: [s],
            }
        }

        pub struct ReturnCustomTypeBorrowed<'a> {
            pub col1: super::super::types::public::CustomCompositeBorrowed<'a>,
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ReturnCustomType {
            pub col1: super::super::types::public::CustomComposite,
        }
        impl<'a> From<ReturnCustomTypeBorrowed<'a>> for ReturnCustomType {
            fn from(ReturnCustomTypeBorrowed { col1 }: ReturnCustomTypeBorrowed<'a>) -> Self {
                Self { col1: col1.into() }
            }
        }
        pub struct ReturnCustomTypeQuery<'a, C: cornucopia_client::GenericClient> {
            client: &'a C,
            params: [&'a (dyn tokio_postgres::types::ToSql + Sync); 0],
        }

        impl<'a, C> ReturnCustomTypeQuery<'a, C>
        where
            C: cornucopia_client::GenericClient,
        {
            pub fn mapper<'b, R: From<ReturnCustomTypeBorrowed<'b>>>(
                row: &'b tokio_postgres::row::Row,
            ) -> R {
                let borrow = ReturnCustomTypeBorrowed { col1: row.get(0) };
                R::from(borrow)
            }

            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client
                    .prepare(
                        "SELECT
    col1
FROM
    CustomTable;",
                    )
                    .await
            }

            pub async fn one<T: for<'b> From<ReturnCustomTypeBorrowed<'b>>>(
                self,
            ) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok(Self::mapper(&row))
            }

            pub async fn list<T: for<'b> From<ReturnCustomTypeBorrowed<'b>>>(
                self,
            ) -> Result<Vec<T>, tokio_postgres::Error> {
                self.raw().await?.try_collect().await
            }

            pub async fn opt<T: for<'b> From<ReturnCustomTypeBorrowed<'b>>>(
                self,
            ) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)
                    .await?
                    .map(|r| Self::mapper(&r)))
            }

            pub async fn raw<T: for<'b> From<ReturnCustomTypeBorrowed<'b>>>(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>>,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt().await?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|r| Self::mapper(&r)));
                Ok(stream.into_stream())
            }
        }
        pub fn return_custom_type<'a, C: cornucopia_client::GenericClient>(
            client: &'a C,
        ) -> ReturnCustomTypeQuery<'a, C> {
            ReturnCustomTypeQuery { client, params: [] }
        }

        #[derive(Debug, Clone)]
        pub struct SelectWhereCustomTypeParams {
            pub spongebob_character: super::super::types::public::SpongebobCharacter,
        }
        impl SelectWhereCustomTypeParams {
            fn select_where_custom_type<'a, C: cornucopia_client::GenericClient>(
                &'a self,
                client: &'a C,
            ) -> SelectWhereCustomTypeQuery<'a, C> {
                select_where_custom_type(client, &self.spongebob_character)
            }
        }

        #[derive(Debug, Copy, Clone, PartialEq)]
        pub struct SelectWhereCustomType {
            pub col2: super::super::types::public::SpongebobCharacter,
        }

        pub struct SelectWhereCustomTypeQuery<'a, C: cornucopia_client::GenericClient> {
            client: &'a C,
            params: [&'a (dyn tokio_postgres::types::ToSql + Sync); 1],
        }

        impl<'a, C> SelectWhereCustomTypeQuery<'a, C>
        where
            C: cornucopia_client::GenericClient,
        {
            pub fn mapper<R: From<SelectWhereCustomType>>(row: &tokio_postgres::row::Row) -> R {
                let borrow = SelectWhereCustomType { col2: row.get(0) };
                R::from(borrow)
            }

            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client
                    .prepare(
                        "SELECT
    col2
FROM
    CustomTable
WHERE (col1).nice = $1;",
                    )
                    .await
            }

            pub async fn one<T: From<SelectWhereCustomType>>(
                self,
            ) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok(Self::mapper(&row))
            }

            pub async fn list<T: From<SelectWhereCustomType>>(
                self,
            ) -> Result<Vec<T>, tokio_postgres::Error> {
                self.raw().await?.try_collect().await
            }

            pub async fn opt<T: From<SelectWhereCustomType>>(
                self,
            ) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)
                    .await?
                    .map(|r| Self::mapper(&r)))
            }

            pub async fn raw<T: From<SelectWhereCustomType>>(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>>,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt().await?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|r| Self::mapper(&r)));
                Ok(stream.into_stream())
            }
        }
        pub fn select_where_custom_type<'a, C: cornucopia_client::GenericClient>(
            client: &'a C,
            spongebob_character: &'a super::super::types::public::SpongebobCharacter,
        ) -> SelectWhereCustomTypeQuery<'a, C> {
            SelectWhereCustomTypeQuery {
                client,
                params: [spongebob_character],
            }
        }

        pub struct SelectEverythingBorrowed<'a> {
            pub array_: cornucopia_client::ArrayIterator<'a, bool>,
            pub custom_array_: cornucopia_client::ArrayIterator<
                'a,
                super::super::types::public::SpongebobCharacter,
            >,
            pub bool_: bool,
            pub boolean_: bool,
            pub char_: i8,
            pub smallint_: i16,
            pub int2_: i16,
            pub smallserial_: i16,
            pub serial2_: i16,
            pub int_: i32,
            pub int4_: i32,
            pub serial_: i32,
            pub serial4_: i32,
            pub bingint_: i64,
            pub int8_: i64,
            pub bigserial_: i64,
            pub serial8_: i64,
            pub float4_: f32,
            pub real_: f32,
            pub float8_: f64,
            pub double_precision_: f64,
            pub text_: &'a str,
            pub varchar_: &'a str,
            pub bytea_: &'a [u8],
            pub timestamp_: time::PrimitiveDateTime,
            pub timestamp_without_time_zone_: time::PrimitiveDateTime,
            pub timestamptz_: time::OffsetDateTime,
            pub timestamp_with_time_zone_: time::OffsetDateTime,
            pub date_: time::Date,
            pub time_: time::Time,
            pub json_: tokio_postgres::types::Json<&'a serde_json::value::RawValue>,
            pub jsonb_: tokio_postgres::types::Json<&'a serde_json::value::RawValue>,
            pub uuid_: uuid::Uuid,
            pub inet_: std::net::IpAddr,
            pub macaddr_: eui48::MacAddress,
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SelectEverything {
            pub array_: Vec<bool>,
            pub custom_array_: Vec<super::super::types::public::SpongebobCharacter>,
            pub bool_: bool,
            pub boolean_: bool,
            pub char_: i8,
            pub smallint_: i16,
            pub int2_: i16,
            pub smallserial_: i16,
            pub serial2_: i16,
            pub int_: i32,
            pub int4_: i32,
            pub serial_: i32,
            pub serial4_: i32,
            pub bingint_: i64,
            pub int8_: i64,
            pub bigserial_: i64,
            pub serial8_: i64,
            pub float4_: f32,
            pub real_: f32,
            pub float8_: f64,
            pub double_precision_: f64,
            pub text_: String,
            pub varchar_: String,
            pub bytea_: Vec<u8>,
            pub timestamp_: time::PrimitiveDateTime,
            pub timestamp_without_time_zone_: time::PrimitiveDateTime,
            pub timestamptz_: time::OffsetDateTime,
            pub timestamp_with_time_zone_: time::OffsetDateTime,
            pub date_: time::Date,
            pub time_: time::Time,
            pub json_: tokio_postgres::types::Json<serde_json::Value>,
            pub jsonb_: tokio_postgres::types::Json<serde_json::Value>,
            pub uuid_: uuid::Uuid,
            pub inet_: std::net::IpAddr,
            pub macaddr_: eui48::MacAddress,
        }
        impl<'a> From<SelectEverythingBorrowed<'a>> for SelectEverything {
            fn from(
                SelectEverythingBorrowed {
                    array_,
                    custom_array_,
                    bool_,
                    boolean_,
                    char_,
                    smallint_,
                    int2_,
                    smallserial_,
                    serial2_,
                    int_,
                    int4_,
                    serial_,
                    serial4_,
                    bingint_,
                    int8_,
                    bigserial_,
                    serial8_,
                    float4_,
                    real_,
                    float8_,
                    double_precision_,
                    text_,
                    varchar_,
                    bytea_,
                    timestamp_,
                    timestamp_without_time_zone_,
                    timestamptz_,
                    timestamp_with_time_zone_,
                    date_,
                    time_,
                    json_,
                    jsonb_,
                    uuid_,
                    inet_,
                    macaddr_,
                }: SelectEverythingBorrowed<'a>,
            ) -> Self {
                Self {
                    array_: array_.collect(),
                    custom_array_: custom_array_.collect(),
                    bool_,
                    boolean_,
                    char_,
                    smallint_,
                    int2_,
                    smallserial_,
                    serial2_,
                    int_,
                    int4_,
                    serial_,
                    serial4_,
                    bingint_,
                    int8_,
                    bigserial_,
                    serial8_,
                    float4_,
                    real_,
                    float8_,
                    double_precision_,
                    text_: text_.to_owned(),
                    varchar_: varchar_.to_owned(),
                    bytea_: bytea_.to_owned(),
                    timestamp_,
                    timestamp_without_time_zone_,
                    timestamptz_,
                    timestamp_with_time_zone_,
                    date_,
                    time_,
                    json_: tokio_postgres::types::Json(
                        serde_json::from_str(json_.0.get()).unwrap(),
                    ),
                    jsonb_: tokio_postgres::types::Json(
                        serde_json::from_str(jsonb_.0.get()).unwrap(),
                    ),
                    uuid_,
                    inet_,
                    macaddr_,
                }
            }
        }
        pub struct SelectEverythingQuery<'a, C: cornucopia_client::GenericClient> {
            client: &'a C,
            params: [&'a (dyn tokio_postgres::types::ToSql + Sync); 0],
        }

        impl<'a, C> SelectEverythingQuery<'a, C>
        where
            C: cornucopia_client::GenericClient,
        {
            pub fn mapper<'b, R: From<SelectEverythingBorrowed<'b>>>(
                row: &'b tokio_postgres::row::Row,
            ) -> R {
                let borrow = SelectEverythingBorrowed {
                    array_: row.get(0),
                    custom_array_: row.get(1),
                    bool_: row.get(2),
                    boolean_: row.get(3),
                    char_: row.get(4),
                    smallint_: row.get(5),
                    int2_: row.get(6),
                    smallserial_: row.get(7),
                    serial2_: row.get(8),
                    int_: row.get(9),
                    int4_: row.get(10),
                    serial_: row.get(11),
                    serial4_: row.get(12),
                    bingint_: row.get(13),
                    int8_: row.get(14),
                    bigserial_: row.get(15),
                    serial8_: row.get(16),
                    float4_: row.get(17),
                    real_: row.get(18),
                    float8_: row.get(19),
                    double_precision_: row.get(20),
                    text_: row.get(21),
                    varchar_: row.get(22),
                    bytea_: row.get(23),
                    timestamp_: row.get(24),
                    timestamp_without_time_zone_: row.get(25),
                    timestamptz_: row.get(26),
                    timestamp_with_time_zone_: row.get(27),
                    date_: row.get(28),
                    time_: row.get(29),
                    json_: row.get(30),
                    jsonb_: row.get(31),
                    uuid_: row.get(32),
                    inet_: row.get(33),
                    macaddr_: row.get(34),
                };
                R::from(borrow)
            }

            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client
                    .prepare(
                        "SELECT
    *
FROM
    Everything;

",
                    )
                    .await
            }

            pub async fn one<T: for<'b> From<SelectEverythingBorrowed<'b>>>(
                self,
            ) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok(Self::mapper(&row))
            }

            pub async fn list<T: for<'b> From<SelectEverythingBorrowed<'b>>>(
                self,
            ) -> Result<Vec<T>, tokio_postgres::Error> {
                self.raw().await?.try_collect().await
            }

            pub async fn opt<T: for<'b> From<SelectEverythingBorrowed<'b>>>(
                self,
            ) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)
                    .await?
                    .map(|r| Self::mapper(&r)))
            }

            pub async fn raw<T: for<'b> From<SelectEverythingBorrowed<'b>>>(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>>,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt().await?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|r| Self::mapper(&r)));
                Ok(stream.into_stream())
            }
        }
        pub fn select_everything<'a, C: cornucopia_client::GenericClient>(
            client: &'a C,
        ) -> SelectEverythingQuery<'a, C> {
            SelectEverythingQuery { client, params: [] }
        }
    }
}
