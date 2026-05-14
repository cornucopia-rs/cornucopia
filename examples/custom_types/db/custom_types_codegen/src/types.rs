// This file was generated with `cornucopia`. Do not modify.

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, Hash, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub enum Quality {
    Sr,
    Ssr,
}
impl<'a> postgres_types::ToSql for Quality {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        buf: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let s = match *self {
            Quality::Sr => "SR",
            Quality::Ssr => "SSR",
        };
        buf.extend_from_slice(s.as_bytes());
        std::result::Result::Ok(postgres_types::IsNull::No)
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "quality" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 2 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "SR" => true,
                    "SSR" => true,
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
impl<'a> postgres_types::FromSql<'a> for Quality {
    fn from_sql(
        ty: &postgres_types::Type,
        buf: &'a [u8],
    ) -> Result<Quality, Box<dyn std::error::Error + Sync + Send>> {
        match std::str::from_utf8(buf)? {
            "SR" => Ok(Quality::Sr),
            "SSR" => Ok(Quality::Ssr),
            s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
        }
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "quality" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 2 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "SR" => true,
                    "SSR" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
}
