use std::{fmt::Debug, marker::PhantomData};

use async_trait::async_trait;
use deadpool_postgres::{Client, ClientWrapper, Transaction};
use postgres::fallible_iterator::FallibleIterator;
use postgres_protocol::types::{array_from_sql, ArrayValues};
use postgres_types::ToSql;
use serde::{Deserialize, Serialize};
use tokio_postgres::{
    types::{BorrowToSql, FromSql, Kind, Type},
    Client as PgClient, Error, RowStream, Statement, ToStatement, Transaction as PgTransaction,
};

#[async_trait]
pub trait GenericClient {
    async fn prepare(&self, query: &str) -> Result<Statement, Error>;
    async fn execute<T>(
        &self,
        query: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send;
    async fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<tokio_postgres::Row, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send;
    async fn query_opt<T>(
        &self,
        statement: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Option<tokio_postgres::Row>, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send;
    async fn query<T>(
        &self,
        query: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Vec<tokio_postgres::Row>, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send;

    async fn query_raw<T, P, I>(&self, statement: &T, params: I) -> Result<RowStream, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
        P: BorrowToSql,
        I: IntoIterator<Item = P> + Sync + Send,
        I::IntoIter: ExactSizeIterator;
}

#[async_trait]
impl GenericClient for Transaction<'_> {
    async fn prepare(&self, query: &str) -> Result<Statement, Error> {
        Transaction::prepare_cached(self, query).await
    }

    async fn execute<T>(
        &self,
        query: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgTransaction::execute(self, query, params).await
    }

    async fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<tokio_postgres::Row, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgTransaction::query_one(self, statement, params).await
    }

    async fn query_opt<T>(
        &self,
        statement: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Option<tokio_postgres::Row>, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgTransaction::query_opt(self, statement, params).await
    }

    async fn query<T>(
        &self,
        query: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Vec<tokio_postgres::Row>, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgTransaction::query(self, query, params).await
    }

    async fn query_raw<T, P, I>(&self, statement: &T, params: I) -> Result<RowStream, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
        P: BorrowToSql,
        I: IntoIterator<Item = P> + Sync + Send,
        I::IntoIter: ExactSizeIterator,
    {
        PgTransaction::query_raw(self, statement, params).await
    }
}

#[async_trait]
impl GenericClient for PgTransaction<'_> {
    async fn prepare(&self, query: &str) -> Result<Statement, Error> {
        PgTransaction::prepare(self, query).await
    }

    async fn execute<T>(
        &self,
        query: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgTransaction::execute(self, query, params).await
    }

    async fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<tokio_postgres::Row, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgTransaction::query_one(self, statement, params).await
    }

    async fn query_opt<T>(
        &self,
        statement: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Option<tokio_postgres::Row>, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgTransaction::query_opt(self, statement, params).await
    }

    async fn query<T>(
        &self,
        query: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Vec<tokio_postgres::Row>, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgTransaction::query(self, query, params).await
    }

    async fn query_raw<T, P, I>(&self, statement: &T, params: I) -> Result<RowStream, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
        P: BorrowToSql,
        I: IntoIterator<Item = P> + Sync + Send,
        I::IntoIter: ExactSizeIterator,
    {
        PgTransaction::query_raw(self, statement, params).await
    }
}

#[async_trait]
impl GenericClient for Client {
    async fn prepare(&self, query: &str) -> Result<Statement, Error> {
        ClientWrapper::prepare_cached(self, query).await
    }

    async fn execute<T>(
        &self,
        query: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgClient::execute(self, query, params).await
    }

    async fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<tokio_postgres::Row, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgClient::query_one(self, statement, params).await
    }

    async fn query_opt<T>(
        &self,
        statement: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Option<tokio_postgres::Row>, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgClient::query_opt(self, statement, params).await
    }

    async fn query<T>(
        &self,
        query: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Vec<tokio_postgres::Row>, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgClient::query(self, query, params).await
    }

    async fn query_raw<T, P, I>(&self, statement: &T, params: I) -> Result<RowStream, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
        P: BorrowToSql,
        I: IntoIterator<Item = P> + Sync + Send,
        I::IntoIter: ExactSizeIterator,
    {
        PgClient::query_raw(self, statement, params).await
    }
}

#[async_trait]
impl GenericClient for PgClient {
    async fn prepare(&self, query: &str) -> Result<Statement, Error> {
        PgClient::prepare(self, query).await
    }

    async fn execute<T>(
        &self,
        query: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgClient::execute(self, query, params).await
    }

    async fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<tokio_postgres::Row, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgClient::query_one(self, statement, params).await
    }

    async fn query_opt<T>(
        &self,
        statement: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Option<tokio_postgres::Row>, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgClient::query_opt(self, statement, params).await
    }

    async fn query<T>(
        &self,
        query: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Vec<tokio_postgres::Row>, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgClient::query(self, query, params).await
    }

    async fn query_raw<T, P, I>(&self, statement: &T, params: I) -> Result<RowStream, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
        P: BorrowToSql,
        I: IntoIterator<Item = P> + Sync + Send,
        I::IntoIter: ExactSizeIterator,
    {
        PgClient::query_raw(self, statement, params).await
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Json<T: Debug>(postgres_types::Json<T>);
impl<'a, T: Debug + Serialize + Deserialize<'a>> From<T> for Json<T> {
    fn from(v: T) -> Self {
        Json(postgres_types::Json(v))
    }
}
impl<'a, T: Debug + Serialize + Deserialize<'a>> Serialize for Json<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value().serialize(serializer)
    }
}
impl<'a, T: Debug + Serialize + Deserialize<'a>> Json<T> {
    pub fn value(&self) -> &T {
        &(self.0).0
    }
}
impl<'a, T: Debug + Serialize + Deserialize<'a>> ToSql for Json<T> {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        self.0.to_sql(ty, out)
    }

    fn accepts(ty: &Type) -> bool
    where
        Self: Sized,
    {
        <postgres_types::Json<T> as ToSql>::accepts(ty)
    }

    fn to_sql_checked(
        &self,
        ty: &Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.0.to_sql_checked(ty, out)
    }
}

impl<'a, T: Debug + Serialize + Deserialize<'a>> FromSql<'a> for Json<T> {
    fn from_sql(
        ty: &Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::Json::from_sql(ty, raw).map(Self)
    }

    fn accepts(ty: &Type) -> bool {
        <postgres_types::Json<T> as FromSql>::accepts(ty)
    }
}

pub struct ArrayIterator<'a, T: FromSql<'a>> {
    values: ArrayValues<'a>,
    ty: Type,
    _type: PhantomData<T>,
}

impl<'a, T: FromSql<'a>> Debug for ArrayIterator<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArrayIterator")
            .field("values", &"[T]")
            .field("ty", &self.ty)
            .field("_type", &self._type)
            .finish()
    }
}

impl<'a, T: FromSql<'a>> Iterator for ArrayIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.values
            .next()
            .unwrap()
            .map(|raw| T::from_sql_nullable(&self.ty, raw).unwrap())
    }
}

impl<'a, T: FromSql<'a>> FromSql<'a> for ArrayIterator<'a, T> {
    fn from_sql(
        ty: &Type,
        raw: &'a [u8],
    ) -> Result<ArrayIterator<'a, T>, Box<dyn std::error::Error + Sync + Send>> {
        let member_type = match *ty.kind() {
            Kind::Array(ref member) => member,
            _ => panic!("expected array type"),
        };

        let array = array_from_sql(raw)?;
        if array.dimensions().count()? > 1 {
            return Err("array contains too many dimensions".into());
        }

        Ok(ArrayIterator {
            ty: member_type.to_owned(),
            values: array.values(),
            _type: PhantomData::default(),
        })
    }

    fn accepts(ty: &Type) -> bool {
        match *ty.kind() {
            Kind::Array(ref inner) => T::accepts(inner),
            _ => false,
        }
    }
}

#[doc(hidden)]
pub mod private {
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
        fn to_sql(
            &self,
            ty: &Type,
            out: &mut BytesMut,
        ) -> Result<IsNull, Box<dyn Error + Sync + Send>>
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
                postgres_types::Kind::Domain(ref type_) => {
                    <T as postgres_types::ToSql>::accepts(type_)
                }
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
}
