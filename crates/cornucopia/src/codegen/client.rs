use codegen_template::code;
use std::fmt::Write;

use crate::CodegenSettings;

use super::{vfs::Vfs, DependencyAnalysis, WARNING};

pub(crate) fn gen_lib(dependency_analysis: &DependencyAnalysis) -> String {
    let mut w = code!($WARNING
        #[allow(clippy::all, clippy::pedantic)]
        #[allow(unused_variables)]
        #[allow(unused_imports)]
        #[allow(dead_code)]
        pub mod types;
        #[allow(clippy::all, clippy::pedantic)]
        #[allow(unused_variables)]
        #[allow(unused_imports)]
        #[allow(dead_code)]
        pub mod queries;
        pub mod client;

        mod array_iterator;
        mod domain;
        mod type_traits;
        mod utils;

        pub (crate) use utils::slice_iter;

        pub use array_iterator::ArrayIterator;
        pub use domain::{Domain, DomainArray};
        pub use type_traits::{ArraySql, BytesSql, IterSql, StringSql};
    );
    if dependency_analysis.json {
        code!(w => pub use type_traits::JsonSql; )
    }

    w
}

pub(crate) fn gen_clients(
    vfs: &mut Vfs,
    dependency_analysis: &DependencyAnalysis,
    settings: &CodegenSettings,
) {
    // Generate common files
    vfs.add("src/utils.rs", core_utils());
    vfs.add("src/domain.rs", core_domain());
    vfs.add("src/array_iterator.rs", core_array());
    vfs.add("src/type_traits.rs", core_type_traits(dependency_analysis));
    if settings.gen_sync {
        vfs.add("src/client/sync.rs", sync());
    }
    if settings.gen_async {
        vfs.add("src/client/async_.rs", async_());
        vfs.add(
            "src/client/async_/generic_client.rs",
            async_generic_client(),
        );
        vfs.add("src/client/async_/deadpool.rs", async_deadpool());
    }
    vfs.add("src/client.rs", client(settings))
}

pub fn client(settings: &CodegenSettings) -> String {
    match (settings.gen_async, settings.gen_sync) {
        (true, false) => code!(
            pub(crate) mod async_;
            pub use async_::*;
        ),
        (false, true) => code!(
            pub(crate) mod sync;
            pub use sync::*;
        ),
        _ => code!(
            pub mod sync;
            pub mod async_;
        ),
    }
}

pub fn core_utils() -> String {
    code!($WARNING
        use postgres_types::{Kind, ToSql, Type};

        pub fn escape_domain(ty: &Type) -> &Type {
            match ty.kind() {
                Kind::Domain(ty) => ty,
                _ => ty,
            }
        }

        pub fn slice_iter<'a>(
            s: &'a [&'a (dyn ToSql + Sync)],
        ) -> impl ExactSizeIterator<Item = &'a dyn ToSql> + 'a {
            s.iter().map(|s| *s as _)
        }
    )
}

pub fn core_domain() -> String {
    code!($WARNING
        use postgres_protocol::types::{array_to_sql, ArrayDimension};
        use postgres_types::{private::BytesMut, IsNull, Kind, ToSql, Type};
        use std::{
            error::Error,
            fmt::{Debug, Formatter},
        };

        use super::{type_traits::ArraySql, utils::escape_domain};

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
                postgres_types::ToSql::to_sql(&self.0, escape_domain(ty), out)
            }

            fn accepts(ty: &Type) -> bool
            where
                Self: Sized,
            {
                return T::accepts(escape_domain(ty));
            }

            fn to_sql_checked(
                &self,
                ty: &Type,
                out: &mut BytesMut,
            ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }

        pub struct DomainArray<'a, T: ToSql + Sync, A: ArraySql<Item = T>>(pub &'a A);

        impl<'a, T: ToSql + Sync, A: ArraySql<Item = T>> Debug for DomainArray<'a, T, A> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple("ArrayDomain").field(&self.0).finish()
            }
        }

        impl<'a, T: ToSql + Sync + 'a, A: ArraySql<Item = T>> ToSql for DomainArray<'a, T, A> {
            fn to_sql(
                &self,
                ty: &Type,
                w: &mut BytesMut,
            ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
                self.0.escape_domain_to_sql(ty, w)
            }

            fn accepts(ty: &Type) -> bool {
                match *ty.kind() {
                    Kind::Array(ref member) => T::accepts(escape_domain(member)),
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

        pub fn escape_domain_to_sql<T: ToSql>(
            ty: &Type,
            w: &mut BytesMut,
            iter: impl Iterator<Item = T> + ExactSizeIterator,
        ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
            let member_type = match *ty.kind() {
                Kind::Array(ref member) => escape_domain(member),
                _ => panic!("expected array type got {ty}"),
            };

            let dimension = ArrayDimension {
                len: downcast(iter.len())?,
                lower_bound: 1,
            };

            array_to_sql(
                Some(dimension),
                member_type.oid(),
                iter,
                |e, w| match Domain(e).to_sql(member_type, w)? {
                    IsNull::No => Ok(postgres_protocol::IsNull::No),
                    IsNull::Yes => Ok(postgres_protocol::IsNull::Yes),
                },
                w,
            )?;
            Ok(IsNull::No)
        }

        fn downcast(len: usize) -> Result<i32, Box<dyn Error + Sync + Send>> {
            if len > i32::max_value() as usize {
                Err("value too large to transmit".into())
            } else {
                Ok(len as i32)
            }
        }
    )
}

pub fn core_array() -> String {
    code!($WARNING
        use fallible_iterator::FallibleIterator;
        use postgres_protocol::types::{array_from_sql, ArrayValues};
        use postgres_types::{FromSql, Kind, Type};
        use std::fmt::Debug;
        use std::marker::PhantomData;

        use super::utils::escape_domain;

        /// Iterator over the items in a PostgreSQL array. You only need this if you are
        /// working with custom zero-cost type mapping of rows containing PostgreSQL arrays.
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
            ) -> Result<ArrayIterator<'a, T>, Box<dyn std::error::Error + Sync + Send>>
            {
                let member_type = match *escape_domain(ty).kind() {
                    Kind::Array(ref member) => escape_domain(member),
                    _ => panic!("expected array type got {ty}"),
                };

                let array = array_from_sql(raw)?;
                if array.dimensions().count()? > 1 {
                    return Err("array contains too many dimensions".into());
                }

                Ok(ArrayIterator {
                    ty: member_type.clone(),
                    values: array.values(),
                    _type: PhantomData,
                })
            }

            fn accepts(ty: &Type) -> bool {
                match *ty.kind() {
                    Kind::Array(ref inner) => T::accepts(escape_domain(inner)),
                    _ => false,
                }
            }
        }
    )
}

pub fn core_type_traits(dependency_analysis: &DependencyAnalysis) -> String {
    let mut w = code!($WARNING
        use std::borrow::Cow;

        use super::domain::escape_domain_to_sql;
        use postgres_protocol::types::{self, ArrayDimension};
        use postgres_types::{private::BytesMut, to_sql_checked, IsNull, Kind, ToSql, Type};

        pub trait StringSql: std::fmt::Debug + ToSql + Sync {}
        impl<T: StringSql> StringSql for &T {}
        impl StringSql for String {}
        impl StringSql for &str {}
        impl StringSql for Cow<'_, str> {}
        impl StringSql for Box<str> {}

        pub trait BytesSql: std::fmt::Debug + ToSql + Send + Sync {}
        impl<T: BytesSql> BytesSql for &T {}
        impl BytesSql for Vec<u8> {}
        impl BytesSql for &[u8] {}
    );
    if dependency_analysis.json {
        code!(w =>
            pub trait JsonSql: std::fmt::Debug + ToSql + Sync + Send {}
            impl<T: JsonSql> JsonSql for &T {}
            impl JsonSql for serde_json::value::Value {}
            impl<T: serde::ser::Serialize + std::fmt::Debug + Sync + Send> JsonSql for postgres_types::Json<T> {}
        );
    }
    code!(w =>
        pub trait ArraySql: std::fmt::Debug + ToSql + Send + Sync {
            type Item;
            fn escape_domain_to_sql(
                &self,
                ty: &Type,
                w: &mut BytesMut,
            ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>>;
        }
        impl<T: std::fmt::Debug + ToSql + Sync, A: ArraySql<Item = T>> ArraySql for &A {
            type Item = T;

            fn escape_domain_to_sql(
                &self,
                ty: &Type,
                w: &mut BytesMut,
            ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
                A::escape_domain_to_sql(self, ty, w)
            }
        }
        impl<T: std::fmt::Debug + ToSql + Send + Sync> ArraySql for Vec<T> {
            type Item = T;

            fn escape_domain_to_sql(
                &self,
                ty: &Type,
                w: &mut BytesMut,
            ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
                escape_domain_to_sql(ty, w, self.iter())
            }
        }

        impl<T: std::fmt::Debug + ToSql + Sync> ArraySql for &[T] {
            type Item = T;

            fn escape_domain_to_sql(
                &self,
                ty: &Type,
                w: &mut BytesMut,
            ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
                escape_domain_to_sql(ty, w, self.iter())
            }
        }

        impl<
                T: std::fmt::Debug + ToSql + Send + Sync,
                I: Iterator<Item = T> + ExactSizeIterator,
                F: Fn() -> I + Send + Sync,
            > ArraySql for IterSql<T, I, F>
        {
            type Item = T;

            fn escape_domain_to_sql(
                &self,
                ty: &Type,
                w: &mut BytesMut,
            ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
                escape_domain_to_sql(ty, w, (self.0)())
            }
        }

        pub struct IterSql<T: ToSql, I: Iterator<Item = T> + ExactSizeIterator, F: Fn() -> I + Sync>(pub F);

        impl<T: ToSql, I: Iterator<Item = T> + ExactSizeIterator, F: Fn() -> I + Sync> std::fmt::Debug
            for IterSql<T, I, F>
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple("ArrayFn").finish()
            }
        }

        // Taken from `postgres`
        impl<T: ToSql, I: Iterator<Item = T> + ExactSizeIterator, F: Fn() -> I + Sync> ToSql
            for IterSql<T, I, F>
        {
            fn to_sql(
                &self,
                ty: &Type,
                w: &mut BytesMut,
            ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
                let member_type = match *ty.kind() {
                    Kind::Array(ref member) => member,
                    _ => panic!("expected array type"),
                };

                let iter = (self.0)();

                let dimension = ArrayDimension {
                    len: downcast(iter.len())?,
                    lower_bound: 1,
                };

                types::array_to_sql(
                    Some(dimension),
                    member_type.oid(),
                    iter,
                    |e, w| match e.to_sql(member_type, w)? {
                        IsNull::No => Ok(postgres_protocol::IsNull::No),
                        IsNull::Yes => Ok(postgres_protocol::IsNull::Yes),
                    },
                    w,
                )?;
                Ok(IsNull::No)
            }

            fn accepts(ty: &Type) -> bool {
                match *ty.kind() {
                    Kind::Array(ref member) => T::accepts(member),
                    _ => false,
                }
            }

            to_sql_checked!();
        }

        // https://github.com/sfackler/rust-postgres/blob/765395f288861209a644c621bf72172acd482515/postgres-types/src/lib.rs
        fn downcast(len: usize) -> Result<i32, Box<dyn std::error::Error + Sync + Send>> {
            if len > i32::max_value() as usize {
                Err("value too large to transmit".into())
            } else {
                Ok(len as i32)
            }
        }
    );
    w
}

pub fn sync() -> String {
    code!($WARNING
        use postgres::Statement;

        /// This trait allows you to bind parameters to a query using a single
        /// struct, rather than passing each bind parameter as a function parameter.
        pub trait Params<'a, P, O, C> {
            fn params(&'a mut self, client: &'a mut C, params: &'a P) -> O;
        }

        /// Cached statement
        #[doc(hidden)]
        pub(crate) struct Stmt {
            query: &'static str,
            cached: Option<Statement>,
        }

        impl Stmt {
            #[must_use]
            pub fn new(query: &'static str) -> Self {
                Self {
                    query,
                    cached: None,
                }
            }

            pub fn prepare<'a, C: postgres::GenericClient>(
                &'a mut self,
                client: &mut C,
            ) -> Result<&'a Statement, postgres::Error> {
                if self.cached.is_none() {
                    let stmt = client.prepare(self.query)?;
                    self.cached = Some(stmt);
                }
                // the statement is always prepared at this point
                Ok(unsafe { self.cached.as_ref().unwrap_unchecked() })
            }
        }
    )
}

pub fn async_() -> String {
    code!($WARNING
        pub use generic_client::GenericClient;
        use tokio_postgres::{Error, Statement};

        #[cfg(feature = "deadpool")]
        mod deadpool;
        mod generic_client;

        /// This trait allows you to bind parameters to a query using a single
        /// struct, rather than passing each bind parameter as a function parameter.
        pub trait Params<'a, P, O, C> {
            fn params(&'a mut self, client: &'a C, params: &'a P) -> O;
        }

        /// Cached statement
        #[doc(hidden)]
        pub struct Stmt {
            query: &'static str,
            cached: Option<Statement>,
        }

        impl Stmt {
            #[must_use]
            pub fn new(query: &'static str) -> Self {
                Self {
                    query,
                    cached: None,
                }
            }

            pub async fn prepare<'a, C: GenericClient>(
                &'a mut self,
                client: &C,
            ) -> Result<&'a Statement, Error> {
                if self.cached.is_none() {
                    let stmt = client.prepare(self.query).await?;
                    self.cached = Some(stmt);
                }
                // the statement is always prepared at this point
                Ok(unsafe { self.cached.as_ref().unwrap_unchecked() })
            }
        }
    )
}

pub fn async_generic_client() -> String {
    code!($WARNING
        use async_trait::async_trait;
        use tokio_postgres::{
            types::BorrowToSql, Client, Error, RowStream, Statement, ToStatement, Transaction,
        };

        /// Abstraction over multiple types of asynchronous clients.
        /// This allows you to use tokio_postgres clients and transactions interchangeably.
        ///
        /// In addition, when the `deadpool` feature is enabled (default), this trait also
        /// abstracts over deadpool clients and transactions
        #[async_trait]
        pub trait GenericClient: Send + Sync {
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

            async fn query_raw<T, P, I>(
                &self,
                statement: &T,
                params: I,
            ) -> Result<RowStream, Error>
            where
                T: ?Sized + ToStatement + Sync + Send,
                P: BorrowToSql,
                I: IntoIterator<Item = P> + Sync + Send,
                I::IntoIter: ExactSizeIterator;
        }

        #[async_trait]
        impl GenericClient for Transaction<'_> {
            async fn prepare(&self, query: &str) -> Result<Statement, Error> {
                Transaction::prepare(self, query).await
            }

            async fn execute<T>(
                &self,
                query: &T,
                params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
            ) -> Result<u64, Error>
            where
                T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
            {
                Transaction::execute(self, query, params).await
            }

            async fn query_one<T>(
                &self,
                statement: &T,
                params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
            ) -> Result<tokio_postgres::Row, Error>
            where
                T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
            {
                Transaction::query_one(self, statement, params).await
            }

            async fn query_opt<T>(
                &self,
                statement: &T,
                params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
            ) -> Result<Option<tokio_postgres::Row>, Error>
            where
                T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
            {
                Transaction::query_opt(self, statement, params).await
            }

            async fn query<T>(
                &self,
                query: &T,
                params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
            ) -> Result<Vec<tokio_postgres::Row>, Error>
            where
                T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
            {
                Transaction::query(self, query, params).await
            }

            async fn query_raw<T, P, I>(&self, statement: &T, params: I) -> Result<RowStream, Error>
            where
                T: ?Sized + ToStatement + Sync + Send,
                P: BorrowToSql,
                I: IntoIterator<Item = P> + Sync + Send,
                I::IntoIter: ExactSizeIterator,
            {
                Transaction::query_raw(self, statement, params).await
            }
        }

        #[async_trait]
        impl GenericClient for Client {
            async fn prepare(&self, query: &str) -> Result<Statement, Error> {
                Client::prepare(self, query).await
            }

            async fn execute<T>(
                &self,
                query: &T,
                params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
            ) -> Result<u64, Error>
            where
                T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
            {
                Client::execute(self, query, params).await
            }

            async fn query_one<T>(
                &self,
                statement: &T,
                params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
            ) -> Result<tokio_postgres::Row, Error>
            where
                T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
            {
                Client::query_one(self, statement, params).await
            }

            async fn query_opt<T>(
                &self,
                statement: &T,
                params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
            ) -> Result<Option<tokio_postgres::Row>, Error>
            where
                T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
            {
                Client::query_opt(self, statement, params).await
            }

            async fn query<T>(
                &self,
                query: &T,
                params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
            ) -> Result<Vec<tokio_postgres::Row>, Error>
            where
                T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
            {
                Client::query(self, query, params).await
            }

            async fn query_raw<T, P, I>(&self, statement: &T, params: I) -> Result<RowStream, Error>
            where
                T: ?Sized + ToStatement + Sync + Send,
                P: BorrowToSql,
                I: IntoIterator<Item = P> + Sync + Send,
                I::IntoIter: ExactSizeIterator,
            {
                Client::query_raw(self, statement, params).await
            }
        }
    )
}

pub fn async_deadpool() -> String {
    code!($WARNING
        use async_trait::async_trait;
        use deadpool_postgres::{
            Client as DeadpoolClient, ClientWrapper, Transaction as DeadpoolTransaction,
        };
        use tokio_postgres::{
            types::BorrowToSql, Client as PgClient, Error, RowStream, Statement, ToStatement,
            Transaction as PgTransaction,
        };

        use super::generic_client::GenericClient;

        #[async_trait]
        impl GenericClient for DeadpoolClient {
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
        impl GenericClient for DeadpoolTransaction<'_> {
            async fn prepare(&self, query: &str) -> Result<Statement, Error> {
                DeadpoolTransaction::prepare_cached(self, query).await
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
    )
}
