use postgres_types::{private::BytesMut, IsNull, ToSql, Type};
use std::{
    error::Error,
    fmt::{Debug, Formatter},
};

pub struct Domain<T: ToSql>(pub T);

impl<T: ToSql + Debug> Debug for Domain<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DomainWrapper").field(&self.0).finish()
    }
}

impl<T: ToSql> ToSql for Domain<T> {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>>
    where
        Self: Sized,
    {
        let ty = match *ty.kind() {
            postgres_types::Kind::Domain(ref ty) => ty,
            _ => unreachable!(),
        };
        postgres_types::ToSql::to_sql(&self.0, ty, out)
    }

    fn accepts(ty: &Type) -> bool
    where
        Self: Sized,
    {
        match *ty.kind() {
            postgres_types::Kind::Domain(ref type_) => <T as postgres_types::ToSql>::accepts(type_),
            _ => false,
        }
    }

    fn to_sql_checked(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}

pub fn slice_iter<'a>(
    s: &'a [&'a (dyn ToSql + Sync)],
) -> impl ExactSizeIterator<Item = &'a dyn ToSql> + 'a {
    s.iter().map(|s| *s as _)
}
