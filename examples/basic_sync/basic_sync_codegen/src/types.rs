// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, postgres_types::FromSql, Clone, PartialEq)]
#[postgres(name = "voice_actor")]
pub struct VoiceActor {
    #[postgres(name = "name")]
    pub name: String,
    #[postgres(name = "age")]
    pub age: i32,
}
#[derive(Debug)]
pub struct VoiceActorBorrowed<'a> {
    pub name: &'a str,
    pub age: i32,
}
impl<'a> From<VoiceActorBorrowed<'a>> for VoiceActor {
    fn from(VoiceActorBorrowed { name, age }: VoiceActorBorrowed<'a>) -> Self {
        Self {
            name: name.into(),
            age,
        }
    }
}
impl<'a> postgres_types::FromSql<'a> for VoiceActorBorrowed<'a> {
    fn from_sql(
        ty: &postgres_types::Type,
        out: &'a [u8],
    ) -> Result<VoiceActorBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>> {
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
        Ok(VoiceActorBorrowed { name, age })
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        ty.name() == "voice_actor" && ty.schema() == "public"
    }
}
impl<'a> postgres_types::ToSql for VoiceActorBorrowed<'a> {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let VoiceActorBorrowed { name, age } = self;
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
                    if len > i32::MAX as usize {
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
        if ty.name() != "voice_actor" {
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
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
