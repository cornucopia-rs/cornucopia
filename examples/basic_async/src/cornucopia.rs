// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod types {
    pub mod public {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[allow(non_camel_case_types)]
        pub enum SpongeBobCharacter {
            Bob,
            Patrick,
            Squidward,
        }
        impl<'a> postgres_types::ToSql for SpongeBobCharacter {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                buf: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                let s = match *self {
                    SpongeBobCharacter::Bob => "Bob",
                    SpongeBobCharacter::Patrick => "Patrick",
                    SpongeBobCharacter::Squidward => "Squidward",
                };
                buf.extend_from_slice(s.as_bytes());
                std::result::Result::Ok(postgres_types::IsNull::No)
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "sponge_bob_character" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Enum(ref variants) => {
                        if variants.len() != 3 {
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
        impl<'a> postgres_types::FromSql<'a> for SpongeBobCharacter {
            fn from_sql(
                ty: &postgres_types::Type,
                buf: &'a [u8],
            ) -> Result<SpongeBobCharacter, Box<dyn std::error::Error + Sync + Send>> {
                match std::str::from_utf8(buf)? {
                    "Bob" => Ok(SpongeBobCharacter::Bob),
                    "Patrick" => Ok(SpongeBobCharacter::Patrick),
                    "Squidward" => Ok(SpongeBobCharacter::Squidward),
                    s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
                }
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "sponge_bob_character" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Enum(ref variants) => {
                        if variants.len() != 3 {
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
        #[derive(Debug, postgres_types :: FromSql, Clone, PartialEq)]
        #[postgres(name = "voiceactor")]
        pub struct Voiceactor {
            pub name: String,
            pub age: i32,
        }
        #[derive(Debug)]
        pub struct VoiceactorBorrowed<'a> {
            pub name: &'a str,
            pub age: i32,
        }
        impl<'a> From<VoiceactorBorrowed<'a>> for Voiceactor {
            fn from(VoiceactorBorrowed { name, age }: VoiceactorBorrowed<'a>) -> Self {
                Self {
                    name: name.into(),
                    age,
                }
            }
        }
        impl<'a> postgres_types::FromSql<'a> for VoiceactorBorrowed<'a> {
            fn from_sql(
                ty: &postgres_types::Type,
                out: &'a [u8],
            ) -> Result<VoiceactorBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>>
            {
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut out = out;
                let num_fields = postgres_types::private::read_be_i32(&mut out)?;
                if num_fields as usize != fields.len() {
                    return std::result::Result::Err(std::convert::Into::into(format!(
                        "invalid field count: {} vs {}",
                        num_fields,
                        fields.len()
                    )));
                }
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let name = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let age = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
                Ok(VoiceactorBorrowed { name, age })
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                ty.name() == "voiceactor" && ty.schema() == "public"
            }
        }
        impl<'a> postgres_types::ToSql for VoiceactorBorrowed<'a> {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                let VoiceactorBorrowed { name, age } = self;
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
                for field in fields {
                    out.extend_from_slice(&field.type_().oid().to_be_bytes());
                    let base = out.len();
                    out.extend_from_slice(&[0; 4]);
                    let r = match field.name() {
                        "name" => postgres_types::ToSql::to_sql(name, field.type_(), out),
                        "age" => postgres_types::ToSql::to_sql(age, field.type_(), out),
                        _ => unreachable!(),
                    };
                    let count = match r? {
                        postgres_types::IsNull::Yes => -1,
                        postgres_types::IsNull::No => {
                            let len = out.len() - base - 4;
                            if len > i32::max_value() as usize {
                                return Err(Into::into("value too large to transmit"));
                            }
                            len as i32
                        }
                    };
                    out[base..base + 4].copy_from_slice(&count.to_be_bytes());
                }
                Ok(postgres_types::IsNull::No)
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "voiceactor" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => {
                        if fields.len() != 2 {
                            return false;
                        }
                        fields.iter().all(|f| match f.name() {
                            "name" => <&'a str as postgres_types::ToSql>::accepts(f.type_()),
                            "age" => <i32 as postgres_types::ToSql>::accepts(f.type_()),
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
    }
}
#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod queries {
    pub mod module_1 {
        pub mod tokio {
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
    }
    pub mod module_2 {
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
        pub mod tokio {
            use cornucopia_async::GenericClient;
            use futures;
            use futures::{StreamExt, TryStreamExt};
            pub struct AuthorsQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor: fn(&tokio_postgres::Row) -> super::AuthorsBorrowed,
                mapper: fn(super::AuthorsBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> AuthorsQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::AuthorsBorrowed) -> R,
                ) -> AuthorsQuery<'a, C, R, N> {
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
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub struct StringQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
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
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub struct AuthorNameStartingWithQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor: fn(&tokio_postgres::Row) -> super::AuthorNameStartingWithBorrowed,
                mapper: fn(super::AuthorNameStartingWithBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> AuthorNameStartingWithQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::AuthorNameStartingWithBorrowed) -> R,
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
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub struct SuperSuperSuperTypesPublicVoiceactorQuery<
                'a,
                C: GenericClient,
                T,
                const N: usize,
            > {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor: fn(
                    &tokio_postgres::Row,
                )
                    -> super::super::super::types::public::VoiceactorBorrowed,
                mapper: fn(super::super::super::types::public::VoiceactorBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> SuperSuperSuperTypesPublicVoiceactorQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::super::super::types::public::VoiceactorBorrowed) -> R,
                ) -> SuperSuperSuperTypesPublicVoiceactorQuery<'a, C, R, N> {
                    SuperSuperSuperTypesPublicVoiceactorQuery {
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
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub struct SelectTranslationsQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor: fn(&tokio_postgres::Row) -> super::SelectTranslationsBorrowed,
                mapper: fn(super::SelectTranslationsBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> SelectTranslationsQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::SelectTranslationsBorrowed) -> R,
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
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
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
                ) -> AuthorsQuery<'a, C, super::Authors, 0> {
                    AuthorsQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| super::AuthorsBorrowed {
                            id: row.get(0),
                            name: row.get(1),
                            country: row.get(2),
                        },
                        mapper: |it| <super::Authors>::from(it),
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
                ) -> AuthorNameStartingWithQuery<'a, C, super::AuthorNameStartingWith, 1>
                {
                    AuthorNameStartingWithQuery {
                        client,
                        params: [start_str],
                        stmt: &mut self.0,
                        extractor: |row| super::AuthorNameStartingWithBorrowed {
                            authorid: row.get(0),
                            name: row.get(1),
                            bookid: row.get(2),
                            title: row.get(3),
                        },
                        mapper: |it| <super::AuthorNameStartingWith>::from(it),
                    }
                }
            }
            impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
                cornucopia_async::Params<
                    'a,
                    super::AuthorNameStartingWithParams<T1>,
                    AuthorNameStartingWithQuery<'a, C, super::AuthorNameStartingWith, 1>,
                    C,
                > for AuthorNameStartingWithStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a C,
                    params: &'a super::AuthorNameStartingWithParams<T1>,
                ) -> AuthorNameStartingWithQuery<'a, C, super::AuthorNameStartingWith, 1>
                {
                    self.bind(client, &params.start_str)
                }
            }
            pub fn select_voice_actor_with_character() -> SelectVoiceActorWithCharacterStmt {
                SelectVoiceActorWithCharacterStmt(cornucopia_async::private::Stmt::new(
                    "SELECT
    voice_actor
FROM
    SpongeBobVoiceActor
WHERE
    character = $1",
                ))
            }
            pub struct SelectVoiceActorWithCharacterStmt(cornucopia_async::private::Stmt);
            impl SelectVoiceActorWithCharacterStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                    spongebob_character: &'a super::super::super::types::public::SpongeBobCharacter,
                ) -> SuperSuperSuperTypesPublicVoiceactorQuery<
                    'a,
                    C,
                    super::super::super::types::public::Voiceactor,
                    1,
                > {
                    SuperSuperSuperTypesPublicVoiceactorQuery {
                        client,
                        params: [spongebob_character],
                        stmt: &mut self.0,
                        extractor: |row| row.get(0),
                        mapper: |it| it.into(),
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
                ) -> SelectTranslationsQuery<'a, C, super::SelectTranslations, 0> {
                    SelectTranslationsQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| super::SelectTranslationsBorrowed {
                            title: row.get(0),
                            translations: row.get(1),
                        },
                        mapper: |it| <super::SelectTranslations>::from(it),
                    }
                }
            }
        }
    }
}
