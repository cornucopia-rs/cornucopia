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
            pub nice: super::public::SpongebobCharacter,
        }
        #[derive(Debug)]
        pub struct CustomCompositeBorrowed<'a> {
            pub wow: &'a str,
            pub such_cool: i32,
            pub nice: super::super::types::public::SpongebobCharacter,
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
        #[derive(Debug, Clone)]
        pub struct CustomCompositeParams<'a> {
            pub wow: &'a str,
            pub such_cool: i32,
            pub nice: super::super::types::public::SpongebobCharacter,
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
        #[postgres(name = "custom_domain")]
        pub struct CustomDomain(pub Vec<super::super::types::public::CustomComposite>);
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
        #[derive(Debug, Clone)]
        pub struct CustomDomainParams<'a>(
            pub &'a [super::super::types::public::CustomCompositeParams<'a>],
        );
        impl<'a> postgres_types::ToSql for CustomDomainParams<'a> {
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
                if type_.name() != "custom_domain" {
                    return false;
                }
                match *type_.kind() {
                    postgres_types::Kind::Domain(ref type_) => {
                        <&'a [super::super::types::public::CustomCompositeParams<
                            'a,
                        >] as postgres_types::ToSql>::accepts(type_)
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
        #[derive(Debug, Clone, PartialEq, postgres_types::ToSql, postgres_types::FromSql)]
        #[postgres(name = "my_domain")]
        pub struct MyDomain(pub String);
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
                Self(inner.into())
            }
        }
        #[derive(Debug, Clone)]
        pub struct MyDomainParams<'a>(pub &'a str);
        impl<'a> postgres_types::ToSql for MyDomainParams<'a> {
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
                Result::Ok(NightmareCompositeBorrowed { custom, spongebob })
            }
            fn accepts(type_: &postgres_types::Type) -> bool {
                type_.name() == "nightmare_composite" && type_.schema() == "public"
            }
        }
        impl<'a> From<NightmareCompositeBorrowed<'a>> for NightmareComposite {
            fn from(
                NightmareCompositeBorrowed { custom, spongebob }: NightmareCompositeBorrowed<'a>,
            ) -> Self {
                Self {
                    custom: custom.map(|v| v.into()).collect(),
                    spongebob: spongebob.map(|v| v.into()).collect(),
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct NightmareCompositeParams<'a> {
            pub custom: &'a [super::super::types::public::CustomCompositeParams<'a>],
            pub spongebob: &'a [super::super::types::public::SpongebobCharacter],
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
                        if fields.len() != 2usize {
                            return false;
                        }
                        fields
                            .iter()
                            .all(|f| match f.name() {
                                "custom" => {
                                    <&'a [super::super::types::public::CustomCompositeParams<
                                        'a,
                                    >] as postgres_types::ToSql>::accepts(f.type_())
                                }
                                "spongebob" => {
                                    <&'a [super::super::types::public::SpongebobCharacter] as postgres_types::ToSql>::accepts(
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
        impl<'a> From<CloneCompositeBorrowed<'a>> for CloneComposite {
            fn from(CloneCompositeBorrowed { first, second }: CloneCompositeBorrowed<'a>) -> Self {
                Self {
                    first,
                    second: second.into(),
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct CloneCompositeParams<'a> {
            pub first: i32,
            pub second: &'a str,
        }
        impl<'a> postgres_types::ToSql for CloneCompositeParams<'a> {
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
        use postgres::fallible_iterator::FallibleIterator;
        use postgres::GenericClient;
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
            pub json_: postgres_types::Json<serde_json::Value>,
            pub jsonb_: postgres_types::Json<serde_json::Value>,
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
            pub json_: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub jsonb_: postgres_types::Json<&'a serde_json::value::RawValue>,
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
                    json_: postgres_types::Json(serde_json::from_str(json_.0.get()).unwrap()),
                    jsonb_: postgres_types::Json(serde_json::from_str(jsonb_.0.get()).unwrap()),
                    uuid_,
                    inet_,
                    macaddr_,
                }
            }
        }
        pub struct SelectEverythingQuery<'a, C: GenericClient, T> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); 0],
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
                self.client.prepare(
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
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)?
                    .map(|row| (self.mapper)(Self::extractor(&row))))
            }
            pub fn stream(
                mut self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
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
        #[derive(Debug, Clone)]
        pub struct InsertEverythingParams<'a> {
            pub custom_domain_: super::super::types::public::CustomDomainParams<'a>,
            pub custom_array_: &'a [super::super::types::public::SpongebobCharacter],
            pub domain_: super::super::types::public::MyDomainParams<'a>,
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
            pub json_: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub jsonb_: postgres_types::Json<&'a serde_json::value::RawValue>,
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
            custom_domain_: &'a super::super::types::public::CustomDomainParams<'a>,
            custom_array_: &'a &'a [super::super::types::public::SpongebobCharacter],
            domain_: &'a super::super::types::public::MyDomainParams<'a>,
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
            json_: &'a postgres_types::Json<&serde_json::value::RawValue>,
            jsonb_: &'a postgres_types::Json<&serde_json::value::RawValue>,
            uuid_: &'a uuid::Uuid,
            inet_: &'a std::net::IpAddr,
            macaddr_: &'a eui48::MacAddress,
        ) -> Result<u64, postgres::Error> {
            let stmt = client
                .prepare(
                    "INSERT INTO Everything (custom_domain_, custom_array_, domain_, array_, bool_, boolean_, char_, smallint_, int2_, smallserial_, serial2_, int_, int4_, serial_, serial4_, bingint_, int8_, bigserial_, serial8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37);",
                )?;
            client.execute(
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
            params: [&'a (dyn postgres_types::ToSql + Sync); 0],
            mapper: fn(NightmareBorrowed) -> T,
        }
        impl<'a, C, T: 'a> NightmareQuery<'a, C, T>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(NightmareBorrowed) -> R) -> NightmareQuery<'a, C, R> {
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
                self.client.prepare(
                    "SELECT * FROM nightmare;
",
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
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)?
                    .map(|row| (self.mapper)(Self::extractor(&row))))
            }
            pub fn stream(
                mut self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
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
    }
    pub mod copy {
        use postgres::fallible_iterator::FallibleIterator;
        use postgres::GenericClient;
        #[derive(Debug, Clone)]
        pub struct InsertCloneParams<'a> {
            pub composite: super::super::types::public::CloneCompositeParams<'a>,
        }
        impl<'a> InsertCloneParams<'a> {
            pub fn query<C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> Result<u64, postgres::Error> {
                insert_clone(client, &self.composite)
            }
        }
        pub fn insert_clone<'a, C: GenericClient>(
            client: &'a mut C,
            composite: &'a super::super::types::public::CloneCompositeParams<'a>,
        ) -> Result<u64, postgres::Error> {
            let stmt = client.prepare("INSERT INTO clone (composite) VALUES ($1);")?;
            client.execute(&stmt, &[composite])
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
        pub struct SelectCloneQuery<'a, C: GenericClient, T> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); 0],
            mapper: fn(SelectCloneBorrowed) -> T,
        }
        impl<'a, C, T: 'a> SelectCloneQuery<'a, C, T>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(SelectCloneBorrowed) -> R,
            ) -> SelectCloneQuery<'a, C, R> {
                SelectCloneQuery {
                    client: self.client,
                    params: self.params,
                    mapper,
                }
            }
            pub fn extractor(row: &postgres::row::Row) -> SelectCloneBorrowed {
                SelectCloneBorrowed {
                    composite: row.get(0),
                }
            }
            pub fn stmt(&mut self) -> Result<postgres::Statement, postgres::Error> {
                self.client.prepare("SELECT * FROM clone;")
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
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)?
                    .map(|row| (self.mapper)(Self::extractor(&row))))
            }
            pub fn stream(
                mut self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)(Self::extractor(&row))));
                Ok(stream)
            }
        }
        pub fn select_clone<'a, C: GenericClient>(
            client: &'a mut C,
        ) -> SelectCloneQuery<'a, C, SelectClone> {
            SelectCloneQuery {
                client,
                params: [],
                mapper: |it| SelectClone::from(it),
            }
        }
        #[derive(Debug, Copy, Clone)]
        pub struct InsertCopyParams {
            pub composite: super::super::types::public::CopyComposite,
        }
        impl InsertCopyParams {
            pub fn query<'a, C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> Result<u64, postgres::Error> {
                insert_copy(client, &self.composite)
            }
        }
        pub fn insert_copy<'a, C: GenericClient>(
            client: &'a mut C,
            composite: &'a super::super::types::public::CopyComposite,
        ) -> Result<u64, postgres::Error> {
            let stmt = client.prepare("INSERT INTO copy (composite) VALUES ($1);")?;
            client.execute(&stmt, &[composite])
        }
        #[derive(Debug, Clone, PartialEq, Copy)]
        pub struct SelectCopy {
            pub composite: super::super::types::public::CopyComposite,
        }
        pub struct SelectCopyQuery<'a, C: GenericClient, T> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); 0],
            mapper: fn(SelectCopy) -> T,
        }
        impl<'a, C, T: 'a> SelectCopyQuery<'a, C, T>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(SelectCopy) -> R) -> SelectCopyQuery<'a, C, R> {
                SelectCopyQuery {
                    client: self.client,
                    params: self.params,
                    mapper,
                }
            }
            pub fn extractor(row: &postgres::row::Row) -> SelectCopy {
                SelectCopy {
                    composite: row.get(0),
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
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)?
                    .map(|row| (self.mapper)(Self::extractor(&row))))
            }
            pub fn stream(
                mut self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)(Self::extractor(&row))));
                Ok(stream)
            }
        }
        pub fn select_copy<'a, C: GenericClient>(
            client: &'a mut C,
        ) -> SelectCopyQuery<'a, C, SelectCopy> {
            SelectCopyQuery {
                client,
                params: [],
                mapper: |it| SelectCopy::from(it),
            }
        }
    }
}
