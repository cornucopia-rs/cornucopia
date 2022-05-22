// This file was generated with `cornucopia`. Do not modify.

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

        #[derive(Debug, postgres_types::ToSql, Clone, PartialEq)]
        #[postgres(name = "custom_composite")]
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
                let num_fields = postgres_types::private::read_be_i32(&mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let wow = postgres_types::private::read_value(fields[0].type_(), &mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let such_cool = postgres_types::private::read_value(fields[1].type_(), &mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let nice = postgres_types::private::read_value(fields[2].type_(), &mut buf)?;
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
                let num_fields = postgres_types::private::read_be_i32(&mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let wow = postgres_types::private::read_value(fields[0].type_(), &mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let such_cool = postgres_types::private::read_value(fields[1].type_(), &mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let nice = postgres_types::private::read_value(fields[2].type_(), &mut buf)?;
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

        #[derive(Debug, Clone, PartialEq, postgres_types::ToSql)]
        #[postgres(name = "custom_domain")]
        pub struct CustomDomain(pub Vec<super::super::types::public::CustomComposite>);

        impl<'a> postgres_types::FromSql<'a> for CustomDomain {
            fn from_sql(
                _type: &postgres_types::Type,
                buf: &'a [u8],
            ) -> std::result::Result<
                CustomDomain,
                std::boxed::Box<dyn std::error::Error + std::marker::Sync + std::marker::Send>,
            > {
                let inner = match *_type.kind() {
                    postgres_types::Kind::Domain(ref inner) => inner,
                    _ => unreachable!(),
                };
                let mut buf = buf;

                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                std::result::Result::Ok(CustomDomain(postgres_types::private::read_value(
                    inner, &mut buf,
                )?))
            }

            fn accepts(type_: &postgres_types::Type) -> bool {
                type_.name() == "custom_domain" && type_.schema() == "public"
            }
        }
        pub struct CustomDomainBorrowed<'a>(
            pub  cornucopia_client::ArrayIterator<
                'a,
                super::super::types::public::CustomCompositeBorrowed<'a>,
            >,
        );

        impl<'a> postgres_types::FromSql<'a> for CustomDomainBorrowed<'a> {
            fn from_sql(
                _type: &postgres_types::Type,
                buf: &'a [u8],
            ) -> std::result::Result<
                CustomDomainBorrowed<'a>,
                std::boxed::Box<dyn std::error::Error + std::marker::Sync + std::marker::Send>,
            > {
                let inner = match *_type.kind() {
                    postgres_types::Kind::Domain(ref inner) => inner,
                    _ => unreachable!(),
                };
                let mut buf = buf;

                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                std::result::Result::Ok(CustomDomainBorrowed(postgres_types::private::read_value(
                    inner, &mut buf,
                )?))
            }

            fn accepts(type_: &postgres_types::Type) -> bool {
                type_.name() == "custom_domain" && type_.schema() == "public"
            }
        }

        impl<'a> From<CustomDomainBorrowed<'a>> for CustomDomain {
            fn from(CustomDomainBorrowed(inner): CustomDomainBorrowed<'a>) -> Self {
                Self(inner.map(|v| v.into()).collect())
            }
        }

        #[derive(Debug, Clone, PartialEq, postgres_types::ToSql)]
        #[postgres(name = "my_domain")]
        pub struct MyDomain(pub String);

        impl<'a> postgres_types::FromSql<'a> for MyDomain {
            fn from_sql(
                _type: &postgres_types::Type,
                buf: &'a [u8],
            ) -> std::result::Result<
                MyDomain,
                std::boxed::Box<dyn std::error::Error + std::marker::Sync + std::marker::Send>,
            > {
                let inner = match *_type.kind() {
                    postgres_types::Kind::Domain(ref inner) => inner,
                    _ => unreachable!(),
                };
                let mut buf = buf;

                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                std::result::Result::Ok(MyDomain(postgres_types::private::read_value(
                    inner, &mut buf,
                )?))
            }

            fn accepts(type_: &postgres_types::Type) -> bool {
                type_.name() == "my_domain" && type_.schema() == "public"
            }
        }
        pub struct MyDomainBorrowed<'a>(pub &'a str);

        impl<'a> postgres_types::FromSql<'a> for MyDomainBorrowed<'a> {
            fn from_sql(
                _type: &postgres_types::Type,
                buf: &'a [u8],
            ) -> std::result::Result<
                MyDomainBorrowed<'a>,
                std::boxed::Box<dyn std::error::Error + std::marker::Sync + std::marker::Send>,
            > {
                let inner = match *_type.kind() {
                    postgres_types::Kind::Domain(ref inner) => inner,
                    _ => unreachable!(),
                };
                let mut buf = buf;

                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                std::result::Result::Ok(MyDomainBorrowed(postgres_types::private::read_value(
                    inner, &mut buf,
                )?))
            }

            fn accepts(type_: &postgres_types::Type) -> bool {
                type_.name() == "my_domain" && type_.schema() == "public"
            }
        }

        impl<'a> From<MyDomainBorrowed<'a>> for MyDomain {
            fn from(MyDomainBorrowed(inner): MyDomainBorrowed<'a>) -> Self {
                Self(inner.to_owned())
            }
        }
    }
}

pub mod queries {
    pub mod module_1 {
        use futures::{StreamExt, TryStreamExt};

        pub struct InsertBookParams<'a> {
            pub book_name: &'a str,
        }
        impl<'a> InsertBookParams<'a> {
            pub fn insert_book<C: cornucopia_client::GenericClient>(
                &'a self,
                client: &'a C,
            ) -> InsertBookQuery<'a, C> {
                insert_book(client, &self.book_name)
            }
        }

        pub struct InsertBookQuery<'a, C: cornucopia_client::GenericClient> {
            client: &'a C,
            params: [&'a (dyn tokio_postgres::types::ToSql + Sync); 1],
        }

        impl<'a, C> InsertBookQuery<'a, C>
        where
            C: cornucopia_client::GenericClient,
        {
            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client
                    .prepare(
                        "INSERT INTO Book (title)
  VALUES ($1);

",
                    )
                    .await
            }

            pub async fn exec(self) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                self.client.execute(&stmt, &self.params).await
            }
        }
        pub fn insert_book<'a, C: cornucopia_client::GenericClient>(
            client: &'a C,
            book_name: &'a &str,
        ) -> InsertBookQuery<'a, C> {
            InsertBookQuery {
                client,
                params: [book_name],
            }
        }
    }

    pub mod module_2 {
        use futures::{StreamExt, TryStreamExt};

        pub struct AuthorNameStartingWithParams<'a> {
            pub s: &'a str,
        }
        impl<'a> AuthorNameStartingWithParams<'a> {
            pub fn author_name_starting_with<C: cornucopia_client::GenericClient>(
                &'a self,
                client: &'a C,
            ) -> AuthorNameStartingWithQuery<'a, C, AuthorNameStartingWith> {
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
        pub struct AuthorNameStartingWithQuery<'a, C: cornucopia_client::GenericClient, T> {
            client: &'a C,
            params: [&'a (dyn tokio_postgres::types::ToSql + Sync); 1],
            mapper: fn(AuthorNameStartingWithBorrowed) -> T,
        }

        impl<'a, C, T> AuthorNameStartingWithQuery<'a, C, T>
        where
            C: cornucopia_client::GenericClient,
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

            pub fn extractor(row: &tokio_postgres::row::Row) -> AuthorNameStartingWithBorrowed {
                AuthorNameStartingWithBorrowed {
                    authorid: row.get(0),
                    name: row.get(1),
                    bookid: row.get(2),
                    title: row.get(3),
                }
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
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)
                    .await?
                    .map(|row| (self.mapper)(Self::extractor(&row))))
            }

            pub async fn stream(
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
                    .map(move |res| res.map(|row| (self.mapper)(Self::extractor(&row))));
                Ok(stream.into_stream())
            }
        }
        pub fn author_name_starting_with<'a, C: cornucopia_client::GenericClient>(
            client: &'a C,
            s: &'a &str,
        ) -> AuthorNameStartingWithQuery<'a, C, AuthorNameStartingWith> {
            AuthorNameStartingWithQuery {
                client,
                params: [s],
                mapper: |it| AuthorNameStartingWith::from(it),
            }
        }

        pub struct SelectEverythingBorrowed<'a> {
            pub custom_domain_: cornucopia_client::ArrayIterator<
                'a,
                super::super::types::public::CustomCompositeBorrowed<'a>,
            >,
            pub domain_: &'a str,
            pub custom_array_: cornucopia_client::ArrayIterator<
                'a,
                super::super::types::public::SpongebobCharacter,
            >,
            pub array_: cornucopia_client::ArrayIterator<'a, bool>,
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
            pub custom_domain_: Vec<super::super::types::public::CustomComposite>,
            pub domain_: String,
            pub custom_array_: Vec<super::super::types::public::SpongebobCharacter>,
            pub array_: Vec<bool>,
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
                    custom_domain_,
                    domain_,
                    custom_array_,
                    array_,
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
                    custom_domain_: custom_domain_.map(|v| v.into()).collect(),
                    domain_: domain_.to_owned(),
                    custom_array_: custom_array_.map(|v| v.into()).collect(),
                    array_: array_.map(|v| v.into()).collect(),
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
        pub struct SelectEverythingQuery<'a, C: cornucopia_client::GenericClient, T> {
            client: &'a C,
            params: [&'a (dyn tokio_postgres::types::ToSql + Sync); 0],
            mapper: fn(SelectEverythingBorrowed) -> T,
        }

        impl<'a, C, T> SelectEverythingQuery<'a, C, T>
        where
            C: cornucopia_client::GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(SelectEverythingBorrowed) -> R,
            ) -> SelectEverythingQuery<'a, C, R> {
                SelectEverythingQuery {
                    client: self.client,
                    params: self.params,
                    mapper,
                }
            }

            pub fn extractor(row: &tokio_postgres::row::Row) -> SelectEverythingBorrowed {
                SelectEverythingBorrowed {
                    custom_domain_: row.get(0),
                    domain_: row.get(1),
                    custom_array_: row.get(2),
                    array_: row.get(3),
                    bool_: row.get(4),
                    boolean_: row.get(5),
                    char_: row.get(6),
                    smallint_: row.get(7),
                    int2_: row.get(8),
                    smallserial_: row.get(9),
                    serial2_: row.get(10),
                    int_: row.get(11),
                    int4_: row.get(12),
                    serial_: row.get(13),
                    serial4_: row.get(14),
                    bingint_: row.get(15),
                    int8_: row.get(16),
                    bigserial_: row.get(17),
                    serial8_: row.get(18),
                    float4_: row.get(19),
                    real_: row.get(20),
                    float8_: row.get(21),
                    double_precision_: row.get(22),
                    text_: row.get(23),
                    varchar_: row.get(24),
                    bytea_: row.get(25),
                    timestamp_: row.get(26),
                    timestamp_without_time_zone_: row.get(27),
                    timestamptz_: row.get(28),
                    timestamp_with_time_zone_: row.get(29),
                    date_: row.get(30),
                    time_: row.get(31),
                    json_: row.get(32),
                    jsonb_: row.get(33),
                    uuid_: row.get(34),
                    inet_: row.get(35),
                    macaddr_: row.get(36),
                }
            }

            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client
                    .prepare(
                        "SELECT
    *
FROM
    Everything;",
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
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)
                    .await?
                    .map(|row| (self.mapper)(Self::extractor(&row))))
            }

            pub async fn stream(
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
                    .map(move |res| res.map(|row| (self.mapper)(Self::extractor(&row))));
                Ok(stream.into_stream())
            }
        }
        pub fn select_everything<'a, C: cornucopia_client::GenericClient>(
            client: &'a C,
        ) -> SelectEverythingQuery<'a, C, SelectEverything> {
            SelectEverythingQuery {
                client,
                params: [],
                mapper: |it| SelectEverything::from(it),
            }
        }

        pub struct InsertEverythingParams<'a> {
            pub custom_domain_: super::super::types::public::CustomDomain,
            pub custom_array_: &'a [super::super::types::public::SpongebobCharacter],
            pub domain_: super::super::types::public::MyDomain,
            pub array_: &'a [bool],
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
        impl<'a> InsertEverythingParams<'a> {
            pub fn insert_everything<C: cornucopia_client::GenericClient>(
                &'a self,
                client: &'a C,
            ) -> InsertEverythingQuery<'a, C> {
                insert_everything(
                    client,
                    &self.custom_domain_,
                    &self.custom_array_,
                    &self.domain_,
                    &self.array_,
                    &self.bool_,
                    &self.boolean_,
                    &self.char_,
                    &self.smallint_,
                    &self.int2_,
                    &self.smallserial_,
                    &self.serial2_,
                    &self.int_,
                    &self.int4_,
                    &self.serial_,
                    &self.serial4_,
                    &self.bingint_,
                    &self.int8_,
                    &self.bigserial_,
                    &self.serial8_,
                    &self.float4_,
                    &self.real_,
                    &self.float8_,
                    &self.double_precision_,
                    &self.text_,
                    &self.varchar_,
                    &self.bytea_,
                    &self.timestamp_,
                    &self.timestamp_without_time_zone_,
                    &self.timestamptz_,
                    &self.timestamp_with_time_zone_,
                    &self.date_,
                    &self.time_,
                    &self.json_,
                    &self.jsonb_,
                    &self.uuid_,
                    &self.inet_,
                    &self.macaddr_,
                )
            }
        }

        pub struct InsertEverythingQuery<'a, C: cornucopia_client::GenericClient> {
            client: &'a C,
            params: [&'a (dyn tokio_postgres::types::ToSql + Sync); 37],
        }

        impl<'a, C> InsertEverythingQuery<'a, C>
        where
            C: cornucopia_client::GenericClient,
        {
            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client.prepare("INSERT INTO Everything (custom_domain_, custom_array_, domain_, array_, bool_, boolean_, char_, smallint_, int2_, smallserial_, serial2_, int_, int4_, serial_, serial4_, bingint_, int8_, bigserial_, serial8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37);

").await
            }

            pub async fn exec(self) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                self.client.execute(&stmt, &self.params).await
            }
        }
        pub fn insert_everything<'a, C: cornucopia_client::GenericClient>(
            client: &'a C,
            custom_domain_: &'a super::super::types::public::CustomDomain,
            custom_array_: &'a &'a [super::super::types::public::SpongebobCharacter],
            domain_: &'a super::super::types::public::MyDomain,
            array_: &'a &'a [bool],
            bool_: &'a bool,
            boolean_: &'a bool,
            char_: &'a i8,
            smallint_: &'a i16,
            int2_: &'a i16,
            smallserial_: &'a i16,
            serial2_: &'a i16,
            int_: &'a i32,
            int4_: &'a i32,
            serial_: &'a i32,
            serial4_: &'a i32,
            bingint_: &'a i64,
            int8_: &'a i64,
            bigserial_: &'a i64,
            serial8_: &'a i64,
            float4_: &'a f32,
            real_: &'a f32,
            float8_: &'a f64,
            double_precision_: &'a f64,
            text_: &'a &str,
            varchar_: &'a &str,
            bytea_: &'a &[u8],
            timestamp_: &'a time::PrimitiveDateTime,
            timestamp_without_time_zone_: &'a time::PrimitiveDateTime,
            timestamptz_: &'a time::OffsetDateTime,
            timestamp_with_time_zone_: &'a time::OffsetDateTime,
            date_: &'a time::Date,
            time_: &'a time::Time,
            json_: &'a tokio_postgres::types::Json<&serde_json::value::RawValue>,
            jsonb_: &'a tokio_postgres::types::Json<&serde_json::value::RawValue>,
            uuid_: &'a uuid::Uuid,
            inet_: &'a std::net::IpAddr,
            macaddr_: &'a eui48::MacAddress,
        ) -> InsertEverythingQuery<'a, C> {
            InsertEverythingQuery {
                client,
                params: [
                    custom_domain_,
                    custom_array_,
                    domain_,
                    array_,
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
                ],
            }
        }
    }
}
