// This file was generated with `cornucopia`. Do not modify.

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
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
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
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
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
#[derive(Debug, postgres_types::FromSql, Clone, PartialEq)]
#[postgres(name = "voiceactor")]
pub struct Voiceactor {
    #[postgres(name = "name")]
    pub name: String,
    #[postgres(name = "age")]
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
    ) -> Result<VoiceactorBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>> {
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
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
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
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
