pub mod types {
    pub mod public {
        #[derive(
            Debug,
            postgres_types::ToSql,
            postgres_types::FromSql,
            Clone,
            Copy,
            PartialEq,
            Eq
        )]
        #[postgres(name = "spongebob_character")]
        pub enum SpongebobCharacter {
            Bob,
            Patrick,
            Squidward,
        }
        #[derive(
            Debug,
            postgres_types::ToSql,
            postgres_types::FromSql,
            Clone,
            PartialEq
        )]
        #[postgres(name = "custom_composite")]
        pub struct CustomComposite {
            pub name: String,
            pub age: i32,
            pub persona: super::public::SpongebobCharacter,
        }
        #[derive(Debug)]
        pub struct CustomCompositeBorrowed<'a> {
            pub name: &'a str,
            pub age: i32,
            pub persona: super::super::types::public::SpongebobCharacter,
        }
        impl<'a> postgres_types::FromSql<'a> for CustomCompositeBorrowed<'a> {
            fn from_sql(
                _type: &postgres_types::Type,
                buf: &'a [u8],
            ) -> Result<
                CustomCompositeBorrowed<'a>,
                std::boxed::Box<
                    dyn std::error::Error + std::marker::Sync + std::marker::Send,
                >,
            > {
                let fields = match *_type.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut buf = buf;
                let num_fields = postgres_types::private::read_be_i32(&mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let name = postgres_types::private::read_value(
                    fields[0].type_(),
                    &mut buf,
                )?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let age = postgres_types::private::read_value(
                    fields[1].type_(),
                    &mut buf,
                )?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let persona = postgres_types::private::read_value(
                    fields[2].type_(),
                    &mut buf,
                )?;
                Result::Ok(CustomCompositeBorrowed {
                    name,
                    age,
                    persona,
                })
            }
            fn accepts(type_: &postgres_types::Type) -> bool {
                type_.name() == "custom_composite" && type_.schema() == "public"
            }
        }
        impl<'a> From<CustomCompositeBorrowed<'a>> for CustomComposite {
            fn from(
                CustomCompositeBorrowed {
                    name,
                    age,
                    persona,
                }: CustomCompositeBorrowed<'a>,
            ) -> Self {
                Self {
                    name: name.into(),
                    age,
                    persona,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct CustomCompositeParams<'a> {
            pub name: &'a str,
            pub age: i32,
            pub persona: super::super::types::public::SpongebobCharacter,
        }
        impl<'a> postgres_types::ToSql for CustomCompositeParams<'a> {
            fn to_sql(
                &self,
                _type: &postgres_types::Type,
                buf: &mut postgres_types::private::BytesMut,
            ) -> std::result::Result<
                postgres_types::IsNull,
                std::boxed::Box<dyn std::error::Error + Sync + Send>,
            > {
                let fields = match *_type.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                buf.extend_from_slice(&(fields.len() as i32).to_be_bytes());
                for field in fields {
                    buf.extend_from_slice(&field.type_().oid().to_be_bytes());
                    let base = buf.len();
                    buf.extend_from_slice(&[0; 4]);
                    let r = match field.name() {
                        "name" => {
                            postgres_types::ToSql::to_sql(&self.name, field.type_(), buf)
                        }
                        "age" => {
                            postgres_types::ToSql::to_sql(&self.age, field.type_(), buf)
                        }
                        "persona" => {
                            postgres_types::ToSql::to_sql(
                                &self.persona,
                                field.type_(),
                                buf,
                            )
                        }
                        _ => unreachable!(),
                    };
                    let count = match r? {
                        postgres_types::IsNull::Yes => -1,
                        postgres_types::IsNull::No => {
                            let len = buf.len() - base - 4;
                            if len > i32::max_value() as usize {
                                return std::result::Result::Err(
                                    std::convert::Into::into("value too large to transmit"),
                                );
                            }
                            len as i32
                        }
                    };
                    buf[base..base + 4].copy_from_slice(&count.to_be_bytes());
                }
                std::result::Result::Ok(postgres_types::IsNull::No)
            }
            fn accepts(type_: &postgres_types::Type) -> bool {
                if type_.name() != "custom_composite" {
                    return false;
                }
                match *type_.kind() {
                    postgres_types::Kind::Composite(ref fields) => {
                        if fields.len() != 3usize {
                            return false;
                        }
                        fields
                            .iter()
                            .all(|f| match f.name() {
                                "name" => {
                                    <&'a str as postgres_types::ToSql>::accepts(f.type_())
                                }
                                "age" => <i32 as postgres_types::ToSql>::accepts(f.type_()),
                                "persona" => {
                                    <super::super::types::public::SpongebobCharacter as postgres_types::ToSql>::accepts(
                                        f.type_(),
                                    )
                                }
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
            ) -> std::result::Result<
                postgres_types::IsNull,
                Box<dyn std::error::Error + Sync + Send>,
            > {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
    }
}
pub mod queries {
    pub mod module_2 {
        use futures::{StreamExt, TryStreamExt};
        use cornucopia_client::GenericClient;
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
        pub struct AuthorsQuery<'a, C: GenericClient, T> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); 0],
            mapper: fn(AuthorsBorrowed) -> T,
        }
        impl<'a, C, T: 'a> AuthorsQuery<'a, C, T>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(AuthorsBorrowed) -> R,
            ) -> AuthorsQuery<'a, C, R> {
                AuthorsQuery {
                    client: self.client,
                    params: self.params,
                    mapper,
                }
            }
            pub fn extractor(row: &tokio_postgres::row::Row) -> AuthorsBorrowed {
                AuthorsBorrowed {
                    id: row.get(0),
                    name: row.get(1),
                    country: row.get(2),
                }
            }
            pub async fn stmt(
                &self,
            ) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client.prepare("SELECT
    *
FROM
    Author;").await
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok((self.mapper)(Self::extractor(&row)))
            }
            pub async fn vec(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.stream().await?.try_collect().await
            }
            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                Ok(
                    self
                        .client
                        .query_opt(&stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)(Self::extractor(&row))),
                )
            }
            pub async fn stream(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt().await?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)(Self::extractor(&row))))
                    .into_stream();
                Ok(stream)
            }
        }
        pub fn authors<'a, C: GenericClient>(
            client: &'a C,
        ) -> AuthorsQuery<'a, C, Authors> {
            AuthorsQuery {
                client,
                params: [],
                mapper: |it| Authors::from(it),
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Books {
            pub title: String,
        }
        pub struct BooksBorrowed<'a> {
            pub title: &'a str,
        }
        impl<'a> From<BooksBorrowed<'a>> for Books {
            fn from(BooksBorrowed { title }: BooksBorrowed<'a>) -> Self {
                Self { title: title.into() }
            }
        }
        pub struct BooksQuery<'a, C: GenericClient, T> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); 0],
            mapper: fn(BooksBorrowed) -> T,
        }
        impl<'a, C, T: 'a> BooksQuery<'a, C, T>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(BooksBorrowed) -> R) -> BooksQuery<'a, C, R> {
                BooksQuery {
                    client: self.client,
                    params: self.params,
                    mapper,
                }
            }
            pub fn extractor(row: &tokio_postgres::row::Row) -> BooksBorrowed {
                BooksBorrowed { title: row.get(0) }
            }
            pub async fn stmt(
                &self,
            ) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client.prepare("SELECT
    Title
FROM
    Book;").await
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok((self.mapper)(Self::extractor(&row)))
            }
            pub async fn vec(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.stream().await?.try_collect().await
            }
            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                Ok(
                    self
                        .client
                        .query_opt(&stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)(Self::extractor(&row))),
                )
            }
            pub async fn stream(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt().await?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)(Self::extractor(&row))))
                    .into_stream();
                Ok(stream)
            }
        }
        pub fn books<'a, C: GenericClient>(client: &'a C) -> BooksQuery<'a, C, Books> {
            BooksQuery {
                client,
                params: [],
                mapper: |it| Books::from(it),
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct BooksOptRetParam {
            pub title: Option<String>,
        }
        pub struct BooksOptRetParamBorrowed<'a> {
            pub title: Option<&'a str>,
        }
        impl<'a> From<BooksOptRetParamBorrowed<'a>> for BooksOptRetParam {
            fn from(
                BooksOptRetParamBorrowed { title }: BooksOptRetParamBorrowed<'a>,
            ) -> Self {
                Self {
                    title: title.map(|v| v.into()),
                }
            }
        }
        pub struct BooksOptRetParamQuery<'a, C: GenericClient, T> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); 0],
            mapper: fn(BooksOptRetParamBorrowed) -> T,
        }
        impl<'a, C, T: 'a> BooksOptRetParamQuery<'a, C, T>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(BooksOptRetParamBorrowed) -> R,
            ) -> BooksOptRetParamQuery<'a, C, R> {
                BooksOptRetParamQuery {
                    client: self.client,
                    params: self.params,
                    mapper,
                }
            }
            pub fn extractor(
                row: &tokio_postgres::row::Row,
            ) -> BooksOptRetParamBorrowed {
                BooksOptRetParamBorrowed {
                    title: row.get(0),
                }
            }
            pub async fn stmt(
                &self,
            ) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client.prepare("SELECT
    Title
FROM
    Book;").await
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok((self.mapper)(Self::extractor(&row)))
            }
            pub async fn vec(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.stream().await?.try_collect().await
            }
            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                Ok(
                    self
                        .client
                        .query_opt(&stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)(Self::extractor(&row))),
                )
            }
            pub async fn stream(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt().await?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)(Self::extractor(&row))))
                    .into_stream();
                Ok(stream)
            }
        }
        pub fn books_opt_ret_param<'a, C: GenericClient>(
            client: &'a C,
        ) -> BooksOptRetParamQuery<'a, C, BooksOptRetParam> {
            BooksOptRetParamQuery {
                client,
                params: [],
                mapper: |it| BooksOptRetParam::from(it),
            }
        }
        #[derive(Debug, Copy, Clone)]
        pub struct AuthorNameByIdParams {
            pub id: i32,
        }
        impl AuthorNameByIdParams {
            pub fn query<'a, C: GenericClient>(
                &'a self,
                client: &'a C,
            ) -> AuthorNameByIdQuery<'a, C, AuthorNameById> {
                author_name_by_id(client, &self.id)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct AuthorNameById {
            pub name: String,
        }
        pub struct AuthorNameByIdBorrowed<'a> {
            pub name: &'a str,
        }
        impl<'a> From<AuthorNameByIdBorrowed<'a>> for AuthorNameById {
            fn from(
                AuthorNameByIdBorrowed { name }: AuthorNameByIdBorrowed<'a>,
            ) -> Self {
                Self { name: name.into() }
            }
        }
        pub struct AuthorNameByIdQuery<'a, C: GenericClient, T> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); 1],
            mapper: fn(AuthorNameByIdBorrowed) -> T,
        }
        impl<'a, C, T: 'a> AuthorNameByIdQuery<'a, C, T>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(AuthorNameByIdBorrowed) -> R,
            ) -> AuthorNameByIdQuery<'a, C, R> {
                AuthorNameByIdQuery {
                    client: self.client,
                    params: self.params,
                    mapper,
                }
            }
            pub fn extractor(row: &tokio_postgres::row::Row) -> AuthorNameByIdBorrowed {
                AuthorNameByIdBorrowed {
                    name: row.get(0),
                }
            }
            pub async fn stmt(
                &self,
            ) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
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
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok((self.mapper)(Self::extractor(&row)))
            }
            pub async fn vec(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.stream().await?.try_collect().await
            }
            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                Ok(
                    self
                        .client
                        .query_opt(&stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)(Self::extractor(&row))),
                )
            }
            pub async fn stream(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt().await?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)(Self::extractor(&row))))
                    .into_stream();
                Ok(stream)
            }
        }
        pub fn author_name_by_id<'a, C: GenericClient>(
            client: &'a C,
            id: &'a i32,
        ) -> AuthorNameByIdQuery<'a, C, AuthorNameById> {
            AuthorNameByIdQuery {
                client,
                params: [id],
                mapper: |it| AuthorNameById::from(it),
            }
        }
        #[derive(Debug, Clone)]
        pub struct AuthorNameStartingWithParams<'a> {
            pub start_str: &'a str,
        }
        impl<'a> AuthorNameStartingWithParams<'a> {
            pub fn query<C: GenericClient>(
                &'a self,
                client: &'a C,
            ) -> AuthorNameStartingWithQuery<'a, C, AuthorNameStartingWith> {
                author_name_starting_with(client, &self.start_str)
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
        pub struct AuthorNameStartingWithQuery<'a, C: GenericClient, T> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); 1],
            mapper: fn(AuthorNameStartingWithBorrowed) -> T,
        }
        impl<'a, C, T: 'a> AuthorNameStartingWithQuery<'a, C, T>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(AuthorNameStartingWithBorrowed) -> R,
            ) -> AuthorNameStartingWithQuery<'a, C, R> {
                AuthorNameStartingWithQuery {
                    client: self.client,
                    params: self.params,
                    mapper,
                }
            }
            pub fn extractor(
                row: &tokio_postgres::row::Row,
            ) -> AuthorNameStartingWithBorrowed {
                AuthorNameStartingWithBorrowed {
                    authorid: row.get(0),
                    name: row.get(1),
                    bookid: row.get(2),
                    title: row.get(3),
                }
            }
            pub async fn stmt(
                &self,
            ) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
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
    Author.Name LIKE CONCAT($1::text, '%');",
                    )
                    .await
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok((self.mapper)(Self::extractor(&row)))
            }
            pub async fn vec(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.stream().await?.try_collect().await
            }
            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                Ok(
                    self
                        .client
                        .query_opt(&stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)(Self::extractor(&row))),
                )
            }
            pub async fn stream(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt().await?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)(Self::extractor(&row))))
                    .into_stream();
                Ok(stream)
            }
        }
        pub fn author_name_starting_with<'a, C: GenericClient>(
            client: &'a C,
            start_str: &'a &str,
        ) -> AuthorNameStartingWithQuery<'a, C, AuthorNameStartingWith> {
            AuthorNameStartingWithQuery {
                client,
                params: [start_str],
                mapper: |it| AuthorNameStartingWith::from(it),
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ReturnCustomType {
            pub col1: super::super::types::public::CustomComposite,
        }
        pub struct ReturnCustomTypeBorrowed<'a> {
            pub col1: super::super::types::public::CustomCompositeBorrowed<'a>,
        }
        impl<'a> From<ReturnCustomTypeBorrowed<'a>> for ReturnCustomType {
            fn from(
                ReturnCustomTypeBorrowed { col1 }: ReturnCustomTypeBorrowed<'a>,
            ) -> Self {
                Self { col1: col1.into() }
            }
        }
        pub struct ReturnCustomTypeQuery<'a, C: GenericClient, T> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); 0],
            mapper: fn(ReturnCustomTypeBorrowed) -> T,
        }
        impl<'a, C, T: 'a> ReturnCustomTypeQuery<'a, C, T>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(ReturnCustomTypeBorrowed) -> R,
            ) -> ReturnCustomTypeQuery<'a, C, R> {
                ReturnCustomTypeQuery {
                    client: self.client,
                    params: self.params,
                    mapper,
                }
            }
            pub fn extractor(
                row: &tokio_postgres::row::Row,
            ) -> ReturnCustomTypeBorrowed {
                ReturnCustomTypeBorrowed {
                    col1: row.get(0),
                }
            }
            pub async fn stmt(
                &self,
            ) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client.prepare("SELECT
    col1
FROM
    CustomTable;").await
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok((self.mapper)(Self::extractor(&row)))
            }
            pub async fn vec(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.stream().await?.try_collect().await
            }
            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                Ok(
                    self
                        .client
                        .query_opt(&stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)(Self::extractor(&row))),
                )
            }
            pub async fn stream(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt().await?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)(Self::extractor(&row))))
                    .into_stream();
                Ok(stream)
            }
        }
        pub fn return_custom_type<'a, C: GenericClient>(
            client: &'a C,
        ) -> ReturnCustomTypeQuery<'a, C, ReturnCustomType> {
            ReturnCustomTypeQuery {
                client,
                params: [],
                mapper: |it| ReturnCustomType::from(it),
            }
        }
        #[derive(Debug, Copy, Clone)]
        pub struct SelectWhereCustomTypeParams {
            pub spongebob_character: super::super::types::public::SpongebobCharacter,
        }
        impl SelectWhereCustomTypeParams {
            pub fn query<'a, C: GenericClient>(
                &'a self,
                client: &'a C,
            ) -> SelectWhereCustomTypeQuery<'a, C, SelectWhereCustomType> {
                select_where_custom_type(client, &self.spongebob_character)
            }
        }
        #[derive(Debug, Copy, Clone, PartialEq)]
        pub struct SelectWhereCustomType {
            pub col2: super::super::types::public::SpongebobCharacter,
        }
        pub struct SelectWhereCustomTypeQuery<'a, C: GenericClient, T> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); 1],
            mapper: fn(SelectWhereCustomType) -> T,
        }
        impl<'a, C, T: 'a> SelectWhereCustomTypeQuery<'a, C, T>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(SelectWhereCustomType) -> R,
            ) -> SelectWhereCustomTypeQuery<'a, C, R> {
                SelectWhereCustomTypeQuery {
                    client: self.client,
                    params: self.params,
                    mapper,
                }
            }
            pub fn extractor(row: &tokio_postgres::row::Row) -> SelectWhereCustomType {
                SelectWhereCustomType {
                    col2: row.get(0),
                }
            }
            pub async fn stmt(
                &self,
            ) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client
                    .prepare(
                        "SELECT
    col2
FROM
    CustomTable
WHERE (col1).persona = $1;",
                    )
                    .await
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok((self.mapper)(Self::extractor(&row)))
            }
            pub async fn vec(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.stream().await?.try_collect().await
            }
            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                Ok(
                    self
                        .client
                        .query_opt(&stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)(Self::extractor(&row))),
                )
            }
            pub async fn stream(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt().await?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)(Self::extractor(&row))))
                    .into_stream();
                Ok(stream)
            }
        }
        pub fn select_where_custom_type<'a, C: GenericClient>(
            client: &'a C,
            spongebob_character: &'a super::super::types::public::SpongebobCharacter,
        ) -> SelectWhereCustomTypeQuery<'a, C, SelectWhereCustomType> {
            SelectWhereCustomTypeQuery {
                client,
                params: [spongebob_character],
                mapper: |it| SelectWhereCustomType::from(it),
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SelectTranslations {
            pub translations: Vec<String>,
        }
        pub struct SelectTranslationsBorrowed<'a> {
            pub translations: cornucopia_client::ArrayIterator<'a, &'a str>,
        }
        impl<'a> From<SelectTranslationsBorrowed<'a>> for SelectTranslations {
            fn from(
                SelectTranslationsBorrowed {
                    translations,
                }: SelectTranslationsBorrowed<'a>,
            ) -> Self {
                Self {
                    translations: translations.map(|v| v.into()).collect(),
                }
            }
        }
        pub struct SelectTranslationsQuery<'a, C: GenericClient, T> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); 0],
            mapper: fn(SelectTranslationsBorrowed) -> T,
        }
        impl<'a, C, T: 'a> SelectTranslationsQuery<'a, C, T>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(SelectTranslationsBorrowed) -> R,
            ) -> SelectTranslationsQuery<'a, C, R> {
                SelectTranslationsQuery {
                    client: self.client,
                    params: self.params,
                    mapper,
                }
            }
            pub fn extractor(
                row: &tokio_postgres::row::Row,
            ) -> SelectTranslationsBorrowed {
                SelectTranslationsBorrowed {
                    translations: row.get(0),
                }
            }
            pub async fn stmt(
                &self,
            ) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client.prepare("SELECT
    Translations
FROM
    Book;

").await
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok((self.mapper)(Self::extractor(&row)))
            }
            pub async fn vec(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.stream().await?.try_collect().await
            }
            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                Ok(
                    self
                        .client
                        .query_opt(&stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)(Self::extractor(&row))),
                )
            }
            pub async fn stream(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt().await?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)(Self::extractor(&row))))
                    .into_stream();
                Ok(stream)
            }
        }
        pub fn select_translations<'a, C: GenericClient>(
            client: &'a C,
        ) -> SelectTranslationsQuery<'a, C, SelectTranslations> {
            SelectTranslationsQuery {
                client,
                params: [],
                mapper: |it| SelectTranslations::from(it),
            }
        }
    }
    pub mod module_1 {
        use futures::{StreamExt, TryStreamExt};
        use cornucopia_client::GenericClient;
        #[derive(Debug, Clone)]
        pub struct InsertBookParams<'a> {
            pub title: &'a str,
        }
        impl<'a> InsertBookParams<'a> {
            pub async fn query<C: GenericClient>(
                &'a self,
                client: &'a C,
            ) -> Result<u64, tokio_postgres::Error> {
                insert_book(client, &self.title).await
            }
        }
        pub async fn insert_book<'a, C: GenericClient>(
            client: &'a C,
            title: &'a &str,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = client
                .prepare("INSERT INTO Book (title)
  VALUES ($1);

")
                .await?;
            client.execute(&stmt, &[title]).await
        }
    }
}
