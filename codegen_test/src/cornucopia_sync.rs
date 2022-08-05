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
            pub arr: cornucopia_sync::ArrayIterator<
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
                            &cornucopia_sync::private::Domain(txt),
                            field.type_(),
                            out,
                        ),
                        "json" => postgres_types::ToSql::to_sql(
                            &cornucopia_sync::private::Domain(json),
                            field.type_(),
                            out,
                        ),
                        "nb" => postgres_types::ToSql::to_sql(
                            &cornucopia_sync::private::Domain(nb),
                            field.type_(),
                            out,
                        ),
                        "arr" => postgres_types::ToSql::to_sql(
                            &cornucopia_sync::private::Domain(
                                &cornucopia_sync::private::DomainArray::new(arr),
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
                            "txt" => <cornucopia_sync::private::Domain::<&'a str> as postgres_types::ToSql>::accepts(f.type_()),
"json" => <cornucopia_sync::private::Domain::<&'a serde_json::value::Value> as postgres_types::ToSql>::accepts(f.type_()),
"nb" => <cornucopia_sync::private::Domain::<i32> as postgres_types::ToSql>::accepts(f.type_()),
"arr" => <cornucopia_sync::private::Domain::<cornucopia_sync::private::DomainArray::<&'a serde_json::value::Value, &[&'a serde_json::value::Value]>> as postgres_types::ToSql>::accepts(f.type_()),
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
                cornucopia_sync::ArrayIterator<
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
            pub custom: cornucopia_sync::ArrayIterator<
                'a,
                super::super::types::public::CustomCompositeBorrowed<'a>,
            >,
            pub spongebob:
                cornucopia_sync::ArrayIterator<'a, super::super::types::public::SpongebobCharacter>,
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
                            &cornucopia_sync::private::Domain(domain),
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
"domain" => <cornucopia_sync::private::Domain::<&'a str> as postgres_types::ToSql>::accepts(f.type_()),
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
        use postgres::{fallible_iterator::FallibleIterator, GenericClient};

        pub struct SuperSuperTypesPublicCloneCompositeQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor: fn(&postgres::Row) -> super::super::types::public::CloneCompositeBorrowed,
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

            pub fn one(self) -> Result<T, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                let row = self.client.query_one(stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }

            pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                self.iter()?.collect()
            }

            pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }

            pub fn iter(
                self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt.prepare(self.client)?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }

        pub struct SuperSuperTypesPublicCopyCompositeQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor: fn(&postgres::Row) -> super::super::types::public::CopyComposite,
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

            pub fn one(self) -> Result<T, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                let row = self.client.query_one(stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }

            pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                self.iter()?.collect()
            }

            pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }

            pub fn iter(
                self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt.prepare(self.client)?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }
        pub fn insert_clone() -> InsertCloneStmt {
            InsertCloneStmt(cornucopia_sync::private::Stmt::new(
                "INSERT INTO clone (composite) VALUES ($1)",
            ))
        }
        pub struct InsertCloneStmt(cornucopia_sync::private::Stmt);
        impl InsertCloneStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                composite: &'a super::super::types::public::CloneCompositeBorrowed<'a>,
            ) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(stmt, &[composite])
            }
        }
        pub fn select_clone() -> SelectCloneStmt {
            SelectCloneStmt(cornucopia_sync::private::Stmt::new("SELECT * FROM clone"))
        }
        pub struct SelectCloneStmt(cornucopia_sync::private::Stmt);
        impl SelectCloneStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
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
            InsertCopyStmt(cornucopia_sync::private::Stmt::new(
                "INSERT INTO copy (composite) VALUES ($1)",
            ))
        }
        pub struct InsertCopyStmt(cornucopia_sync::private::Stmt);
        impl InsertCopyStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                composite: &'a super::super::types::public::CopyComposite,
            ) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(stmt, &[composite])
            }
        }
        pub fn select_copy() -> SelectCopyStmt {
            SelectCopyStmt(cornucopia_sync::private::Stmt::new("SELECT * FROM copy"))
        }
        pub struct SelectCopyStmt(cornucopia_sync::private::Stmt);
        impl SelectCopyStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
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
        use postgres::{fallible_iterator::FallibleIterator, GenericClient};
        #[derive(Debug)]
        pub struct InsertNightmareDomainParams<
            'a,
            T1: cornucopia_sync::StringSql,
            T2: cornucopia_sync::ArraySql<&'a serde_json::value::Value>,
        > {
            pub txt: T1,
            pub json: &'a serde_json::value::Value,
            pub nb: i32,
            pub arr: T2,
            pub composite: Option<super::super::types::public::DomainCompositeParams<'a>>,
            pub _i_am_ugly: std::marker::PhantomData<&'a str>,
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
            pub arr: cornucopia_sync::ArrayIterator<
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
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor: fn(&postgres::Row) -> SelectNightmareDomainBorrowed,
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

            pub fn one(self) -> Result<T, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                let row = self.client.query_one(stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }

            pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                self.iter()?.collect()
            }

            pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }

            pub fn iter(
                self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt.prepare(self.client)?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
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
                cornucopia_sync::ArrayIterator<
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
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor: fn(&postgres::Row) -> SelectNightmareDomainNullBorrowed,
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

            pub fn one(self) -> Result<T, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                let row = self.client.query_one(stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }

            pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                self.iter()?.collect()
            }

            pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }

            pub fn iter(
                self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt.prepare(self.client)?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }
        pub fn select_nightmare_domain() -> SelectNightmareDomainStmt {
            SelectNightmareDomainStmt(cornucopia_sync::private::Stmt::new(
                "SELECT txt, json, nb, arr FROM nightmare_domain",
            ))
        }
        pub struct SelectNightmareDomainStmt(cornucopia_sync::private::Stmt);
        impl SelectNightmareDomainStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
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
            InsertNightmareDomainStmt(cornucopia_sync::private::Stmt::new("INSERT INTO nightmare_domain (txt, json, nb, arr, composite) VALUES ($1, $2, $3, $4, $5)"))
        }
        pub struct InsertNightmareDomainStmt(cornucopia_sync::private::Stmt);
        impl InsertNightmareDomainStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_sync::StringSql,
                T2: cornucopia_sync::ArraySql<&'a serde_json::value::Value>,
            >(
                &'a mut self,
                client: &'a mut C,
                txt: &'a T1,
                json: &'a &'a serde_json::value::Value,
                nb: &'a i32,
                arr: &'a T2,
                composite: &'a Option<super::super::types::public::DomainCompositeParams<'a>>,
            ) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(
                    stmt,
                    &[
                        &cornucopia_sync::private::Domain(txt),
                        &cornucopia_sync::private::Domain(json),
                        &cornucopia_sync::private::Domain(nb),
                        &cornucopia_sync::private::Domain(
                            &cornucopia_sync::private::DomainArray::new(arr),
                        ),
                        composite,
                    ],
                )
            }
        }
        impl<'a, C: GenericClient, T1, T2>
            cornucopia_sync::Params<
                'a,
                InsertNightmareDomainParams<'a, T1, T2>,
                Result<u64, postgres::Error>,
                C,
            > for InsertNightmareDomainStmt
        where
            T1: cornucopia_sync::StringSql,
            T2: cornucopia_sync::ArraySql<&'a serde_json::value::Value>,
        {
            fn params(
                &'a mut self,
                client: &'a mut C,
                params: &'a InsertNightmareDomainParams<'a, T1, T2>,
            ) -> Result<u64, postgres::Error> {
                self.bind(
                    client,
                    &params.txt,
                    &params.json,
                    &params.nb,
                    &params.arr,
                    &params.composite,
                )
            }
        }

        pub fn select_nightmare_domain_null() -> SelectNightmareDomainNullStmt {
            SelectNightmareDomainNullStmt(cornucopia_sync::private::Stmt::new(
                "SELECT * FROM nightmare_domain",
            ))
        }
        pub struct SelectNightmareDomainNullStmt(cornucopia_sync::private::Stmt);
        impl SelectNightmareDomainNullStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
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
        use postgres::{fallible_iterator::FallibleIterator, GenericClient};
        #[derive(Debug)]
        pub struct NamedParams<T1: cornucopia_sync::StringSql> {
            pub name: T1,
            pub price: Option<f64>,
        }

        #[derive(Debug)]
        pub struct NamedComplexParams<'a> {
            pub named: super::super::types::public::NamedCompositeBorrowed<'a>,
            pub _i_am_ugly: std::marker::PhantomData<&'a str>,
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq, Copy)]
        pub struct Id {
            pub id: i32,
        }
        pub struct IdQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor: fn(&postgres::Row) -> Id,
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

            pub fn one(self) -> Result<T, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                let row = self.client.query_one(stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }

            pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                self.iter()?.collect()
            }

            pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }

            pub fn iter(
                self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt.prepare(self.client)?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
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
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor: fn(&postgres::Row) -> NamedBorrowed,
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

            pub fn one(self) -> Result<T, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                let row = self.client.query_one(stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }

            pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                self.iter()?.collect()
            }

            pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }

            pub fn iter(
                self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt.prepare(self.client)?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }

        pub struct SuperSuperTypesPublicNamedCompositeQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor: fn(&postgres::Row) -> super::super::types::public::NamedCompositeBorrowed,
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

            pub fn one(self) -> Result<T, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                let row = self.client.query_one(stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }

            pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                self.iter()?.collect()
            }

            pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }

            pub fn iter(
                self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt.prepare(self.client)?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }
        pub fn new_named_visible() -> NewNamedVisibleStmt {
            NewNamedVisibleStmt(cornucopia_sync::private::Stmt::new(
                "INSERT INTO named (name, price, show) VALUES ($1, $2, true) RETURNING id ",
            ))
        }
        pub struct NewNamedVisibleStmt(cornucopia_sync::private::Stmt);
        impl NewNamedVisibleStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_sync::StringSql>(
                &'a mut self,
                client: &'a mut C,
                name: &'a T1,
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
        }
        impl<'a, C: GenericClient, T1>
            cornucopia_sync::Params<'a, NamedParams<T1>, IdQuery<'a, C, Id, 2>, C>
            for NewNamedVisibleStmt
        where
            T1: cornucopia_sync::StringSql,
        {
            fn params(
                &'a mut self,
                client: &'a mut C,
                params: &'a NamedParams<T1>,
            ) -> IdQuery<'a, C, Id, 2> {
                self.bind(client, &params.name, &params.price)
            }
        }
        pub fn new_named_hidden() -> NewNamedHiddenStmt {
            NewNamedHiddenStmt(cornucopia_sync::private::Stmt::new(
                "INSERT INTO named (price, name, show) VALUES ($1, $2, false) RETURNING id",
            ))
        }
        pub struct NewNamedHiddenStmt(cornucopia_sync::private::Stmt);
        impl NewNamedHiddenStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_sync::StringSql>(
                &'a mut self,
                client: &'a mut C,
                price: &'a Option<f64>,
                name: &'a T1,
            ) -> IdQuery<'a, C, Id, 2> {
                IdQuery {
                    client,
                    params: [price, name],
                    stmt: &mut self.0,
                    extractor: |row| Id { id: row.get(0) },
                    mapper: |it| <Id>::from(it),
                }
            }
        }
        impl<'a, C: GenericClient, T1>
            cornucopia_sync::Params<'a, NamedParams<T1>, IdQuery<'a, C, Id, 2>, C>
            for NewNamedHiddenStmt
        where
            T1: cornucopia_sync::StringSql,
        {
            fn params(
                &'a mut self,
                client: &'a mut C,
                params: &'a NamedParams<T1>,
            ) -> IdQuery<'a, C, Id, 2> {
                self.bind(client, &params.price, &params.name)
            }
        }
        pub fn named() -> NamedStmt {
            NamedStmt(cornucopia_sync::private::Stmt::new("SELECT * FROM named"))
        }
        pub struct NamedStmt(cornucopia_sync::private::Stmt);
        impl NamedStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
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
            NamedByIdStmt(cornucopia_sync::private::Stmt::new(
                "SELECT * FROM named WHERE id = $1",
            ))
        }
        pub struct NamedByIdStmt(cornucopia_sync::private::Stmt);
        impl NamedByIdStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
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
            NewNamedComplexStmt(cornucopia_sync::private::Stmt::new(
                "INSERT INTO named_complex (named) VALUES ($1)",
            ))
        }
        pub struct NewNamedComplexStmt(cornucopia_sync::private::Stmt);
        impl NewNamedComplexStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                named: &'a super::super::types::public::NamedCompositeBorrowed<'a>,
            ) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(stmt, &[named])
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_sync::Params<'a, NamedComplexParams<'a>, Result<u64, postgres::Error>, C>
            for NewNamedComplexStmt
        {
            fn params(
                &'a mut self,
                client: &'a mut C,
                params: &'a NamedComplexParams<'a>,
            ) -> Result<u64, postgres::Error> {
                self.bind(client, &params.named)
            }
        }

        pub fn named_complex() -> NamedComplexStmt {
            NamedComplexStmt(cornucopia_sync::private::Stmt::new(
                "SELECT * FROM named_complex",
            ))
        }
        pub struct NamedComplexStmt(cornucopia_sync::private::Stmt);
        impl NamedComplexStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
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
        use postgres::{fallible_iterator::FallibleIterator, GenericClient};
        #[derive(Debug)]
        pub struct NullityParams<
            'a,
            T1: cornucopia_sync::StringSql,
            T2: cornucopia_sync::ArraySql<Option<T1>>,
            T3: cornucopia_sync::StringSql,
        > {
            pub texts: T2,
            pub name: T3,
            pub composite: Option<super::super::types::public::NullityCompositeParams<'a>>,
            pub _i_am_ugly: std::marker::PhantomData<(T1, &'a str)>,
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct Nullity {
            pub texts: Vec<Option<String>>,
            pub name: String,
            pub composite: Option<super::super::types::public::NullityComposite>,
        }
        pub struct NullityBorrowed<'a> {
            pub texts: cornucopia_sync::ArrayIterator<'a, Option<&'a str>>,
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
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor: fn(&postgres::Row) -> NullityBorrowed,
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

            pub fn one(self) -> Result<T, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                let row = self.client.query_one(stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }

            pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                self.iter()?.collect()
            }

            pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }

            pub fn iter(
                self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt.prepare(self.client)?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }
        pub fn new_nullity() -> NewNullityStmt {
            NewNullityStmt(cornucopia_sync::private::Stmt::new(
                "INSERT INTO nullity(texts, name, composite) VALUES ($1, $2, $3)",
            ))
        }
        pub struct NewNullityStmt(cornucopia_sync::private::Stmt);
        impl NewNullityStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_sync::StringSql,
                T2: cornucopia_sync::ArraySql<Option<T1>>,
                T3: cornucopia_sync::StringSql,
            >(
                &'a mut self,
                client: &'a mut C,
                texts: &'a T2,
                name: &'a T3,
                composite: &'a Option<super::super::types::public::NullityCompositeParams<'a>>,
            ) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(stmt, &[texts, name, composite])
            }
        }
        impl<'a, C: GenericClient, T1, T2, T3>
            cornucopia_sync::Params<
                'a,
                NullityParams<'a, T1, T2, T3>,
                Result<u64, postgres::Error>,
                C,
            > for NewNullityStmt
        where
            T1: cornucopia_sync::StringSql,
            T2: cornucopia_sync::ArraySql<Option<T1>>,
            T3: cornucopia_sync::StringSql,
        {
            fn params(
                &'a mut self,
                client: &'a mut C,
                params: &'a NullityParams<'a, T1, T2, T3>,
            ) -> Result<u64, postgres::Error> {
                self.bind(client, &params.texts, &params.name, &params.composite)
            }
        }

        pub fn nullity() -> NullityStmt {
            NullityStmt(cornucopia_sync::private::Stmt::new("SELECT * FROM nullity"))
        }
        pub struct NullityStmt(cornucopia_sync::private::Stmt);
        impl NullityStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
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
        use postgres::{fallible_iterator::FallibleIterator, GenericClient};
        #[derive(Debug)]
        pub struct InsertBookParams<T1: cornucopia_sync::StringSql, T2: cornucopia_sync::StringSql> {
            pub author: Option<T1>,
            pub name: T2,
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
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor: fn(&postgres::Row) -> SelectBookBorrowed,
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

            pub fn one(self) -> Result<T, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                let row = self.client.query_one(stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }

            pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                self.iter()?.collect()
            }

            pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }

            pub fn iter(
                self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt.prepare(self.client)?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct FindBooks {
            pub name: String,
            pub author: Option<String>,
        }
        pub struct FindBooksBorrowed<'a> {
            pub name: &'a str,
            pub author: Option<&'a str>,
        }
        impl<'a> From<FindBooksBorrowed<'a>> for FindBooks {
            fn from(FindBooksBorrowed { name, author }: FindBooksBorrowed<'a>) -> Self {
                Self {
                    name: name.into(),
                    author: author.map(|v| v.into()),
                }
            }
        }
        pub struct FindBooksQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor: fn(&postgres::Row) -> FindBooksBorrowed,
            mapper: fn(FindBooksBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> FindBooksQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(FindBooksBorrowed) -> R) -> FindBooksQuery<'a, C, R, N> {
                FindBooksQuery {
                    client: self.client,
                    params: self.params,
                    stmt: self.stmt,
                    extractor: self.extractor,
                    mapper,
                }
            }

            pub fn one(self) -> Result<T, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                let row = self.client.query_one(stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }

            pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                self.iter()?.collect()
            }

            pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }

            pub fn iter(
                self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt.prepare(self.client)?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }
        pub fn insert_book() -> InsertBookStmt {
            InsertBookStmt(cornucopia_sync::private::Stmt::new(
                "INSERT INTO book (author, name) VALUES ($1, $2)",
            ))
        }
        pub struct InsertBookStmt(cornucopia_sync::private::Stmt);
        impl InsertBookStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_sync::StringSql,
                T2: cornucopia_sync::StringSql,
            >(
                &'a mut self,
                client: &'a mut C,
                author: &'a Option<T1>,
                name: &'a T2,
            ) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(stmt, &[author, name])
            }
        }
        impl<'a, C: GenericClient, T1, T2>
            cornucopia_sync::Params<'a, InsertBookParams<T1, T2>, Result<u64, postgres::Error>, C>
            for InsertBookStmt
        where
            T1: cornucopia_sync::StringSql,
            T2: cornucopia_sync::StringSql,
        {
            fn params(
                &'a mut self,
                client: &'a mut C,
                params: &'a InsertBookParams<T1, T2>,
            ) -> Result<u64, postgres::Error> {
                self.bind(client, &params.author, &params.name)
            }
        }

        pub fn select_book() -> SelectBookStmt {
            SelectBookStmt(cornucopia_sync::private::Stmt::new("SELECT * FROM book"))
        }
        pub struct SelectBookStmt(cornucopia_sync::private::Stmt);
        impl SelectBookStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
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
        pub fn find_books() -> FindBooksStmt {
            FindBooksStmt(cornucopia_sync::private::Stmt::new(
                "SELECT * FROM book WHERE name = ANY ($1)",
            ))
        }
        pub struct FindBooksStmt(cornucopia_sync::private::Stmt);
        impl FindBooksStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_sync::StringSql,
                T2: cornucopia_sync::ArraySql<T1>,
            >(
                &'a mut self,
                client: &'a mut C,
                title: &'a T2,
            ) -> FindBooksQuery<'a, C, FindBooks, 1> {
                FindBooksQuery {
                    client,
                    params: [title],
                    stmt: &mut self.0,
                    extractor: |row| FindBooksBorrowed {
                        name: row.get(0),
                        author: row.get(1),
                    },
                    mapper: |it| <FindBooks>::from(it),
                }
            }
        }
        pub fn params_use_twice() -> ParamsUseTwiceStmt {
            ParamsUseTwiceStmt(cornucopia_sync::private::Stmt::new(
                "UPDATE book SET name = $1 WHERE length(name) > 42 AND length($1) < 42",
            ))
        }
        pub struct ParamsUseTwiceStmt(cornucopia_sync::private::Stmt);
        impl ParamsUseTwiceStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_sync::StringSql>(
                &'a mut self,
                client: &'a mut C,
                name: &'a T1,
            ) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(stmt, &[name])
            }
        }
        pub fn params_order() -> ParamsOrderStmt {
            ParamsOrderStmt(cornucopia_sync::private::Stmt::new(
                "UPDATE imaginary SET c=$1, a=$2, z=$2, r=$1",
            ))
        }
        pub struct ParamsOrderStmt(cornucopia_sync::private::Stmt);
        impl ParamsOrderStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                c: &'a i32,
                a: &'a i32,
            ) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(stmt, &[c, a])
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_sync::Params<'a, ParamsOrderParams, Result<u64, postgres::Error>, C>
            for ParamsOrderStmt
        {
            fn params(
                &'a mut self,
                client: &'a mut C,
                params: &'a ParamsOrderParams,
            ) -> Result<u64, postgres::Error> {
                self.bind(client, &params.c, &params.a)
            }
        }
    }
    pub mod stress {
        use postgres::{fallible_iterator::FallibleIterator, GenericClient};
        #[derive(Debug)]
        pub struct EverythingParams<
            'a,
            T1: cornucopia_sync::StringSql,
            T2: cornucopia_sync::StringSql,
            T3: cornucopia_sync::BytesSql,
        > {
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
            pub text_: T1,
            pub varchar_: T2,
            pub bytea_: T3,
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
            pub _i_am_ugly: std::marker::PhantomData<&'a str>,
        }
        #[derive(Debug)]
        pub struct EverythingArrayParams<
            'a,
            T1: cornucopia_sync::ArraySql<bool>,
            T2: cornucopia_sync::ArraySql<bool>,
            T3: cornucopia_sync::ArraySql<i8>,
            T4: cornucopia_sync::ArraySql<i16>,
            T5: cornucopia_sync::ArraySql<i16>,
            T6: cornucopia_sync::ArraySql<i32>,
            T7: cornucopia_sync::ArraySql<i32>,
            T8: cornucopia_sync::ArraySql<i64>,
            T9: cornucopia_sync::ArraySql<i64>,
            T10: cornucopia_sync::ArraySql<f32>,
            T11: cornucopia_sync::ArraySql<f32>,
            T12: cornucopia_sync::ArraySql<f64>,
            T13: cornucopia_sync::ArraySql<f64>,
            T14: cornucopia_sync::StringSql,
            T15: cornucopia_sync::ArraySql<T14>,
            T16: cornucopia_sync::StringSql,
            T17: cornucopia_sync::ArraySql<T16>,
            T18: cornucopia_sync::BytesSql,
            T19: cornucopia_sync::ArraySql<T18>,
            T20: cornucopia_sync::ArraySql<time::PrimitiveDateTime>,
            T21: cornucopia_sync::ArraySql<time::PrimitiveDateTime>,
            T22: cornucopia_sync::ArraySql<time::OffsetDateTime>,
            T23: cornucopia_sync::ArraySql<time::OffsetDateTime>,
            T24: cornucopia_sync::ArraySql<time::Date>,
            T25: cornucopia_sync::ArraySql<time::Time>,
            T26: cornucopia_sync::ArraySql<&'a serde_json::value::Value>,
            T27: cornucopia_sync::ArraySql<&'a serde_json::value::Value>,
            T28: cornucopia_sync::ArraySql<uuid::Uuid>,
            T29: cornucopia_sync::ArraySql<std::net::IpAddr>,
            T30: cornucopia_sync::ArraySql<eui48::MacAddress>,
        > {
            pub bool_: T1,
            pub boolean_: T2,
            pub char_: T3,
            pub smallint_: T4,
            pub int2_: T5,
            pub int_: T6,
            pub int4_: T7,
            pub bingint_: T8,
            pub int8_: T9,
            pub float4_: T10,
            pub real_: T11,
            pub float8_: T12,
            pub double_precision_: T13,
            pub text_: T15,
            pub varchar_: T17,
            pub bytea_: T19,
            pub timestamp_: T20,
            pub timestamp_without_time_zone_: T21,
            pub timestamptz_: T22,
            pub timestamp_with_time_zone_: T23,
            pub date_: T24,
            pub time_: T25,
            pub json_: T26,
            pub jsonb_: T27,
            pub uuid_: T28,
            pub inet_: T29,
            pub macaddr_: T30,
            pub _i_am_ugly: std::marker::PhantomData<(T14, T16, T18, &'a str)>,
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
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor: fn(&postgres::Row) -> EverythingBorrowed,
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

            pub fn one(self) -> Result<T, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                let row = self.client.query_one(stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }

            pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                self.iter()?.collect()
            }

            pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }

            pub fn iter(
                self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt.prepare(self.client)?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
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
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor: fn(&postgres::Row) -> EverythingNullBorrowed,
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

            pub fn one(self) -> Result<T, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                let row = self.client.query_one(stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }

            pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                self.iter()?.collect()
            }

            pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }

            pub fn iter(
                self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt.prepare(self.client)?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
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
            pub bool_: cornucopia_sync::ArrayIterator<'a, bool>,
            pub boolean_: cornucopia_sync::ArrayIterator<'a, bool>,
            pub char_: cornucopia_sync::ArrayIterator<'a, i8>,
            pub smallint_: cornucopia_sync::ArrayIterator<'a, i16>,
            pub int2_: cornucopia_sync::ArrayIterator<'a, i16>,
            pub int_: cornucopia_sync::ArrayIterator<'a, i32>,
            pub int4_: cornucopia_sync::ArrayIterator<'a, i32>,
            pub bingint_: cornucopia_sync::ArrayIterator<'a, i64>,
            pub int8_: cornucopia_sync::ArrayIterator<'a, i64>,
            pub float4_: cornucopia_sync::ArrayIterator<'a, f32>,
            pub real_: cornucopia_sync::ArrayIterator<'a, f32>,
            pub float8_: cornucopia_sync::ArrayIterator<'a, f64>,
            pub double_precision_: cornucopia_sync::ArrayIterator<'a, f64>,
            pub text_: cornucopia_sync::ArrayIterator<'a, &'a str>,
            pub varchar_: cornucopia_sync::ArrayIterator<'a, &'a str>,
            pub bytea_: cornucopia_sync::ArrayIterator<'a, &'a [u8]>,
            pub timestamp_: cornucopia_sync::ArrayIterator<'a, time::PrimitiveDateTime>,
            pub timestamp_without_time_zone_:
                cornucopia_sync::ArrayIterator<'a, time::PrimitiveDateTime>,
            pub timestamptz_: cornucopia_sync::ArrayIterator<'a, time::OffsetDateTime>,
            pub timestamp_with_time_zone_: cornucopia_sync::ArrayIterator<'a, time::OffsetDateTime>,
            pub date_: cornucopia_sync::ArrayIterator<'a, time::Date>,
            pub time_: cornucopia_sync::ArrayIterator<'a, time::Time>,
            pub json_: cornucopia_sync::ArrayIterator<
                'a,
                postgres_types::Json<&'a serde_json::value::RawValue>,
            >,
            pub jsonb_: cornucopia_sync::ArrayIterator<
                'a,
                postgres_types::Json<&'a serde_json::value::RawValue>,
            >,
            pub uuid_: cornucopia_sync::ArrayIterator<'a, uuid::Uuid>,
            pub inet_: cornucopia_sync::ArrayIterator<'a, std::net::IpAddr>,
            pub macaddr_: cornucopia_sync::ArrayIterator<'a, eui48::MacAddress>,
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
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor: fn(&postgres::Row) -> EverythingArrayBorrowed,
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

            pub fn one(self) -> Result<T, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                let row = self.client.query_one(stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }

            pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                self.iter()?.collect()
            }

            pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }

            pub fn iter(
                self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt.prepare(self.client)?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
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
            pub bool_: Option<cornucopia_sync::ArrayIterator<'a, bool>>,
            pub boolean_: Option<cornucopia_sync::ArrayIterator<'a, bool>>,
            pub char_: Option<cornucopia_sync::ArrayIterator<'a, i8>>,
            pub smallint_: Option<cornucopia_sync::ArrayIterator<'a, i16>>,
            pub int2_: Option<cornucopia_sync::ArrayIterator<'a, i16>>,
            pub int_: Option<cornucopia_sync::ArrayIterator<'a, i32>>,
            pub int4_: Option<cornucopia_sync::ArrayIterator<'a, i32>>,
            pub bingint_: Option<cornucopia_sync::ArrayIterator<'a, i64>>,
            pub int8_: Option<cornucopia_sync::ArrayIterator<'a, i64>>,
            pub float4_: Option<cornucopia_sync::ArrayIterator<'a, f32>>,
            pub real_: Option<cornucopia_sync::ArrayIterator<'a, f32>>,
            pub float8_: Option<cornucopia_sync::ArrayIterator<'a, f64>>,
            pub double_precision_: Option<cornucopia_sync::ArrayIterator<'a, f64>>,
            pub text_: Option<cornucopia_sync::ArrayIterator<'a, &'a str>>,
            pub varchar_: Option<cornucopia_sync::ArrayIterator<'a, &'a str>>,
            pub bytea_: Option<cornucopia_sync::ArrayIterator<'a, &'a [u8]>>,
            pub timestamp_: Option<cornucopia_sync::ArrayIterator<'a, time::PrimitiveDateTime>>,
            pub timestamp_without_time_zone_:
                Option<cornucopia_sync::ArrayIterator<'a, time::PrimitiveDateTime>>,
            pub timestamptz_: Option<cornucopia_sync::ArrayIterator<'a, time::OffsetDateTime>>,
            pub timestamp_with_time_zone_:
                Option<cornucopia_sync::ArrayIterator<'a, time::OffsetDateTime>>,
            pub date_: Option<cornucopia_sync::ArrayIterator<'a, time::Date>>,
            pub time_: Option<cornucopia_sync::ArrayIterator<'a, time::Time>>,
            pub json_: Option<
                cornucopia_sync::ArrayIterator<
                    'a,
                    postgres_types::Json<&'a serde_json::value::RawValue>,
                >,
            >,
            pub jsonb_: Option<
                cornucopia_sync::ArrayIterator<
                    'a,
                    postgres_types::Json<&'a serde_json::value::RawValue>,
                >,
            >,
            pub uuid_: Option<cornucopia_sync::ArrayIterator<'a, uuid::Uuid>>,
            pub inet_: Option<cornucopia_sync::ArrayIterator<'a, std::net::IpAddr>>,
            pub macaddr_: Option<cornucopia_sync::ArrayIterator<'a, eui48::MacAddress>>,
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
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor: fn(&postgres::Row) -> EverythingArrayNullBorrowed,
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

            pub fn one(self) -> Result<T, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                let row = self.client.query_one(stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }

            pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                self.iter()?.collect()
            }

            pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }

            pub fn iter(
                self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt.prepare(self.client)?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }

        pub struct SuperSuperTypesPublicNightmareCompositeQuery<
            'a,
            C: GenericClient,
            T,
            const N: usize,
        > {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor:
                fn(&postgres::Row) -> super::super::types::public::NightmareCompositeBorrowed,
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

            pub fn one(self) -> Result<T, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                let row = self.client.query_one(stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }

            pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                self.iter()?.collect()
            }

            pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }

            pub fn iter(
                self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt.prepare(self.client)?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }
        pub fn select_everything() -> SelectEverythingStmt {
            SelectEverythingStmt(cornucopia_sync::private::Stmt::new(
                "SELECT * FROM Everything",
            ))
        }
        pub struct SelectEverythingStmt(cornucopia_sync::private::Stmt);
        impl SelectEverythingStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
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
            SelectEverythingNullStmt(cornucopia_sync::private::Stmt::new(
                "SELECT * FROM Everything",
            ))
        }
        pub struct SelectEverythingNullStmt(cornucopia_sync::private::Stmt);
        impl SelectEverythingNullStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
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
            InsertEverythingStmt(cornucopia_sync::private::Stmt::new("INSERT INTO Everything (bool_, boolean_, char_, smallint_, int2_, smallserial_, serial2_, int_, int4_, serial_, serial4_, bingint_, int8_, bigserial_, serial8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33)"))
        }
        pub struct InsertEverythingStmt(cornucopia_sync::private::Stmt);
        impl InsertEverythingStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_sync::StringSql,
                T2: cornucopia_sync::StringSql,
                T3: cornucopia_sync::BytesSql,
            >(
                &'a mut self,
                client: &'a mut C,
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
                text_: &'a T1,
                varchar_: &'a T2,
                bytea_: &'a T3,
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
            ) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(
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
            }
        }
        impl<'a, C: GenericClient, T1, T2, T3>
            cornucopia_sync::Params<
                'a,
                EverythingParams<'a, T1, T2, T3>,
                Result<u64, postgres::Error>,
                C,
            > for InsertEverythingStmt
        where
            T1: cornucopia_sync::StringSql,
            T2: cornucopia_sync::StringSql,
            T3: cornucopia_sync::BytesSql,
        {
            fn params(
                &'a mut self,
                client: &'a mut C,
                params: &'a EverythingParams<'a, T1, T2, T3>,
            ) -> Result<u64, postgres::Error> {
                self.bind(
                    client,
                    &params.bool_,
                    &params.boolean_,
                    &params.char_,
                    &params.smallint_,
                    &params.int2_,
                    &params.smallserial_,
                    &params.serial2_,
                    &params.int_,
                    &params.int4_,
                    &params.serial_,
                    &params.serial4_,
                    &params.bingint_,
                    &params.int8_,
                    &params.bigserial_,
                    &params.serial8_,
                    &params.float4_,
                    &params.real_,
                    &params.float8_,
                    &params.double_precision_,
                    &params.text_,
                    &params.varchar_,
                    &params.bytea_,
                    &params.timestamp_,
                    &params.timestamp_without_time_zone_,
                    &params.timestamptz_,
                    &params.timestamp_with_time_zone_,
                    &params.date_,
                    &params.time_,
                    &params.json_,
                    &params.jsonb_,
                    &params.uuid_,
                    &params.inet_,
                    &params.macaddr_,
                )
            }
        }

        pub fn select_everything_array() -> SelectEverythingArrayStmt {
            SelectEverythingArrayStmt(cornucopia_sync::private::Stmt::new(
                "SELECT * FROM EverythingArray",
            ))
        }
        pub struct SelectEverythingArrayStmt(cornucopia_sync::private::Stmt);
        impl SelectEverythingArrayStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
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
            SelectEverythingArrayNullStmt(cornucopia_sync::private::Stmt::new(
                "SELECT * FROM EverythingArray",
            ))
        }
        pub struct SelectEverythingArrayNullStmt(cornucopia_sync::private::Stmt);
        impl SelectEverythingArrayNullStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
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
            InsertEverythingArrayStmt(cornucopia_sync::private::Stmt::new("INSERT INTO EverythingArray (bool_, boolean_, char_, smallint_, int2_, int_, int4_, bingint_, int8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27)"))
        }
        pub struct InsertEverythingArrayStmt(cornucopia_sync::private::Stmt);
        impl InsertEverythingArrayStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_sync::ArraySql<bool>,
                T2: cornucopia_sync::ArraySql<bool>,
                T3: cornucopia_sync::ArraySql<i8>,
                T4: cornucopia_sync::ArraySql<i16>,
                T5: cornucopia_sync::ArraySql<i16>,
                T6: cornucopia_sync::ArraySql<i32>,
                T7: cornucopia_sync::ArraySql<i32>,
                T8: cornucopia_sync::ArraySql<i64>,
                T9: cornucopia_sync::ArraySql<i64>,
                T10: cornucopia_sync::ArraySql<f32>,
                T11: cornucopia_sync::ArraySql<f32>,
                T12: cornucopia_sync::ArraySql<f64>,
                T13: cornucopia_sync::ArraySql<f64>,
                T14: cornucopia_sync::StringSql,
                T15: cornucopia_sync::ArraySql<T14>,
                T16: cornucopia_sync::StringSql,
                T17: cornucopia_sync::ArraySql<T16>,
                T18: cornucopia_sync::BytesSql,
                T19: cornucopia_sync::ArraySql<T18>,
                T20: cornucopia_sync::ArraySql<time::PrimitiveDateTime>,
                T21: cornucopia_sync::ArraySql<time::PrimitiveDateTime>,
                T22: cornucopia_sync::ArraySql<time::OffsetDateTime>,
                T23: cornucopia_sync::ArraySql<time::OffsetDateTime>,
                T24: cornucopia_sync::ArraySql<time::Date>,
                T25: cornucopia_sync::ArraySql<time::Time>,
                T26: cornucopia_sync::ArraySql<&'a serde_json::value::Value>,
                T27: cornucopia_sync::ArraySql<&'a serde_json::value::Value>,
                T28: cornucopia_sync::ArraySql<uuid::Uuid>,
                T29: cornucopia_sync::ArraySql<std::net::IpAddr>,
                T30: cornucopia_sync::ArraySql<eui48::MacAddress>,
            >(
                &'a mut self,
                client: &'a mut C,
                bool_: &'a T1,
                boolean_: &'a T2,
                char_: &'a T3,
                smallint_: &'a T4,
                int2_: &'a T5,
                int_: &'a T6,
                int4_: &'a T7,
                bingint_: &'a T8,
                int8_: &'a T9,
                float4_: &'a T10,
                real_: &'a T11,
                float8_: &'a T12,
                double_precision_: &'a T13,
                text_: &'a T15,
                varchar_: &'a T17,
                bytea_: &'a T19,
                timestamp_: &'a T20,
                timestamp_without_time_zone_: &'a T21,
                timestamptz_: &'a T22,
                timestamp_with_time_zone_: &'a T23,
                date_: &'a T24,
                time_: &'a T25,
                json_: &'a T26,
                jsonb_: &'a T27,
                uuid_: &'a T28,
                inet_: &'a T29,
                macaddr_: &'a T30,
            ) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(
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
            }
        }
        impl<
                'a,
                C: GenericClient,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                T24,
                T25,
                T26,
                T27,
                T28,
                T29,
                T30,
            >
            cornucopia_sync::Params<
                'a,
                EverythingArrayParams<
                    'a,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    T14,
                    T15,
                    T16,
                    T17,
                    T18,
                    T19,
                    T20,
                    T21,
                    T22,
                    T23,
                    T24,
                    T25,
                    T26,
                    T27,
                    T28,
                    T29,
                    T30,
                >,
                Result<u64, postgres::Error>,
                C,
            > for InsertEverythingArrayStmt
        where
            T1: cornucopia_sync::ArraySql<bool>,
            T2: cornucopia_sync::ArraySql<bool>,
            T3: cornucopia_sync::ArraySql<i8>,
            T4: cornucopia_sync::ArraySql<i16>,
            T5: cornucopia_sync::ArraySql<i16>,
            T6: cornucopia_sync::ArraySql<i32>,
            T7: cornucopia_sync::ArraySql<i32>,
            T8: cornucopia_sync::ArraySql<i64>,
            T9: cornucopia_sync::ArraySql<i64>,
            T10: cornucopia_sync::ArraySql<f32>,
            T11: cornucopia_sync::ArraySql<f32>,
            T12: cornucopia_sync::ArraySql<f64>,
            T13: cornucopia_sync::ArraySql<f64>,
            T14: cornucopia_sync::StringSql,
            T15: cornucopia_sync::ArraySql<T14>,
            T16: cornucopia_sync::StringSql,
            T17: cornucopia_sync::ArraySql<T16>,
            T18: cornucopia_sync::BytesSql,
            T19: cornucopia_sync::ArraySql<T18>,
            T20: cornucopia_sync::ArraySql<time::PrimitiveDateTime>,
            T21: cornucopia_sync::ArraySql<time::PrimitiveDateTime>,
            T22: cornucopia_sync::ArraySql<time::OffsetDateTime>,
            T23: cornucopia_sync::ArraySql<time::OffsetDateTime>,
            T24: cornucopia_sync::ArraySql<time::Date>,
            T25: cornucopia_sync::ArraySql<time::Time>,
            T26: cornucopia_sync::ArraySql<&'a serde_json::value::Value>,
            T27: cornucopia_sync::ArraySql<&'a serde_json::value::Value>,
            T28: cornucopia_sync::ArraySql<uuid::Uuid>,
            T29: cornucopia_sync::ArraySql<std::net::IpAddr>,
            T30: cornucopia_sync::ArraySql<eui48::MacAddress>,
        {
            fn params(
                &'a mut self,
                client: &'a mut C,
                params: &'a EverythingArrayParams<
                    'a,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    T14,
                    T15,
                    T16,
                    T17,
                    T18,
                    T19,
                    T20,
                    T21,
                    T22,
                    T23,
                    T24,
                    T25,
                    T26,
                    T27,
                    T28,
                    T29,
                    T30,
                >,
            ) -> Result<u64, postgres::Error> {
                self.bind(
                    client,
                    &params.bool_,
                    &params.boolean_,
                    &params.char_,
                    &params.smallint_,
                    &params.int2_,
                    &params.int_,
                    &params.int4_,
                    &params.bingint_,
                    &params.int8_,
                    &params.float4_,
                    &params.real_,
                    &params.float8_,
                    &params.double_precision_,
                    &params.text_,
                    &params.varchar_,
                    &params.bytea_,
                    &params.timestamp_,
                    &params.timestamp_without_time_zone_,
                    &params.timestamptz_,
                    &params.timestamp_with_time_zone_,
                    &params.date_,
                    &params.time_,
                    &params.json_,
                    &params.jsonb_,
                    &params.uuid_,
                    &params.inet_,
                    &params.macaddr_,
                )
            }
        }

        pub fn select_nightmare() -> SelectNightmareStmt {
            SelectNightmareStmt(cornucopia_sync::private::Stmt::new(
                "SELECT * FROM nightmare",
            ))
        }
        pub struct SelectNightmareStmt(cornucopia_sync::private::Stmt);
        impl SelectNightmareStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
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
            InsertNightmareStmt(cornucopia_sync::private::Stmt::new(
                "INSERT INTO nightmare (composite) VALUES ($1)",
            ))
        }
        pub struct InsertNightmareStmt(cornucopia_sync::private::Stmt);
        impl InsertNightmareStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                composite: &'a super::super::types::public::NightmareCompositeParams<'a>,
            ) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(stmt, &[composite])
            }
        }
    }
    pub mod syntax {
        use postgres::{fallible_iterator::FallibleIterator, GenericClient};
        #[derive(Debug)]
        pub struct ImplicitCompactParams<T1: cornucopia_sync::StringSql> {
            pub name: Option<T1>,
            pub price: Option<f64>,
        }
        #[derive(Debug)]
        pub struct ImplicitSpacedParams<T1: cornucopia_sync::StringSql> {
            pub name: Option<T1>,
            pub price: Option<f64>,
        }
        #[derive(Debug)]
        pub struct Params<T1: cornucopia_sync::StringSql> {
            pub name: T1,
            pub price: f64,
        }
        #[derive(Debug)]
        pub struct ParamsSpace<T1: cornucopia_sync::StringSql> {
            pub name: T1,
            pub price: f64,
        }

        pub struct SuperSuperTypesPublicCloneCompositeQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor: fn(&postgres::Row) -> super::super::types::public::CloneCompositeBorrowed,
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

            pub fn one(self) -> Result<T, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                let row = self.client.query_one(stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }

            pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                self.iter()?.collect()
            }

            pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }

            pub fn iter(
                self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt.prepare(self.client)?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }

        pub struct Optioni32Query<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor: fn(&postgres::Row) -> Option<i32>,
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

            pub fn one(self) -> Result<T, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                let row = self.client.query_one(stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }

            pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                self.iter()?.collect()
            }

            pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }

            pub fn iter(
                self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt.prepare(self.client)?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq, Copy)]
        pub struct Row {
            pub id: i32,
        }
        pub struct RowQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor: fn(&postgres::Row) -> Row,
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

            pub fn one(self) -> Result<T, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                let row = self.client.query_one(stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }

            pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                self.iter()?.collect()
            }

            pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }

            pub fn iter(
                self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt.prepare(self.client)?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq, Copy)]
        pub struct RowSpace {
            pub id: i32,
        }
        pub struct RowSpaceQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor: fn(&postgres::Row) -> RowSpace,
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

            pub fn one(self) -> Result<T, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                let row = self.client.query_one(stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }

            pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                self.iter()?.collect()
            }

            pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }

            pub fn iter(
                self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt.prepare(self.client)?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
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
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_sync::private::Stmt,
            extractor: fn(&postgres::Row) -> SyntaxBorrowed,
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

            pub fn one(self) -> Result<T, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                let row = self.client.query_one(stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }

            pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                self.iter()?.collect()
            }

            pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt.prepare(self.client)?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }

            pub fn iter(
                self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt.prepare(self.client)?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }
        pub fn select_compact() -> SelectCompactStmt {
            SelectCompactStmt(cornucopia_sync::private::Stmt::new("SELECT * FROM clone"))
        }
        pub struct SelectCompactStmt(cornucopia_sync::private::Stmt);
        impl SelectCompactStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
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
            SelectSpacedStmt(cornucopia_sync::private::Stmt::new(
                "      SELECT * FROM clone ",
            ))
        }
        pub struct SelectSpacedStmt(cornucopia_sync::private::Stmt);
        impl SelectSpacedStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
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
            ImplicitCompactStmt(cornucopia_sync::private::Stmt::new(
                "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
            ))
        }
        pub struct ImplicitCompactStmt(cornucopia_sync::private::Stmt);
        impl ImplicitCompactStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_sync::StringSql>(
                &'a mut self,
                client: &'a mut C,
                name: &'a Option<T1>,
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
        }
        impl<'a, C: GenericClient, T1>
            cornucopia_sync::Params<
                'a,
                ImplicitCompactParams<T1>,
                Optioni32Query<'a, C, Option<i32>, 2>,
                C,
            > for ImplicitCompactStmt
        where
            T1: cornucopia_sync::StringSql,
        {
            fn params(
                &'a mut self,
                client: &'a mut C,
                params: &'a ImplicitCompactParams<T1>,
            ) -> Optioni32Query<'a, C, Option<i32>, 2> {
                self.bind(client, &params.name, &params.price)
            }
        }
        pub fn implicit_spaced() -> ImplicitSpacedStmt {
            ImplicitSpacedStmt(cornucopia_sync::private::Stmt::new(
                "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
            ))
        }
        pub struct ImplicitSpacedStmt(cornucopia_sync::private::Stmt);
        impl ImplicitSpacedStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_sync::StringSql>(
                &'a mut self,
                client: &'a mut C,
                name: &'a Option<T1>,
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
        }
        impl<'a, C: GenericClient, T1>
            cornucopia_sync::Params<
                'a,
                ImplicitSpacedParams<T1>,
                Optioni32Query<'a, C, Option<i32>, 2>,
                C,
            > for ImplicitSpacedStmt
        where
            T1: cornucopia_sync::StringSql,
        {
            fn params(
                &'a mut self,
                client: &'a mut C,
                params: &'a ImplicitSpacedParams<T1>,
            ) -> Optioni32Query<'a, C, Option<i32>, 2> {
                self.bind(client, &params.name, &params.price)
            }
        }
        pub fn named_compact() -> NamedCompactStmt {
            NamedCompactStmt(cornucopia_sync::private::Stmt::new(
                "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
            ))
        }
        pub struct NamedCompactStmt(cornucopia_sync::private::Stmt);
        impl NamedCompactStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_sync::StringSql>(
                &'a mut self,
                client: &'a mut C,
                name: &'a T1,
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
        }
        impl<'a, C: GenericClient, T1>
            cornucopia_sync::Params<'a, Params<T1>, RowQuery<'a, C, Row, 2>, C> for NamedCompactStmt
        where
            T1: cornucopia_sync::StringSql,
        {
            fn params(
                &'a mut self,
                client: &'a mut C,
                params: &'a Params<T1>,
            ) -> RowQuery<'a, C, Row, 2> {
                self.bind(client, &params.name, &params.price)
            }
        }
        pub fn named_spaced() -> NamedSpacedStmt {
            NamedSpacedStmt(cornucopia_sync::private::Stmt::new(
                "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
            ))
        }
        pub struct NamedSpacedStmt(cornucopia_sync::private::Stmt);
        impl NamedSpacedStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_sync::StringSql>(
                &'a mut self,
                client: &'a mut C,
                name: &'a T1,
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
        }
        impl<'a, C: GenericClient, T1>
            cornucopia_sync::Params<'a, ParamsSpace<T1>, RowSpaceQuery<'a, C, RowSpace, 2>, C>
            for NamedSpacedStmt
        where
            T1: cornucopia_sync::StringSql,
        {
            fn params(
                &'a mut self,
                client: &'a mut C,
                params: &'a ParamsSpace<T1>,
            ) -> RowSpaceQuery<'a, C, RowSpace, 2> {
                self.bind(client, &params.name, &params.price)
            }
        }
        pub fn tricky_sql() -> TrickySqlStmt {
            TrickySqlStmt(cornucopia_sync::private::Stmt::new(
                "INSERT INTO syntax (\"trick:y\", price) VALUES ('this is not a bind_param\', $1)",
            ))
        }
        pub struct TrickySqlStmt(cornucopia_sync::private::Stmt);
        impl TrickySqlStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                price: &'a f64,
            ) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(stmt, &[price])
            }
        }
        pub fn tricky_sql1() -> TrickySql1Stmt {
            TrickySql1Stmt(cornucopia_sync::private::Stmt::new(
                "INSERT INTO syntax (\"trick:y\", price) VALUES ('this is not a :bind_param', $1)",
            ))
        }
        pub struct TrickySql1Stmt(cornucopia_sync::private::Stmt);
        impl TrickySql1Stmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                price: &'a f64,
            ) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(stmt, &[price])
            }
        }
        pub fn tricky_sql2() -> TrickySql2Stmt {
            TrickySql2Stmt(cornucopia_sync::private::Stmt::new("INSERT INTO syntax (\"trick:y\", price) VALUES ('this is not a '':bind_param''', $1)"))
        }
        pub struct TrickySql2Stmt(cornucopia_sync::private::Stmt);
        impl TrickySql2Stmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                price: &'a f64,
            ) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(stmt, &[price])
            }
        }
        pub fn tricky_sql3() -> TrickySql3Stmt {
            TrickySql3Stmt(cornucopia_sync::private::Stmt::new("INSERT INTO syntax (\"trick:y\", price)  VALUES ($$this is not a :bind_param$$, $1)"))
        }
        pub struct TrickySql3Stmt(cornucopia_sync::private::Stmt);
        impl TrickySql3Stmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                price: &'a f64,
            ) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(stmt, &[price])
            }
        }
        pub fn tricky_sql4() -> TrickySql4Stmt {
            TrickySql4Stmt(cornucopia_sync::private::Stmt::new("INSERT INTO syntax (\"trick:y\", price) VALUES ($tag$this is not a :bind_param$tag$, $1)"))
        }
        pub struct TrickySql4Stmt(cornucopia_sync::private::Stmt);
        impl TrickySql4Stmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                price: &'a f64,
            ) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(stmt, &[price])
            }
        }
        pub fn tricky_sql6() -> TrickySql6Stmt {
            TrickySql6Stmt(cornucopia_sync::private::Stmt::new("INSERT INTO syntax (\"trick:y\", price) VALUES (e'this is not a '':bind_param''', $1)"))
        }
        pub struct TrickySql6Stmt(cornucopia_sync::private::Stmt);
        impl TrickySql6Stmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                price: &'a f64,
            ) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(stmt, &[price])
            }
        }
        pub fn tricky_sql7() -> TrickySql7Stmt {
            TrickySql7Stmt(cornucopia_sync::private::Stmt::new("INSERT INTO syntax (\"trick:y\", price) VALUES (E'this is not a \':bind_param\'', $1)"))
        }
        pub struct TrickySql7Stmt(cornucopia_sync::private::Stmt);
        impl TrickySql7Stmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                price: &'a f64,
            ) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(stmt, &[price])
            }
        }
        pub fn tricky_sql8() -> TrickySql8Stmt {
            TrickySql8Stmt(cornucopia_sync::private::Stmt::new("INSERT INTO syntax (\"trick:y\", price) VALUES (e'this is ''not'' a \':bind_param\'', $1)"))
        }
        pub struct TrickySql8Stmt(cornucopia_sync::private::Stmt);
        impl TrickySql8Stmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                price: &'a f64,
            ) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(stmt, &[price])
            }
        }
        pub fn tricky_sql9() -> TrickySql9Stmt {
            TrickySql9Stmt(cornucopia_sync::private::Stmt::new("INSERT INTO syntax (\"trick:y\", price) VALUES (E'this is \'not\' a \':bind_param\'', $1)"))
        }
        pub struct TrickySql9Stmt(cornucopia_sync::private::Stmt);
        impl TrickySql9Stmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                price: &'a f64,
            ) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(stmt, &[price])
            }
        }
        pub fn tricky_sql10() -> TrickySql10Stmt {
            TrickySql10Stmt(cornucopia_sync::private::Stmt::new(
                "INSERT INTO syntax (\"trick:y\", price) VALUES ('this is just a cast'::text, $1)",
            ))
        }
        pub struct TrickySql10Stmt(cornucopia_sync::private::Stmt);
        impl TrickySql10Stmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                price: &'a f64,
            ) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(stmt, &[price])
            }
        }
        pub fn syntax() -> SyntaxStmt {
            SyntaxStmt(cornucopia_sync::private::Stmt::new("SELECT * FROM syntax"))
        }
        pub struct SyntaxStmt(cornucopia_sync::private::Stmt);
        impl SyntaxStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
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
