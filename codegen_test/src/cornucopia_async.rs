// This file was generated with `cornucopia`. Do not modify.
#![allow(clippy::all, clippy::pedantic)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
pub mod types {
    pub mod public {
        #[derive(serde::Serialize, Debug, postgres_types::FromSql, Clone, PartialEq)]
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
                    first,
                    second: second.into(),
                }
            }
        }
        impl<'a> postgres_types::FromSql<'a> for CloneCompositeBorrowed<'a> {
            fn from_sql(
                ty: &postgres_types::Type,
                out: &'a [u8],
            ) -> Result<CloneCompositeBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>>
            {
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut out = out;
                let num_fields = postgres_types::private::read_be_i32(&mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let first = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let second = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
                Ok(CloneCompositeBorrowed { first, second })
            }

            fn accepts(ty: &postgres_types::Type) -> bool {
                ty.name() == "clone_composite" && ty.schema() == "public"
            }
        }
        impl<'a> postgres_types::ToSql for CloneCompositeBorrowed<'a> {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                let CloneCompositeBorrowed { first, second } = self;
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
                        "first" => postgres_types::ToSql::to_sql(first, field.type_(), out),
                        "second" => postgres_types::ToSql::to_sql(second, field.type_(), out),
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
                if ty.name() != "clone_composite" {
                    return false;
                }
                match *ty.kind() {
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
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
        #[derive(serde::Serialize, Debug, postgres_types::FromSql, Copy, Clone, PartialEq)]
        #[postgres(name = "copy_composite")]
        pub struct CopyComposite {
            pub first: i32,
            pub second: f64,
        }
        impl<'a> postgres_types::ToSql for CopyComposite {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                let CopyComposite { first, second } = self;
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
                        "first" => postgres_types::ToSql::to_sql(first, field.type_(), out),
                        "second" => postgres_types::ToSql::to_sql(second, field.type_(), out),
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
                if ty.name() != "copy_composite" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => {
                        if fields.len() != 2usize {
                            return false;
                        }
                        fields.iter().all(|f| match f.name() {
                            "first" => <i32 as postgres_types::ToSql>::accepts(f.type_()),
                            "second" => <f64 as postgres_types::ToSql>::accepts(f.type_()),
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
        #[derive(serde::Serialize, Debug, postgres_types::FromSql, Clone, PartialEq)]
        #[postgres(name = "domain_composite")]
        pub struct DomainComposite {
            pub txt: String,
            pub json: serde_json::Value,
            pub nb: i32,
            pub arr: Vec<serde_json::Value>,
        }
        #[derive(Debug)]
        pub struct DomainCompositeBorrowed<'a> {
            pub txt: &'a str,
            pub json: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub nb: i32,
            pub arr: cornucopia_client::ArrayIterator<
                'a,
                postgres_types::Json<&'a serde_json::value::RawValue>,
            >,
        }
        impl<'a> From<DomainCompositeBorrowed<'a>> for DomainComposite {
            fn from(
                DomainCompositeBorrowed { txt, json, nb, arr }: DomainCompositeBorrowed<'a>,
            ) -> Self {
                Self {
                    txt: txt.into(),
                    json: serde_json::from_str(json.0.get()).unwrap(),
                    nb,
                    arr: arr
                        .map(|v| serde_json::from_str(v.0.get()).unwrap())
                        .collect(),
                }
            }
        }
        impl<'a> postgres_types::FromSql<'a> for DomainCompositeBorrowed<'a> {
            fn from_sql(
                ty: &postgres_types::Type,
                out: &'a [u8],
            ) -> Result<DomainCompositeBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>>
            {
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut out = out;
                let num_fields = postgres_types::private::read_be_i32(&mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let txt = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let json = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let nb = postgres_types::private::read_value(fields[2].type_(), &mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let arr = postgres_types::private::read_value(fields[3].type_(), &mut out)?;
                Ok(DomainCompositeBorrowed { txt, json, nb, arr })
            }

            fn accepts(ty: &postgres_types::Type) -> bool {
                ty.name() == "domain_composite" && ty.schema() == "public"
            }
        }
        #[derive(Debug)]
        pub struct DomainCompositeParams<'a> {
            pub txt: &'a str,
            pub json: &'a serde_json::value::Value,
            pub nb: i32,
            pub arr: &'a [&'a serde_json::value::Value],
        }
        impl<'a> postgres_types::ToSql for DomainCompositeParams<'a> {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                let DomainCompositeParams { txt, json, nb, arr } = self;
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
                        "txt" => postgres_types::ToSql::to_sql(
                            &cornucopia_client::private::Domain(txt),
                            field.type_(),
                            out,
                        ),
                        "json" => postgres_types::ToSql::to_sql(
                            &cornucopia_client::private::Domain(json),
                            field.type_(),
                            out,
                        ),
                        "nb" => postgres_types::ToSql::to_sql(
                            &cornucopia_client::private::Domain(nb),
                            field.type_(),
                            out,
                        ),
                        "arr" => postgres_types::ToSql::to_sql(
                            &cornucopia_client::private::Domain(
                                &cornucopia_client::private::DomainArray(arr),
                            ),
                            field.type_(),
                            out,
                        ),
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
                if ty.name() != "domain_composite" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => {
                        if fields.len() != 4usize {
                            return false;
                        }
                        fields.iter().all(|f| match f.name() {
                            "txt" => <cornucopia_client::private::Domain::<&'a str> as postgres_types::ToSql>::accepts(f.type_()),
"json" => <cornucopia_client::private::Domain::<&'a serde_json::value::Value> as postgres_types::ToSql>::accepts(f.type_()),
"nb" => <cornucopia_client::private::Domain::<i32> as postgres_types::ToSql>::accepts(f.type_()),
"arr" => <cornucopia_client::private::Domain::<cornucopia_client::private::DomainArray::<&'a serde_json::value::Value>> as postgres_types::ToSql>::accepts(f.type_()),
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
        #[derive(serde::Serialize, Debug, postgres_types::FromSql, Clone, PartialEq)]
        #[postgres(name = "named_composite")]
        pub struct NamedComposite {
            pub wow: Option<String>,
            pub such_cool: Option<i32>,
        }
        #[derive(Debug)]
        pub struct NamedCompositeBorrowed<'a> {
            pub wow: Option<&'a str>,
            pub such_cool: Option<i32>,
        }
        impl<'a> From<NamedCompositeBorrowed<'a>> for NamedComposite {
            fn from(NamedCompositeBorrowed { wow, such_cool }: NamedCompositeBorrowed<'a>) -> Self {
                Self {
                    wow: wow.map(|v| v.into()),
                    such_cool,
                }
            }
        }
        impl<'a> postgres_types::FromSql<'a> for NamedCompositeBorrowed<'a> {
            fn from_sql(
                ty: &postgres_types::Type,
                out: &'a [u8],
            ) -> Result<NamedCompositeBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>>
            {
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut out = out;
                let num_fields = postgres_types::private::read_be_i32(&mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let wow = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let such_cool = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
                Ok(NamedCompositeBorrowed { wow, such_cool })
            }

            fn accepts(ty: &postgres_types::Type) -> bool {
                ty.name() == "named_composite" && ty.schema() == "public"
            }
        }
        impl<'a> postgres_types::ToSql for NamedCompositeBorrowed<'a> {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                let NamedCompositeBorrowed { wow, such_cool } = self;
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
                        "wow" => postgres_types::ToSql::to_sql(wow, field.type_(), out),
                        "such_cool" => postgres_types::ToSql::to_sql(such_cool, field.type_(), out),
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
                if ty.name() != "named_composite" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => {
                        if fields.len() != 2usize {
                            return false;
                        }
                        fields.iter().all(|f| match f.name() {
                            "wow" => <&'a str as postgres_types::ToSql>::accepts(f.type_()),
                            "such_cool" => <i32 as postgres_types::ToSql>::accepts(f.type_()),
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
        #[derive(serde::Serialize, Debug, postgres_types::FromSql, Clone, PartialEq)]
        #[postgres(name = "nullity_composite")]
        pub struct NullityComposite {
            pub jsons: Option<Vec<Option<serde_json::Value>>>,
            pub id: i32,
        }
        #[derive(Debug)]
        pub struct NullityCompositeBorrowed<'a> {
            pub jsons: Option<
                cornucopia_client::ArrayIterator<
                    'a,
                    Option<postgres_types::Json<&'a serde_json::value::RawValue>>,
                >,
            >,
            pub id: i32,
        }
        impl<'a> From<NullityCompositeBorrowed<'a>> for NullityComposite {
            fn from(NullityCompositeBorrowed { jsons, id }: NullityCompositeBorrowed<'a>) -> Self {
                Self {
                    jsons: jsons.map(|v| {
                        v.map(|v| v.map(|v| serde_json::from_str(v.0.get()).unwrap()))
                            .collect()
                    }),
                    id,
                }
            }
        }
        impl<'a> postgres_types::FromSql<'a> for NullityCompositeBorrowed<'a> {
            fn from_sql(
                ty: &postgres_types::Type,
                out: &'a [u8],
            ) -> Result<NullityCompositeBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>>
            {
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut out = out;
                let num_fields = postgres_types::private::read_be_i32(&mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let jsons = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let id = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
                Ok(NullityCompositeBorrowed { jsons, id })
            }

            fn accepts(ty: &postgres_types::Type) -> bool {
                ty.name() == "nullity_composite" && ty.schema() == "public"
            }
        }
        #[derive(Debug)]
        pub struct NullityCompositeParams<'a> {
            pub jsons: Option<&'a [Option<&'a serde_json::value::Value>]>,
            pub id: i32,
        }
        impl<'a> postgres_types::ToSql for NullityCompositeParams<'a> {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                let NullityCompositeParams { jsons, id } = self;
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
                        "jsons" => postgres_types::ToSql::to_sql(jsons, field.type_(), out),
                        "id" => postgres_types::ToSql::to_sql(id, field.type_(), out),
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
                if ty.name() != "nullity_composite" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => {
                        if fields.len() != 2usize {
                            return false;
                        }
                        fields.iter().all(|f| match f.name() {
                            "jsons" => <&'a [&'a serde_json::value::Value] as postgres_types::ToSql>::accepts(f.type_()),
"id" => <i32 as postgres_types::ToSql>::accepts(f.type_()),
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
        #[derive(
            serde::Serialize,
            Debug,
            postgres_types::ToSql,
            postgres_types::FromSql,
            Clone,
            Copy,
            PartialEq,
            Eq,
        )]
        #[postgres(name = "spongebob_character")]
        pub enum SpongebobCharacter {
            Bob,
            Patrick,
            Squidward,
        }
        #[derive(serde::Serialize, Debug, postgres_types::FromSql, Clone, PartialEq)]
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
                    such_cool,
                    nice,
                }
            }
        }
        impl<'a> postgres_types::FromSql<'a> for CustomCompositeBorrowed<'a> {
            fn from_sql(
                ty: &postgres_types::Type,
                out: &'a [u8],
            ) -> Result<CustomCompositeBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>>
            {
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut out = out;
                let num_fields = postgres_types::private::read_be_i32(&mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let wow = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let such_cool = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let nice = postgres_types::private::read_value(fields[2].type_(), &mut out)?;
                Ok(CustomCompositeBorrowed {
                    wow,
                    such_cool,
                    nice,
                })
            }

            fn accepts(ty: &postgres_types::Type) -> bool {
                ty.name() == "custom_composite" && ty.schema() == "public"
            }
        }
        impl<'a> postgres_types::ToSql for CustomCompositeBorrowed<'a> {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                let CustomCompositeBorrowed {
                    wow,
                    such_cool,
                    nice,
                } = self;
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
                        "wow" => postgres_types::ToSql::to_sql(wow, field.type_(), out),
                        "such_cool" => postgres_types::ToSql::to_sql(such_cool, field.type_(), out),
                        "nice" => postgres_types::ToSql::to_sql(nice, field.type_(), out),
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
                if ty.name() != "custom_composite" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => {
                        if fields.len() != 3usize {
                            return false;
                        }
                        fields.iter().all(|f| match f.name() {
                            "wow" => <&'a str as postgres_types::ToSql>::accepts(f.type_()),
"such_cool" => <i32 as postgres_types::ToSql>::accepts(f.type_()),
"nice" => <super::super::types::public::SpongebobCharacter as postgres_types::ToSql>::accepts(f.type_()),
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
        #[derive(serde::Serialize, Debug, postgres_types::FromSql, Clone, PartialEq)]
        #[postgres(name = "nightmare_composite")]
        pub struct NightmareComposite {
            pub custom: Vec<super::super::types::public::CustomComposite>,
            pub spongebob: Vec<super::super::types::public::SpongebobCharacter>,
            pub domain: String,
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
            pub domain: &'a str,
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
                    spongebob: spongebob.map(|v| v).collect(),
                    domain: domain.into(),
                }
            }
        }
        impl<'a> postgres_types::FromSql<'a> for NightmareCompositeBorrowed<'a> {
            fn from_sql(
                ty: &postgres_types::Type,
                out: &'a [u8],
            ) -> Result<NightmareCompositeBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>>
            {
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut out = out;
                let num_fields = postgres_types::private::read_be_i32(&mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let custom = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let spongebob = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let domain = postgres_types::private::read_value(fields[2].type_(), &mut out)?;
                Ok(NightmareCompositeBorrowed {
                    custom,
                    spongebob,
                    domain,
                })
            }

            fn accepts(ty: &postgres_types::Type) -> bool {
                ty.name() == "nightmare_composite" && ty.schema() == "public"
            }
        }
        #[derive(Debug)]
        pub struct NightmareCompositeParams<'a> {
            pub custom: &'a [super::super::types::public::CustomCompositeBorrowed<'a>],
            pub spongebob: &'a [super::super::types::public::SpongebobCharacter],
            pub domain: &'a str,
        }
        impl<'a> postgres_types::ToSql for NightmareCompositeParams<'a> {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                let NightmareCompositeParams {
                    custom,
                    spongebob,
                    domain,
                } = self;
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
                        "custom" => postgres_types::ToSql::to_sql(custom, field.type_(), out),
                        "spongebob" => postgres_types::ToSql::to_sql(spongebob, field.type_(), out),
                        "domain" => postgres_types::ToSql::to_sql(
                            &cornucopia_client::private::Domain(domain),
                            field.type_(),
                            out,
                        ),
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
                if ty.name() != "nightmare_composite" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => {
                        if fields.len() != 3usize {
                            return false;
                        }
                        fields.iter().all(|f| match f.name() {
                            "custom" => <&'a [super::super::types::public::CustomCompositeBorrowed<'a>] as postgres_types::ToSql>::accepts(f.type_()),
"spongebob" => <&'a [super::super::types::public::SpongebobCharacter] as postgres_types::ToSql>::accepts(f.type_()),
"domain" => <cornucopia_client::private::Domain::<&'a str> as postgres_types::ToSql>::accepts(f.type_()),
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
pub mod queries {
    pub mod copy {
        use cornucopia_client::async_::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};

        pub struct SuperSuperTypesPublicCloneCompositeQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::async_::Stmt,
            extractor:
                fn(&tokio_postgres::Row) -> super::super::types::public::CloneCompositeBorrowed,
            mapper: fn(super::super::types::public::CloneCompositeBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SuperSuperTypesPublicCloneCompositeQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(super::super::types::public::CloneCompositeBorrowed) -> R,
            ) -> SuperSuperTypesPublicCloneCompositeQuery<'a, C, R, N> {
                SuperSuperTypesPublicCloneCompositeQuery {
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
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }

        pub struct SuperSuperTypesPublicCopyCompositeQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::async_::Stmt,
            extractor: fn(&tokio_postgres::Row) -> super::super::types::public::CopyComposite,
            mapper: fn(super::super::types::public::CopyComposite) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SuperSuperTypesPublicCopyCompositeQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(super::super::types::public::CopyComposite) -> R,
            ) -> SuperSuperTypesPublicCopyCompositeQuery<'a, C, R, N> {
                SuperSuperTypesPublicCopyCompositeQuery {
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
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn insert_clone() -> InsertCloneStmt {
            InsertCloneStmt(cornucopia_client::async_::Stmt::new(
                "INSERT INTO clone (composite) VALUES ($1)",
            ))
        }
        pub struct InsertCloneStmt(cornucopia_client::async_::Stmt);
        impl InsertCloneStmt {
            pub async fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                composite: &'a super::super::types::public::CloneCompositeBorrowed<'a>,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[composite]).await
            }
        }
        pub fn select_clone() -> SelectCloneStmt {
            SelectCloneStmt(cornucopia_client::async_::Stmt::new("SELECT * FROM clone"))
        }
        pub struct SelectCloneStmt(cornucopia_client::async_::Stmt);
        impl SelectCloneStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> SuperSuperTypesPublicCloneCompositeQuery<
                'a,
                C,
                super::super::types::public::CloneComposite,
                0,
            > {
                SuperSuperTypesPublicCloneCompositeQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it.into(),
                }
            }
        }
        pub fn insert_copy() -> InsertCopyStmt {
            InsertCopyStmt(cornucopia_client::async_::Stmt::new(
                "INSERT INTO copy (composite) VALUES ($1)",
            ))
        }
        pub struct InsertCopyStmt(cornucopia_client::async_::Stmt);
        impl InsertCopyStmt {
            pub async fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                composite: &'a super::super::types::public::CopyComposite,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[composite]).await
            }
        }
        pub fn select_copy() -> SelectCopyStmt {
            SelectCopyStmt(cornucopia_client::async_::Stmt::new("SELECT * FROM copy"))
        }
        pub struct SelectCopyStmt(cornucopia_client::async_::Stmt);
        impl SelectCopyStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> SuperSuperTypesPublicCopyCompositeQuery<
                'a,
                C,
                super::super::types::public::CopyComposite,
                0,
            > {
                SuperSuperTypesPublicCopyCompositeQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it,
                }
            }
        }
    }
    pub mod domain {
        use cornucopia_client::async_::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug)]
        pub struct InsertNightmareDomainParams<'a> {
            pub txt: &'a str,
            pub json: &'a serde_json::value::Value,
            pub nb: i32,
            pub arr: &'a [&'a serde_json::value::Value],
            pub composite: Option<super::super::types::public::DomainCompositeParams<'a>>,
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct SelectNightmareDomain {
            pub txt: String,
            pub json: serde_json::Value,
            pub nb: i32,
            pub arr: Vec<serde_json::Value>,
        }
        pub struct SelectNightmareDomainBorrowed<'a> {
            pub txt: &'a str,
            pub json: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub nb: i32,
            pub arr: cornucopia_client::ArrayIterator<
                'a,
                postgres_types::Json<&'a serde_json::value::RawValue>,
            >,
        }
        impl<'a> From<SelectNightmareDomainBorrowed<'a>> for SelectNightmareDomain {
            fn from(
                SelectNightmareDomainBorrowed { txt, json, nb, arr }: SelectNightmareDomainBorrowed<
                    'a,
                >,
            ) -> Self {
                Self {
                    txt: txt.into(),
                    json: serde_json::from_str(json.0.get()).unwrap(),
                    nb,
                    arr: arr
                        .map(|v| serde_json::from_str(v.0.get()).unwrap())
                        .collect(),
                }
            }
        }
        pub struct SelectNightmareDomainQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::async_::Stmt,
            extractor: fn(&tokio_postgres::Row) -> SelectNightmareDomainBorrowed,
            mapper: fn(SelectNightmareDomainBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SelectNightmareDomainQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(SelectNightmareDomainBorrowed) -> R,
            ) -> SelectNightmareDomainQuery<'a, C, R, N> {
                SelectNightmareDomainQuery {
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
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct SelectNightmareDomainNull {
            pub txt: Option<String>,
            pub json: Option<serde_json::Value>,
            pub nb: Option<i32>,
            pub arr: Option<Vec<Option<serde_json::Value>>>,
            pub composite: Option<super::super::types::public::DomainComposite>,
        }
        pub struct SelectNightmareDomainNullBorrowed<'a> {
            pub txt: Option<&'a str>,
            pub json: Option<postgres_types::Json<&'a serde_json::value::RawValue>>,
            pub nb: Option<i32>,
            pub arr: Option<
                cornucopia_client::ArrayIterator<
                    'a,
                    Option<postgres_types::Json<&'a serde_json::value::RawValue>>,
                >,
            >,
            pub composite: Option<super::super::types::public::DomainCompositeBorrowed<'a>>,
        }
        impl<'a> From<SelectNightmareDomainNullBorrowed<'a>> for SelectNightmareDomainNull {
            fn from(
                SelectNightmareDomainNullBorrowed {
                    txt,
                    json,
                    nb,
                    arr,
                    composite,
                }: SelectNightmareDomainNullBorrowed<'a>,
            ) -> Self {
                Self {
                    txt: txt.map(|v| v.into()),
                    json: json.map(|v| serde_json::from_str(v.0.get()).unwrap()),
                    nb,
                    arr: arr.map(|v| {
                        v.map(|v| v.map(|v| serde_json::from_str(v.0.get()).unwrap()))
                            .collect()
                    }),
                    composite: composite.map(|v| v.into()),
                }
            }
        }
        pub struct SelectNightmareDomainNullQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::async_::Stmt,
            extractor: fn(&tokio_postgres::Row) -> SelectNightmareDomainNullBorrowed,
            mapper: fn(SelectNightmareDomainNullBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SelectNightmareDomainNullQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(SelectNightmareDomainNullBorrowed) -> R,
            ) -> SelectNightmareDomainNullQuery<'a, C, R, N> {
                SelectNightmareDomainNullQuery {
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
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn select_nightmare_domain() -> SelectNightmareDomainStmt {
            SelectNightmareDomainStmt(cornucopia_client::async_::Stmt::new(
                "SELECT txt, json, nb, arr FROM nightmare_domain",
            ))
        }
        pub struct SelectNightmareDomainStmt(cornucopia_client::async_::Stmt);
        impl SelectNightmareDomainStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> SelectNightmareDomainQuery<'a, C, SelectNightmareDomain, 0> {
                SelectNightmareDomainQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| SelectNightmareDomainBorrowed {
                        txt: row.get(0),
                        json: row.get(1),
                        nb: row.get(2),
                        arr: row.get(3),
                    },
                    mapper: |it| <SelectNightmareDomain>::from(it),
                }
            }
        }
        pub fn insert_nightmare_domain() -> InsertNightmareDomainStmt {
            InsertNightmareDomainStmt(cornucopia_client::async_::Stmt::new("INSERT INTO nightmare_domain (txt, json, nb, arr, composite) VALUES ($1, $2, $3, $4, $5)"))
        }
        pub struct InsertNightmareDomainStmt(cornucopia_client::async_::Stmt);
        impl InsertNightmareDomainStmt {
            pub async fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                txt: &'a &'a str,
                json: &'a &'a serde_json::value::Value,
                nb: &'a i32,
                arr: &'a &'a [&'a serde_json::value::Value],
                composite: &'a Option<super::super::types::public::DomainCompositeParams<'a>>,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client
                    .execute(
                        stmt,
                        &[
                            &cornucopia_client::private::Domain(txt),
                            &cornucopia_client::private::Domain(json),
                            &cornucopia_client::private::Domain(nb),
                            &cornucopia_client::private::Domain(
                                &cornucopia_client::private::DomainArray(arr),
                            ),
                            composite,
                        ],
                    )
                    .await
            }
            pub async fn params<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                params: &'a impl cornucopia_client::async_::Params<
                    'a,
                    Self,
                    std::pin::Pin<
                        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>>,
                    >,
                    C,
                >,
            ) -> Result<u64, tokio_postgres::Error> {
                params.bind(client, self).await
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_client::async_::Params<
                'a,
                InsertNightmareDomainStmt,
                std::pin::Pin<
                    Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + 'a>,
                >,
                C,
            > for InsertNightmareDomainParams<'a>
        {
            fn bind(
                &'a self,
                client: &'a C,
                stmt: &'a mut InsertNightmareDomainStmt,
            ) -> std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + 'a>,
            > {
                Box::pin(stmt.bind(
                    client,
                    &self.txt,
                    &self.json,
                    &self.nb,
                    &self.arr,
                    &self.composite,
                ))
            }
        }
        pub fn select_nightmare_domain_null() -> SelectNightmareDomainNullStmt {
            SelectNightmareDomainNullStmt(cornucopia_client::async_::Stmt::new(
                "SELECT * FROM nightmare_domain",
            ))
        }
        pub struct SelectNightmareDomainNullStmt(cornucopia_client::async_::Stmt);
        impl SelectNightmareDomainNullStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> SelectNightmareDomainNullQuery<'a, C, SelectNightmareDomainNull, 0> {
                SelectNightmareDomainNullQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| SelectNightmareDomainNullBorrowed {
                        txt: row.get(0),
                        json: row.get(1),
                        nb: row.get(2),
                        arr: row.get(3),
                        composite: row.get(4),
                    },
                    mapper: |it| <SelectNightmareDomainNull>::from(it),
                }
            }
        }
    }
    pub mod named {
        use cornucopia_client::async_::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug)]
        pub struct NamedParams<'a> {
            pub name: &'a str,
            pub price: Option<f64>,
        }

        #[derive(Debug)]
        pub struct NamedComplexParams<'a> {
            pub named: super::super::types::public::NamedCompositeBorrowed<'a>,
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq, Copy)]
        pub struct Id {
            pub id: i32,
        }
        pub struct IdQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::async_::Stmt,
            extractor: fn(&tokio_postgres::Row) -> Id,
            mapper: fn(Id) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> IdQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(Id) -> R) -> IdQuery<'a, C, R, N> {
                IdQuery {
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
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct Named {
            pub id: i32,
            pub name: String,
            pub price: Option<f64>,
            pub show: bool,
        }
        pub struct NamedBorrowed<'a> {
            pub id: i32,
            pub name: &'a str,
            pub price: Option<f64>,
            pub show: bool,
        }
        impl<'a> From<NamedBorrowed<'a>> for Named {
            fn from(
                NamedBorrowed {
                    id,
                    name,
                    price,
                    show,
                }: NamedBorrowed<'a>,
            ) -> Self {
                Self {
                    id,
                    name: name.into(),
                    price,
                    show,
                }
            }
        }
        pub struct NamedQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::async_::Stmt,
            extractor: fn(&tokio_postgres::Row) -> NamedBorrowed,
            mapper: fn(NamedBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> NamedQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(NamedBorrowed) -> R) -> NamedQuery<'a, C, R, N> {
                NamedQuery {
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
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }

        pub struct SuperSuperTypesPublicNamedCompositeQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::async_::Stmt,
            extractor:
                fn(&tokio_postgres::Row) -> super::super::types::public::NamedCompositeBorrowed,
            mapper: fn(super::super::types::public::NamedCompositeBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SuperSuperTypesPublicNamedCompositeQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(super::super::types::public::NamedCompositeBorrowed) -> R,
            ) -> SuperSuperTypesPublicNamedCompositeQuery<'a, C, R, N> {
                SuperSuperTypesPublicNamedCompositeQuery {
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
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn new_named_visible() -> NewNamedVisibleStmt {
            NewNamedVisibleStmt(cornucopia_client::async_::Stmt::new(
                "INSERT INTO named (name, price, show) VALUES ($1, $2, true) RETURNING id ",
            ))
        }
        pub struct NewNamedVisibleStmt(cornucopia_client::async_::Stmt);
        impl NewNamedVisibleStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                name: &'a &'a str,
                price: &'a Option<f64>,
            ) -> IdQuery<'a, C, Id, 2> {
                IdQuery {
                    client,
                    params: [name, price],
                    stmt: &mut self.0,
                    extractor: |row| Id { id: row.get(0) },
                    mapper: |it| <Id>::from(it),
                }
            }
            pub fn params<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                params: &'a impl cornucopia_client::async_::Params<'a, Self, IdQuery<'a, C, Id, 2>, C>,
            ) -> IdQuery<'a, C, Id, 2> {
                params.bind(client, self)
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_client::async_::Params<'a, NewNamedVisibleStmt, IdQuery<'a, C, Id, 2>, C>
            for NamedParams<'a>
        {
            fn bind(
                &'a self,
                client: &'a C,
                stmt: &'a mut NewNamedVisibleStmt,
            ) -> IdQuery<'a, C, Id, 2> {
                stmt.bind(client, &self.name, &self.price)
            }
        }
        pub fn new_named_hidden() -> NewNamedHiddenStmt {
            NewNamedHiddenStmt(cornucopia_client::async_::Stmt::new(
                "INSERT INTO named (price, name, show) VALUES ($1, $2, false) RETURNING id",
            ))
        }
        pub struct NewNamedHiddenStmt(cornucopia_client::async_::Stmt);
        impl NewNamedHiddenStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                price: &'a Option<f64>,
                name: &'a &'a str,
            ) -> IdQuery<'a, C, Id, 2> {
                IdQuery {
                    client,
                    params: [price, name],
                    stmt: &mut self.0,
                    extractor: |row| Id { id: row.get(0) },
                    mapper: |it| <Id>::from(it),
                }
            }
            pub fn params<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                params: &'a impl cornucopia_client::async_::Params<'a, Self, IdQuery<'a, C, Id, 2>, C>,
            ) -> IdQuery<'a, C, Id, 2> {
                params.bind(client, self)
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_client::async_::Params<'a, NewNamedHiddenStmt, IdQuery<'a, C, Id, 2>, C>
            for NamedParams<'a>
        {
            fn bind(
                &'a self,
                client: &'a C,
                stmt: &'a mut NewNamedHiddenStmt,
            ) -> IdQuery<'a, C, Id, 2> {
                stmt.bind(client, &self.price, &self.name)
            }
        }
        pub fn named() -> NamedStmt {
            NamedStmt(cornucopia_client::async_::Stmt::new("SELECT * FROM named"))
        }
        pub struct NamedStmt(cornucopia_client::async_::Stmt);
        impl NamedStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> NamedQuery<'a, C, Named, 0> {
                NamedQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| NamedBorrowed {
                        id: row.get(0),
                        name: row.get(1),
                        price: row.get(2),
                        show: row.get(3),
                    },
                    mapper: |it| <Named>::from(it),
                }
            }
        }
        pub fn named_by_id() -> NamedByIdStmt {
            NamedByIdStmt(cornucopia_client::async_::Stmt::new(
                "SELECT * FROM named WHERE id = $1",
            ))
        }
        pub struct NamedByIdStmt(cornucopia_client::async_::Stmt);
        impl NamedByIdStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                id: &'a i32,
            ) -> NamedQuery<'a, C, Named, 1> {
                NamedQuery {
                    client,
                    params: [id],
                    stmt: &mut self.0,
                    extractor: |row| NamedBorrowed {
                        id: row.get(0),
                        name: row.get(1),
                        price: row.get(2),
                        show: row.get(3),
                    },
                    mapper: |it| <Named>::from(it),
                }
            }
        }
        pub fn new_named_complex() -> NewNamedComplexStmt {
            NewNamedComplexStmt(cornucopia_client::async_::Stmt::new(
                "INSERT INTO named_complex (named) VALUES ($1)",
            ))
        }
        pub struct NewNamedComplexStmt(cornucopia_client::async_::Stmt);
        impl NewNamedComplexStmt {
            pub async fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                named: &'a super::super::types::public::NamedCompositeBorrowed<'a>,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[named]).await
            }
            pub async fn params<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                params: &'a impl cornucopia_client::async_::Params<
                    'a,
                    Self,
                    std::pin::Pin<
                        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>>,
                    >,
                    C,
                >,
            ) -> Result<u64, tokio_postgres::Error> {
                params.bind(client, self).await
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_client::async_::Params<
                'a,
                NewNamedComplexStmt,
                std::pin::Pin<
                    Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + 'a>,
                >,
                C,
            > for NamedComplexParams<'a>
        {
            fn bind(
                &'a self,
                client: &'a C,
                stmt: &'a mut NewNamedComplexStmt,
            ) -> std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + 'a>,
            > {
                Box::pin(stmt.bind(client, &self.named))
            }
        }
        pub fn named_complex() -> NamedComplexStmt {
            NamedComplexStmt(cornucopia_client::async_::Stmt::new(
                "SELECT * FROM named_complex",
            ))
        }
        pub struct NamedComplexStmt(cornucopia_client::async_::Stmt);
        impl NamedComplexStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> SuperSuperTypesPublicNamedCompositeQuery<
                'a,
                C,
                super::super::types::public::NamedComposite,
                0,
            > {
                SuperSuperTypesPublicNamedCompositeQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it.into(),
                }
            }
        }
    }
    pub mod nullity {
        use cornucopia_client::async_::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug)]
        pub struct NullityParams<'a> {
            pub texts: &'a [Option<&'a str>],
            pub name: &'a str,
            pub composite: Option<super::super::types::public::NullityCompositeParams<'a>>,
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct Nullity {
            pub texts: Vec<Option<String>>,
            pub name: String,
            pub composite: Option<super::super::types::public::NullityComposite>,
        }
        pub struct NullityBorrowed<'a> {
            pub texts: cornucopia_client::ArrayIterator<'a, Option<&'a str>>,
            pub name: &'a str,
            pub composite: Option<super::super::types::public::NullityCompositeBorrowed<'a>>,
        }
        impl<'a> From<NullityBorrowed<'a>> for Nullity {
            fn from(
                NullityBorrowed {
                    texts,
                    name,
                    composite,
                }: NullityBorrowed<'a>,
            ) -> Self {
                Self {
                    texts: texts.map(|v| v.map(|v| v.into())).collect(),
                    name: name.into(),
                    composite: composite.map(|v| v.into()),
                }
            }
        }
        pub struct NullityQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::async_::Stmt,
            extractor: fn(&tokio_postgres::Row) -> NullityBorrowed,
            mapper: fn(NullityBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> NullityQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(NullityBorrowed) -> R) -> NullityQuery<'a, C, R, N> {
                NullityQuery {
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
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn new_nullity() -> NewNullityStmt {
            NewNullityStmt(cornucopia_client::async_::Stmt::new(
                "INSERT INTO nullity(texts, name, composite) VALUES ($1, $2, $3)",
            ))
        }
        pub struct NewNullityStmt(cornucopia_client::async_::Stmt);
        impl NewNullityStmt {
            pub async fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                texts: &'a &'a [Option<&'a str>],
                name: &'a &'a str,
                composite: &'a Option<super::super::types::public::NullityCompositeParams<'a>>,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[texts, name, composite]).await
            }
            pub async fn params<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                params: &'a impl cornucopia_client::async_::Params<
                    'a,
                    Self,
                    std::pin::Pin<
                        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>>,
                    >,
                    C,
                >,
            ) -> Result<u64, tokio_postgres::Error> {
                params.bind(client, self).await
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_client::async_::Params<
                'a,
                NewNullityStmt,
                std::pin::Pin<
                    Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + 'a>,
                >,
                C,
            > for NullityParams<'a>
        {
            fn bind(
                &'a self,
                client: &'a C,
                stmt: &'a mut NewNullityStmt,
            ) -> std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + 'a>,
            > {
                Box::pin(stmt.bind(client, &self.texts, &self.name, &self.composite))
            }
        }
        pub fn nullity() -> NullityStmt {
            NullityStmt(cornucopia_client::async_::Stmt::new(
                "SELECT * FROM nullity",
            ))
        }
        pub struct NullityStmt(cornucopia_client::async_::Stmt);
        impl NullityStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> NullityQuery<'a, C, Nullity, 0> {
                NullityQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| NullityBorrowed {
                        texts: row.get(0),
                        name: row.get(1),
                        composite: row.get(2),
                    },
                    mapper: |it| <Nullity>::from(it),
                }
            }
        }
    }
    pub mod params {
        use cornucopia_client::async_::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug)]
        pub struct InsertBookParams<'a> {
            pub author: Option<&'a str>,
            pub name: &'a str,
        }

        #[derive(Clone, Copy, Debug)]
        pub struct ParamsOrderParams {
            pub c: i32,
            pub a: i32,
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct SelectBook {
            pub name: String,
            pub author: Option<String>,
        }
        pub struct SelectBookBorrowed<'a> {
            pub name: &'a str,
            pub author: Option<&'a str>,
        }
        impl<'a> From<SelectBookBorrowed<'a>> for SelectBook {
            fn from(SelectBookBorrowed { name, author }: SelectBookBorrowed<'a>) -> Self {
                Self {
                    name: name.into(),
                    author: author.map(|v| v.into()),
                }
            }
        }
        pub struct SelectBookQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::async_::Stmt,
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
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn insert_book() -> InsertBookStmt {
            InsertBookStmt(cornucopia_client::async_::Stmt::new(
                "INSERT INTO book (author, name) VALUES ($1, $2)",
            ))
        }
        pub struct InsertBookStmt(cornucopia_client::async_::Stmt);
        impl InsertBookStmt {
            pub async fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                author: &'a Option<&'a str>,
                name: &'a &'a str,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[author, name]).await
            }
            pub async fn params<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                params: &'a impl cornucopia_client::async_::Params<
                    'a,
                    Self,
                    std::pin::Pin<
                        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>>,
                    >,
                    C,
                >,
            ) -> Result<u64, tokio_postgres::Error> {
                params.bind(client, self).await
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_client::async_::Params<
                'a,
                InsertBookStmt,
                std::pin::Pin<
                    Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + 'a>,
                >,
                C,
            > for InsertBookParams<'a>
        {
            fn bind(
                &'a self,
                client: &'a C,
                stmt: &'a mut InsertBookStmt,
            ) -> std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + 'a>,
            > {
                Box::pin(stmt.bind(client, &self.author, &self.name))
            }
        }
        pub fn select_book() -> SelectBookStmt {
            SelectBookStmt(cornucopia_client::async_::Stmt::new("SELECT * FROM book"))
        }
        pub struct SelectBookStmt(cornucopia_client::async_::Stmt);
        impl SelectBookStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> SelectBookQuery<'a, C, SelectBook, 0> {
                SelectBookQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| SelectBookBorrowed {
                        name: row.get(0),
                        author: row.get(1),
                    },
                    mapper: |it| <SelectBook>::from(it),
                }
            }
        }
        pub fn params_use_twice() -> ParamsUseTwiceStmt {
            ParamsUseTwiceStmt(cornucopia_client::async_::Stmt::new(
                "UPDATE book SET name = $1 WHERE length(name) > 42 AND length($1) < 42",
            ))
        }
        pub struct ParamsUseTwiceStmt(cornucopia_client::async_::Stmt);
        impl ParamsUseTwiceStmt {
            pub async fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                name: &'a &'a str,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[name]).await
            }
        }
        pub fn params_order() -> ParamsOrderStmt {
            ParamsOrderStmt(cornucopia_client::async_::Stmt::new(
                "UPDATE imaginary SET c=$1, a=$2, z=$2, r=$1",
            ))
        }
        pub struct ParamsOrderStmt(cornucopia_client::async_::Stmt);
        impl ParamsOrderStmt {
            pub async fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                c: &'a i32,
                a: &'a i32,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[c, a]).await
            }
            pub async fn params<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                params: &'a impl cornucopia_client::async_::Params<
                    'a,
                    Self,
                    std::pin::Pin<
                        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>>,
                    >,
                    C,
                >,
            ) -> Result<u64, tokio_postgres::Error> {
                params.bind(client, self).await
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_client::async_::Params<
                'a,
                ParamsOrderStmt,
                std::pin::Pin<
                    Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + 'a>,
                >,
                C,
            > for ParamsOrderParams
        {
            fn bind(
                &'a self,
                client: &'a C,
                stmt: &'a mut ParamsOrderStmt,
            ) -> std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + 'a>,
            > {
                Box::pin(stmt.bind(client, &self.c, &self.a))
            }
        }
    }
    pub mod stress {
        use cornucopia_client::async_::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug)]
        pub struct EverythingParams<'a> {
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
            pub json_: &'a serde_json::value::Value,
            pub jsonb_: &'a serde_json::value::Value,
            pub uuid_: uuid::Uuid,
            pub inet_: std::net::IpAddr,
            pub macaddr_: eui48::MacAddress,
        }
        #[derive(Debug)]
        pub struct EverythingArrayParams<'a> {
            pub bool_: &'a [bool],
            pub boolean_: &'a [bool],
            pub char_: &'a [i8],
            pub smallint_: &'a [i16],
            pub int2_: &'a [i16],
            pub int_: &'a [i32],
            pub int4_: &'a [i32],
            pub bingint_: &'a [i64],
            pub int8_: &'a [i64],
            pub float4_: &'a [f32],
            pub real_: &'a [f32],
            pub float8_: &'a [f64],
            pub double_precision_: &'a [f64],
            pub text_: &'a [&'a str],
            pub varchar_: &'a [&'a str],
            pub bytea_: &'a [&'a [u8]],
            pub timestamp_: &'a [time::PrimitiveDateTime],
            pub timestamp_without_time_zone_: &'a [time::PrimitiveDateTime],
            pub timestamptz_: &'a [time::OffsetDateTime],
            pub timestamp_with_time_zone_: &'a [time::OffsetDateTime],
            pub date_: &'a [time::Date],
            pub time_: &'a [time::Time],
            pub json_: &'a [&'a serde_json::value::Value],
            pub jsonb_: &'a [&'a serde_json::value::Value],
            pub uuid_: &'a [uuid::Uuid],
            pub inet_: &'a [std::net::IpAddr],
            pub macaddr_: &'a [eui48::MacAddress],
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct Everything {
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
            pub json_: serde_json::Value,
            pub jsonb_: serde_json::Value,
            pub uuid_: uuid::Uuid,
            pub inet_: std::net::IpAddr,
            pub macaddr_: eui48::MacAddress,
        }
        pub struct EverythingBorrowed<'a> {
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
        impl<'a> From<EverythingBorrowed<'a>> for Everything {
            fn from(
                EverythingBorrowed {
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
                }: EverythingBorrowed<'a>,
            ) -> Self {
                Self {
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
                    text_: text_.into(),
                    varchar_: varchar_.into(),
                    bytea_: bytea_.into(),
                    timestamp_,
                    timestamp_without_time_zone_,
                    timestamptz_,
                    timestamp_with_time_zone_,
                    date_,
                    time_,
                    json_: serde_json::from_str(json_.0.get()).unwrap(),
                    jsonb_: serde_json::from_str(jsonb_.0.get()).unwrap(),
                    uuid_,
                    inet_,
                    macaddr_,
                }
            }
        }
        pub struct EverythingQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::async_::Stmt,
            extractor: fn(&tokio_postgres::Row) -> EverythingBorrowed,
            mapper: fn(EverythingBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> EverythingQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(EverythingBorrowed) -> R,
            ) -> EverythingQuery<'a, C, R, N> {
                EverythingQuery {
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
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct EverythingNull {
            pub bool_: Option<bool>,
            pub boolean_: Option<bool>,
            pub char_: Option<i8>,
            pub smallint_: Option<i16>,
            pub int2_: Option<i16>,
            pub smallserial_: Option<i16>,
            pub serial2_: Option<i16>,
            pub int_: Option<i32>,
            pub int4_: Option<i32>,
            pub serial_: Option<i32>,
            pub serial4_: Option<i32>,
            pub bingint_: Option<i64>,
            pub int8_: Option<i64>,
            pub bigserial_: Option<i64>,
            pub serial8_: Option<i64>,
            pub float4_: Option<f32>,
            pub real_: Option<f32>,
            pub float8_: Option<f64>,
            pub double_precision_: Option<f64>,
            pub text_: Option<String>,
            pub varchar_: Option<String>,
            pub bytea_: Option<Vec<u8>>,
            pub timestamp_: Option<time::PrimitiveDateTime>,
            pub timestamp_without_time_zone_: Option<time::PrimitiveDateTime>,
            pub timestamptz_: Option<time::OffsetDateTime>,
            pub timestamp_with_time_zone_: Option<time::OffsetDateTime>,
            pub date_: Option<time::Date>,
            pub time_: Option<time::Time>,
            pub json_: Option<serde_json::Value>,
            pub jsonb_: Option<serde_json::Value>,
            pub uuid_: Option<uuid::Uuid>,
            pub inet_: Option<std::net::IpAddr>,
            pub macaddr_: Option<eui48::MacAddress>,
        }
        pub struct EverythingNullBorrowed<'a> {
            pub bool_: Option<bool>,
            pub boolean_: Option<bool>,
            pub char_: Option<i8>,
            pub smallint_: Option<i16>,
            pub int2_: Option<i16>,
            pub smallserial_: Option<i16>,
            pub serial2_: Option<i16>,
            pub int_: Option<i32>,
            pub int4_: Option<i32>,
            pub serial_: Option<i32>,
            pub serial4_: Option<i32>,
            pub bingint_: Option<i64>,
            pub int8_: Option<i64>,
            pub bigserial_: Option<i64>,
            pub serial8_: Option<i64>,
            pub float4_: Option<f32>,
            pub real_: Option<f32>,
            pub float8_: Option<f64>,
            pub double_precision_: Option<f64>,
            pub text_: Option<&'a str>,
            pub varchar_: Option<&'a str>,
            pub bytea_: Option<&'a [u8]>,
            pub timestamp_: Option<time::PrimitiveDateTime>,
            pub timestamp_without_time_zone_: Option<time::PrimitiveDateTime>,
            pub timestamptz_: Option<time::OffsetDateTime>,
            pub timestamp_with_time_zone_: Option<time::OffsetDateTime>,
            pub date_: Option<time::Date>,
            pub time_: Option<time::Time>,
            pub json_: Option<postgres_types::Json<&'a serde_json::value::RawValue>>,
            pub jsonb_: Option<postgres_types::Json<&'a serde_json::value::RawValue>>,
            pub uuid_: Option<uuid::Uuid>,
            pub inet_: Option<std::net::IpAddr>,
            pub macaddr_: Option<eui48::MacAddress>,
        }
        impl<'a> From<EverythingNullBorrowed<'a>> for EverythingNull {
            fn from(
                EverythingNullBorrowed {
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
                }: EverythingNullBorrowed<'a>,
            ) -> Self {
                Self {
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
                    text_: text_.map(|v| v.into()),
                    varchar_: varchar_.map(|v| v.into()),
                    bytea_: bytea_.map(|v| v.into()),
                    timestamp_,
                    timestamp_without_time_zone_,
                    timestamptz_,
                    timestamp_with_time_zone_,
                    date_,
                    time_,
                    json_: json_.map(|v| serde_json::from_str(v.0.get()).unwrap()),
                    jsonb_: jsonb_.map(|v| serde_json::from_str(v.0.get()).unwrap()),
                    uuid_,
                    inet_,
                    macaddr_,
                }
            }
        }
        pub struct EverythingNullQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::async_::Stmt,
            extractor: fn(&tokio_postgres::Row) -> EverythingNullBorrowed,
            mapper: fn(EverythingNullBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> EverythingNullQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(EverythingNullBorrowed) -> R,
            ) -> EverythingNullQuery<'a, C, R, N> {
                EverythingNullQuery {
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
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct EverythingArray {
            pub bool_: Vec<bool>,
            pub boolean_: Vec<bool>,
            pub char_: Vec<i8>,
            pub smallint_: Vec<i16>,
            pub int2_: Vec<i16>,
            pub int_: Vec<i32>,
            pub int4_: Vec<i32>,
            pub bingint_: Vec<i64>,
            pub int8_: Vec<i64>,
            pub float4_: Vec<f32>,
            pub real_: Vec<f32>,
            pub float8_: Vec<f64>,
            pub double_precision_: Vec<f64>,
            pub text_: Vec<String>,
            pub varchar_: Vec<String>,
            pub bytea_: Vec<Vec<u8>>,
            pub timestamp_: Vec<time::PrimitiveDateTime>,
            pub timestamp_without_time_zone_: Vec<time::PrimitiveDateTime>,
            pub timestamptz_: Vec<time::OffsetDateTime>,
            pub timestamp_with_time_zone_: Vec<time::OffsetDateTime>,
            pub date_: Vec<time::Date>,
            pub time_: Vec<time::Time>,
            pub json_: Vec<serde_json::Value>,
            pub jsonb_: Vec<serde_json::Value>,
            pub uuid_: Vec<uuid::Uuid>,
            pub inet_: Vec<std::net::IpAddr>,
            pub macaddr_: Vec<eui48::MacAddress>,
        }
        pub struct EverythingArrayBorrowed<'a> {
            pub bool_: cornucopia_client::ArrayIterator<'a, bool>,
            pub boolean_: cornucopia_client::ArrayIterator<'a, bool>,
            pub char_: cornucopia_client::ArrayIterator<'a, i8>,
            pub smallint_: cornucopia_client::ArrayIterator<'a, i16>,
            pub int2_: cornucopia_client::ArrayIterator<'a, i16>,
            pub int_: cornucopia_client::ArrayIterator<'a, i32>,
            pub int4_: cornucopia_client::ArrayIterator<'a, i32>,
            pub bingint_: cornucopia_client::ArrayIterator<'a, i64>,
            pub int8_: cornucopia_client::ArrayIterator<'a, i64>,
            pub float4_: cornucopia_client::ArrayIterator<'a, f32>,
            pub real_: cornucopia_client::ArrayIterator<'a, f32>,
            pub float8_: cornucopia_client::ArrayIterator<'a, f64>,
            pub double_precision_: cornucopia_client::ArrayIterator<'a, f64>,
            pub text_: cornucopia_client::ArrayIterator<'a, &'a str>,
            pub varchar_: cornucopia_client::ArrayIterator<'a, &'a str>,
            pub bytea_: cornucopia_client::ArrayIterator<'a, &'a [u8]>,
            pub timestamp_: cornucopia_client::ArrayIterator<'a, time::PrimitiveDateTime>,
            pub timestamp_without_time_zone_:
                cornucopia_client::ArrayIterator<'a, time::PrimitiveDateTime>,
            pub timestamptz_: cornucopia_client::ArrayIterator<'a, time::OffsetDateTime>,
            pub timestamp_with_time_zone_:
                cornucopia_client::ArrayIterator<'a, time::OffsetDateTime>,
            pub date_: cornucopia_client::ArrayIterator<'a, time::Date>,
            pub time_: cornucopia_client::ArrayIterator<'a, time::Time>,
            pub json_: cornucopia_client::ArrayIterator<
                'a,
                postgres_types::Json<&'a serde_json::value::RawValue>,
            >,
            pub jsonb_: cornucopia_client::ArrayIterator<
                'a,
                postgres_types::Json<&'a serde_json::value::RawValue>,
            >,
            pub uuid_: cornucopia_client::ArrayIterator<'a, uuid::Uuid>,
            pub inet_: cornucopia_client::ArrayIterator<'a, std::net::IpAddr>,
            pub macaddr_: cornucopia_client::ArrayIterator<'a, eui48::MacAddress>,
        }
        impl<'a> From<EverythingArrayBorrowed<'a>> for EverythingArray {
            fn from(
                EverythingArrayBorrowed {
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
                }: EverythingArrayBorrowed<'a>,
            ) -> Self {
                Self {
                    bool_: bool_.map(|v| v).collect(),
                    boolean_: boolean_.map(|v| v).collect(),
                    char_: char_.map(|v| v).collect(),
                    smallint_: smallint_.map(|v| v).collect(),
                    int2_: int2_.map(|v| v).collect(),
                    int_: int_.map(|v| v).collect(),
                    int4_: int4_.map(|v| v).collect(),
                    bingint_: bingint_.map(|v| v).collect(),
                    int8_: int8_.map(|v| v).collect(),
                    float4_: float4_.map(|v| v).collect(),
                    real_: real_.map(|v| v).collect(),
                    float8_: float8_.map(|v| v).collect(),
                    double_precision_: double_precision_.map(|v| v).collect(),
                    text_: text_.map(|v| v.into()).collect(),
                    varchar_: varchar_.map(|v| v.into()).collect(),
                    bytea_: bytea_.map(|v| v.into()).collect(),
                    timestamp_: timestamp_.map(|v| v).collect(),
                    timestamp_without_time_zone_: timestamp_without_time_zone_.map(|v| v).collect(),
                    timestamptz_: timestamptz_.map(|v| v).collect(),
                    timestamp_with_time_zone_: timestamp_with_time_zone_.map(|v| v).collect(),
                    date_: date_.map(|v| v).collect(),
                    time_: time_.map(|v| v).collect(),
                    json_: json_
                        .map(|v| serde_json::from_str(v.0.get()).unwrap())
                        .collect(),
                    jsonb_: jsonb_
                        .map(|v| serde_json::from_str(v.0.get()).unwrap())
                        .collect(),
                    uuid_: uuid_.map(|v| v).collect(),
                    inet_: inet_.map(|v| v).collect(),
                    macaddr_: macaddr_.map(|v| v).collect(),
                }
            }
        }
        pub struct EverythingArrayQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::async_::Stmt,
            extractor: fn(&tokio_postgres::Row) -> EverythingArrayBorrowed,
            mapper: fn(EverythingArrayBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> EverythingArrayQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(EverythingArrayBorrowed) -> R,
            ) -> EverythingArrayQuery<'a, C, R, N> {
                EverythingArrayQuery {
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
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct EverythingArrayNull {
            pub bool_: Option<Vec<bool>>,
            pub boolean_: Option<Vec<bool>>,
            pub char_: Option<Vec<i8>>,
            pub smallint_: Option<Vec<i16>>,
            pub int2_: Option<Vec<i16>>,
            pub int_: Option<Vec<i32>>,
            pub int4_: Option<Vec<i32>>,
            pub bingint_: Option<Vec<i64>>,
            pub int8_: Option<Vec<i64>>,
            pub float4_: Option<Vec<f32>>,
            pub real_: Option<Vec<f32>>,
            pub float8_: Option<Vec<f64>>,
            pub double_precision_: Option<Vec<f64>>,
            pub text_: Option<Vec<String>>,
            pub varchar_: Option<Vec<String>>,
            pub bytea_: Option<Vec<Vec<u8>>>,
            pub timestamp_: Option<Vec<time::PrimitiveDateTime>>,
            pub timestamp_without_time_zone_: Option<Vec<time::PrimitiveDateTime>>,
            pub timestamptz_: Option<Vec<time::OffsetDateTime>>,
            pub timestamp_with_time_zone_: Option<Vec<time::OffsetDateTime>>,
            pub date_: Option<Vec<time::Date>>,
            pub time_: Option<Vec<time::Time>>,
            pub json_: Option<Vec<serde_json::Value>>,
            pub jsonb_: Option<Vec<serde_json::Value>>,
            pub uuid_: Option<Vec<uuid::Uuid>>,
            pub inet_: Option<Vec<std::net::IpAddr>>,
            pub macaddr_: Option<Vec<eui48::MacAddress>>,
        }
        pub struct EverythingArrayNullBorrowed<'a> {
            pub bool_: Option<cornucopia_client::ArrayIterator<'a, bool>>,
            pub boolean_: Option<cornucopia_client::ArrayIterator<'a, bool>>,
            pub char_: Option<cornucopia_client::ArrayIterator<'a, i8>>,
            pub smallint_: Option<cornucopia_client::ArrayIterator<'a, i16>>,
            pub int2_: Option<cornucopia_client::ArrayIterator<'a, i16>>,
            pub int_: Option<cornucopia_client::ArrayIterator<'a, i32>>,
            pub int4_: Option<cornucopia_client::ArrayIterator<'a, i32>>,
            pub bingint_: Option<cornucopia_client::ArrayIterator<'a, i64>>,
            pub int8_: Option<cornucopia_client::ArrayIterator<'a, i64>>,
            pub float4_: Option<cornucopia_client::ArrayIterator<'a, f32>>,
            pub real_: Option<cornucopia_client::ArrayIterator<'a, f32>>,
            pub float8_: Option<cornucopia_client::ArrayIterator<'a, f64>>,
            pub double_precision_: Option<cornucopia_client::ArrayIterator<'a, f64>>,
            pub text_: Option<cornucopia_client::ArrayIterator<'a, &'a str>>,
            pub varchar_: Option<cornucopia_client::ArrayIterator<'a, &'a str>>,
            pub bytea_: Option<cornucopia_client::ArrayIterator<'a, &'a [u8]>>,
            pub timestamp_: Option<cornucopia_client::ArrayIterator<'a, time::PrimitiveDateTime>>,
            pub timestamp_without_time_zone_:
                Option<cornucopia_client::ArrayIterator<'a, time::PrimitiveDateTime>>,
            pub timestamptz_: Option<cornucopia_client::ArrayIterator<'a, time::OffsetDateTime>>,
            pub timestamp_with_time_zone_:
                Option<cornucopia_client::ArrayIterator<'a, time::OffsetDateTime>>,
            pub date_: Option<cornucopia_client::ArrayIterator<'a, time::Date>>,
            pub time_: Option<cornucopia_client::ArrayIterator<'a, time::Time>>,
            pub json_: Option<
                cornucopia_client::ArrayIterator<
                    'a,
                    postgres_types::Json<&'a serde_json::value::RawValue>,
                >,
            >,
            pub jsonb_: Option<
                cornucopia_client::ArrayIterator<
                    'a,
                    postgres_types::Json<&'a serde_json::value::RawValue>,
                >,
            >,
            pub uuid_: Option<cornucopia_client::ArrayIterator<'a, uuid::Uuid>>,
            pub inet_: Option<cornucopia_client::ArrayIterator<'a, std::net::IpAddr>>,
            pub macaddr_: Option<cornucopia_client::ArrayIterator<'a, eui48::MacAddress>>,
        }
        impl<'a> From<EverythingArrayNullBorrowed<'a>> for EverythingArrayNull {
            fn from(
                EverythingArrayNullBorrowed {
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
                }: EverythingArrayNullBorrowed<'a>,
            ) -> Self {
                Self {
                    bool_: bool_.map(|v| v.map(|v| v).collect()),
                    boolean_: boolean_.map(|v| v.map(|v| v).collect()),
                    char_: char_.map(|v| v.map(|v| v).collect()),
                    smallint_: smallint_.map(|v| v.map(|v| v).collect()),
                    int2_: int2_.map(|v| v.map(|v| v).collect()),
                    int_: int_.map(|v| v.map(|v| v).collect()),
                    int4_: int4_.map(|v| v.map(|v| v).collect()),
                    bingint_: bingint_.map(|v| v.map(|v| v).collect()),
                    int8_: int8_.map(|v| v.map(|v| v).collect()),
                    float4_: float4_.map(|v| v.map(|v| v).collect()),
                    real_: real_.map(|v| v.map(|v| v).collect()),
                    float8_: float8_.map(|v| v.map(|v| v).collect()),
                    double_precision_: double_precision_.map(|v| v.map(|v| v).collect()),
                    text_: text_.map(|v| v.map(|v| v.into()).collect()),
                    varchar_: varchar_.map(|v| v.map(|v| v.into()).collect()),
                    bytea_: bytea_.map(|v| v.map(|v| v.into()).collect()),
                    timestamp_: timestamp_.map(|v| v.map(|v| v).collect()),
                    timestamp_without_time_zone_: timestamp_without_time_zone_
                        .map(|v| v.map(|v| v).collect()),
                    timestamptz_: timestamptz_.map(|v| v.map(|v| v).collect()),
                    timestamp_with_time_zone_: timestamp_with_time_zone_
                        .map(|v| v.map(|v| v).collect()),
                    date_: date_.map(|v| v.map(|v| v).collect()),
                    time_: time_.map(|v| v.map(|v| v).collect()),
                    json_: json_.map(|v| {
                        v.map(|v| serde_json::from_str(v.0.get()).unwrap())
                            .collect()
                    }),
                    jsonb_: jsonb_.map(|v| {
                        v.map(|v| serde_json::from_str(v.0.get()).unwrap())
                            .collect()
                    }),
                    uuid_: uuid_.map(|v| v.map(|v| v).collect()),
                    inet_: inet_.map(|v| v.map(|v| v).collect()),
                    macaddr_: macaddr_.map(|v| v.map(|v| v).collect()),
                }
            }
        }
        pub struct EverythingArrayNullQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::async_::Stmt,
            extractor: fn(&tokio_postgres::Row) -> EverythingArrayNullBorrowed,
            mapper: fn(EverythingArrayNullBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> EverythingArrayNullQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(EverythingArrayNullBorrowed) -> R,
            ) -> EverythingArrayNullQuery<'a, C, R, N> {
                EverythingArrayNullQuery {
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
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }

        pub struct SuperSuperTypesPublicNightmareCompositeQuery<
            'a,
            C: GenericClient,
            T,
            const N: usize,
        > {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::async_::Stmt,
            extractor:
                fn(&tokio_postgres::Row) -> super::super::types::public::NightmareCompositeBorrowed,
            mapper: fn(super::super::types::public::NightmareCompositeBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SuperSuperTypesPublicNightmareCompositeQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(super::super::types::public::NightmareCompositeBorrowed) -> R,
            ) -> SuperSuperTypesPublicNightmareCompositeQuery<'a, C, R, N> {
                SuperSuperTypesPublicNightmareCompositeQuery {
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
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn select_everything() -> SelectEverythingStmt {
            SelectEverythingStmt(cornucopia_client::async_::Stmt::new(
                "SELECT * FROM Everything",
            ))
        }
        pub struct SelectEverythingStmt(cornucopia_client::async_::Stmt);
        impl SelectEverythingStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> EverythingQuery<'a, C, Everything, 0> {
                EverythingQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| EverythingBorrowed {
                        bool_: row.get(0),
                        boolean_: row.get(1),
                        char_: row.get(2),
                        smallint_: row.get(3),
                        int2_: row.get(4),
                        smallserial_: row.get(5),
                        serial2_: row.get(6),
                        int_: row.get(7),
                        int4_: row.get(8),
                        serial_: row.get(9),
                        serial4_: row.get(10),
                        bingint_: row.get(11),
                        int8_: row.get(12),
                        bigserial_: row.get(13),
                        serial8_: row.get(14),
                        float4_: row.get(15),
                        real_: row.get(16),
                        float8_: row.get(17),
                        double_precision_: row.get(18),
                        text_: row.get(19),
                        varchar_: row.get(20),
                        bytea_: row.get(21),
                        timestamp_: row.get(22),
                        timestamp_without_time_zone_: row.get(23),
                        timestamptz_: row.get(24),
                        timestamp_with_time_zone_: row.get(25),
                        date_: row.get(26),
                        time_: row.get(27),
                        json_: row.get(28),
                        jsonb_: row.get(29),
                        uuid_: row.get(30),
                        inet_: row.get(31),
                        macaddr_: row.get(32),
                    },
                    mapper: |it| <Everything>::from(it),
                }
            }
        }
        pub fn select_everything_null() -> SelectEverythingNullStmt {
            SelectEverythingNullStmt(cornucopia_client::async_::Stmt::new(
                "SELECT * FROM Everything",
            ))
        }
        pub struct SelectEverythingNullStmt(cornucopia_client::async_::Stmt);
        impl SelectEverythingNullStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> EverythingNullQuery<'a, C, EverythingNull, 0> {
                EverythingNullQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| EverythingNullBorrowed {
                        bool_: row.get(0),
                        boolean_: row.get(1),
                        char_: row.get(2),
                        smallint_: row.get(3),
                        int2_: row.get(4),
                        smallserial_: row.get(5),
                        serial2_: row.get(6),
                        int_: row.get(7),
                        int4_: row.get(8),
                        serial_: row.get(9),
                        serial4_: row.get(10),
                        bingint_: row.get(11),
                        int8_: row.get(12),
                        bigserial_: row.get(13),
                        serial8_: row.get(14),
                        float4_: row.get(15),
                        real_: row.get(16),
                        float8_: row.get(17),
                        double_precision_: row.get(18),
                        text_: row.get(19),
                        varchar_: row.get(20),
                        bytea_: row.get(21),
                        timestamp_: row.get(22),
                        timestamp_without_time_zone_: row.get(23),
                        timestamptz_: row.get(24),
                        timestamp_with_time_zone_: row.get(25),
                        date_: row.get(26),
                        time_: row.get(27),
                        json_: row.get(28),
                        jsonb_: row.get(29),
                        uuid_: row.get(30),
                        inet_: row.get(31),
                        macaddr_: row.get(32),
                    },
                    mapper: |it| <EverythingNull>::from(it),
                }
            }
        }
        pub fn insert_everything() -> InsertEverythingStmt {
            InsertEverythingStmt(cornucopia_client::async_::Stmt::new("INSERT INTO Everything (bool_, boolean_, char_, smallint_, int2_, smallserial_, serial2_, int_, int4_, serial_, serial4_, bingint_, int8_, bigserial_, serial8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33)"))
        }
        pub struct InsertEverythingStmt(cornucopia_client::async_::Stmt);
        impl InsertEverythingStmt {
            pub async fn bind<'a, C: GenericClient>(
                &'a mut self,
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
                json_: &'a &'a serde_json::value::Value,
                jsonb_: &'a &'a serde_json::value::Value,
                uuid_: &'a uuid::Uuid,
                inet_: &'a std::net::IpAddr,
                macaddr_: &'a eui48::MacAddress,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client
                    .execute(
                        stmt,
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
            pub async fn params<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                params: &'a impl cornucopia_client::async_::Params<
                    'a,
                    Self,
                    std::pin::Pin<
                        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>>,
                    >,
                    C,
                >,
            ) -> Result<u64, tokio_postgres::Error> {
                params.bind(client, self).await
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_client::async_::Params<
                'a,
                InsertEverythingStmt,
                std::pin::Pin<
                    Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + 'a>,
                >,
                C,
            > for EverythingParams<'a>
        {
            fn bind(
                &'a self,
                client: &'a C,
                stmt: &'a mut InsertEverythingStmt,
            ) -> std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + 'a>,
            > {
                Box::pin(stmt.bind(
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
                ))
            }
        }
        pub fn select_everything_array() -> SelectEverythingArrayStmt {
            SelectEverythingArrayStmt(cornucopia_client::async_::Stmt::new(
                "SELECT * FROM EverythingArray",
            ))
        }
        pub struct SelectEverythingArrayStmt(cornucopia_client::async_::Stmt);
        impl SelectEverythingArrayStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> EverythingArrayQuery<'a, C, EverythingArray, 0> {
                EverythingArrayQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| EverythingArrayBorrowed {
                        bool_: row.get(0),
                        boolean_: row.get(1),
                        char_: row.get(2),
                        smallint_: row.get(3),
                        int2_: row.get(4),
                        int_: row.get(5),
                        int4_: row.get(6),
                        bingint_: row.get(7),
                        int8_: row.get(8),
                        float4_: row.get(9),
                        real_: row.get(10),
                        float8_: row.get(11),
                        double_precision_: row.get(12),
                        text_: row.get(13),
                        varchar_: row.get(14),
                        bytea_: row.get(15),
                        timestamp_: row.get(16),
                        timestamp_without_time_zone_: row.get(17),
                        timestamptz_: row.get(18),
                        timestamp_with_time_zone_: row.get(19),
                        date_: row.get(20),
                        time_: row.get(21),
                        json_: row.get(22),
                        jsonb_: row.get(23),
                        uuid_: row.get(24),
                        inet_: row.get(25),
                        macaddr_: row.get(26),
                    },
                    mapper: |it| <EverythingArray>::from(it),
                }
            }
        }
        pub fn select_everything_array_null() -> SelectEverythingArrayNullStmt {
            SelectEverythingArrayNullStmt(cornucopia_client::async_::Stmt::new(
                "SELECT * FROM EverythingArray",
            ))
        }
        pub struct SelectEverythingArrayNullStmt(cornucopia_client::async_::Stmt);
        impl SelectEverythingArrayNullStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> EverythingArrayNullQuery<'a, C, EverythingArrayNull, 0> {
                EverythingArrayNullQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| EverythingArrayNullBorrowed {
                        bool_: row.get(0),
                        boolean_: row.get(1),
                        char_: row.get(2),
                        smallint_: row.get(3),
                        int2_: row.get(4),
                        int_: row.get(5),
                        int4_: row.get(6),
                        bingint_: row.get(7),
                        int8_: row.get(8),
                        float4_: row.get(9),
                        real_: row.get(10),
                        float8_: row.get(11),
                        double_precision_: row.get(12),
                        text_: row.get(13),
                        varchar_: row.get(14),
                        bytea_: row.get(15),
                        timestamp_: row.get(16),
                        timestamp_without_time_zone_: row.get(17),
                        timestamptz_: row.get(18),
                        timestamp_with_time_zone_: row.get(19),
                        date_: row.get(20),
                        time_: row.get(21),
                        json_: row.get(22),
                        jsonb_: row.get(23),
                        uuid_: row.get(24),
                        inet_: row.get(25),
                        macaddr_: row.get(26),
                    },
                    mapper: |it| <EverythingArrayNull>::from(it),
                }
            }
        }
        pub fn insert_everything_array() -> InsertEverythingArrayStmt {
            InsertEverythingArrayStmt(cornucopia_client::async_::Stmt::new("INSERT INTO EverythingArray (bool_, boolean_, char_, smallint_, int2_, int_, int4_, bingint_, int8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27)"))
        }
        pub struct InsertEverythingArrayStmt(cornucopia_client::async_::Stmt);
        impl InsertEverythingArrayStmt {
            pub async fn bind<'a, C: GenericClient>(
                &'a mut self,
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
                json_: &'a &'a [&'a serde_json::value::Value],
                jsonb_: &'a &'a [&'a serde_json::value::Value],
                uuid_: &'a &'a [uuid::Uuid],
                inet_: &'a &'a [std::net::IpAddr],
                macaddr_: &'a &'a [eui48::MacAddress],
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client
                    .execute(
                        stmt,
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
            pub async fn params<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                params: &'a impl cornucopia_client::async_::Params<
                    'a,
                    Self,
                    std::pin::Pin<
                        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>>,
                    >,
                    C,
                >,
            ) -> Result<u64, tokio_postgres::Error> {
                params.bind(client, self).await
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_client::async_::Params<
                'a,
                InsertEverythingArrayStmt,
                std::pin::Pin<
                    Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + 'a>,
                >,
                C,
            > for EverythingArrayParams<'a>
        {
            fn bind(
                &'a self,
                client: &'a C,
                stmt: &'a mut InsertEverythingArrayStmt,
            ) -> std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + 'a>,
            > {
                Box::pin(stmt.bind(
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
                ))
            }
        }
        pub fn select_nightmare() -> SelectNightmareStmt {
            SelectNightmareStmt(cornucopia_client::async_::Stmt::new(
                "SELECT * FROM nightmare",
            ))
        }
        pub struct SelectNightmareStmt(cornucopia_client::async_::Stmt);
        impl SelectNightmareStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> SuperSuperTypesPublicNightmareCompositeQuery<
                'a,
                C,
                super::super::types::public::NightmareComposite,
                0,
            > {
                SuperSuperTypesPublicNightmareCompositeQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it.into(),
                }
            }
        }
        pub fn insert_nightmare() -> InsertNightmareStmt {
            InsertNightmareStmt(cornucopia_client::async_::Stmt::new(
                "INSERT INTO nightmare (composite) VALUES ($1)",
            ))
        }
        pub struct InsertNightmareStmt(cornucopia_client::async_::Stmt);
        impl InsertNightmareStmt {
            pub async fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                composite: &'a super::super::types::public::NightmareCompositeParams<'a>,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[composite]).await
            }
        }
    }
    pub mod syntax {
        use cornucopia_client::async_::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug)]
        pub struct ImplicitCompactParams<'a> {
            pub name: Option<&'a str>,
            pub price: Option<f64>,
        }
        #[derive(Debug)]
        pub struct ImplicitSpacedParams<'a> {
            pub name: Option<&'a str>,
            pub price: Option<f64>,
        }
        #[derive(Debug)]
        pub struct Params<'a> {
            pub name: &'a str,
            pub price: f64,
        }
        #[derive(Debug)]
        pub struct ParamsSpace<'a> {
            pub name: &'a str,
            pub price: f64,
        }

        pub struct SuperSuperTypesPublicCloneCompositeQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::async_::Stmt,
            extractor:
                fn(&tokio_postgres::Row) -> super::super::types::public::CloneCompositeBorrowed,
            mapper: fn(super::super::types::public::CloneCompositeBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SuperSuperTypesPublicCloneCompositeQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(super::super::types::public::CloneCompositeBorrowed) -> R,
            ) -> SuperSuperTypesPublicCloneCompositeQuery<'a, C, R, N> {
                SuperSuperTypesPublicCloneCompositeQuery {
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
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }

        pub struct Optioni32Query<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::async_::Stmt,
            extractor: fn(&tokio_postgres::Row) -> Option<i32>,
            mapper: fn(Option<i32>) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> Optioni32Query<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(Option<i32>) -> R) -> Optioni32Query<'a, C, R, N> {
                Optioni32Query {
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
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq, Copy)]
        pub struct Row {
            pub id: i32,
        }
        pub struct RowQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::async_::Stmt,
            extractor: fn(&tokio_postgres::Row) -> Row,
            mapper: fn(Row) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> RowQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(Row) -> R) -> RowQuery<'a, C, R, N> {
                RowQuery {
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
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq, Copy)]
        pub struct RowSpace {
            pub id: i32,
        }
        pub struct RowSpaceQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::async_::Stmt,
            extractor: fn(&tokio_postgres::Row) -> RowSpace,
            mapper: fn(RowSpace) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> RowSpaceQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(RowSpace) -> R) -> RowSpaceQuery<'a, C, R, N> {
                RowSpaceQuery {
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
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct Syntax {
            pub trick_y: String,
            pub price: f64,
        }
        pub struct SyntaxBorrowed<'a> {
            pub trick_y: &'a str,
            pub price: f64,
        }
        impl<'a> From<SyntaxBorrowed<'a>> for Syntax {
            fn from(SyntaxBorrowed { trick_y, price }: SyntaxBorrowed<'a>) -> Self {
                Self {
                    trick_y: trick_y.into(),
                    price,
                }
            }
        }
        pub struct SyntaxQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::async_::Stmt,
            extractor: fn(&tokio_postgres::Row) -> SyntaxBorrowed,
            mapper: fn(SyntaxBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SyntaxQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(SyntaxBorrowed) -> R) -> SyntaxQuery<'a, C, R, N> {
                SyntaxQuery {
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
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn select_compact() -> SelectCompactStmt {
            SelectCompactStmt(cornucopia_client::async_::Stmt::new("SELECT * FROM clone"))
        }
        pub struct SelectCompactStmt(cornucopia_client::async_::Stmt);
        impl SelectCompactStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> SuperSuperTypesPublicCloneCompositeQuery<
                'a,
                C,
                super::super::types::public::CloneComposite,
                0,
            > {
                SuperSuperTypesPublicCloneCompositeQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it.into(),
                }
            }
        }
        pub fn select_spaced() -> SelectSpacedStmt {
            SelectSpacedStmt(cornucopia_client::async_::Stmt::new(
                "      SELECT * FROM clone ",
            ))
        }
        pub struct SelectSpacedStmt(cornucopia_client::async_::Stmt);
        impl SelectSpacedStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> SuperSuperTypesPublicCloneCompositeQuery<
                'a,
                C,
                super::super::types::public::CloneComposite,
                0,
            > {
                SuperSuperTypesPublicCloneCompositeQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it.into(),
                }
            }
        }
        pub fn implicit_compact() -> ImplicitCompactStmt {
            ImplicitCompactStmt(cornucopia_client::async_::Stmt::new(
                "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
            ))
        }
        pub struct ImplicitCompactStmt(cornucopia_client::async_::Stmt);
        impl ImplicitCompactStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                name: &'a Option<&'a str>,
                price: &'a Option<f64>,
            ) -> Optioni32Query<'a, C, Option<i32>, 2> {
                Optioni32Query {
                    client,
                    params: [name, price],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it,
                }
            }
            pub fn params<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                params: &'a impl cornucopia_client::async_::Params<
                    'a,
                    Self,
                    Optioni32Query<'a, C, Option<i32>, 2>,
                    C,
                >,
            ) -> Optioni32Query<'a, C, Option<i32>, 2> {
                params.bind(client, self)
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_client::async_::Params<
                'a,
                ImplicitCompactStmt,
                Optioni32Query<'a, C, Option<i32>, 2>,
                C,
            > for ImplicitCompactParams<'a>
        {
            fn bind(
                &'a self,
                client: &'a C,
                stmt: &'a mut ImplicitCompactStmt,
            ) -> Optioni32Query<'a, C, Option<i32>, 2> {
                stmt.bind(client, &self.name, &self.price)
            }
        }
        pub fn implicit_spaced() -> ImplicitSpacedStmt {
            ImplicitSpacedStmt(cornucopia_client::async_::Stmt::new(
                "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
            ))
        }
        pub struct ImplicitSpacedStmt(cornucopia_client::async_::Stmt);
        impl ImplicitSpacedStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                name: &'a Option<&'a str>,
                price: &'a Option<f64>,
            ) -> Optioni32Query<'a, C, Option<i32>, 2> {
                Optioni32Query {
                    client,
                    params: [name, price],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it,
                }
            }
            pub fn params<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                params: &'a impl cornucopia_client::async_::Params<
                    'a,
                    Self,
                    Optioni32Query<'a, C, Option<i32>, 2>,
                    C,
                >,
            ) -> Optioni32Query<'a, C, Option<i32>, 2> {
                params.bind(client, self)
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_client::async_::Params<
                'a,
                ImplicitSpacedStmt,
                Optioni32Query<'a, C, Option<i32>, 2>,
                C,
            > for ImplicitSpacedParams<'a>
        {
            fn bind(
                &'a self,
                client: &'a C,
                stmt: &'a mut ImplicitSpacedStmt,
            ) -> Optioni32Query<'a, C, Option<i32>, 2> {
                stmt.bind(client, &self.name, &self.price)
            }
        }
        pub fn named_compact() -> NamedCompactStmt {
            NamedCompactStmt(cornucopia_client::async_::Stmt::new(
                "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
            ))
        }
        pub struct NamedCompactStmt(cornucopia_client::async_::Stmt);
        impl NamedCompactStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                name: &'a &'a str,
                price: &'a f64,
            ) -> RowQuery<'a, C, Row, 2> {
                RowQuery {
                    client,
                    params: [name, price],
                    stmt: &mut self.0,
                    extractor: |row| Row { id: row.get(0) },
                    mapper: |it| <Row>::from(it),
                }
            }
            pub fn params<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                params: &'a impl cornucopia_client::async_::Params<'a, Self, RowQuery<'a, C, Row, 2>, C>,
            ) -> RowQuery<'a, C, Row, 2> {
                params.bind(client, self)
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_client::async_::Params<'a, NamedCompactStmt, RowQuery<'a, C, Row, 2>, C>
            for Params<'a>
        {
            fn bind(
                &'a self,
                client: &'a C,
                stmt: &'a mut NamedCompactStmt,
            ) -> RowQuery<'a, C, Row, 2> {
                stmt.bind(client, &self.name, &self.price)
            }
        }
        pub fn named_spaced() -> NamedSpacedStmt {
            NamedSpacedStmt(cornucopia_client::async_::Stmt::new(
                "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
            ))
        }
        pub struct NamedSpacedStmt(cornucopia_client::async_::Stmt);
        impl NamedSpacedStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                name: &'a &'a str,
                price: &'a f64,
            ) -> RowSpaceQuery<'a, C, RowSpace, 2> {
                RowSpaceQuery {
                    client,
                    params: [name, price],
                    stmt: &mut self.0,
                    extractor: |row| RowSpace { id: row.get(0) },
                    mapper: |it| <RowSpace>::from(it),
                }
            }
            pub fn params<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                params: &'a impl cornucopia_client::async_::Params<
                    'a,
                    Self,
                    RowSpaceQuery<'a, C, RowSpace, 2>,
                    C,
                >,
            ) -> RowSpaceQuery<'a, C, RowSpace, 2> {
                params.bind(client, self)
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_client::async_::Params<
                'a,
                NamedSpacedStmt,
                RowSpaceQuery<'a, C, RowSpace, 2>,
                C,
            > for ParamsSpace<'a>
        {
            fn bind(
                &'a self,
                client: &'a C,
                stmt: &'a mut NamedSpacedStmt,
            ) -> RowSpaceQuery<'a, C, RowSpace, 2> {
                stmt.bind(client, &self.name, &self.price)
            }
        }
        pub fn tricky_sql() -> TrickySqlStmt {
            TrickySqlStmt(cornucopia_client::async_::Stmt::new(
                "INSERT INTO syntax (\"trick:y\", price) VALUES ('this is not a bind_param\', $1)",
            ))
        }
        pub struct TrickySqlStmt(cornucopia_client::async_::Stmt);
        impl TrickySqlStmt {
            pub async fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                price: &'a f64,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[price]).await
            }
        }
        pub fn tricky_sql1() -> TrickySql1Stmt {
            TrickySql1Stmt(cornucopia_client::async_::Stmt::new(
                "INSERT INTO syntax (\"trick:y\", price) VALUES ('this is not a :bind_param', $1)",
            ))
        }
        pub struct TrickySql1Stmt(cornucopia_client::async_::Stmt);
        impl TrickySql1Stmt {
            pub async fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                price: &'a f64,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[price]).await
            }
        }
        pub fn tricky_sql2() -> TrickySql2Stmt {
            TrickySql2Stmt(cornucopia_client::async_::Stmt::new("INSERT INTO syntax (\"trick:y\", price) VALUES ('this is not a '':bind_param''', $1)"))
        }
        pub struct TrickySql2Stmt(cornucopia_client::async_::Stmt);
        impl TrickySql2Stmt {
            pub async fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                price: &'a f64,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[price]).await
            }
        }
        pub fn tricky_sql3() -> TrickySql3Stmt {
            TrickySql3Stmt(cornucopia_client::async_::Stmt::new("INSERT INTO syntax (\"trick:y\", price)  VALUES ($$this is not a :bind_param$$, $1)"))
        }
        pub struct TrickySql3Stmt(cornucopia_client::async_::Stmt);
        impl TrickySql3Stmt {
            pub async fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                price: &'a f64,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[price]).await
            }
        }
        pub fn tricky_sql4() -> TrickySql4Stmt {
            TrickySql4Stmt(cornucopia_client::async_::Stmt::new("INSERT INTO syntax (\"trick:y\", price) VALUES ($tag$this is not a :bind_param$tag$, $1)"))
        }
        pub struct TrickySql4Stmt(cornucopia_client::async_::Stmt);
        impl TrickySql4Stmt {
            pub async fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                price: &'a f64,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[price]).await
            }
        }
        pub fn tricky_sql6() -> TrickySql6Stmt {
            TrickySql6Stmt(cornucopia_client::async_::Stmt::new("INSERT INTO syntax (\"trick:y\", price) VALUES (e'this is not a '':bind_param''', $1)"))
        }
        pub struct TrickySql6Stmt(cornucopia_client::async_::Stmt);
        impl TrickySql6Stmt {
            pub async fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                price: &'a f64,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[price]).await
            }
        }
        pub fn tricky_sql7() -> TrickySql7Stmt {
            TrickySql7Stmt(cornucopia_client::async_::Stmt::new("INSERT INTO syntax (\"trick:y\", price) VALUES (E'this is not a \':bind_param\'', $1)"))
        }
        pub struct TrickySql7Stmt(cornucopia_client::async_::Stmt);
        impl TrickySql7Stmt {
            pub async fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                price: &'a f64,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[price]).await
            }
        }
        pub fn tricky_sql8() -> TrickySql8Stmt {
            TrickySql8Stmt(cornucopia_client::async_::Stmt::new("INSERT INTO syntax (\"trick:y\", price) VALUES (e'this is ''not'' a \':bind_param\'', $1)"))
        }
        pub struct TrickySql8Stmt(cornucopia_client::async_::Stmt);
        impl TrickySql8Stmt {
            pub async fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                price: &'a f64,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[price]).await
            }
        }
        pub fn tricky_sql9() -> TrickySql9Stmt {
            TrickySql9Stmt(cornucopia_client::async_::Stmt::new("INSERT INTO syntax (\"trick:y\", price) VALUES (E'this is \'not\' a \':bind_param\'', $1)"))
        }
        pub struct TrickySql9Stmt(cornucopia_client::async_::Stmt);
        impl TrickySql9Stmt {
            pub async fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                price: &'a f64,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[price]).await
            }
        }
        pub fn tricky_sql10() -> TrickySql10Stmt {
            TrickySql10Stmt(cornucopia_client::async_::Stmt::new(
                "INSERT INTO syntax (\"trick:y\", price) VALUES ('this is just a cast'::text, $1)",
            ))
        }
        pub struct TrickySql10Stmt(cornucopia_client::async_::Stmt);
        impl TrickySql10Stmt {
            pub async fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                price: &'a f64,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[price]).await
            }
        }
        pub fn syntax() -> SyntaxStmt {
            SyntaxStmt(cornucopia_client::async_::Stmt::new("SELECT * FROM syntax"))
        }
        pub struct SyntaxStmt(cornucopia_client::async_::Stmt);
        impl SyntaxStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> SyntaxQuery<'a, C, Syntax, 0> {
                SyntaxQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| SyntaxBorrowed {
                        trick_y: row.get(0),
                        price: row.get(1),
                    },
                    mapper: |it| <Syntax>::from(it),
                }
            }
        }
    }
}
