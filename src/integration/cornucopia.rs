pub mod types {
    pub mod public {
        #[derive(
            Debug,
            cornucopia_client::types::ToSql,
            cornucopia_client::types::FromSql,
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
        #[derive(Debug, cornucopia_client::types::ToSql, Clone, PartialEq)]
        #[postgres(name = "custom_composite")]
        pub struct CustomComposite {
            pub wow: String,
            pub such_cool: i32,
            pub nice: super::public::SpongebobCharacter,
        }
        impl<'a> cornucopia_client::types::FromSql<'a> for CustomComposite {
            fn from_sql(
                _type: &cornucopia_client::types::Type,
                buf: &'a [u8],
            ) -> std::result::Result<
                    CustomComposite,
                    std::boxed::Box<
                        dyn std::error::Error + std::marker::Sync + std::marker::Send,
                    >,
                > {
                let fields = match *_type.kind() {
                    cornucopia_client::types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut buf = buf;
                let num_fields = cornucopia_client::types::private::read_be_i32(
                    &mut buf,
                )?;
                let _oid = cornucopia_client::types::private::read_be_i32(&mut buf)?;
                let wow = cornucopia_client::types::private::read_value(
                    fields[0].type_(),
                    &mut buf,
                )?;
                let _oid = cornucopia_client::types::private::read_be_i32(&mut buf)?;
                let such_cool = cornucopia_client::types::private::read_value(
                    fields[1].type_(),
                    &mut buf,
                )?;
                let _oid = cornucopia_client::types::private::read_be_i32(&mut buf)?;
                let nice = cornucopia_client::types::private::read_value(
                    fields[2].type_(),
                    &mut buf,
                )?;
                std::result::Result::Ok(CustomComposite {
                    wow,
                    such_cool,
                    nice,
                })
            }
            fn accepts(type_: &cornucopia_client::types::Type) -> bool {
                type_.name() == "custom_composite" && type_.schema() == "public"
            }
        }
        pub struct CustomCompositeBorrowed<'a> {
            pub wow: &'a str,
            pub such_cool: i32,
            pub nice: super::super::types::public::SpongebobCharacter,
        }
        impl<'a> cornucopia_client::types::FromSql<'a> for CustomCompositeBorrowed<'a> {
            fn from_sql(
                _type: &cornucopia_client::types::Type,
                buf: &'a [u8],
            ) -> std::result::Result<
                    CustomCompositeBorrowed<'a>,
                    std::boxed::Box<
                        dyn std::error::Error + std::marker::Sync + std::marker::Send,
                    >,
                > {
                let fields = match *_type.kind() {
                    cornucopia_client::types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut buf = buf;
                let num_fields = cornucopia_client::types::private::read_be_i32(
                    &mut buf,
                )?;
                let _oid = cornucopia_client::types::private::read_be_i32(&mut buf)?;
                let wow = cornucopia_client::types::private::read_value(
                    fields[0].type_(),
                    &mut buf,
                )?;
                let _oid = cornucopia_client::types::private::read_be_i32(&mut buf)?;
                let such_cool = cornucopia_client::types::private::read_value(
                    fields[1].type_(),
                    &mut buf,
                )?;
                let _oid = cornucopia_client::types::private::read_be_i32(&mut buf)?;
                let nice = cornucopia_client::types::private::read_value(
                    fields[2].type_(),
                    &mut buf,
                )?;
                std::result::Result::Ok(CustomCompositeBorrowed {
                    wow,
                    such_cool,
                    nice,
                })
            }
            fn accepts(type_: &cornucopia_client::types::Type) -> bool {
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
                    wow: wow.into(),
                    such_cool,
                    nice,
                }
            }
        }
        #[derive(Debug, Clone, PartialEq, cornucopia_client::types::ToSql)]
        #[postgres(name = "custom_domain")]
        pub struct CustomDomain(pub Vec<super::super::types::public::CustomComposite>);
        impl<'a> cornucopia_client::types::FromSql<'a> for CustomDomain {
            fn from_sql(
                _type: &cornucopia_client::types::Type,
                buf: &'a [u8],
            ) -> std::result::Result<
                    CustomDomain,
                    std::boxed::Box<
                        dyn std::error::Error + std::marker::Sync + std::marker::Send,
                    >,
                > {
                let inner = match *_type.kind() {
                    cornucopia_client::types::Kind::Domain(ref inner) => inner,
                    _ => unreachable!(),
                };
                let mut buf = buf;
                let _oid = cornucopia_client::types::private::read_be_i32(&mut buf)?;
                std::result::Result::Ok(
                    CustomDomain(
                        cornucopia_client::types::private::read_value(inner, &mut buf)?,
                    ),
                )
            }
            fn accepts(type_: &cornucopia_client::types::Type) -> bool {
                type_.name() == "custom_domain" && type_.schema() == "public"
            }
        }
        pub struct CustomDomainBorrowed<'a>(
            pub cornucopia_client::ArrayIterator<
                'a,
                super::super::types::public::CustomCompositeBorrowed<'a>,
            >,
        );
        impl<'a> cornucopia_client::types::FromSql<'a> for CustomDomainBorrowed<'a> {
            fn from_sql(
                _type: &cornucopia_client::types::Type,
                buf: &'a [u8],
            ) -> std::result::Result<
                    CustomDomainBorrowed<'a>,
                    std::boxed::Box<
                        dyn std::error::Error + std::marker::Sync + std::marker::Send,
                    >,
                > {
                let inner = match *_type.kind() {
                    cornucopia_client::types::Kind::Domain(ref inner) => inner,
                    _ => unreachable!(),
                };
                let mut buf = buf;
                let _oid = cornucopia_client::types::private::read_be_i32(&mut buf)?;
                std::result::Result::Ok(
                    CustomDomainBorrowed(
                        cornucopia_client::types::private::read_value(inner, &mut buf)?,
                    ),
                )
            }
            fn accepts(type_: &cornucopia_client::types::Type) -> bool {
                type_.name() == "custom_domain" && type_.schema() == "public"
            }
        }
        impl<'a> From<CustomDomainBorrowed<'a>> for CustomDomain {
            fn from(CustomDomainBorrowed(inner): CustomDomainBorrowed<'a>) -> Self {
                Self(inner.map(|v| v.into()).collect())
            }
        }
        #[derive(Debug, Clone, PartialEq, cornucopia_client::types::ToSql)]
        #[postgres(name = "my_domain")]
        pub struct MyDomain(pub String);
        impl<'a> cornucopia_client::types::FromSql<'a> for MyDomain {
            fn from_sql(
                _type: &cornucopia_client::types::Type,
                buf: &'a [u8],
            ) -> std::result::Result<
                    MyDomain,
                    std::boxed::Box<
                        dyn std::error::Error + std::marker::Sync + std::marker::Send,
                    >,
                > {
                let inner = match *_type.kind() {
                    cornucopia_client::types::Kind::Domain(ref inner) => inner,
                    _ => unreachable!(),
                };
                let mut buf = buf;
                let _oid = cornucopia_client::types::private::read_be_i32(&mut buf)?;
                std::result::Result::Ok(
                    MyDomain(
                        cornucopia_client::types::private::read_value(inner, &mut buf)?,
                    ),
                )
            }
            fn accepts(type_: &cornucopia_client::types::Type) -> bool {
                type_.name() == "my_domain" && type_.schema() == "public"
            }
        }
        pub struct MyDomainBorrowed<'a>(pub &'a str);
        impl<'a> cornucopia_client::types::FromSql<'a> for MyDomainBorrowed<'a> {
            fn from_sql(
                _type: &cornucopia_client::types::Type,
                buf: &'a [u8],
            ) -> std::result::Result<
                    MyDomainBorrowed<'a>,
                    std::boxed::Box<
                        dyn std::error::Error + std::marker::Sync + std::marker::Send,
                    >,
                > {
                let inner = match *_type.kind() {
                    cornucopia_client::types::Kind::Domain(ref inner) => inner,
                    _ => unreachable!(),
                };
                let mut buf = buf;
                let _oid = cornucopia_client::types::private::read_be_i32(&mut buf)?;
                std::result::Result::Ok(
                    MyDomainBorrowed(
                        cornucopia_client::types::private::read_value(inner, &mut buf)?,
                    ),
                )
            }
            fn accepts(type_: &cornucopia_client::types::Type) -> bool {
                type_.name() == "my_domain" && type_.schema() == "public"
            }
        }
        impl<'a> From<MyDomainBorrowed<'a>> for MyDomain {
            fn from(MyDomainBorrowed(inner): MyDomainBorrowed<'a>) -> Self {
                Self(inner.into())
            }
        }
        #[derive(Debug, cornucopia_client::types::ToSql, Clone, PartialEq)]
        #[postgres(name = "nightmare_composite")]
        pub struct NightmareComposite {
            pub custom: Vec<super::super::types::public::CustomComposite>,
            pub spongebob: Vec<super::super::types::public::SpongebobCharacter>,
        }
        impl<'a> cornucopia_client::types::FromSql<'a> for NightmareComposite {
            fn from_sql(
                _type: &cornucopia_client::types::Type,
                buf: &'a [u8],
            ) -> std::result::Result<
                    NightmareComposite,
                    std::boxed::Box<
                        dyn std::error::Error + std::marker::Sync + std::marker::Send,
                    >,
                > {
                let fields = match *_type.kind() {
                    cornucopia_client::types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut buf = buf;
                let num_fields = cornucopia_client::types::private::read_be_i32(
                    &mut buf,
                )?;
                let _oid = cornucopia_client::types::private::read_be_i32(&mut buf)?;
                let custom = cornucopia_client::types::private::read_value(
                    fields[0].type_(),
                    &mut buf,
                )?;
                let _oid = cornucopia_client::types::private::read_be_i32(&mut buf)?;
                let spongebob = cornucopia_client::types::private::read_value(
                    fields[1].type_(),
                    &mut buf,
                )?;
                std::result::Result::Ok(NightmareComposite {
                    custom,
                    spongebob,
                })
            }
            fn accepts(type_: &cornucopia_client::types::Type) -> bool {
                type_.name() == "nightmare_composite" && type_.schema() == "public"
            }
        }
        pub struct NightmareCompositeBorrowed<'a> {
            pub custom: cornucopia_client::ArrayIterator<
                'a,
                super::super::types::public::CustomCompositeBorrowed<'a>,
            >,
            pub spongebob: cornucopia_client::ArrayIterator<
                'a,
                super::super::types::public::SpongebobCharacter,
            >,
        }
        impl<'a> cornucopia_client::types::FromSql<'a>
        for NightmareCompositeBorrowed<'a> {
            fn from_sql(
                _type: &cornucopia_client::types::Type,
                buf: &'a [u8],
            ) -> std::result::Result<
                    NightmareCompositeBorrowed<'a>,
                    std::boxed::Box<
                        dyn std::error::Error + std::marker::Sync + std::marker::Send,
                    >,
                > {
                let fields = match *_type.kind() {
                    cornucopia_client::types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut buf = buf;
                let num_fields = cornucopia_client::types::private::read_be_i32(
                    &mut buf,
                )?;
                let _oid = cornucopia_client::types::private::read_be_i32(&mut buf)?;
                let custom = cornucopia_client::types::private::read_value(
                    fields[0].type_(),
                    &mut buf,
                )?;
                let _oid = cornucopia_client::types::private::read_be_i32(&mut buf)?;
                let spongebob = cornucopia_client::types::private::read_value(
                    fields[1].type_(),
                    &mut buf,
                )?;
                std::result::Result::Ok(NightmareCompositeBorrowed {
                    custom,
                    spongebob,
                })
            }
            fn accepts(type_: &cornucopia_client::types::Type) -> bool {
                type_.name() == "nightmare_composite" && type_.schema() == "public"
            }
        }
        impl<'a> From<NightmareCompositeBorrowed<'a>> for NightmareComposite {
            fn from(
                NightmareCompositeBorrowed {
                    custom,
                    spongebob,
                }: NightmareCompositeBorrowed<'a>,
            ) -> Self {
                Self {
                    custom: custom.map(|v| v.into()).collect(),
                    spongebob: spongebob.map(|v| v.into()).collect(),
                }
            }
        }
        #[derive(Debug, cornucopia_client::types::ToSql, Copy, Clone, PartialEq)]
        #[postgres(name = "copy_composite")]
        pub struct CopyComposite {
            pub first: i32,
            pub second: f64,
        }
        impl<'a> cornucopia_client::types::FromSql<'a> for CopyComposite {
            fn from_sql(
                _type: &cornucopia_client::types::Type,
                buf: &'a [u8],
            ) -> std::result::Result<
                    CopyComposite,
                    std::boxed::Box<
                        dyn std::error::Error + std::marker::Sync + std::marker::Send,
                    >,
                > {
                let fields = match *_type.kind() {
                    cornucopia_client::types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut buf = buf;
                let num_fields = cornucopia_client::types::private::read_be_i32(
                    &mut buf,
                )?;
                let _oid = cornucopia_client::types::private::read_be_i32(&mut buf)?;
                let first = cornucopia_client::types::private::read_value(
                    fields[0].type_(),
                    &mut buf,
                )?;
                let _oid = cornucopia_client::types::private::read_be_i32(&mut buf)?;
                let second = cornucopia_client::types::private::read_value(
                    fields[1].type_(),
                    &mut buf,
                )?;
                std::result::Result::Ok(CopyComposite { first, second })
            }
            fn accepts(type_: &cornucopia_client::types::Type) -> bool {
                type_.name() == "copy_composite" && type_.schema() == "public"
            }
        }
        #[derive(Debug, Copy, Clone, PartialEq, cornucopia_client::types::ToSql)]
        #[postgres(name = "copy_domain")]
        pub struct CopyDomain(pub i32);
        impl<'a> cornucopia_client::types::FromSql<'a> for CopyDomain {
            fn from_sql(
                _type: &cornucopia_client::types::Type,
                buf: &'a [u8],
            ) -> std::result::Result<
                    CopyDomain,
                    std::boxed::Box<
                        dyn std::error::Error + std::marker::Sync + std::marker::Send,
                    >,
                > {
                let inner = match *_type.kind() {
                    cornucopia_client::types::Kind::Domain(ref inner) => inner,
                    _ => unreachable!(),
                };
                let mut buf = buf;
                let _oid = cornucopia_client::types::private::read_be_i32(&mut buf)?;
                std::result::Result::Ok(
                    CopyDomain(
                        cornucopia_client::types::private::read_value(inner, &mut buf)?,
                    ),
                )
            }
            fn accepts(type_: &cornucopia_client::types::Type) -> bool {
                type_.name() == "copy_domain" && type_.schema() == "public"
            }
        }
    }
}
pub mod queries {
    pub mod module_2 {
        use postgres::fallible_iterator::FallibleIterator;
        use postgres::GenericClient;
        pub struct AuthorNameStartingWithParams<'a> {
            pub start_str: &'a str,
        }
        impl<'a> AuthorNameStartingWithParams<'a> {
            pub fn query<C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> AuthorNameStartingWithQuery<'a, C, AuthorNameStartingWith> {
                author_name_starting_with(client, &self.start_str)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct AuthorNameStartingWith {
            pub name: String,
        }
        pub struct AuthorNameStartingWithBorrowed<'a> {
            pub name: &'a str,
        }
        impl<'a> From<AuthorNameStartingWithBorrowed<'a>> for AuthorNameStartingWith {
            fn from(
                AuthorNameStartingWithBorrowed {
                    name,
                }: AuthorNameStartingWithBorrowed<'a>,
            ) -> Self {
                Self { name: name.into() }
            }
        }
        pub struct AuthorNameStartingWithQuery<'a, C: GenericClient, T> {
            client: &'a mut C,
            params: [&'a (dyn cornucopia_client::types::ToSql + Sync); 1],
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
                row: &postgres::row::Row,
            ) -> AuthorNameStartingWithBorrowed {
                AuthorNameStartingWithBorrowed {
                    name: row.get(0),
                }
            }
            pub fn stmt(&mut self) -> Result<postgres::Statement, postgres::Error> {
                self.client
                    .prepare(
                        "SELECT
    name
FROM
    Author
WHERE
    name LIKE CONCAT($1::text, '%');",
                    )
            }
            pub fn one(mut self) -> Result<T, postgres::Error> {
                let stmt = self.stmt()?;
                let row = self.client.query_one(&stmt, &self.params)?;
                Ok((self.mapper)(Self::extractor(&row)))
            }
            pub fn vec(self) -> Result<Vec<T>, postgres::Error> {
                self.stream()?.collect()
            }
            pub fn opt(mut self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt()?;
                Ok(
                    self
                        .client
                        .query_opt(&stmt, &self.params)?
                        .map(|row| (self.mapper)(Self::extractor(&row))),
                )
            }
            pub fn stream(
                mut self,
            ) -> Result<
                    impl Iterator<Item = Result<T, postgres::Error>> + 'a,
                    postgres::Error,
                > {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)(Self::extractor(&row))));
                Ok(stream)
            }
        }
        pub fn author_name_starting_with<'a, C: GenericClient>(
            client: &'a mut C,
            start_str: &'a &str,
        ) -> AuthorNameStartingWithQuery<'a, C, AuthorNameStartingWith> {
            AuthorNameStartingWithQuery {
                client,
                params: [start_str],
                mapper: |it| AuthorNameStartingWith::from(it),
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SelectEverything {
            pub custom_domain_: Vec<super::super::types::public::CustomComposite>,
            pub custom_array_: Vec<super::super::types::public::SpongebobCharacter>,
            pub domain_: String,
            pub array_: Vec<bool>,
            pub bool_: bool,
            pub bool_opt: Option<bool>,
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
            pub json_: cornucopia_client::types::Json<serde_json::Value>,
            pub jsonb_: cornucopia_client::types::Json<serde_json::Value>,
            pub uuid_: uuid::Uuid,
            pub inet_: std::net::IpAddr,
            pub macaddr_: eui48::MacAddress,
        }
        pub struct SelectEverythingBorrowed<'a> {
            pub custom_domain_: cornucopia_client::ArrayIterator<
                'a,
                super::super::types::public::CustomCompositeBorrowed<'a>,
            >,
            pub custom_array_: cornucopia_client::ArrayIterator<
                'a,
                super::super::types::public::SpongebobCharacter,
            >,
            pub domain_: &'a str,
            pub array_: cornucopia_client::ArrayIterator<'a, bool>,
            pub bool_: bool,
            pub bool_opt: Option<bool>,
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
            pub json_: cornucopia_client::types::Json<&'a serde_json::value::RawValue>,
            pub jsonb_: cornucopia_client::types::Json<&'a serde_json::value::RawValue>,
            pub uuid_: uuid::Uuid,
            pub inet_: std::net::IpAddr,
            pub macaddr_: eui48::MacAddress,
        }
        impl<'a> From<SelectEverythingBorrowed<'a>> for SelectEverything {
            fn from(
                SelectEverythingBorrowed {
                    custom_domain_,
                    custom_array_,
                    domain_,
                    array_,
                    bool_,
                    bool_opt,
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
                    custom_array_: custom_array_.map(|v| v.into()).collect(),
                    domain_: domain_.into(),
                    array_: array_.map(|v| v.into()).collect(),
                    bool_,
                    bool_opt,
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
                    text_: text_.into(),
                    varchar_: varchar_.into(),
                    bytea_: bytea_.into(),
                    timestamp_,
                    timestamp_without_time_zone_,
                    timestamptz_,
                    timestamp_with_time_zone_,
                    date_,
                    time_,
                    json_: cornucopia_client::types::Json(
                        serde_json::from_str(json_.0.get()).unwrap(),
                    ),
                    jsonb_: cornucopia_client::types::Json(
                        serde_json::from_str(jsonb_.0.get()).unwrap(),
                    ),
                    uuid_,
                    inet_,
                    macaddr_,
                }
            }
        }
        pub struct SelectEverythingQuery<'a, C: GenericClient, T> {
            client: &'a mut C,
            params: [&'a (dyn cornucopia_client::types::ToSql + Sync); 0],
            mapper: fn(SelectEverythingBorrowed) -> T,
        }
        impl<'a, C, T: 'a> SelectEverythingQuery<'a, C, T>
        where
            C: GenericClient,
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
            pub fn extractor(row: &postgres::row::Row) -> SelectEverythingBorrowed {
                SelectEverythingBorrowed {
                    custom_domain_: row.get(0),
                    custom_array_: row.get(1),
                    domain_: row.get(2),
                    array_: row.get(3),
                    bool_: row.get(4),
                    bool_opt: row.get(5),
                    boolean_: row.get(6),
                    char_: row.get(7),
                    smallint_: row.get(8),
                    int2_: row.get(9),
                    smallserial_: row.get(10),
                    serial2_: row.get(11),
                    int_: row.get(12),
                    int4_: row.get(13),
                    serial_: row.get(14),
                    serial4_: row.get(15),
                    bingint_: row.get(16),
                    int8_: row.get(17),
                    bigserial_: row.get(18),
                    serial8_: row.get(19),
                    float4_: row.get(20),
                    real_: row.get(21),
                    float8_: row.get(22),
                    double_precision_: row.get(23),
                    text_: row.get(24),
                    varchar_: row.get(25),
                    bytea_: row.get(26),
                    timestamp_: row.get(27),
                    timestamp_without_time_zone_: row.get(28),
                    timestamptz_: row.get(29),
                    timestamp_with_time_zone_: row.get(30),
                    date_: row.get(31),
                    time_: row.get(32),
                    json_: row.get(33),
                    jsonb_: row.get(34),
                    uuid_: row.get(35),
                    inet_: row.get(36),
                    macaddr_: row.get(37),
                }
            }
            pub fn stmt(&mut self) -> Result<postgres::Statement, postgres::Error> {
                self.client
                    .prepare(
                        "SELECT
    custom_domain_,
    custom_array_,
    domain_,
    array_,
    bool_,
    bool_ AS bool_opt,
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
    macaddr_
FROM
    Everything;",
                    )
            }
            pub fn one(mut self) -> Result<T, postgres::Error> {
                let stmt = self.stmt()?;
                let row = self.client.query_one(&stmt, &self.params)?;
                Ok((self.mapper)(Self::extractor(&row)))
            }
            pub fn vec(self) -> Result<Vec<T>, postgres::Error> {
                self.stream()?.collect()
            }
            pub fn opt(mut self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt()?;
                Ok(
                    self
                        .client
                        .query_opt(&stmt, &self.params)?
                        .map(|row| (self.mapper)(Self::extractor(&row))),
                )
            }
            pub fn stream(
                mut self,
            ) -> Result<
                    impl Iterator<Item = Result<T, postgres::Error>> + 'a,
                    postgres::Error,
                > {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)(Self::extractor(&row))));
                Ok(stream)
            }
        }
        pub fn select_everything<'a, C: GenericClient>(
            client: &'a mut C,
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
            pub json_: cornucopia_client::types::Json<&'a serde_json::value::RawValue>,
            pub jsonb_: cornucopia_client::types::Json<&'a serde_json::value::RawValue>,
            pub uuid_: uuid::Uuid,
            pub inet_: std::net::IpAddr,
            pub macaddr_: eui48::MacAddress,
        }
        impl<'a> InsertEverythingParams<'a> {
            pub fn query<C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> Result<u64, postgres::Error> {
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
        pub fn insert_everything<'a, C: GenericClient>(
            client: &'a mut C,
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
            json_: &'a cornucopia_client::types::Json<&serde_json::value::RawValue>,
            jsonb_: &'a cornucopia_client::types::Json<&serde_json::value::RawValue>,
            uuid_: &'a uuid::Uuid,
            inet_: &'a std::net::IpAddr,
            macaddr_: &'a eui48::MacAddress,
        ) -> Result<u64, postgres::Error> {
            let stmt = client
                .prepare(
                    "INSERT INTO Everything (custom_domain_, custom_array_, domain_, array_, bool_, boolean_, char_, smallint_, int2_, smallserial_, serial2_, int_, int4_, serial_, serial4_, bingint_, int8_, bigserial_, serial8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37);

",
                )?;
            client
                .execute(
                    &stmt,
                    &[
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
                )
        }
    }
    pub mod module_1 {
        use postgres::fallible_iterator::FallibleIterator;
        use postgres::GenericClient;
        pub struct InsertBookParams<'a> {
            pub book_name: &'a str,
        }
        impl<'a> InsertBookParams<'a> {
            pub fn query<C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> Result<u64, postgres::Error> {
                insert_book(client, &self.book_name)
            }
        }
        pub fn insert_book<'a, C: GenericClient>(
            client: &'a mut C,
            book_name: &'a &str,
        ) -> Result<u64, postgres::Error> {
            let stmt = client.prepare("INSERT INTO Book (title)
  VALUES ($1);")?;
            client.execute(&stmt, &[book_name])
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Nightmare {
            pub composite: super::super::types::public::NightmareComposite,
            pub name: String,
            pub names: Vec<String>,
            pub data: Option<Vec<u8>>,
            pub datas: Option<Vec<Vec<u8>>>,
        }
        pub struct NightmareBorrowed<'a> {
            pub composite: super::super::types::public::NightmareCompositeBorrowed<'a>,
            pub name: &'a str,
            pub names: cornucopia_client::ArrayIterator<'a, &'a str>,
            pub data: Option<&'a [u8]>,
            pub datas: Option<cornucopia_client::ArrayIterator<'a, &'a [u8]>>,
        }
        impl<'a> From<NightmareBorrowed<'a>> for Nightmare {
            fn from(
                NightmareBorrowed {
                    composite,
                    name,
                    names,
                    data,
                    datas,
                }: NightmareBorrowed<'a>,
            ) -> Self {
                Self {
                    composite: composite.into(),
                    name: name.into(),
                    names: names.map(|v| v.into()).collect(),
                    data: data.map(|v| v.into()),
                    datas: datas.map(|v| v.map(|v| v.into()).collect()),
                }
            }
        }
        pub struct NightmareQuery<'a, C: GenericClient, T> {
            client: &'a mut C,
            params: [&'a (dyn cornucopia_client::types::ToSql + Sync); 0],
            mapper: fn(NightmareBorrowed) -> T,
        }
        impl<'a, C, T: 'a> NightmareQuery<'a, C, T>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(NightmareBorrowed) -> R,
            ) -> NightmareQuery<'a, C, R> {
                NightmareQuery {
                    client: self.client,
                    params: self.params,
                    mapper,
                }
            }
            pub fn extractor(row: &postgres::row::Row) -> NightmareBorrowed {
                NightmareBorrowed {
                    composite: row.get(0),
                    name: row.get(1),
                    names: row.get(2),
                    data: row.get(3),
                    datas: row.get(4),
                }
            }
            pub fn stmt(&mut self) -> Result<postgres::Statement, postgres::Error> {
                self.client.prepare("SELECT
  *
FROM
  nightmare;")
            }
            pub fn one(mut self) -> Result<T, postgres::Error> {
                let stmt = self.stmt()?;
                let row = self.client.query_one(&stmt, &self.params)?;
                Ok((self.mapper)(Self::extractor(&row)))
            }
            pub fn vec(self) -> Result<Vec<T>, postgres::Error> {
                self.stream()?.collect()
            }
            pub fn opt(mut self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt()?;
                Ok(
                    self
                        .client
                        .query_opt(&stmt, &self.params)?
                        .map(|row| (self.mapper)(Self::extractor(&row))),
                )
            }
            pub fn stream(
                mut self,
            ) -> Result<
                    impl Iterator<Item = Result<T, postgres::Error>> + 'a,
                    postgres::Error,
                > {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)(Self::extractor(&row))));
                Ok(stream)
            }
        }
        pub fn nightmare<'a, C: GenericClient>(
            client: &'a mut C,
        ) -> NightmareQuery<'a, C, Nightmare> {
            NightmareQuery {
                client,
                params: [],
                mapper: |it| Nightmare::from(it),
            }
        }
        #[derive(Debug, Copy, Clone, PartialEq)]
        pub struct Copies {
            pub composite: super::super::types::public::CopyComposite,
            pub domain: i32,
        }
        pub struct CopiesQuery<'a, C: GenericClient, T> {
            client: &'a mut C,
            params: [&'a (dyn cornucopia_client::types::ToSql + Sync); 0],
            mapper: fn(Copies) -> T,
        }
        impl<'a, C, T: 'a> CopiesQuery<'a, C, T>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(Copies) -> R) -> CopiesQuery<'a, C, R> {
                CopiesQuery {
                    client: self.client,
                    params: self.params,
                    mapper,
                }
            }
            pub fn extractor(row: &postgres::row::Row) -> Copies {
                Copies {
                    composite: row.get(0),
                    domain: row.get(1),
                }
            }
            pub fn stmt(&mut self) -> Result<postgres::Statement, postgres::Error> {
                self.client.prepare("SELECT * FROM copy;")
            }
            pub fn one(mut self) -> Result<T, postgres::Error> {
                let stmt = self.stmt()?;
                let row = self.client.query_one(&stmt, &self.params)?;
                Ok((self.mapper)(Self::extractor(&row)))
            }
            pub fn vec(self) -> Result<Vec<T>, postgres::Error> {
                self.stream()?.collect()
            }
            pub fn opt(mut self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt()?;
                Ok(
                    self
                        .client
                        .query_opt(&stmt, &self.params)?
                        .map(|row| (self.mapper)(Self::extractor(&row))),
                )
            }
            pub fn stream(
                mut self,
            ) -> Result<
                    impl Iterator<Item = Result<T, postgres::Error>> + 'a,
                    postgres::Error,
                > {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)(Self::extractor(&row))));
                Ok(stream)
            }
        }
        pub fn copies<'a, C: GenericClient>(
            client: &'a mut C,
        ) -> CopiesQuery<'a, C, Copies> {
            CopiesQuery {
                client,
                params: [],
                mapper: |it| Copies::from(it),
            }
        }
        #[derive(Debug, Clone)]
        pub struct InsertCopyParams {
            pub composite: super::super::types::public::CopyComposite,
            pub domain: super::super::types::public::CopyDomain,
        }
        impl InsertCopyParams {
            pub fn query<'a, C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> Result<u64, postgres::Error> {
                insert_copy(client, &self.composite, &self.domain)
            }
        }
        pub fn insert_copy<'a, C: GenericClient>(
            client: &'a mut C,
            composite: &'a super::super::types::public::CopyComposite,
            domain: &'a super::super::types::public::CopyDomain,
        ) -> Result<u64, postgres::Error> {
            let stmt = client
                .prepare("INSERT INTO Copy (composite, domain)
  VALUES ($1, $2);
")?;
            client.execute(&stmt, &[composite, domain])
        }
    }
}
