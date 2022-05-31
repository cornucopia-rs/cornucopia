#![allow(clippy::all)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
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
        #[derive(Debug, postgres_types::ToSql, postgres_types::FromSql, Clone, PartialEq)]
        #[postgres(name = "custom_composite")]
        pub struct CustomComposite {
            pub wow: String,
            pub such_cool: i32,
            pub nice: super::super::types::public::SpongebobCharacter,
        }
        #[derive(Debug)]
        pub struct CustomCompositeBorrowed<'a> {
            pub wow: &'a str,
            pub such_cool: i32,
            pub nice: super::super::types::public::SpongebobCharacter,
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
                    such_cool: such_cool,
                    nice: nice,
                }
            }
        }
        impl<'a> postgres_types::FromSql<'a> for CustomCompositeBorrowed<'a> {
            fn from_sql(
                _type: &postgres_types::Type,
                buf: &'a [u8],
            ) -> Result<
                CustomCompositeBorrowed<'a>,
                std::boxed::Box<dyn std::error::Error + Sync + Send>,
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
                Result::Ok(CustomCompositeBorrowed {
                    wow,
                    such_cool,
                    nice,
                })
            }
            fn accepts(type_: &postgres_types::Type) -> bool {
                type_.name() == "custom_composite" && type_.schema() == "public"
            }
        }
        impl<'a> postgres_types::ToSql for CustomCompositeBorrowed<'a> {
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
                        "wow" => postgres_types::ToSql::to_sql(&self.wow, field.type_(), buf),
                        "such_cool" => {
                            postgres_types::ToSql::to_sql(&self.such_cool, field.type_(), buf)
                        }
                        "nice" => postgres_types::ToSql::to_sql(&self.nice, field.type_(), buf),
                        _ => unreachable!(),
                    };
                    let count = match r? {
                        postgres_types::IsNull::Yes => -1,
                        postgres_types::IsNull::No => {
                            let len = buf.len() - base - 4;
                            if len > i32::max_value() as usize {
                                return std::result::Result::Err(std::convert::Into::into(
                                    "value too large to transmit",
                                ));
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
                                "wow" => {
                                    <&'a str as postgres_types::ToSql>::accepts(f.type_())
                                }
                                "such_cool" => {
                                    <i32 as postgres_types::ToSql>::accepts(f.type_())
                                }
                                "nice" => {
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
            ) -> std::result::Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
        #[derive(Debug, Clone, PartialEq, postgres_types::ToSql, postgres_types::FromSql)]
        #[postgres(name = "my_domain")]
        pub struct MyDomain(pub String);
        #[derive(Debug)]
        pub struct MyDomainBorrowed<'a>(pub &'a str);
        impl<'a> From<MyDomainBorrowed<'a>> for MyDomain {
            fn from(MyDomainBorrowed(inner): MyDomainBorrowed<'a>) -> Self {
                Self(inner.into())
            }
        }
        impl<'a> postgres_types::FromSql<'a> for MyDomainBorrowed<'a> {
            fn from_sql(
                _type: &postgres_types::Type,
                buf: &'a [u8],
            ) -> std::result::Result<
                MyDomainBorrowed<'a>,
                std::boxed::Box<dyn std::error::Error + Sync + Send>,
            > {
                <&'a str as postgres_types::FromSql>::from_sql(_type, buf).map(MyDomainBorrowed)
            }
            fn accepts(type_: &postgres_types::Type) -> bool {
                if <&'a str as postgres_types::FromSql>::accepts(type_) {
                    return true;
                }
                if type_.name() != "my_domain" || type_.schema() != "public" {
                    return false;
                }
                match *type_.kind() {
                    postgres_types::Kind::Domain(ref type_) => {
                        <&'a str as postgres_types::ToSql>::accepts(type_)
                    }
                    _ => false,
                }
            }
        }
        impl<'a> postgres_types::ToSql for MyDomainBorrowed<'a> {
            fn to_sql(
                &self,
                _type: &postgres_types::Type,
                buf: &mut postgres_types::private::BytesMut,
            ) -> std::result::Result<
                postgres_types::IsNull,
                std::boxed::Box<dyn std::error::Error + Sync + Send>,
            > {
                let type_ = match *_type.kind() {
                    postgres_types::Kind::Domain(ref type_) => type_,
                    _ => unreachable!(),
                };
                postgres_types::ToSql::to_sql(&self.0, type_, buf)
            }
            fn accepts(type_: &postgres_types::Type) -> bool {
                if type_.name() != "my_domain" {
                    return false;
                }
                match *type_.kind() {
                    postgres_types::Kind::Domain(ref type_) => {
                        <&'a str as postgres_types::ToSql>::accepts(type_)
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
                Box<dyn std::error::Error + std::marker::Sync + std::marker::Send>,
            > {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
        #[derive(Debug, postgres_types::ToSql, postgres_types::FromSql, Clone, PartialEq)]
        #[postgres(name = "nightmare_composite")]
        pub struct NightmareComposite {
            pub custom: Vec<super::super::types::public::CustomComposite>,
            pub spongebob: Vec<super::super::types::public::SpongebobCharacter>,
            pub domain: super::super::types::public::MyDomain,
        }
        #[derive(Debug)]
        pub struct NightmareCompositeBorrowed<'a> {
            pub custom: cornucopia_client::ArrayIterator<
                'a,
                super::super::types::public::CustomCompositeBorrowed<'a>,
            >,
            pub spongebob: cornucopia_client::ArrayIterator<
                'a,
                super::super::types::public::SpongebobCharacter,
            >,
            pub domain: super::super::types::public::MyDomainBorrowed<'a>,
        }
        impl<'a> From<NightmareCompositeBorrowed<'a>> for NightmareComposite {
            fn from(
                NightmareCompositeBorrowed {
                    custom,
                    spongebob,
                    domain,
                }: NightmareCompositeBorrowed<'a>,
            ) -> Self {
                Self {
                    custom: custom.map(|v| v.into()).collect(),
                    spongebob: spongebob.map(|v| v.into()).collect(),
                    domain: domain.into(),
                }
            }
        }
        impl<'a> postgres_types::FromSql<'a> for NightmareCompositeBorrowed<'a> {
            fn from_sql(
                _type: &postgres_types::Type,
                buf: &'a [u8],
            ) -> Result<
                NightmareCompositeBorrowed<'a>,
                std::boxed::Box<dyn std::error::Error + Sync + Send>,
            > {
                let fields = match *_type.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut buf = buf;
                let num_fields = postgres_types::private::read_be_i32(&mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let custom = postgres_types::private::read_value(fields[0].type_(), &mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let spongebob = postgres_types::private::read_value(fields[1].type_(), &mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let domain = postgres_types::private::read_value(fields[2].type_(), &mut buf)?;
                Result::Ok(NightmareCompositeBorrowed {
                    custom,
                    spongebob,
                    domain,
                })
            }
            fn accepts(type_: &postgres_types::Type) -> bool {
                type_.name() == "nightmare_composite" && type_.schema() == "public"
            }
        }
        #[derive(Debug)]
        pub struct NightmareCompositeParams<'a> {
            pub custom: &'a [super::super::types::public::CustomCompositeBorrowed<'a>],
            pub spongebob: &'a [super::super::types::public::SpongebobCharacter],
            pub domain: super::super::types::public::MyDomainBorrowed<'a>,
        }
        impl<'a> postgres_types::ToSql for NightmareCompositeParams<'a> {
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
                        "custom" => postgres_types::ToSql::to_sql(&self.custom, field.type_(), buf),
                        "spongebob" => {
                            postgres_types::ToSql::to_sql(&self.spongebob, field.type_(), buf)
                        }
                        "domain" => postgres_types::ToSql::to_sql(&self.domain, field.type_(), buf),
                        _ => unreachable!(),
                    };
                    let count = match r? {
                        postgres_types::IsNull::Yes => -1,
                        postgres_types::IsNull::No => {
                            let len = buf.len() - base - 4;
                            if len > i32::max_value() as usize {
                                return std::result::Result::Err(std::convert::Into::into(
                                    "value too large to transmit",
                                ));
                            }
                            len as i32
                        }
                    };
                    buf[base..base + 4].copy_from_slice(&count.to_be_bytes());
                }
                std::result::Result::Ok(postgres_types::IsNull::No)
            }
            fn accepts(type_: &postgres_types::Type) -> bool {
                if type_.name() != "nightmare_composite" {
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
                                "custom" => {
                                    <&'a [super::super::types::public::CustomCompositeBorrowed<
                                        'a,
                                    >] as postgres_types::ToSql>::accepts(f.type_())
                                }
                                "spongebob" => {
                                    <&'a [super::super::types::public::SpongebobCharacter] as postgres_types::ToSql>::accepts(
                                        f.type_(),
                                    )
                                }
                                "domain" => {
                                    <super::super::types::public::MyDomainBorrowed<
                                        'a,
                                    > as postgres_types::ToSql>::accepts(f.type_())
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
            ) -> std::result::Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
        #[derive(Debug, postgres_types::ToSql, postgres_types::FromSql, Clone, PartialEq)]
        #[postgres(name = "clone_composite")]
        pub struct CloneComposite {
            pub first: i32,
            pub second: String,
        }
        #[derive(Debug)]
        pub struct CloneCompositeBorrowed<'a> {
            pub first: i32,
            pub second: &'a str,
        }
        impl<'a> From<CloneCompositeBorrowed<'a>> for CloneComposite {
            fn from(CloneCompositeBorrowed { first, second }: CloneCompositeBorrowed<'a>) -> Self {
                Self {
                    first: first,
                    second: second.into(),
                }
            }
        }
        impl<'a> postgres_types::FromSql<'a> for CloneCompositeBorrowed<'a> {
            fn from_sql(
                _type: &postgres_types::Type,
                buf: &'a [u8],
            ) -> Result<
                CloneCompositeBorrowed<'a>,
                std::boxed::Box<dyn std::error::Error + Sync + Send>,
            > {
                let fields = match *_type.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut buf = buf;
                let num_fields = postgres_types::private::read_be_i32(&mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let first = postgres_types::private::read_value(fields[0].type_(), &mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let second = postgres_types::private::read_value(fields[1].type_(), &mut buf)?;
                Result::Ok(CloneCompositeBorrowed { first, second })
            }
            fn accepts(type_: &postgres_types::Type) -> bool {
                type_.name() == "clone_composite" && type_.schema() == "public"
            }
        }
        impl<'a> postgres_types::ToSql for CloneCompositeBorrowed<'a> {
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
                        "first" => postgres_types::ToSql::to_sql(&self.first, field.type_(), buf),
                        "second" => postgres_types::ToSql::to_sql(&self.second, field.type_(), buf),
                        _ => unreachable!(),
                    };
                    let count = match r? {
                        postgres_types::IsNull::Yes => -1,
                        postgres_types::IsNull::No => {
                            let len = buf.len() - base - 4;
                            if len > i32::max_value() as usize {
                                return std::result::Result::Err(std::convert::Into::into(
                                    "value too large to transmit",
                                ));
                            }
                            len as i32
                        }
                    };
                    buf[base..base + 4].copy_from_slice(&count.to_be_bytes());
                }
                std::result::Result::Ok(postgres_types::IsNull::No)
            }
            fn accepts(type_: &postgres_types::Type) -> bool {
                if type_.name() != "clone_composite" {
                    return false;
                }
                match *type_.kind() {
                    postgres_types::Kind::Composite(ref fields) => {
                        if fields.len() != 2usize {
                            return false;
                        }
                        fields.iter().all(|f| match f.name() {
                            "first" => <i32 as postgres_types::ToSql>::accepts(f.type_()),
                            "second" => <&'a str as postgres_types::ToSql>::accepts(f.type_()),
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
            ) -> std::result::Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
        #[derive(Debug, postgres_types::ToSql, postgres_types::FromSql, Copy, Clone, PartialEq)]
        #[postgres(name = "copy_composite")]
        pub struct CopyComposite {
            pub first: i32,
            pub second: f64,
        }
    }
}
pub mod queries {
    pub mod stress {
        use cornucopia_client::GenericClient;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug)]
        pub struct InsertEverythingParams<'a> {
            pub bigserial_: i64,
            pub bingint_: i64,
            pub bool_: bool,
            pub boolean_: bool,
            pub bytea_: &'a [u8],
            pub char_: i8,
            pub date_: time::Date,
            pub double_precision_: f64,
            pub float4_: f32,
            pub float8_: f64,
            pub inet_: std::net::IpAddr,
            pub int2_: i16,
            pub int4_: i32,
            pub int8_: i64,
            pub int_: i32,
            pub json_: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub jsonb_: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub macaddr_: eui48::MacAddress,
            pub real_: f32,
            pub serial2_: i16,
            pub serial4_: i32,
            pub serial8_: i64,
            pub serial_: i32,
            pub smallint_: i16,
            pub smallserial_: i16,
            pub text_: &'a str,
            pub time_: time::Time,
            pub timestamp_: time::PrimitiveDateTime,
            pub timestamp_with_time_zone_: time::OffsetDateTime,
            pub timestamp_without_time_zone_: time::PrimitiveDateTime,
            pub timestamptz_: time::OffsetDateTime,
            pub uuid_: uuid::Uuid,
            pub varchar_: &'a str,
        }
        impl<'a> InsertEverythingParams<'a> {
            pub async fn insert_everything<C: GenericClient>(
                &'a self,
                client: &'a C,
            ) -> Result<u64, tokio_postgres::Error> {
                insert_everything(
                    client,
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
                .await
            }
        }
        #[derive(Debug)]
        pub struct InsertEverythingArrayParams<'a> {
            pub bingint_: &'a [i64],
            pub bool_: &'a [bool],
            pub boolean_: &'a [bool],
            pub bytea_: &'a [&'a [u8]],
            pub char_: &'a [i8],
            pub date_: &'a [time::Date],
            pub double_precision_: &'a [f64],
            pub float4_: &'a [f32],
            pub float8_: &'a [f64],
            pub inet_: &'a [std::net::IpAddr],
            pub int2_: &'a [i16],
            pub int4_: &'a [i32],
            pub int8_: &'a [i64],
            pub int_: &'a [i32],
            pub json_: &'a [postgres_types::Json<&'a serde_json::value::RawValue>],
            pub jsonb_: &'a [postgres_types::Json<&'a serde_json::value::RawValue>],
            pub macaddr_: &'a [eui48::MacAddress],
            pub real_: &'a [f32],
            pub smallint_: &'a [i16],
            pub text_: &'a [&'a str],
            pub time_: &'a [time::Time],
            pub timestamp_: &'a [time::PrimitiveDateTime],
            pub timestamp_with_time_zone_: &'a [time::OffsetDateTime],
            pub timestamp_without_time_zone_: &'a [time::PrimitiveDateTime],
            pub timestamptz_: &'a [time::OffsetDateTime],
            pub uuid_: &'a [uuid::Uuid],
            pub varchar_: &'a [&'a str],
        }
        impl<'a> InsertEverythingArrayParams<'a> {
            pub async fn insert_everything_array<C: GenericClient>(
                &'a self,
                client: &'a C,
            ) -> Result<u64, tokio_postgres::Error> {
                insert_everything_array(
                    client,
                    &self.bool_,
                    &self.boolean_,
                    &self.char_,
                    &self.smallint_,
                    &self.int2_,
                    &self.int_,
                    &self.int4_,
                    &self.bingint_,
                    &self.int8_,
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
                .await
            }
        }
        #[derive(Debug)]
        pub struct InsertNightmareParams<'a> {
            pub composite: super::super::types::public::NightmareCompositeParams<'a>,
        }
        impl<'a> InsertNightmareParams<'a> {
            pub async fn insert_nightmare<C: GenericClient>(
                &'a self,
                client: &'a C,
            ) -> Result<u64, tokio_postgres::Error> {
                insert_nightmare(client, &self.composite).await
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SelectEverything {
            pub bigserial_: i64,
            pub bingint_: i64,
            pub bool_: bool,
            pub boolean_: bool,
            pub bytea_: Vec<u8>,
            pub char_: i8,
            pub date_: time::Date,
            pub double_precision_: f64,
            pub float4_: f32,
            pub float8_: f64,
            pub inet_: std::net::IpAddr,
            pub int2_: i16,
            pub int4_: i32,
            pub int8_: i64,
            pub int_: i32,
            pub json_: postgres_types::Json<serde_json::Value>,
            pub jsonb_: postgres_types::Json<serde_json::Value>,
            pub macaddr_: eui48::MacAddress,
            pub real_: f32,
            pub serial2_: i16,
            pub serial4_: i32,
            pub serial8_: i64,
            pub serial_: i32,
            pub smallint_: i16,
            pub smallserial_: i16,
            pub text_: String,
            pub time_: time::Time,
            pub timestamp_: time::PrimitiveDateTime,
            pub timestamp_with_time_zone_: time::OffsetDateTime,
            pub timestamp_without_time_zone_: time::PrimitiveDateTime,
            pub timestamptz_: time::OffsetDateTime,
            pub uuid_: uuid::Uuid,
            pub varchar_: String,
        }
        pub struct SelectEverythingBorrowed<'a> {
            pub bigserial_: i64,
            pub bingint_: i64,
            pub bool_: bool,
            pub boolean_: bool,
            pub bytea_: &'a [u8],
            pub char_: i8,
            pub date_: time::Date,
            pub double_precision_: f64,
            pub float4_: f32,
            pub float8_: f64,
            pub inet_: std::net::IpAddr,
            pub int2_: i16,
            pub int4_: i32,
            pub int8_: i64,
            pub int_: i32,
            pub json_: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub jsonb_: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub macaddr_: eui48::MacAddress,
            pub real_: f32,
            pub serial2_: i16,
            pub serial4_: i32,
            pub serial8_: i64,
            pub serial_: i32,
            pub smallint_: i16,
            pub smallserial_: i16,
            pub text_: &'a str,
            pub time_: time::Time,
            pub timestamp_: time::PrimitiveDateTime,
            pub timestamp_with_time_zone_: time::OffsetDateTime,
            pub timestamp_without_time_zone_: time::PrimitiveDateTime,
            pub timestamptz_: time::OffsetDateTime,
            pub uuid_: uuid::Uuid,
            pub varchar_: &'a str,
        }
        impl<'a> From<SelectEverythingBorrowed<'a>> for SelectEverything {
            fn from(
                SelectEverythingBorrowed {
                    bigserial_,
                    bingint_,
                    bool_,
                    boolean_,
                    bytea_,
                    char_,
                    date_,
                    double_precision_,
                    float4_,
                    float8_,
                    inet_,
                    int2_,
                    int4_,
                    int8_,
                    int_,
                    json_,
                    jsonb_,
                    macaddr_,
                    real_,
                    serial2_,
                    serial4_,
                    serial8_,
                    serial_,
                    smallint_,
                    smallserial_,
                    text_,
                    time_,
                    timestamp_,
                    timestamp_with_time_zone_,
                    timestamp_without_time_zone_,
                    timestamptz_,
                    uuid_,
                    varchar_,
                }: SelectEverythingBorrowed<'a>,
            ) -> Self {
                Self {
                    bigserial_,
                    bingint_,
                    bool_,
                    boolean_,
                    bytea_: bytea_.into(),
                    char_,
                    date_,
                    double_precision_,
                    float4_,
                    float8_,
                    inet_,
                    int2_,
                    int4_,
                    int8_,
                    int_,
                    json_: postgres_types::Json(serde_json::from_str(json_.0.get()).unwrap()),
                    jsonb_: postgres_types::Json(serde_json::from_str(jsonb_.0.get()).unwrap()),
                    macaddr_,
                    real_,
                    serial2_,
                    serial4_,
                    serial8_,
                    serial_,
                    smallint_,
                    smallserial_,
                    text_: text_.into(),
                    time_,
                    timestamp_,
                    timestamp_with_time_zone_,
                    timestamp_without_time_zone_,
                    timestamptz_,
                    uuid_,
                    varchar_: varchar_.into(),
                }
            }
        }
        pub struct SelectEverythingQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            extractor: fn(&tokio_postgres::Row) -> SelectEverythingBorrowed,
            mapper: fn(SelectEverythingBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SelectEverythingQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(SelectEverythingBorrowed) -> R,
            ) -> SelectEverythingQuery<'a, C, R, N> {
                SelectEverythingQuery {
                    client: self.client,
                    params: self.params,
                    query: self.query,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client.prepare(self.query).await
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok((self.mapper)((self.extractor)(&row)))
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
                    .map(|row| (self.mapper)((self.extractor)(&row))))
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
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(stream)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SelectEverythingArray {
            pub bingint_: Vec<i64>,
            pub bool_: Vec<bool>,
            pub boolean_: Vec<bool>,
            pub bytea_: Vec<Vec<u8>>,
            pub char_: Vec<i8>,
            pub date_: Vec<time::Date>,
            pub double_precision_: Vec<f64>,
            pub float4_: Vec<f32>,
            pub float8_: Vec<f64>,
            pub inet_: Vec<std::net::IpAddr>,
            pub int2_: Vec<i16>,
            pub int4_: Vec<i32>,
            pub int8_: Vec<i64>,
            pub int_: Vec<i32>,
            pub json_: Vec<postgres_types::Json<serde_json::Value>>,
            pub jsonb_: Vec<postgres_types::Json<serde_json::Value>>,
            pub macaddr_: Vec<eui48::MacAddress>,
            pub real_: Vec<f32>,
            pub smallint_: Vec<i16>,
            pub text_: Vec<String>,
            pub time_: Vec<time::Time>,
            pub timestamp_: Vec<time::PrimitiveDateTime>,
            pub timestamp_with_time_zone_: Vec<time::OffsetDateTime>,
            pub timestamp_without_time_zone_: Vec<time::PrimitiveDateTime>,
            pub timestamptz_: Vec<time::OffsetDateTime>,
            pub uuid_: Vec<uuid::Uuid>,
            pub varchar_: Vec<String>,
        }
        pub struct SelectEverythingArrayBorrowed<'a> {
            pub bingint_: cornucopia_client::ArrayIterator<'a, i64>,
            pub bool_: cornucopia_client::ArrayIterator<'a, bool>,
            pub boolean_: cornucopia_client::ArrayIterator<'a, bool>,
            pub bytea_: cornucopia_client::ArrayIterator<'a, &'a [u8]>,
            pub char_: cornucopia_client::ArrayIterator<'a, i8>,
            pub date_: cornucopia_client::ArrayIterator<'a, time::Date>,
            pub double_precision_: cornucopia_client::ArrayIterator<'a, f64>,
            pub float4_: cornucopia_client::ArrayIterator<'a, f32>,
            pub float8_: cornucopia_client::ArrayIterator<'a, f64>,
            pub inet_: cornucopia_client::ArrayIterator<'a, std::net::IpAddr>,
            pub int2_: cornucopia_client::ArrayIterator<'a, i16>,
            pub int4_: cornucopia_client::ArrayIterator<'a, i32>,
            pub int8_: cornucopia_client::ArrayIterator<'a, i64>,
            pub int_: cornucopia_client::ArrayIterator<'a, i32>,
            pub json_: cornucopia_client::ArrayIterator<
                'a,
                postgres_types::Json<&'a serde_json::value::RawValue>,
            >,
            pub jsonb_: cornucopia_client::ArrayIterator<
                'a,
                postgres_types::Json<&'a serde_json::value::RawValue>,
            >,
            pub macaddr_: cornucopia_client::ArrayIterator<'a, eui48::MacAddress>,
            pub real_: cornucopia_client::ArrayIterator<'a, f32>,
            pub smallint_: cornucopia_client::ArrayIterator<'a, i16>,
            pub text_: cornucopia_client::ArrayIterator<'a, &'a str>,
            pub time_: cornucopia_client::ArrayIterator<'a, time::Time>,
            pub timestamp_: cornucopia_client::ArrayIterator<'a, time::PrimitiveDateTime>,
            pub timestamp_with_time_zone_:
                cornucopia_client::ArrayIterator<'a, time::OffsetDateTime>,
            pub timestamp_without_time_zone_:
                cornucopia_client::ArrayIterator<'a, time::PrimitiveDateTime>,
            pub timestamptz_: cornucopia_client::ArrayIterator<'a, time::OffsetDateTime>,
            pub uuid_: cornucopia_client::ArrayIterator<'a, uuid::Uuid>,
            pub varchar_: cornucopia_client::ArrayIterator<'a, &'a str>,
        }
        impl<'a> From<SelectEverythingArrayBorrowed<'a>> for SelectEverythingArray {
            fn from(
                SelectEverythingArrayBorrowed {
                    bingint_,
                    bool_,
                    boolean_,
                    bytea_,
                    char_,
                    date_,
                    double_precision_,
                    float4_,
                    float8_,
                    inet_,
                    int2_,
                    int4_,
                    int8_,
                    int_,
                    json_,
                    jsonb_,
                    macaddr_,
                    real_,
                    smallint_,
                    text_,
                    time_,
                    timestamp_,
                    timestamp_with_time_zone_,
                    timestamp_without_time_zone_,
                    timestamptz_,
                    uuid_,
                    varchar_,
                }: SelectEverythingArrayBorrowed<'a>,
            ) -> Self {
                Self {
                    bingint_: bingint_.map(|v| v.into()).collect(),
                    bool_: bool_.map(|v| v.into()).collect(),
                    boolean_: boolean_.map(|v| v.into()).collect(),
                    bytea_: bytea_.map(|v| v.into()).collect(),
                    char_: char_.map(|v| v.into()).collect(),
                    date_: date_.map(|v| v.into()).collect(),
                    double_precision_: double_precision_.map(|v| v.into()).collect(),
                    float4_: float4_.map(|v| v.into()).collect(),
                    float8_: float8_.map(|v| v.into()).collect(),
                    inet_: inet_.map(|v| v.into()).collect(),
                    int2_: int2_.map(|v| v.into()).collect(),
                    int4_: int4_.map(|v| v.into()).collect(),
                    int8_: int8_.map(|v| v.into()).collect(),
                    int_: int_.map(|v| v.into()).collect(),
                    json_: json_
                        .map(|v| postgres_types::Json(serde_json::from_str(v.0.get()).unwrap()))
                        .collect(),
                    jsonb_: jsonb_
                        .map(|v| postgres_types::Json(serde_json::from_str(v.0.get()).unwrap()))
                        .collect(),
                    macaddr_: macaddr_.map(|v| v.into()).collect(),
                    real_: real_.map(|v| v.into()).collect(),
                    smallint_: smallint_.map(|v| v.into()).collect(),
                    text_: text_.map(|v| v.into()).collect(),
                    time_: time_.map(|v| v.into()).collect(),
                    timestamp_: timestamp_.map(|v| v.into()).collect(),
                    timestamp_with_time_zone_: timestamp_with_time_zone_
                        .map(|v| v.into())
                        .collect(),
                    timestamp_without_time_zone_: timestamp_without_time_zone_
                        .map(|v| v.into())
                        .collect(),
                    timestamptz_: timestamptz_.map(|v| v.into()).collect(),
                    uuid_: uuid_.map(|v| v.into()).collect(),
                    varchar_: varchar_.map(|v| v.into()).collect(),
                }
            }
        }
        pub struct SelectEverythingArrayQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            extractor: fn(&tokio_postgres::Row) -> SelectEverythingArrayBorrowed,
            mapper: fn(SelectEverythingArrayBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SelectEverythingArrayQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(SelectEverythingArrayBorrowed) -> R,
            ) -> SelectEverythingArrayQuery<'a, C, R, N> {
                SelectEverythingArrayQuery {
                    client: self.client,
                    params: self.params,
                    query: self.query,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client.prepare(self.query).await
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok((self.mapper)((self.extractor)(&row)))
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
                    .map(|row| (self.mapper)((self.extractor)(&row))))
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
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(stream)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SelectNightmare {
            pub composite: super::super::types::public::NightmareComposite,
        }
        pub struct SelectNightmareBorrowed<'a> {
            pub composite: super::super::types::public::NightmareCompositeBorrowed<'a>,
        }
        impl<'a> From<SelectNightmareBorrowed<'a>> for SelectNightmare {
            fn from(SelectNightmareBorrowed { composite }: SelectNightmareBorrowed<'a>) -> Self {
                Self {
                    composite: composite.into(),
                }
            }
        }
        pub struct SelectNightmareQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            extractor: fn(&tokio_postgres::Row) -> SelectNightmareBorrowed,
            mapper: fn(SelectNightmareBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SelectNightmareQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(SelectNightmareBorrowed) -> R,
            ) -> SelectNightmareQuery<'a, C, R, N> {
                SelectNightmareQuery {
                    client: self.client,
                    params: self.params,
                    query: self.query,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client.prepare(self.query).await
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok((self.mapper)((self.extractor)(&row)))
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
                    .map(|row| (self.mapper)((self.extractor)(&row))))
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
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(stream)
            }
        }
        pub fn select_everything<'a, C: GenericClient>(
            client: &'a C,
        ) -> SelectEverythingQuery<'a, C, SelectEverything, 0> {
            SelectEverythingQuery {
                client,
                params: [],
                query: "SELECT * FROM Everything;
",
                extractor: |row| SelectEverythingBorrowed {
                    bigserial_: row.get(13),
                    bingint_: row.get(11),
                    bool_: row.get(0),
                    boolean_: row.get(1),
                    bytea_: row.get(21),
                    char_: row.get(2),
                    date_: row.get(26),
                    double_precision_: row.get(18),
                    float4_: row.get(15),
                    float8_: row.get(17),
                    inet_: row.get(31),
                    int2_: row.get(4),
                    int4_: row.get(8),
                    int8_: row.get(12),
                    int_: row.get(7),
                    json_: row.get(28),
                    jsonb_: row.get(29),
                    macaddr_: row.get(32),
                    real_: row.get(16),
                    serial2_: row.get(6),
                    serial4_: row.get(10),
                    serial8_: row.get(14),
                    serial_: row.get(9),
                    smallint_: row.get(3),
                    smallserial_: row.get(5),
                    text_: row.get(19),
                    time_: row.get(27),
                    timestamp_: row.get(22),
                    timestamp_with_time_zone_: row.get(25),
                    timestamp_without_time_zone_: row.get(23),
                    timestamptz_: row.get(24),
                    uuid_: row.get(30),
                    varchar_: row.get(20),
                },
                mapper: |it| SelectEverything::from(it),
            }
        }
        pub async fn insert_everything<'a, C: GenericClient>(
            client: &'a C,
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
            text_: &'a &'a str,
            varchar_: &'a &'a str,
            bytea_: &'a &'a [u8],
            timestamp_: &'a time::PrimitiveDateTime,
            timestamp_without_time_zone_: &'a time::PrimitiveDateTime,
            timestamptz_: &'a time::OffsetDateTime,
            timestamp_with_time_zone_: &'a time::OffsetDateTime,
            date_: &'a time::Date,
            time_: &'a time::Time,
            json_: &'a postgres_types::Json<&'a serde_json::value::RawValue>,
            jsonb_: &'a postgres_types::Json<&'a serde_json::value::RawValue>,
            uuid_: &'a uuid::Uuid,
            inet_: &'a std::net::IpAddr,
            macaddr_: &'a eui48::MacAddress,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = client
                .prepare(
                    "INSERT INTO Everything (bool_, boolean_, char_, smallint_, int2_, smallserial_, serial2_, int_, int4_, serial_, serial4_, bingint_, int8_, bigserial_, serial8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33);
",
                )
                .await?;
            client
                .execute(
                    &stmt,
                    &[
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
                .await
        }
        pub fn select_everything_array<'a, C: GenericClient>(
            client: &'a C,
        ) -> SelectEverythingArrayQuery<'a, C, SelectEverythingArray, 0> {
            SelectEverythingArrayQuery {
                client,
                params: [],
                query: "SELECT * FROM EverythingArray
",
                extractor: |row| SelectEverythingArrayBorrowed {
                    bingint_: row.get(7),
                    bool_: row.get(0),
                    boolean_: row.get(1),
                    bytea_: row.get(15),
                    char_: row.get(2),
                    date_: row.get(20),
                    double_precision_: row.get(12),
                    float4_: row.get(9),
                    float8_: row.get(11),
                    inet_: row.get(25),
                    int2_: row.get(4),
                    int4_: row.get(6),
                    int8_: row.get(8),
                    int_: row.get(5),
                    json_: row.get(22),
                    jsonb_: row.get(23),
                    macaddr_: row.get(26),
                    real_: row.get(10),
                    smallint_: row.get(3),
                    text_: row.get(13),
                    time_: row.get(21),
                    timestamp_: row.get(16),
                    timestamp_with_time_zone_: row.get(19),
                    timestamp_without_time_zone_: row.get(17),
                    timestamptz_: row.get(18),
                    uuid_: row.get(24),
                    varchar_: row.get(14),
                },
                mapper: |it| SelectEverythingArray::from(it),
            }
        }
        pub async fn insert_everything_array<'a, C: GenericClient>(
            client: &'a C,
            bool_: &'a &'a [bool],
            boolean_: &'a &'a [bool],
            char_: &'a &'a [i8],
            smallint_: &'a &'a [i16],
            int2_: &'a &'a [i16],
            int_: &'a &'a [i32],
            int4_: &'a &'a [i32],
            bingint_: &'a &'a [i64],
            int8_: &'a &'a [i64],
            float4_: &'a &'a [f32],
            real_: &'a &'a [f32],
            float8_: &'a &'a [f64],
            double_precision_: &'a &'a [f64],
            text_: &'a &'a [&'a str],
            varchar_: &'a &'a [&'a str],
            bytea_: &'a &'a [&'a [u8]],
            timestamp_: &'a &'a [time::PrimitiveDateTime],
            timestamp_without_time_zone_: &'a &'a [time::PrimitiveDateTime],
            timestamptz_: &'a &'a [time::OffsetDateTime],
            timestamp_with_time_zone_: &'a &'a [time::OffsetDateTime],
            date_: &'a &'a [time::Date],
            time_: &'a &'a [time::Time],
            json_: &'a &'a [postgres_types::Json<&'a serde_json::value::RawValue>],
            jsonb_: &'a &'a [postgres_types::Json<&'a serde_json::value::RawValue>],
            uuid_: &'a &'a [uuid::Uuid],
            inet_: &'a &'a [std::net::IpAddr],
            macaddr_: &'a &'a [eui48::MacAddress],
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = client
                .prepare(
                    "INSERT INTO EverythingArray (bool_, boolean_, char_, smallint_, int2_, int_, int4_, bingint_, int8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27);
",
                )
                .await?;
            client
                .execute(
                    &stmt,
                    &[
                        bool_,
                        boolean_,
                        char_,
                        smallint_,
                        int2_,
                        int_,
                        int4_,
                        bingint_,
                        int8_,
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
                .await
        }
        pub fn select_nightmare<'a, C: GenericClient>(
            client: &'a C,
        ) -> SelectNightmareQuery<'a, C, SelectNightmare, 0> {
            SelectNightmareQuery {
                client,
                params: [],
                query: "SELECT * FROM nightmare;
",
                extractor: |row| SelectNightmareBorrowed {
                    composite: row.get(0),
                },
                mapper: |it| SelectNightmare::from(it),
            }
        }
        pub async fn insert_nightmare<'a, C: GenericClient>(
            client: &'a C,
            composite: &'a super::super::types::public::NightmareCompositeParams<'a>,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = client
                .prepare("INSERT INTO nightmare (composite) VALUES ($1);")
                .await?;
            client.execute(&stmt, &[composite]).await
        }
    }
    pub mod copy {
        use cornucopia_client::GenericClient;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug)]
        pub struct InsertCloneParams<'a> {
            pub composite: super::super::types::public::CloneCompositeBorrowed<'a>,
        }
        impl<'a> InsertCloneParams<'a> {
            pub async fn insert_clone<C: GenericClient>(
                &'a self,
                client: &'a C,
            ) -> Result<u64, tokio_postgres::Error> {
                insert_clone(client, &self.composite).await
            }
        }
        #[derive(Debug, Clone, Copy)]
        pub struct InsertCopyParams {
            pub composite: super::super::types::public::CopyComposite,
        }
        impl InsertCopyParams {
            pub async fn insert_copy<'a, C: GenericClient>(
                &'a self,
                client: &'a C,
            ) -> Result<u64, tokio_postgres::Error> {
                insert_copy(client, &self.composite).await
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SelectClone {
            pub composite: super::super::types::public::CloneComposite,
        }
        pub struct SelectCloneBorrowed<'a> {
            pub composite: super::super::types::public::CloneCompositeBorrowed<'a>,
        }
        impl<'a> From<SelectCloneBorrowed<'a>> for SelectClone {
            fn from(SelectCloneBorrowed { composite }: SelectCloneBorrowed<'a>) -> Self {
                Self {
                    composite: composite.into(),
                }
            }
        }
        pub struct SelectCloneQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            extractor: fn(&tokio_postgres::Row) -> SelectCloneBorrowed,
            mapper: fn(SelectCloneBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SelectCloneQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(SelectCloneBorrowed) -> R,
            ) -> SelectCloneQuery<'a, C, R, N> {
                SelectCloneQuery {
                    client: self.client,
                    params: self.params,
                    query: self.query,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client.prepare(self.query).await
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok((self.mapper)((self.extractor)(&row)))
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
                    .map(|row| (self.mapper)((self.extractor)(&row))))
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
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(stream)
            }
        }
        #[derive(Debug, Clone, PartialEq, Copy)]
        pub struct SelectCopy {
            pub composite: super::super::types::public::CopyComposite,
        }
        pub struct SelectCopyQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            extractor: fn(&tokio_postgres::Row) -> SelectCopy,
            mapper: fn(SelectCopy) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SelectCopyQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(SelectCopy) -> R) -> SelectCopyQuery<'a, C, R, N> {
                SelectCopyQuery {
                    client: self.client,
                    params: self.params,
                    query: self.query,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client.prepare(self.query).await
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok((self.mapper)((self.extractor)(&row)))
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
                    .map(|row| (self.mapper)((self.extractor)(&row))))
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
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(stream)
            }
        }
        pub async fn insert_clone<'a, C: GenericClient>(
            client: &'a C,
            composite: &'a super::super::types::public::CloneCompositeBorrowed<'a>,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = client
                .prepare(
                    "INSERT INTO clone (composite) VALUES ($1);
",
                )
                .await?;
            client.execute(&stmt, &[composite]).await
        }
        pub fn select_clone<'a, C: GenericClient>(
            client: &'a C,
        ) -> SelectCloneQuery<'a, C, SelectClone, 0> {
            SelectCloneQuery {
                client,
                params: [],
                query: "SELECT * FROM clone;
",
                extractor: |row| SelectCloneBorrowed {
                    composite: row.get(0),
                },
                mapper: |it| SelectClone::from(it),
            }
        }
        pub async fn insert_copy<'a, C: GenericClient>(
            client: &'a C,
            composite: &'a super::super::types::public::CopyComposite,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = client
                .prepare(
                    "INSERT INTO copy (composite) VALUES ($1);
",
                )
                .await?;
            client.execute(&stmt, &[composite]).await
        }
        pub fn select_copy<'a, C: GenericClient>(
            client: &'a C,
        ) -> SelectCopyQuery<'a, C, SelectCopy, 0> {
            SelectCopyQuery {
                client,
                params: [],
                query: "SELECT * FROM copy;",
                extractor: |row| SelectCopy {
                    composite: row.get(0),
                },
                mapper: |it| SelectCopy::from(it),
            }
        }
    }
    pub mod params {
        use cornucopia_client::GenericClient;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug)]
        pub struct InsertBookParams<'a> {
            pub author: &'a str,
            pub name: &'a str,
        }
        impl<'a> InsertBookParams<'a> {
            pub async fn insert_book<C: GenericClient>(
                &'a self,
                client: &'a C,
            ) -> Result<u64, tokio_postgres::Error> {
                insert_book(client, &self.author, &self.name).await
            }
        }
        #[derive(Debug)]
        pub struct ParamsUseTwiceParams<'a> {
            pub name: &'a str,
        }
        impl<'a> ParamsUseTwiceParams<'a> {
            pub async fn params_use_twice<C: GenericClient>(
                &'a self,
                client: &'a C,
            ) -> Result<u64, tokio_postgres::Error> {
                params_use_twice(client, &self.name).await
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SelectBook {
            pub author: String,
            pub name: String,
        }
        pub struct SelectBookBorrowed<'a> {
            pub author: &'a str,
            pub name: &'a str,
        }
        impl<'a> From<SelectBookBorrowed<'a>> for SelectBook {
            fn from(SelectBookBorrowed { author, name }: SelectBookBorrowed<'a>) -> Self {
                Self {
                    author: author.into(),
                    name: name.into(),
                }
            }
        }
        pub struct SelectBookQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            extractor: fn(&tokio_postgres::Row) -> SelectBookBorrowed,
            mapper: fn(SelectBookBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SelectBookQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(SelectBookBorrowed) -> R,
            ) -> SelectBookQuery<'a, C, R, N> {
                SelectBookQuery {
                    client: self.client,
                    params: self.params,
                    query: self.query,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client.prepare(self.query).await
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok((self.mapper)((self.extractor)(&row)))
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
                    .map(|row| (self.mapper)((self.extractor)(&row))))
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
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(stream)
            }
        }
        pub async fn insert_book<'a, C: GenericClient>(
            client: &'a C,
            author: &'a &'a str,
            name: &'a &'a str,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = client
                .prepare(
                    "INSERT INTO book (author, name) VALUES ($1, $2);
",
                )
                .await?;
            client.execute(&stmt, &[author, name]).await
        }
        pub fn select_book<'a, C: GenericClient>(
            client: &'a C,
        ) -> SelectBookQuery<'a, C, SelectBook, 0> {
            SelectBookQuery {
                client,
                params: [],
                query: "SELECT * FROM book;
",
                extractor: |row| SelectBookBorrowed {
                    author: row.get(1),
                    name: row.get(0),
                },
                mapper: |it| SelectBook::from(it),
            }
        }
        pub async fn params_use_twice<'a, C: GenericClient>(
            client: &'a C,
            name: &'a &'a str,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = client
                .prepare("UPDATE book SET name = $1 WHERE length(name) > 42 AND length($1) < 42;")
                .await?;
            client.execute(&stmt, &[name]).await
        }
    }
}
