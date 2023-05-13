// This file was generated with `cornucopia`. Do not modify.

#[derive(serde::Serialize, Debug, postgres_types :: FromSql, Clone, PartialEq)]
#[postgres(name = "clone_composite")]
pub struct CloneComposite {
    #[postgres(name = "first")]
    pub first: i32,
    #[postgres(name = "second")]
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
    ) -> Result<CloneCompositeBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>> {
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
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
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
                if fields.len() != 2 {
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
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
#[derive(serde::Serialize, Debug, postgres_types :: FromSql, Copy, Clone, PartialEq)]
#[postgres(name = "copy_composite")]
pub struct CopyComposite {
    #[postgres(name = "first")]
    pub first: i32,
    #[postgres(name = "second")]
    pub second: f64,
}
impl<'a> postgres_types::ToSql for CopyComposite {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
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
                if fields.len() != 2 {
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
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
#[derive(serde::Serialize, Debug, postgres_types :: FromSql, Clone, PartialEq)]
#[postgres(name = "domain_composite")]
pub struct DomainComposite {
    #[postgres(name = "txt")]
    pub txt: String,
    #[postgres(name = "json")]
    pub json: serde_json::Value,
    #[postgres(name = "nb")]
    pub nb: i32,
    #[postgres(name = "arr")]
    pub arr: Vec<serde_json::Value>,
}
#[derive(Debug)]
pub struct DomainCompositeBorrowed<'a> {
    pub txt: &'a str,
    pub json: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub nb: i32,
    pub arr:
        crate::client::ArrayIterator<'a, postgres_types::Json<&'a serde_json::value::RawValue>>,
}
impl<'a> From<DomainCompositeBorrowed<'a>> for DomainComposite {
    fn from(DomainCompositeBorrowed { txt, json, nb, arr }: DomainCompositeBorrowed<'a>) -> Self {
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
    ) -> Result<DomainCompositeBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>> {
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
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
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
                "txt" => {
                    postgres_types::ToSql::to_sql(&crate::client::Domain(txt), field.type_(), out)
                }
                "json" => {
                    postgres_types::ToSql::to_sql(&crate::client::Domain(json), field.type_(), out)
                }
                "nb" => {
                    postgres_types::ToSql::to_sql(&crate::client::Domain(nb), field.type_(), out)
                }
                "arr" => postgres_types::ToSql::to_sql(
                    &crate::client::Domain(&crate::client::DomainArray(arr)),
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
                if fields.len() != 4 {
                    return false;
                }
                fields.iter().all(| f | match f.name()
                {
                    "txt" => < crate::client::Domain::<&'a str> as postgres_types ::
                    ToSql > :: accepts(f.type_()),"json" => < crate::client::Domain::<&'a serde_json::value::Value> as postgres_types ::
                    ToSql > :: accepts(f.type_()),"nb" => < crate::client::Domain::<i32> as postgres_types ::
                    ToSql > :: accepts(f.type_()),"arr" => < crate::client::Domain::<crate::client::DomainArray::<&'a serde_json::value::Value, &[&'a serde_json::value::Value]>> as postgres_types ::
                    ToSql > :: accepts(f.type_()),_ => false,
                })
            }
            _ => false,
        }
    }
    fn to_sql_checked(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
#[derive(serde::Serialize, Debug, postgres_types :: FromSql, Clone, PartialEq)]
#[postgres(name = "named_composite")]
pub struct NamedComposite {
    #[postgres(name = "wow")]
    pub wow: Option<String>,
    #[postgres(name = "such_cool")]
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
    ) -> Result<NamedCompositeBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>> {
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
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
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
                if fields.len() != 2 {
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
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
#[derive(serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum EnumWithDot {
    variant_with_dot,
}
impl<'a> postgres_types::ToSql for EnumWithDot {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        buf: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let s = match *self {
            EnumWithDot::variant_with_dot => "variant.with_dot",
        };
        buf.extend_from_slice(s.as_bytes());
        std::result::Result::Ok(postgres_types::IsNull::No)
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "enum.with_dot" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 1 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "variant.with_dot" => true,
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
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
impl<'a> postgres_types::FromSql<'a> for EnumWithDot {
    fn from_sql(
        ty: &postgres_types::Type,
        buf: &'a [u8],
    ) -> Result<EnumWithDot, Box<dyn std::error::Error + Sync + Send>> {
        match std::str::from_utf8(buf)? {
            "variant.with_dot" => Ok(EnumWithDot::variant_with_dot),
            s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
        }
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "enum.with_dot" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 1 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "variant.with_dot" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
}
#[derive(serde::Serialize, Debug, postgres_types :: FromSql, Copy, Clone, PartialEq)]
#[postgres(name = "named_composite.with_dot")]
pub struct NamedCompositeWithDot {
    #[postgres(name = "this.is.inconceivable")]
    pub this_is_inconceivable: Option<EnumWithDot>,
}
impl<'a> postgres_types::ToSql for NamedCompositeWithDot {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let NamedCompositeWithDot {
            this_is_inconceivable,
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
                "this.is.inconceivable" => {
                    postgres_types::ToSql::to_sql(this_is_inconceivable, field.type_(), out)
                }
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
        if ty.name() != "named_composite.with_dot" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Composite(ref fields) => {
                if fields.len() != 1 {
                    return false;
                }
                fields.iter().all(|f| match f.name() {
                    "this.is.inconceivable" => {
                        <EnumWithDot as postgres_types::ToSql>::accepts(f.type_())
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
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
#[derive(serde::Serialize, Debug, postgres_types :: FromSql, Clone, PartialEq)]
#[postgres(name = "nullity_composite")]
pub struct NullityComposite {
    #[postgres(name = "jsons")]
    pub jsons: Option<Vec<Option<serde_json::Value>>>,
    #[postgres(name = "id")]
    pub id: i32,
}
#[derive(Debug)]
pub struct NullityCompositeBorrowed<'a> {
    pub jsons: Option<
        crate::client::ArrayIterator<
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
    ) -> Result<NullityCompositeBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>> {
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
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
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
                if fields.len() != 2 {
                    return false;
                }
                fields.iter().all(|f| match f.name() {
                    "jsons" => {
                        <&'a [&'a serde_json::value::Value] as postgres_types::ToSql>::accepts(
                            f.type_(),
                        )
                    }
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
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
#[derive(serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum SpongebobCharacter {
    Bob,
    Patrick,
    Squidward,
}
impl<'a> postgres_types::ToSql for SpongebobCharacter {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        buf: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let s = match *self {
            SpongebobCharacter::Bob => "Bob",
            SpongebobCharacter::Patrick => "Patrick",
            SpongebobCharacter::Squidward => "Squidward",
        };
        buf.extend_from_slice(s.as_bytes());
        std::result::Result::Ok(postgres_types::IsNull::No)
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "spongebob_character" {
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
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
impl<'a> postgres_types::FromSql<'a> for SpongebobCharacter {
    fn from_sql(
        ty: &postgres_types::Type,
        buf: &'a [u8],
    ) -> Result<SpongebobCharacter, Box<dyn std::error::Error + Sync + Send>> {
        match std::str::from_utf8(buf)? {
            "Bob" => Ok(SpongebobCharacter::Bob),
            "Patrick" => Ok(SpongebobCharacter::Patrick),
            "Squidward" => Ok(SpongebobCharacter::Squidward),
            s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
        }
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "spongebob_character" {
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
#[derive(serde::Serialize, Debug, postgres_types :: FromSql, Clone, PartialEq)]
#[postgres(name = "custom_composite")]
pub struct CustomComposite {
    #[postgres(name = "wow")]
    pub wow: String,
    #[postgres(name = "such_cool")]
    pub such_cool: i32,
    #[postgres(name = "nice")]
    pub nice: SpongebobCharacter,
}
#[derive(Debug)]
pub struct CustomCompositeBorrowed<'a> {
    pub wow: &'a str,
    pub such_cool: i32,
    pub nice: SpongebobCharacter,
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
    ) -> Result<CustomCompositeBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>> {
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
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
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
                if fields.len() != 3 {
                    return false;
                }
                fields.iter().all(|f| match f.name() {
                    "wow" => <&'a str as postgres_types::ToSql>::accepts(f.type_()),
                    "such_cool" => <i32 as postgres_types::ToSql>::accepts(f.type_()),
                    "nice" => <SpongebobCharacter as postgres_types::ToSql>::accepts(f.type_()),
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
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
#[derive(serde::Serialize, Debug, postgres_types :: FromSql, Clone, PartialEq)]
#[postgres(name = "nightmare_composite")]
pub struct NightmareComposite {
    #[postgres(name = "custom")]
    pub custom: Vec<CustomComposite>,
    #[postgres(name = "spongebob")]
    pub spongebob: Vec<SpongebobCharacter>,
    #[postgres(name = "domain")]
    pub domain: String,
}
#[derive(Debug)]
pub struct NightmareCompositeBorrowed<'a> {
    pub custom: crate::client::ArrayIterator<'a, CustomCompositeBorrowed<'a>>,
    pub spongebob: crate::client::ArrayIterator<'a, SpongebobCharacter>,
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
    ) -> Result<NightmareCompositeBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>> {
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
    pub custom: &'a [CustomCompositeBorrowed<'a>],
    pub spongebob: &'a [SpongebobCharacter],
    pub domain: &'a str,
}
impl<'a> postgres_types::ToSql for NightmareCompositeParams<'a> {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
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
                    &crate::client::Domain(domain),
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
                if fields.len() != 3 {
                    return false;
                }
                fields.iter().all(|f| match f.name() {
                    "custom" => {
                        <&'a [CustomCompositeBorrowed<'a>] as postgres_types::ToSql>::accepts(
                            f.type_(),
                        )
                    }
                    "spongebob" => {
                        <&'a [SpongebobCharacter] as postgres_types::ToSql>::accepts(f.type_())
                    }
                    "domain" => <crate::client::Domain<&'a str> as postgres_types::ToSql>::accepts(
                        f.type_(),
                    ),
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
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
#[derive(serde::Serialize, Debug, postgres_types :: FromSql, Copy, Clone, PartialEq)]
#[postgres(name = "syntax_composite")]
pub struct SyntaxComposite {
    #[postgres(name = "async")]
    pub r#async: i32,
}
impl<'a> postgres_types::ToSql for SyntaxComposite {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let SyntaxComposite { r#async } = self;
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
                "async" => postgres_types::ToSql::to_sql(r#async, field.type_(), out),
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
        if ty.name() != "syntax_composite" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Composite(ref fields) => {
                if fields.len() != 1 {
                    return false;
                }
                fields.iter().all(|f| match f.name() {
                    "async" => <i32 as postgres_types::ToSql>::accepts(f.type_()),
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
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
#[derive(serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum SyntaxEnum {
    r#async,
    r#box,
    I_Love_Chocolate,
}
impl<'a> postgres_types::ToSql for SyntaxEnum {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        buf: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let s = match *self {
            SyntaxEnum::r#async => "async",
            SyntaxEnum::r#box => "box",
            SyntaxEnum::I_Love_Chocolate => "I Love Chocolate",
        };
        buf.extend_from_slice(s.as_bytes());
        std::result::Result::Ok(postgres_types::IsNull::No)
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "syntax_enum" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 3 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "async" => true,
                    "box" => true,
                    "I Love Chocolate" => true,
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
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
impl<'a> postgres_types::FromSql<'a> for SyntaxEnum {
    fn from_sql(
        ty: &postgres_types::Type,
        buf: &'a [u8],
    ) -> Result<SyntaxEnum, Box<dyn std::error::Error + Sync + Send>> {
        match std::str::from_utf8(buf)? {
            "async" => Ok(SyntaxEnum::r#async),
            "box" => Ok(SyntaxEnum::r#box),
            "I Love Chocolate" => Ok(SyntaxEnum::I_Love_Chocolate),
            s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
        }
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "syntax_enum" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 3 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "async" => true,
                    "box" => true,
                    "I Love Chocolate" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
}
