#![allow(clippy::all)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
pub mod types {
    pub mod public {
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
                        "wow" => postgres_types::ToSql::to_sql(&self.wow, field.type_(), out),
                        "such_cool" => {
                            postgres_types::ToSql::to_sql(&self.such_cool, field.type_(), out)
                        }
                        "nice" => postgres_types::ToSql::to_sql(&self.nice, field.type_(), out),
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
                        "custom" => postgres_types::ToSql::to_sql(&self.custom, field.type_(), out),
                        "spongebob" => {
                            postgres_types::ToSql::to_sql(&self.spongebob, field.type_(), out)
                        }
                        "domain" => postgres_types::ToSql::to_sql(
                            &cornucopia_client::private::Domain::<&'a str>(&self.domain),
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
                                    <cornucopia_client::private::Domain::<
                                        &'a str,
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
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
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
                        "first" => postgres_types::ToSql::to_sql(&self.first, field.type_(), out),
                        "second" => postgres_types::ToSql::to_sql(&self.second, field.type_(), out),
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
                        "first" => postgres_types::ToSql::to_sql(&self.first, field.type_(), out),
                        "second" => postgres_types::ToSql::to_sql(&self.second, field.type_(), out),
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
    }
}
pub mod queries {
    pub mod copy {
        use postgres::{fallible_iterator::FallibleIterator, GenericClient};
        #[derive(Debug)]
        pub struct InsertCloneParams<'a> {
            pub composite: super::super::types::public::CloneCompositeBorrowed<'a>,
        }
        impl<'a> InsertCloneParams<'a> {
            pub fn insert_clone<C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> Result<u64, postgres::Error> {
                insert_clone(client, &self.composite)
            }
        }
        #[derive(Debug, Clone, Copy)]
        pub struct InsertCopyParams {
            pub composite: super::super::types::public::CopyComposite,
        }
        impl InsertCopyParams {
            pub fn insert_copy<'a, C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> Result<u64, postgres::Error> {
                insert_copy(client, &self.composite)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
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
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            extractor: fn(&postgres::Row) -> SelectCloneBorrowed,
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
            pub fn stmt(&mut self) -> Result<postgres::Statement, postgres::Error> {
                self.client.prepare(self.query)
            }
            pub fn one(mut self) -> Result<T, postgres::Error> {
                let stmt = self.stmt()?;
                let row = self.client.query_one(&stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub fn vec(self) -> Result<Vec<T>, postgres::Error> {
                self.stream()?.collect()
            }
            pub fn opt(mut self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt()?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub fn stream(
                mut self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq, Copy)]
        pub struct SelectCopy {
            pub composite: super::super::types::public::CopyComposite,
        }
        pub struct SelectCopyQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            extractor: fn(&postgres::Row) -> SelectCopy,
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
            pub fn stmt(&mut self) -> Result<postgres::Statement, postgres::Error> {
                self.client.prepare(self.query)
            }
            pub fn one(mut self) -> Result<T, postgres::Error> {
                let stmt = self.stmt()?;
                let row = self.client.query_one(&stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub fn vec(self) -> Result<Vec<T>, postgres::Error> {
                self.stream()?.collect()
            }
            pub fn opt(mut self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt()?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub fn stream(
                mut self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        pub fn insert_clone<'a, C: GenericClient>(
            client: &'a mut C,
            composite: &'a super::super::types::public::CloneCompositeBorrowed<'a>,
        ) -> Result<u64, postgres::Error> {
            let stmt = client.prepare("INSERT INTO clone (composite) VALUES ($1)")?;
            client.execute(&stmt, &[composite])
        }
        pub fn select_clone<'a, C: GenericClient>(
            client: &'a mut C,
        ) -> SelectCloneQuery<'a, C, SelectClone, 0> {
            SelectCloneQuery {
                client,
                params: [],
                query: "SELECT * FROM clone",
                extractor: |row| SelectCloneBorrowed {
                    composite: row.get(0),
                },
                mapper: |it| SelectClone::from(it),
            }
        }
        pub fn insert_copy<'a, C: GenericClient>(
            client: &'a mut C,
            composite: &'a super::super::types::public::CopyComposite,
        ) -> Result<u64, postgres::Error> {
            let stmt = client.prepare("INSERT INTO copy (composite) VALUES ($1)")?;
            client.execute(&stmt, &[composite])
        }
        pub fn select_copy<'a, C: GenericClient>(
            client: &'a mut C,
        ) -> SelectCopyQuery<'a, C, SelectCopy, 0> {
            SelectCopyQuery {
                client,
                params: [],
                query: "SELECT * FROM copy",
                extractor: |row| SelectCopy {
                    composite: row.get(0),
                },
                mapper: |it| SelectCopy::from(it),
            }
        }
    }
    pub mod domain {
        use postgres::{fallible_iterator::FallibleIterator, GenericClient};
        #[derive(Debug)]
        pub struct InsertNightmareDomainParams<'a> {
            pub arr: &'a [postgres_types::Json<&'a serde_json::value::RawValue>],
            pub json: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub nb: i32,
            pub txt: &'a str,
        }
        impl<'a> InsertNightmareDomainParams<'a> {
            pub fn insert_nightmare_domain<C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> Result<u64, postgres::Error> {
                insert_nightmare_domain(client, &self.arr, &self.json, &self.nb, &self.txt)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct SelectNightmareDomain {
            pub arr: Vec<serde_json::Value>,
            pub json: serde_json::Value,
            pub nb: i32,
            pub txt: String,
        }
        pub struct SelectNightmareDomainBorrowed<'a> {
            pub arr: cornucopia_client::ArrayIterator<
                'a,
                postgres_types::Json<&'a serde_json::value::RawValue>,
            >,
            pub json: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub nb: i32,
            pub txt: &'a str,
        }
        impl<'a> From<SelectNightmareDomainBorrowed<'a>> for SelectNightmareDomain {
            fn from(
                SelectNightmareDomainBorrowed { arr, json, nb, txt }: SelectNightmareDomainBorrowed<
                    'a,
                >,
            ) -> Self {
                Self {
                    arr: arr
                        .map(|v| serde_json::from_str(v.0.get()).unwrap())
                        .collect(),
                    json: serde_json::from_str(json.0.get()).unwrap(),
                    nb,
                    txt: txt.into(),
                }
            }
        }
        pub struct SelectNightmareDomainQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
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
                    query: self.query,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub fn stmt(&mut self) -> Result<postgres::Statement, postgres::Error> {
                self.client.prepare(self.query)
            }
            pub fn one(mut self) -> Result<T, postgres::Error> {
                let stmt = self.stmt()?;
                let row = self.client.query_one(&stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub fn vec(self) -> Result<Vec<T>, postgres::Error> {
                self.stream()?.collect()
            }
            pub fn opt(mut self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt()?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub fn stream(
                mut self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct SelectNightmareDomainNull {
            pub arr: Option<Vec<serde_json::Value>>,
            pub json: Option<serde_json::Value>,
            pub nb: Option<i32>,
            pub txt: Option<String>,
        }
        pub struct SelectNightmareDomainNullBorrowed<'a> {
            pub arr: Option<
                cornucopia_client::ArrayIterator<
                    'a,
                    postgres_types::Json<&'a serde_json::value::RawValue>,
                >,
            >,
            pub json: Option<postgres_types::Json<&'a serde_json::value::RawValue>>,
            pub nb: Option<i32>,
            pub txt: Option<&'a str>,
        }
        impl<'a> From<SelectNightmareDomainNullBorrowed<'a>> for SelectNightmareDomainNull {
            fn from(
                SelectNightmareDomainNullBorrowed {
                    arr,
                    json,
                    nb,
                    txt,
                }: SelectNightmareDomainNullBorrowed<'a>,
            ) -> Self {
                Self {
                    arr: arr.map(|v| {
                        v.map(|v| serde_json::from_str(v.0.get()).unwrap())
                            .collect()
                    }),
                    json: json.map(|v| serde_json::from_str(v.0.get()).unwrap()),
                    nb,
                    txt: txt.map(|v| v.into()),
                }
            }
        }
        pub struct SelectNightmareDomainNullQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
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
                    query: self.query,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub fn stmt(&mut self) -> Result<postgres::Statement, postgres::Error> {
                self.client.prepare(self.query)
            }
            pub fn one(mut self) -> Result<T, postgres::Error> {
                let stmt = self.stmt()?;
                let row = self.client.query_one(&stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub fn vec(self) -> Result<Vec<T>, postgres::Error> {
                self.stream()?.collect()
            }
            pub fn opt(mut self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt()?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub fn stream(
                mut self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        pub fn select_nightmare_domain<'a, C: GenericClient>(
            client: &'a mut C,
        ) -> SelectNightmareDomainQuery<'a, C, SelectNightmareDomain, 0> {
            SelectNightmareDomainQuery {
                client,
                params: [],
                query: "SELECT txt, json, nb, arr FROM nightmare_domain",
                extractor: |row| SelectNightmareDomainBorrowed {
                    arr: row.get(3),
                    json: row.get(1),
                    nb: row.get(2),
                    txt: row.get(0),
                },
                mapper: |it| SelectNightmareDomain::from(it),
            }
        }
        pub fn insert_nightmare_domain<'a, C: GenericClient>(
            client: &'a mut C,
            arr: &'a &'a [postgres_types::Json<&'a serde_json::value::RawValue>],
            json: &'a postgres_types::Json<&'a serde_json::value::RawValue>,
            nb: &'a i32,
            txt: &'a &'a str,
        ) -> Result<u64, postgres::Error> {
            let stmt = client.prepare(
                "INSERT INTO nightmare_domain (txt, json, nb, arr) VALUES ($4, $2, $3, $1)",
            )?;
            client.execute(&stmt, &[arr, json, nb, txt])
        }
        pub fn select_nightmare_domain_null<'a, C: GenericClient>(
            client: &'a mut C,
        ) -> SelectNightmareDomainNullQuery<'a, C, SelectNightmareDomainNull, 0> {
            SelectNightmareDomainNullQuery {
                client,
                params: [],
                query: "SELECT txt, json, nb, arr FROM nightmare_domain",
                extractor: |row| SelectNightmareDomainNullBorrowed {
                    arr: row.get(3),
                    json: row.get(1),
                    nb: row.get(2),
                    txt: row.get(0),
                },
                mapper: |it| SelectNightmareDomainNull::from(it),
            }
        }
    }
    pub mod named {
        use postgres::{fallible_iterator::FallibleIterator, GenericClient};
        #[derive(Debug)]
        pub struct ItemParams<'a> {
            pub name: &'a str,
            pub price: Option<f64>,
        }
        impl<'a> ItemParams<'a> {
            pub fn new_item_visible<C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> IdQuery<'a, C, Id, 2> {
                new_item_visible(client, &self.name, &self.price)
            }
            pub fn new_item_hidden<C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> IdQuery<'a, C, Id, 2> {
                new_item_hidden(client, &self.name, &self.price)
            }
        }
        #[derive(Debug, Clone, Copy)]
        pub struct ItemByIdParams {
            pub id: i32,
        }
        impl ItemByIdParams {
            pub fn item_by_id<'a, C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> ItemQuery<'a, C, Item, 1> {
                item_by_id(client, &self.id)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq, Copy)]
        pub struct Id {
            pub id: i32,
        }
        pub struct IdQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
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
                    query: self.query,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub fn stmt(&mut self) -> Result<postgres::Statement, postgres::Error> {
                self.client.prepare(self.query)
            }
            pub fn one(mut self) -> Result<T, postgres::Error> {
                let stmt = self.stmt()?;
                let row = self.client.query_one(&stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub fn vec(self) -> Result<Vec<T>, postgres::Error> {
                self.stream()?.collect()
            }
            pub fn opt(mut self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt()?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub fn stream(
                mut self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct Item {
            pub id: i32,
            pub name: String,
            pub price: Option<f64>,
            pub show: bool,
        }
        pub struct ItemBorrowed<'a> {
            pub id: i32,
            pub name: &'a str,
            pub price: Option<f64>,
            pub show: bool,
        }
        impl<'a> From<ItemBorrowed<'a>> for Item {
            fn from(
                ItemBorrowed {
                    id,
                    name,
                    price,
                    show,
                }: ItemBorrowed<'a>,
            ) -> Self {
                Self {
                    id,
                    name: name.into(),
                    price,
                    show,
                }
            }
        }
        pub struct ItemQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            extractor: fn(&postgres::Row) -> ItemBorrowed,
            mapper: fn(ItemBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> ItemQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(ItemBorrowed) -> R) -> ItemQuery<'a, C, R, N> {
                ItemQuery {
                    client: self.client,
                    params: self.params,
                    query: self.query,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub fn stmt(&mut self) -> Result<postgres::Statement, postgres::Error> {
                self.client.prepare(self.query)
            }
            pub fn one(mut self) -> Result<T, postgres::Error> {
                let stmt = self.stmt()?;
                let row = self.client.query_one(&stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub fn vec(self) -> Result<Vec<T>, postgres::Error> {
                self.stream()?.collect()
            }
            pub fn opt(mut self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt()?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub fn stream(
                mut self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        pub fn new_item_visible<'a, C: GenericClient>(
            client: &'a mut C,
            name: &'a &'a str,
            price: &'a Option<f64>,
        ) -> IdQuery<'a, C, Id, 2> {
            IdQuery {
                client,
                params: [name, price],
                query: "INSERT INTO item (name, price, show) VALUES ($1, $2, true) RETURNING id ",
                extractor: |row| Id { id: row.get(0) },
                mapper: |it| Id::from(it),
            }
        }
        pub fn new_item_hidden<'a, C: GenericClient>(
            client: &'a mut C,
            name: &'a &'a str,
            price: &'a Option<f64>,
        ) -> IdQuery<'a, C, Id, 2> {
            IdQuery {
                client,
                params: [name, price],
                query: "INSERT INTO item (name, price, show) VALUES ($1, $2, false) RETURNING id",
                extractor: |row| Id { id: row.get(0) },
                mapper: |it| Id::from(it),
            }
        }
        pub fn items<'a, C: GenericClient>(client: &'a mut C) -> ItemQuery<'a, C, Item, 0> {
            ItemQuery {
                client,
                params: [],
                query: "SELECT * FROM item",
                extractor: |row| ItemBorrowed {
                    id: row.get(0),
                    name: row.get(1),
                    price: row.get(2),
                    show: row.get(3),
                },
                mapper: |it| Item::from(it),
            }
        }
        pub fn item_by_id<'a, C: GenericClient>(
            client: &'a mut C,
            id: &'a i32,
        ) -> ItemQuery<'a, C, Item, 1> {
            ItemQuery {
                client,
                params: [id],
                query: "SELECT * FROM item WHERE id = $1",
                extractor: |row| ItemBorrowed {
                    id: row.get(0),
                    name: row.get(1),
                    price: row.get(2),
                    show: row.get(3),
                },
                mapper: |it| Item::from(it),
            }
        }
    }
    pub mod params {
        use postgres::{fallible_iterator::FallibleIterator, GenericClient};
        #[derive(Debug)]
        pub struct InsertBookParams<'a> {
            pub author: Option<&'a str>,
            pub name: &'a str,
        }
        impl<'a> InsertBookParams<'a> {
            pub fn insert_book<C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> Result<u64, postgres::Error> {
                insert_book(client, &self.author, &self.name)
            }
        }
        #[derive(Debug)]
        pub struct ParamsUseTwiceParams<'a> {
            pub name: &'a str,
        }
        impl<'a> ParamsUseTwiceParams<'a> {
            pub fn params_use_twice<C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> Result<u64, postgres::Error> {
                params_use_twice(client, &self.name)
            }
        }
        #[derive(Debug, Clone, Copy)]
        pub struct ParamsOrderParams {
            pub a: i32,
            pub c: i32,
        }
        impl ParamsOrderParams {
            pub fn params_order<'a, C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> Result<u64, postgres::Error> {
                params_order(client, &self.a, &self.c)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct SelectBook {
            pub author: Option<String>,
            pub name: String,
        }
        pub struct SelectBookBorrowed<'a> {
            pub author: Option<&'a str>,
            pub name: &'a str,
        }
        impl<'a> From<SelectBookBorrowed<'a>> for SelectBook {
            fn from(SelectBookBorrowed { author, name }: SelectBookBorrowed<'a>) -> Self {
                Self {
                    author: author.map(|v| v.into()),
                    name: name.into(),
                }
            }
        }
        pub struct SelectBookQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
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
                    query: self.query,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub fn stmt(&mut self) -> Result<postgres::Statement, postgres::Error> {
                self.client.prepare(self.query)
            }
            pub fn one(mut self) -> Result<T, postgres::Error> {
                let stmt = self.stmt()?;
                let row = self.client.query_one(&stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub fn vec(self) -> Result<Vec<T>, postgres::Error> {
                self.stream()?.collect()
            }
            pub fn opt(mut self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt()?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub fn stream(
                mut self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        pub fn insert_book<'a, C: GenericClient>(
            client: &'a mut C,
            author: &'a Option<&'a str>,
            name: &'a &'a str,
        ) -> Result<u64, postgres::Error> {
            let stmt = client.prepare("INSERT INTO book (author, name) VALUES ($1, $2)")?;
            client.execute(&stmt, &[author, name])
        }
        pub fn select_book<'a, C: GenericClient>(
            client: &'a mut C,
        ) -> SelectBookQuery<'a, C, SelectBook, 0> {
            SelectBookQuery {
                client,
                params: [],
                query: "SELECT * FROM book",
                extractor: |row| SelectBookBorrowed {
                    author: row.get(1),
                    name: row.get(0),
                },
                mapper: |it| SelectBook::from(it),
            }
        }
        pub fn params_use_twice<'a, C: GenericClient>(
            client: &'a mut C,
            name: &'a &'a str,
        ) -> Result<u64, postgres::Error> {
            let stmt = client
                .prepare("UPDATE book SET name = $1 WHERE length(name) > 42 AND length($1) < 42")?;
            client.execute(&stmt, &[name])
        }
        pub fn params_order<'a, C: GenericClient>(
            client: &'a mut C,
            a: &'a i32,
            c: &'a i32,
        ) -> Result<u64, postgres::Error> {
            let stmt = client.prepare("UPDATE imaginary SET c=$2, a=$1, z=$1, r=$2")?;
            client.execute(&stmt, &[a, c])
        }
    }
    pub mod stress {
        use postgres::{fallible_iterator::FallibleIterator, GenericClient};
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
            pub fn insert_everything<C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> Result<u64, postgres::Error> {
                insert_everything(
                    client,
                    &self.bigserial_,
                    &self.bingint_,
                    &self.bool_,
                    &self.boolean_,
                    &self.bytea_,
                    &self.char_,
                    &self.date_,
                    &self.double_precision_,
                    &self.float4_,
                    &self.float8_,
                    &self.inet_,
                    &self.int2_,
                    &self.int4_,
                    &self.int8_,
                    &self.int_,
                    &self.json_,
                    &self.jsonb_,
                    &self.macaddr_,
                    &self.real_,
                    &self.serial2_,
                    &self.serial4_,
                    &self.serial8_,
                    &self.serial_,
                    &self.smallint_,
                    &self.smallserial_,
                    &self.text_,
                    &self.time_,
                    &self.timestamp_,
                    &self.timestamp_with_time_zone_,
                    &self.timestamp_without_time_zone_,
                    &self.timestamptz_,
                    &self.uuid_,
                    &self.varchar_,
                )
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
            pub fn insert_everything_array<C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> Result<u64, postgres::Error> {
                insert_everything_array(
                    client,
                    &self.bingint_,
                    &self.bool_,
                    &self.boolean_,
                    &self.bytea_,
                    &self.char_,
                    &self.date_,
                    &self.double_precision_,
                    &self.float4_,
                    &self.float8_,
                    &self.inet_,
                    &self.int2_,
                    &self.int4_,
                    &self.int8_,
                    &self.int_,
                    &self.json_,
                    &self.jsonb_,
                    &self.macaddr_,
                    &self.real_,
                    &self.smallint_,
                    &self.text_,
                    &self.time_,
                    &self.timestamp_,
                    &self.timestamp_with_time_zone_,
                    &self.timestamp_without_time_zone_,
                    &self.timestamptz_,
                    &self.uuid_,
                    &self.varchar_,
                )
            }
        }
        #[derive(Debug)]
        pub struct InsertNightmareParams<'a> {
            pub composite: super::super::types::public::NightmareCompositeParams<'a>,
        }
        impl<'a> InsertNightmareParams<'a> {
            pub fn insert_nightmare<C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> Result<u64, postgres::Error> {
                insert_nightmare(client, &self.composite)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
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
            pub json_: serde_json::Value,
            pub jsonb_: serde_json::Value,
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
                    json_: serde_json::from_str(json_.0.get()).unwrap(),
                    jsonb_: serde_json::from_str(jsonb_.0.get()).unwrap(),
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
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            extractor: fn(&postgres::Row) -> SelectEverythingBorrowed,
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
            pub fn stmt(&mut self) -> Result<postgres::Statement, postgres::Error> {
                self.client.prepare(self.query)
            }
            pub fn one(mut self) -> Result<T, postgres::Error> {
                let stmt = self.stmt()?;
                let row = self.client.query_one(&stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub fn vec(self) -> Result<Vec<T>, postgres::Error> {
                self.stream()?.collect()
            }
            pub fn opt(mut self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt()?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub fn stream(
                mut self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct SelectEverythingNull {
            pub bigserial_: Option<i64>,
            pub bingint_: Option<i64>,
            pub bool_: Option<bool>,
            pub boolean_: Option<bool>,
            pub bytea_: Option<Vec<u8>>,
            pub char_: Option<i8>,
            pub date_: Option<time::Date>,
            pub double_precision_: Option<f64>,
            pub float4_: Option<f32>,
            pub float8_: Option<f64>,
            pub inet_: Option<std::net::IpAddr>,
            pub int2_: Option<i16>,
            pub int4_: Option<i32>,
            pub int8_: Option<i64>,
            pub int_: Option<i32>,
            pub json_: Option<serde_json::Value>,
            pub jsonb_: Option<serde_json::Value>,
            pub macaddr_: Option<eui48::MacAddress>,
            pub real_: Option<f32>,
            pub serial2_: Option<i16>,
            pub serial4_: Option<i32>,
            pub serial8_: Option<i64>,
            pub serial_: Option<i32>,
            pub smallint_: Option<i16>,
            pub smallserial_: Option<i16>,
            pub text_: Option<String>,
            pub time_: Option<time::Time>,
            pub timestamp_: Option<time::PrimitiveDateTime>,
            pub timestamp_with_time_zone_: Option<time::OffsetDateTime>,
            pub timestamp_without_time_zone_: Option<time::PrimitiveDateTime>,
            pub timestamptz_: Option<time::OffsetDateTime>,
            pub uuid_: Option<uuid::Uuid>,
            pub varchar_: Option<String>,
        }
        pub struct SelectEverythingNullBorrowed<'a> {
            pub bigserial_: Option<i64>,
            pub bingint_: Option<i64>,
            pub bool_: Option<bool>,
            pub boolean_: Option<bool>,
            pub bytea_: Option<&'a [u8]>,
            pub char_: Option<i8>,
            pub date_: Option<time::Date>,
            pub double_precision_: Option<f64>,
            pub float4_: Option<f32>,
            pub float8_: Option<f64>,
            pub inet_: Option<std::net::IpAddr>,
            pub int2_: Option<i16>,
            pub int4_: Option<i32>,
            pub int8_: Option<i64>,
            pub int_: Option<i32>,
            pub json_: Option<postgres_types::Json<&'a serde_json::value::RawValue>>,
            pub jsonb_: Option<postgres_types::Json<&'a serde_json::value::RawValue>>,
            pub macaddr_: Option<eui48::MacAddress>,
            pub real_: Option<f32>,
            pub serial2_: Option<i16>,
            pub serial4_: Option<i32>,
            pub serial8_: Option<i64>,
            pub serial_: Option<i32>,
            pub smallint_: Option<i16>,
            pub smallserial_: Option<i16>,
            pub text_: Option<&'a str>,
            pub time_: Option<time::Time>,
            pub timestamp_: Option<time::PrimitiveDateTime>,
            pub timestamp_with_time_zone_: Option<time::OffsetDateTime>,
            pub timestamp_without_time_zone_: Option<time::PrimitiveDateTime>,
            pub timestamptz_: Option<time::OffsetDateTime>,
            pub uuid_: Option<uuid::Uuid>,
            pub varchar_: Option<&'a str>,
        }
        impl<'a> From<SelectEverythingNullBorrowed<'a>> for SelectEverythingNull {
            fn from(
                SelectEverythingNullBorrowed {
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
                }: SelectEverythingNullBorrowed<'a>,
            ) -> Self {
                Self {
                    bigserial_,
                    bingint_,
                    bool_,
                    boolean_,
                    bytea_: bytea_.map(|v| v.into()),
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
                    json_: json_.map(|v| serde_json::from_str(v.0.get()).unwrap()),
                    jsonb_: jsonb_.map(|v| serde_json::from_str(v.0.get()).unwrap()),
                    macaddr_,
                    real_,
                    serial2_,
                    serial4_,
                    serial8_,
                    serial_,
                    smallint_,
                    smallserial_,
                    text_: text_.map(|v| v.into()),
                    time_,
                    timestamp_,
                    timestamp_with_time_zone_,
                    timestamp_without_time_zone_,
                    timestamptz_,
                    uuid_,
                    varchar_: varchar_.map(|v| v.into()),
                }
            }
        }
        pub struct SelectEverythingNullQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            extractor: fn(&postgres::Row) -> SelectEverythingNullBorrowed,
            mapper: fn(SelectEverythingNullBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SelectEverythingNullQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(SelectEverythingNullBorrowed) -> R,
            ) -> SelectEverythingNullQuery<'a, C, R, N> {
                SelectEverythingNullQuery {
                    client: self.client,
                    params: self.params,
                    query: self.query,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub fn stmt(&mut self) -> Result<postgres::Statement, postgres::Error> {
                self.client.prepare(self.query)
            }
            pub fn one(mut self) -> Result<T, postgres::Error> {
                let stmt = self.stmt()?;
                let row = self.client.query_one(&stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub fn vec(self) -> Result<Vec<T>, postgres::Error> {
                self.stream()?.collect()
            }
            pub fn opt(mut self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt()?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub fn stream(
                mut self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
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
            pub json_: Vec<serde_json::Value>,
            pub jsonb_: Vec<serde_json::Value>,
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
                    bingint_: bingint_.map(|v| v).collect(),
                    bool_: bool_.map(|v| v).collect(),
                    boolean_: boolean_.map(|v| v).collect(),
                    bytea_: bytea_.map(|v| v.into()).collect(),
                    char_: char_.map(|v| v).collect(),
                    date_: date_.map(|v| v).collect(),
                    double_precision_: double_precision_.map(|v| v).collect(),
                    float4_: float4_.map(|v| v).collect(),
                    float8_: float8_.map(|v| v).collect(),
                    inet_: inet_.map(|v| v).collect(),
                    int2_: int2_.map(|v| v).collect(),
                    int4_: int4_.map(|v| v).collect(),
                    int8_: int8_.map(|v| v).collect(),
                    int_: int_.map(|v| v).collect(),
                    json_: json_
                        .map(|v| serde_json::from_str(v.0.get()).unwrap())
                        .collect(),
                    jsonb_: jsonb_
                        .map(|v| serde_json::from_str(v.0.get()).unwrap())
                        .collect(),
                    macaddr_: macaddr_.map(|v| v).collect(),
                    real_: real_.map(|v| v).collect(),
                    smallint_: smallint_.map(|v| v).collect(),
                    text_: text_.map(|v| v.into()).collect(),
                    time_: time_.map(|v| v).collect(),
                    timestamp_: timestamp_.map(|v| v).collect(),
                    timestamp_with_time_zone_: timestamp_with_time_zone_.map(|v| v).collect(),
                    timestamp_without_time_zone_: timestamp_without_time_zone_.map(|v| v).collect(),
                    timestamptz_: timestamptz_.map(|v| v).collect(),
                    uuid_: uuid_.map(|v| v).collect(),
                    varchar_: varchar_.map(|v| v.into()).collect(),
                }
            }
        }
        pub struct SelectEverythingArrayQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            extractor: fn(&postgres::Row) -> SelectEverythingArrayBorrowed,
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
            pub fn stmt(&mut self) -> Result<postgres::Statement, postgres::Error> {
                self.client.prepare(self.query)
            }
            pub fn one(mut self) -> Result<T, postgres::Error> {
                let stmt = self.stmt()?;
                let row = self.client.query_one(&stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub fn vec(self) -> Result<Vec<T>, postgres::Error> {
                self.stream()?.collect()
            }
            pub fn opt(mut self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt()?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub fn stream(
                mut self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct SelectEverythingArrayNull {
            pub bingint_: Option<Vec<i64>>,
            pub bool_: Option<Vec<bool>>,
            pub boolean_: Option<Vec<bool>>,
            pub bytea_: Option<Vec<Vec<u8>>>,
            pub char_: Option<Vec<i8>>,
            pub date_: Option<Vec<time::Date>>,
            pub double_precision_: Option<Vec<f64>>,
            pub float4_: Option<Vec<f32>>,
            pub float8_: Option<Vec<f64>>,
            pub inet_: Option<Vec<std::net::IpAddr>>,
            pub int2_: Option<Vec<i16>>,
            pub int4_: Option<Vec<i32>>,
            pub int8_: Option<Vec<i64>>,
            pub int_: Option<Vec<i32>>,
            pub json_: Option<Vec<serde_json::Value>>,
            pub jsonb_: Option<Vec<serde_json::Value>>,
            pub macaddr_: Option<Vec<eui48::MacAddress>>,
            pub real_: Option<Vec<f32>>,
            pub smallint_: Option<Vec<i16>>,
            pub text_: Option<Vec<String>>,
            pub time_: Option<Vec<time::Time>>,
            pub timestamp_: Option<Vec<time::PrimitiveDateTime>>,
            pub timestamp_with_time_zone_: Option<Vec<time::OffsetDateTime>>,
            pub timestamp_without_time_zone_: Option<Vec<time::PrimitiveDateTime>>,
            pub timestamptz_: Option<Vec<time::OffsetDateTime>>,
            pub uuid_: Option<Vec<uuid::Uuid>>,
            pub varchar_: Option<Vec<String>>,
        }
        pub struct SelectEverythingArrayNullBorrowed<'a> {
            pub bingint_: Option<cornucopia_client::ArrayIterator<'a, i64>>,
            pub bool_: Option<cornucopia_client::ArrayIterator<'a, bool>>,
            pub boolean_: Option<cornucopia_client::ArrayIterator<'a, bool>>,
            pub bytea_: Option<cornucopia_client::ArrayIterator<'a, &'a [u8]>>,
            pub char_: Option<cornucopia_client::ArrayIterator<'a, i8>>,
            pub date_: Option<cornucopia_client::ArrayIterator<'a, time::Date>>,
            pub double_precision_: Option<cornucopia_client::ArrayIterator<'a, f64>>,
            pub float4_: Option<cornucopia_client::ArrayIterator<'a, f32>>,
            pub float8_: Option<cornucopia_client::ArrayIterator<'a, f64>>,
            pub inet_: Option<cornucopia_client::ArrayIterator<'a, std::net::IpAddr>>,
            pub int2_: Option<cornucopia_client::ArrayIterator<'a, i16>>,
            pub int4_: Option<cornucopia_client::ArrayIterator<'a, i32>>,
            pub int8_: Option<cornucopia_client::ArrayIterator<'a, i64>>,
            pub int_: Option<cornucopia_client::ArrayIterator<'a, i32>>,
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
            pub macaddr_: Option<cornucopia_client::ArrayIterator<'a, eui48::MacAddress>>,
            pub real_: Option<cornucopia_client::ArrayIterator<'a, f32>>,
            pub smallint_: Option<cornucopia_client::ArrayIterator<'a, i16>>,
            pub text_: Option<cornucopia_client::ArrayIterator<'a, &'a str>>,
            pub time_: Option<cornucopia_client::ArrayIterator<'a, time::Time>>,
            pub timestamp_: Option<cornucopia_client::ArrayIterator<'a, time::PrimitiveDateTime>>,
            pub timestamp_with_time_zone_:
                Option<cornucopia_client::ArrayIterator<'a, time::OffsetDateTime>>,
            pub timestamp_without_time_zone_:
                Option<cornucopia_client::ArrayIterator<'a, time::PrimitiveDateTime>>,
            pub timestamptz_: Option<cornucopia_client::ArrayIterator<'a, time::OffsetDateTime>>,
            pub uuid_: Option<cornucopia_client::ArrayIterator<'a, uuid::Uuid>>,
            pub varchar_: Option<cornucopia_client::ArrayIterator<'a, &'a str>>,
        }
        impl<'a> From<SelectEverythingArrayNullBorrowed<'a>> for SelectEverythingArrayNull {
            fn from(
                SelectEverythingArrayNullBorrowed {
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
                }: SelectEverythingArrayNullBorrowed<'a>,
            ) -> Self {
                Self {
                    bingint_: bingint_.map(|v| v.map(|v| v).collect()),
                    bool_: bool_.map(|v| v.map(|v| v).collect()),
                    boolean_: boolean_.map(|v| v.map(|v| v).collect()),
                    bytea_: bytea_.map(|v| v.map(|v| v.into()).collect()),
                    char_: char_.map(|v| v.map(|v| v).collect()),
                    date_: date_.map(|v| v.map(|v| v).collect()),
                    double_precision_: double_precision_.map(|v| v.map(|v| v).collect()),
                    float4_: float4_.map(|v| v.map(|v| v).collect()),
                    float8_: float8_.map(|v| v.map(|v| v).collect()),
                    inet_: inet_.map(|v| v.map(|v| v).collect()),
                    int2_: int2_.map(|v| v.map(|v| v).collect()),
                    int4_: int4_.map(|v| v.map(|v| v).collect()),
                    int8_: int8_.map(|v| v.map(|v| v).collect()),
                    int_: int_.map(|v| v.map(|v| v).collect()),
                    json_: json_.map(|v| {
                        v.map(|v| serde_json::from_str(v.0.get()).unwrap())
                            .collect()
                    }),
                    jsonb_: jsonb_.map(|v| {
                        v.map(|v| serde_json::from_str(v.0.get()).unwrap())
                            .collect()
                    }),
                    macaddr_: macaddr_.map(|v| v.map(|v| v).collect()),
                    real_: real_.map(|v| v.map(|v| v).collect()),
                    smallint_: smallint_.map(|v| v.map(|v| v).collect()),
                    text_: text_.map(|v| v.map(|v| v.into()).collect()),
                    time_: time_.map(|v| v.map(|v| v).collect()),
                    timestamp_: timestamp_.map(|v| v.map(|v| v).collect()),
                    timestamp_with_time_zone_: timestamp_with_time_zone_
                        .map(|v| v.map(|v| v).collect()),
                    timestamp_without_time_zone_: timestamp_without_time_zone_
                        .map(|v| v.map(|v| v).collect()),
                    timestamptz_: timestamptz_.map(|v| v.map(|v| v).collect()),
                    uuid_: uuid_.map(|v| v.map(|v| v).collect()),
                    varchar_: varchar_.map(|v| v.map(|v| v.into()).collect()),
                }
            }
        }
        pub struct SelectEverythingArrayNullQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            extractor: fn(&postgres::Row) -> SelectEverythingArrayNullBorrowed,
            mapper: fn(SelectEverythingArrayNullBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SelectEverythingArrayNullQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(SelectEverythingArrayNullBorrowed) -> R,
            ) -> SelectEverythingArrayNullQuery<'a, C, R, N> {
                SelectEverythingArrayNullQuery {
                    client: self.client,
                    params: self.params,
                    query: self.query,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub fn stmt(&mut self) -> Result<postgres::Statement, postgres::Error> {
                self.client.prepare(self.query)
            }
            pub fn one(mut self) -> Result<T, postgres::Error> {
                let stmt = self.stmt()?;
                let row = self.client.query_one(&stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub fn vec(self) -> Result<Vec<T>, postgres::Error> {
                self.stream()?.collect()
            }
            pub fn opt(mut self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt()?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub fn stream(
                mut self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
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
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            extractor: fn(&postgres::Row) -> SelectNightmareBorrowed,
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
            pub fn stmt(&mut self) -> Result<postgres::Statement, postgres::Error> {
                self.client.prepare(self.query)
            }
            pub fn one(mut self) -> Result<T, postgres::Error> {
                let stmt = self.stmt()?;
                let row = self.client.query_one(&stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub fn vec(self) -> Result<Vec<T>, postgres::Error> {
                self.stream()?.collect()
            }
            pub fn opt(mut self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt()?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub fn stream(
                mut self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        pub fn select_everything<'a, C: GenericClient>(
            client: &'a mut C,
        ) -> SelectEverythingQuery<'a, C, SelectEverything, 0> {
            SelectEverythingQuery {
                client,
                params: [],
                query: "SELECT * FROM Everything",
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
        pub fn select_everything_null<'a, C: GenericClient>(
            client: &'a mut C,
        ) -> SelectEverythingNullQuery<'a, C, SelectEverythingNull, 0> {
            SelectEverythingNullQuery {
                client,
                params: [],
                query: "SELECT * FROM Everything",
                extractor: |row| SelectEverythingNullBorrowed {
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
                mapper: |it| SelectEverythingNull::from(it),
            }
        }
        pub fn insert_everything<'a, C: GenericClient>(
            client: &'a mut C,
            bigserial_: &'a i64,
            bingint_: &'a i64,
            bool_: &'a bool,
            boolean_: &'a bool,
            bytea_: &'a &'a [u8],
            char_: &'a i8,
            date_: &'a time::Date,
            double_precision_: &'a f64,
            float4_: &'a f32,
            float8_: &'a f64,
            inet_: &'a std::net::IpAddr,
            int2_: &'a i16,
            int4_: &'a i32,
            int8_: &'a i64,
            int_: &'a i32,
            json_: &'a postgres_types::Json<&'a serde_json::value::RawValue>,
            jsonb_: &'a postgres_types::Json<&'a serde_json::value::RawValue>,
            macaddr_: &'a eui48::MacAddress,
            real_: &'a f32,
            serial2_: &'a i16,
            serial4_: &'a i32,
            serial8_: &'a i64,
            serial_: &'a i32,
            smallint_: &'a i16,
            smallserial_: &'a i16,
            text_: &'a &'a str,
            time_: &'a time::Time,
            timestamp_: &'a time::PrimitiveDateTime,
            timestamp_with_time_zone_: &'a time::OffsetDateTime,
            timestamp_without_time_zone_: &'a time::PrimitiveDateTime,
            timestamptz_: &'a time::OffsetDateTime,
            uuid_: &'a uuid::Uuid,
            varchar_: &'a &'a str,
        ) -> Result<u64, postgres::Error> {
            let stmt = client
                .prepare(
                    "INSERT INTO Everything (bool_, boolean_, char_, smallint_, int2_, smallserial_, serial2_, int_, int4_, serial_, serial4_, bingint_, int8_, bigserial_, serial8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_)
    VALUES ($3, $4, $6, $24, $12, $25, $20, $15, $13, $23, $21, $2, $14, $1, $22, $9, $19, $10, $8, $26, $33, $5, $28, $30, $31, $29, $7, $27, $16, $17, $32, $11, $18)",
                )?;
            client.execute(
                &stmt,
                &[
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
                ],
            )
        }
        pub fn select_everything_array<'a, C: GenericClient>(
            client: &'a mut C,
        ) -> SelectEverythingArrayQuery<'a, C, SelectEverythingArray, 0> {
            SelectEverythingArrayQuery {
                client,
                params: [],
                query: "SELECT * FROM EverythingArray",
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
        pub fn select_everything_array_null<'a, C: GenericClient>(
            client: &'a mut C,
        ) -> SelectEverythingArrayNullQuery<'a, C, SelectEverythingArrayNull, 0> {
            SelectEverythingArrayNullQuery {
                client,
                params: [],
                query: "SELECT * FROM EverythingArray",
                extractor: |row| SelectEverythingArrayNullBorrowed {
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
                mapper: |it| SelectEverythingArrayNull::from(it),
            }
        }
        pub fn insert_everything_array<'a, C: GenericClient>(
            client: &'a mut C,
            bingint_: &'a &'a [i64],
            bool_: &'a &'a [bool],
            boolean_: &'a &'a [bool],
            bytea_: &'a &'a [&'a [u8]],
            char_: &'a &'a [i8],
            date_: &'a &'a [time::Date],
            double_precision_: &'a &'a [f64],
            float4_: &'a &'a [f32],
            float8_: &'a &'a [f64],
            inet_: &'a &'a [std::net::IpAddr],
            int2_: &'a &'a [i16],
            int4_: &'a &'a [i32],
            int8_: &'a &'a [i64],
            int_: &'a &'a [i32],
            json_: &'a &'a [postgres_types::Json<&'a serde_json::value::RawValue>],
            jsonb_: &'a &'a [postgres_types::Json<&'a serde_json::value::RawValue>],
            macaddr_: &'a &'a [eui48::MacAddress],
            real_: &'a &'a [f32],
            smallint_: &'a &'a [i16],
            text_: &'a &'a [&'a str],
            time_: &'a &'a [time::Time],
            timestamp_: &'a &'a [time::PrimitiveDateTime],
            timestamp_with_time_zone_: &'a &'a [time::OffsetDateTime],
            timestamp_without_time_zone_: &'a &'a [time::PrimitiveDateTime],
            timestamptz_: &'a &'a [time::OffsetDateTime],
            uuid_: &'a &'a [uuid::Uuid],
            varchar_: &'a &'a [&'a str],
        ) -> Result<u64, postgres::Error> {
            let stmt = client
                .prepare(
                    "INSERT INTO EverythingArray (bool_, boolean_, char_, smallint_, int2_, int_, int4_, bingint_, int8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_)
    VALUES ($2, $3, $5, $19, $11, $14, $12, $1, $13, $8, $18, $9, $7, $20, $27, $4, $22, $24, $25, $23, $6, $21, $15, $16, $26, $10, $17)",
                )?;
            client.execute(
                &stmt,
                &[
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
                ],
            )
        }
        pub fn select_nightmare<'a, C: GenericClient>(
            client: &'a mut C,
        ) -> SelectNightmareQuery<'a, C, SelectNightmare, 0> {
            SelectNightmareQuery {
                client,
                params: [],
                query: "SELECT * FROM nightmare",
                extractor: |row| SelectNightmareBorrowed {
                    composite: row.get(0),
                },
                mapper: |it| SelectNightmare::from(it),
            }
        }
        pub fn insert_nightmare<'a, C: GenericClient>(
            client: &'a mut C,
            composite: &'a super::super::types::public::NightmareCompositeParams<'a>,
        ) -> Result<u64, postgres::Error> {
            let stmt = client.prepare("INSERT INTO nightmare (composite) VALUES ($1)")?;
            client.execute(&stmt, &[composite])
        }
    }
    pub mod syntax {
        use postgres::{fallible_iterator::FallibleIterator, GenericClient};
        #[derive(Debug)]
        pub struct ImplicitCompactParams<'a> {
            pub name: Option<&'a str>,
            pub price: Option<f64>,
        }
        impl<'a> ImplicitCompactParams<'a> {
            pub fn implicit_compact<C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> ImplicitCompactQuery<'a, C, ImplicitCompact, 2> {
                implicit_compact(client, &self.name, &self.price)
            }
        }
        #[derive(Debug)]
        pub struct ImplicitSpacedParams<'a> {
            pub name: Option<&'a str>,
            pub price: Option<f64>,
        }
        impl<'a> ImplicitSpacedParams<'a> {
            pub fn implicit_spaced<C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> ImplicitSpacedQuery<'a, C, ImplicitSpaced, 2> {
                implicit_spaced(client, &self.name, &self.price)
            }
        }
        #[derive(Debug)]
        pub struct Params<'a> {
            pub name: &'a str,
            pub price: f64,
        }
        impl<'a> Params<'a> {
            pub fn named_compact<C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> RowQuery<'a, C, Row, 2> {
                named_compact(client, &self.name, &self.price)
            }
            pub fn named_spaced<C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> RowQuery<'a, C, Row, 2> {
                named_spaced(client, &self.name, &self.price)
            }
        }
        #[derive(Debug, Clone, Copy)]
        pub struct TrickySqlParams {
            pub price: f64,
        }
        impl TrickySqlParams {
            pub fn tricky_sql<'a, C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> Result<u64, postgres::Error> {
                tricky_sql(client, &self.price)
            }
        }
        #[derive(Debug, Clone, Copy)]
        pub struct TrickySql1Params {
            pub price: f64,
        }
        impl TrickySql1Params {
            pub fn tricky_sql1<'a, C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> Result<u64, postgres::Error> {
                tricky_sql1(client, &self.price)
            }
        }
        #[derive(Debug, Clone, Copy)]
        pub struct TrickySql2Params {
            pub price: f64,
        }
        impl TrickySql2Params {
            pub fn tricky_sql2<'a, C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> Result<u64, postgres::Error> {
                tricky_sql2(client, &self.price)
            }
        }
        #[derive(Debug, Clone, Copy)]
        pub struct TrickySql3Params {
            pub price: f64,
        }
        impl TrickySql3Params {
            pub fn tricky_sql3<'a, C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> Result<u64, postgres::Error> {
                tricky_sql3(client, &self.price)
            }
        }
        #[derive(Debug, Clone, Copy)]
        pub struct TrickySql4Params {
            pub price: f64,
        }
        impl TrickySql4Params {
            pub fn tricky_sql4<'a, C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> Result<u64, postgres::Error> {
                tricky_sql4(client, &self.price)
            }
        }
        #[derive(Debug, Clone, Copy)]
        pub struct TrickySql6Params {
            pub price: f64,
        }
        impl TrickySql6Params {
            pub fn tricky_sql6<'a, C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> Result<u64, postgres::Error> {
                tricky_sql6(client, &self.price)
            }
        }
        #[derive(Debug, Clone, Copy)]
        pub struct TrickySql7Params {
            pub price: f64,
        }
        impl TrickySql7Params {
            pub fn tricky_sql7<'a, C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> Result<u64, postgres::Error> {
                tricky_sql7(client, &self.price)
            }
        }
        #[derive(Debug, Clone, Copy)]
        pub struct TrickySql8Params {
            pub price: f64,
        }
        impl TrickySql8Params {
            pub fn tricky_sql8<'a, C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> Result<u64, postgres::Error> {
                tricky_sql8(client, &self.price)
            }
        }
        #[derive(Debug, Clone, Copy)]
        pub struct TrickySql9Params {
            pub price: f64,
        }
        impl TrickySql9Params {
            pub fn tricky_sql9<'a, C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> Result<u64, postgres::Error> {
                tricky_sql9(client, &self.price)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct SelectCompact {
            pub composite: super::super::types::public::CloneComposite,
        }
        pub struct SelectCompactBorrowed<'a> {
            pub composite: super::super::types::public::CloneCompositeBorrowed<'a>,
        }
        impl<'a> From<SelectCompactBorrowed<'a>> for SelectCompact {
            fn from(SelectCompactBorrowed { composite }: SelectCompactBorrowed<'a>) -> Self {
                Self {
                    composite: composite.into(),
                }
            }
        }
        pub struct SelectCompactQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            extractor: fn(&postgres::Row) -> SelectCompactBorrowed,
            mapper: fn(SelectCompactBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SelectCompactQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(SelectCompactBorrowed) -> R,
            ) -> SelectCompactQuery<'a, C, R, N> {
                SelectCompactQuery {
                    client: self.client,
                    params: self.params,
                    query: self.query,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub fn stmt(&mut self) -> Result<postgres::Statement, postgres::Error> {
                self.client.prepare(self.query)
            }
            pub fn one(mut self) -> Result<T, postgres::Error> {
                let stmt = self.stmt()?;
                let row = self.client.query_one(&stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub fn vec(self) -> Result<Vec<T>, postgres::Error> {
                self.stream()?.collect()
            }
            pub fn opt(mut self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt()?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub fn stream(
                mut self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct SelectSpaced {
            pub composite: super::super::types::public::CloneComposite,
        }
        pub struct SelectSpacedBorrowed<'a> {
            pub composite: super::super::types::public::CloneCompositeBorrowed<'a>,
        }
        impl<'a> From<SelectSpacedBorrowed<'a>> for SelectSpaced {
            fn from(SelectSpacedBorrowed { composite }: SelectSpacedBorrowed<'a>) -> Self {
                Self {
                    composite: composite.into(),
                }
            }
        }
        pub struct SelectSpacedQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            extractor: fn(&postgres::Row) -> SelectSpacedBorrowed,
            mapper: fn(SelectSpacedBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SelectSpacedQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(SelectSpacedBorrowed) -> R,
            ) -> SelectSpacedQuery<'a, C, R, N> {
                SelectSpacedQuery {
                    client: self.client,
                    params: self.params,
                    query: self.query,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub fn stmt(&mut self) -> Result<postgres::Statement, postgres::Error> {
                self.client.prepare(self.query)
            }
            pub fn one(mut self) -> Result<T, postgres::Error> {
                let stmt = self.stmt()?;
                let row = self.client.query_one(&stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub fn vec(self) -> Result<Vec<T>, postgres::Error> {
                self.stream()?.collect()
            }
            pub fn opt(mut self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt()?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub fn stream(
                mut self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq, Copy)]
        pub struct ImplicitCompact {
            pub id: Option<i32>,
        }
        pub struct ImplicitCompactQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            extractor: fn(&postgres::Row) -> ImplicitCompact,
            mapper: fn(ImplicitCompact) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> ImplicitCompactQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(ImplicitCompact) -> R,
            ) -> ImplicitCompactQuery<'a, C, R, N> {
                ImplicitCompactQuery {
                    client: self.client,
                    params: self.params,
                    query: self.query,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub fn stmt(&mut self) -> Result<postgres::Statement, postgres::Error> {
                self.client.prepare(self.query)
            }
            pub fn one(mut self) -> Result<T, postgres::Error> {
                let stmt = self.stmt()?;
                let row = self.client.query_one(&stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub fn vec(self) -> Result<Vec<T>, postgres::Error> {
                self.stream()?.collect()
            }
            pub fn opt(mut self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt()?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub fn stream(
                mut self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq, Copy)]
        pub struct ImplicitSpaced {
            pub id: Option<i32>,
        }
        pub struct ImplicitSpacedQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            extractor: fn(&postgres::Row) -> ImplicitSpaced,
            mapper: fn(ImplicitSpaced) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> ImplicitSpacedQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(ImplicitSpaced) -> R,
            ) -> ImplicitSpacedQuery<'a, C, R, N> {
                ImplicitSpacedQuery {
                    client: self.client,
                    params: self.params,
                    query: self.query,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub fn stmt(&mut self) -> Result<postgres::Statement, postgres::Error> {
                self.client.prepare(self.query)
            }
            pub fn one(mut self) -> Result<T, postgres::Error> {
                let stmt = self.stmt()?;
                let row = self.client.query_one(&stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub fn vec(self) -> Result<Vec<T>, postgres::Error> {
                self.stream()?.collect()
            }
            pub fn opt(mut self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt()?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub fn stream(
                mut self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq, Copy)]
        pub struct Row {
            pub id: i32,
        }
        pub struct RowQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
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
                    query: self.query,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub fn stmt(&mut self) -> Result<postgres::Statement, postgres::Error> {
                self.client.prepare(self.query)
            }
            pub fn one(mut self) -> Result<T, postgres::Error> {
                let stmt = self.stmt()?;
                let row = self.client.query_one(&stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub fn vec(self) -> Result<Vec<T>, postgres::Error> {
                self.stream()?.collect()
            }
            pub fn opt(mut self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt()?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub fn stream(
                mut self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct Syntax {
            pub price: f64,
            pub trick_y: String,
        }
        pub struct SyntaxBorrowed<'a> {
            pub price: f64,
            pub trick_y: &'a str,
        }
        impl<'a> From<SyntaxBorrowed<'a>> for Syntax {
            fn from(SyntaxBorrowed { price, trick_y }: SyntaxBorrowed<'a>) -> Self {
                Self {
                    price,
                    trick_y: trick_y.into(),
                }
            }
        }
        pub struct SyntaxQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
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
                    query: self.query,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub fn stmt(&mut self) -> Result<postgres::Statement, postgres::Error> {
                self.client.prepare(self.query)
            }
            pub fn one(mut self) -> Result<T, postgres::Error> {
                let stmt = self.stmt()?;
                let row = self.client.query_one(&stmt, &self.params)?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub fn vec(self) -> Result<Vec<T>, postgres::Error> {
                self.stream()?.collect()
            }
            pub fn opt(mut self) -> Result<Option<T>, postgres::Error> {
                let stmt = self.stmt()?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub fn stream(
                mut self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
            {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        pub fn select_compact<'a, C: GenericClient>(
            client: &'a mut C,
        ) -> SelectCompactQuery<'a, C, SelectCompact, 0> {
            SelectCompactQuery {
                client,
                params: [],
                query: "SELECT * FROM clone",
                extractor: |row| SelectCompactBorrowed {
                    composite: row.get(0),
                },
                mapper: |it| SelectCompact::from(it),
            }
        }
        pub fn select_spaced<'a, C: GenericClient>(
            client: &'a mut C,
        ) -> SelectSpacedQuery<'a, C, SelectSpaced, 0> {
            SelectSpacedQuery {
                client,
                params: [],
                query: "      SELECT * FROM clone ",
                extractor: |row| SelectSpacedBorrowed {
                    composite: row.get(0),
                },
                mapper: |it| SelectSpaced::from(it),
            }
        }
        pub fn implicit_compact<'a, C: GenericClient>(
            client: &'a mut C,
            name: &'a Option<&'a str>,
            price: &'a Option<f64>,
        ) -> ImplicitCompactQuery<'a, C, ImplicitCompact, 2> {
            ImplicitCompactQuery {
                client,
                params: [name, price],
                query: "INSERT INTO item (name, price, show) VALUES ($1, $2, false) RETURNING id",
                extractor: |row| ImplicitCompact { id: row.get(0) },
                mapper: |it| ImplicitCompact::from(it),
            }
        }
        pub fn implicit_spaced<'a, C: GenericClient>(
            client: &'a mut C,
            name: &'a Option<&'a str>,
            price: &'a Option<f64>,
        ) -> ImplicitSpacedQuery<'a, C, ImplicitSpaced, 2> {
            ImplicitSpacedQuery {
                client,
                params: [name, price],
                query: "INSERT INTO item (name, price, show) VALUES ($1, $2, false) RETURNING id",
                extractor: |row| ImplicitSpaced { id: row.get(0) },
                mapper: |it| ImplicitSpaced::from(it),
            }
        }
        pub fn named_compact<'a, C: GenericClient>(
            client: &'a mut C,
            name: &'a &'a str,
            price: &'a f64,
        ) -> RowQuery<'a, C, Row, 2> {
            RowQuery {
                client,
                params: [name, price],
                query: "INSERT INTO item (name, price, show) VALUES ($1, $2, false) RETURNING id",
                extractor: |row| Row { id: row.get(0) },
                mapper: |it| Row::from(it),
            }
        }
        pub fn named_spaced<'a, C: GenericClient>(
            client: &'a mut C,
            name: &'a &'a str,
            price: &'a f64,
        ) -> RowQuery<'a, C, Row, 2> {
            RowQuery {
                client,
                params: [name, price],
                query: "INSERT INTO item (name, price, show) VALUES ($1, $2, false) RETURNING id",
                extractor: |row| Row { id: row.get(0) },
                mapper: |it| Row::from(it),
            }
        }
        pub fn tricky_sql<'a, C: GenericClient>(
            client: &'a mut C,
            price: &'a f64,
        ) -> Result<u64, postgres::Error> {
            let stmt = client.prepare(
                "INSERT INTO syntax (\"trick:y\", price) VALUES ('this is not a bind_param\', $1)",
            )?;
            client.execute(&stmt, &[price])
        }
        pub fn tricky_sql1<'a, C: GenericClient>(
            client: &'a mut C,
            price: &'a f64,
        ) -> Result<u64, postgres::Error> {
            let stmt = client.prepare(
                "INSERT INTO syntax (\"trick:y\", price) VALUES ('this is not a :bind_param', $1)",
            )?;
            client.execute(&stmt, &[price])
        }
        pub fn tricky_sql2<'a, C: GenericClient>(
            client: &'a mut C,
            price: &'a f64,
        ) -> Result<u64, postgres::Error> {
            let stmt = client
                .prepare(
                    "INSERT INTO syntax (\"trick:y\", price) VALUES ('this is not a '':bind_param''', $1)",
                )?;
            client.execute(&stmt, &[price])
        }
        pub fn tricky_sql3<'a, C: GenericClient>(
            client: &'a mut C,
            price: &'a f64,
        ) -> Result<u64, postgres::Error> {
            let stmt = client
                .prepare(
                    "INSERT INTO syntax (\"trick:y\", price)  VALUES ($$this is not a :bind_param$$, $1)",
                )?;
            client.execute(&stmt, &[price])
        }
        pub fn tricky_sql4<'a, C: GenericClient>(
            client: &'a mut C,
            price: &'a f64,
        ) -> Result<u64, postgres::Error> {
            let stmt = client
                .prepare(
                    "INSERT INTO syntax (\"trick:y\", price) VALUES ($tag$this is not a :bind_param$tag$, $1)",
                )?;
            client.execute(&stmt, &[price])
        }
        pub fn tricky_sql6<'a, C: GenericClient>(
            client: &'a mut C,
            price: &'a f64,
        ) -> Result<u64, postgres::Error> {
            let stmt = client
                .prepare(
                    "INSERT INTO syntax (\"trick:y\", price) VALUES (e'this is not a '':bind_param''', $1)",
                )?;
            client.execute(&stmt, &[price])
        }
        pub fn tricky_sql7<'a, C: GenericClient>(
            client: &'a mut C,
            price: &'a f64,
        ) -> Result<u64, postgres::Error> {
            let stmt = client
                .prepare(
                    "INSERT INTO syntax (\"trick:y\", price) VALUES (E'this is not a \':bind_param\'', $1)",
                )?;
            client.execute(&stmt, &[price])
        }
        pub fn tricky_sql8<'a, C: GenericClient>(
            client: &'a mut C,
            price: &'a f64,
        ) -> Result<u64, postgres::Error> {
            let stmt = client
                .prepare(
                    "INSERT INTO syntax (\"trick:y\", price) VALUES (e'this is ''not'' a \':bind_param\'', $1)",
                )?;
            client.execute(&stmt, &[price])
        }
        pub fn tricky_sql9<'a, C: GenericClient>(
            client: &'a mut C,
            price: &'a f64,
        ) -> Result<u64, postgres::Error> {
            let stmt = client
                .prepare(
                    "INSERT INTO syntax (\"trick:y\", price) VALUES (E'this is \'not\' a \':bind_param\'', $1)",
                )?;
            client.execute(&stmt, &[price])
        }
        pub fn syntax<'a, C: GenericClient>(client: &'a mut C) -> SyntaxQuery<'a, C, Syntax, 0> {
            SyntaxQuery {
                client,
                params: [],
                query: "SELECT * FROM syntax",
                extractor: |row| SyntaxBorrowed {
                    price: row.get(1),
                    trick_y: row.get(0),
                },
                mapper: |it| Syntax::from(it),
            }
        }
    }
}
