// This file was generated with `cornucopia`. Do not modify.
#![allow(clippy::all, clippy::pedantic)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
pub mod types {
    pub mod public {
        #[derive(
            Debug, postgres_types::ToSql, postgres_types::FromSql, Clone, Copy, PartialEq, Eq,
        )]
        #[postgres(name = "spongebob_character")]
        pub enum SpongebobCharacter {
            Bob,
            Patrick,
            Squidward,
        }
        #[derive(Debug, postgres_types::FromSql, Clone, PartialEq)]
        #[postgres(name = "custom_composite")]
        pub struct CustomComposite {
            pub name: String,
            pub age: i32,
            pub persona: super::super::types::public::SpongebobCharacter,
        }
        #[derive(Debug)]
        pub struct CustomCompositeBorrowed<'a> {
            pub name: &'a str,
            pub age: i32,
            pub persona: super::super::types::public::SpongebobCharacter,
        }
        impl<'a> From<CustomCompositeBorrowed<'a>> for CustomComposite {
            fn from(
                CustomCompositeBorrowed { name, age, persona }: CustomCompositeBorrowed<'a>,
            ) -> Self {
                Self {
                    name: name.into(),
                    age,
                    persona,
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
                let name = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let age = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let persona = postgres_types::private::read_value(fields[2].type_(), &mut out)?;
                Ok(CustomCompositeBorrowed { name, age, persona })
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
                let CustomCompositeBorrowed { name, age, persona } = self;
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
                        "persona" => postgres_types::ToSql::to_sql(persona, field.type_(), out),
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
                            "name" => <&'a str as postgres_types::ToSql>::accepts(f.type_()),
"age" => <i32 as postgres_types::ToSql>::accepts(f.type_()),
"persona" => <super::super::types::public::SpongebobCharacter as postgres_types::ToSql>::accepts(f.type_()),
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
    pub mod module_1 {
        use postgres::{fallible_iterator::FallibleIterator, GenericClient};
        pub fn insert_book() -> InsertBookStmt {
            InsertBookStmt(cornucopia_client::sync::Stmt::new(
                "INSERT INTO Book (title)
  VALUES ($1)",
            ))
        }
        pub struct InsertBookStmt(cornucopia_client::sync::Stmt);
        impl InsertBookStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                title: &'a &'a str,
            ) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(stmt, &[title])
            }
        }
    }
    pub mod module_2 {
        use postgres::{fallible_iterator::FallibleIterator, GenericClient};

        #[derive(Debug, Clone, PartialEq)]
        pub struct Authors {
            pub id: i32,
            pub name: String,
            pub country: String,
        }
        pub struct AuthorsBorrowed<'a> {
            pub id: i32,
            pub name: &'a str,
            pub country: &'a str,
        }
        impl<'a> From<AuthorsBorrowed<'a>> for Authors {
            fn from(AuthorsBorrowed { id, name, country }: AuthorsBorrowed<'a>) -> Self {
                Self {
                    id,
                    name: name.into(),
                    country: country.into(),
                }
            }
        }
        pub struct AuthorsQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::sync::Stmt,
            extractor: fn(&postgres::Row) -> AuthorsBorrowed,
            mapper: fn(AuthorsBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> AuthorsQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(AuthorsBorrowed) -> R) -> AuthorsQuery<'a, C, R, N> {
                AuthorsQuery {
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
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }

        pub struct StringQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::sync::Stmt,
            extractor: fn(&postgres::Row) -> &str,
            mapper: fn(&str) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> StringQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(&str) -> R) -> StringQuery<'a, C, R, N> {
                StringQuery {
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
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct AuthorNameStartingWith {
            pub authorid: i32,
            pub name: String,
            pub bookid: i32,
            pub title: String,
        }
        pub struct AuthorNameStartingWithBorrowed<'a> {
            pub authorid: i32,
            pub name: &'a str,
            pub bookid: i32,
            pub title: &'a str,
        }
        impl<'a> From<AuthorNameStartingWithBorrowed<'a>> for AuthorNameStartingWith {
            fn from(
                AuthorNameStartingWithBorrowed {
                    authorid,
                    name,
                    bookid,
                    title,
                }: AuthorNameStartingWithBorrowed<'a>,
            ) -> Self {
                Self {
                    authorid,
                    name: name.into(),
                    bookid,
                    title: title.into(),
                }
            }
        }
        pub struct AuthorNameStartingWithQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::sync::Stmt,
            extractor: fn(&postgres::Row) -> AuthorNameStartingWithBorrowed,
            mapper: fn(AuthorNameStartingWithBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> AuthorNameStartingWithQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(AuthorNameStartingWithBorrowed) -> R,
            ) -> AuthorNameStartingWithQuery<'a, C, R, N> {
                AuthorNameStartingWithQuery {
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
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }

        pub struct SuperSuperTypesPublicCustomCompositeQuery<
            'a,
            C: GenericClient,
            T,
            const N: usize,
        > {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::sync::Stmt,
            extractor: fn(&postgres::Row) -> super::super::types::public::CustomCompositeBorrowed,
            mapper: fn(super::super::types::public::CustomCompositeBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SuperSuperTypesPublicCustomCompositeQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(super::super::types::public::CustomCompositeBorrowed) -> R,
            ) -> SuperSuperTypesPublicCustomCompositeQuery<'a, C, R, N> {
                SuperSuperTypesPublicCustomCompositeQuery {
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
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }

        pub struct SuperSuperTypesPublicSpongebobCharacterQuery<
            'a,
            C: GenericClient,
            T,
            const N: usize,
        > {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::sync::Stmt,
            extractor: fn(&postgres::Row) -> super::super::types::public::SpongebobCharacter,
            mapper: fn(super::super::types::public::SpongebobCharacter) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SuperSuperTypesPublicSpongebobCharacterQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(super::super::types::public::SpongebobCharacter) -> R,
            ) -> SuperSuperTypesPublicSpongebobCharacterQuery<'a, C, R, N> {
                SuperSuperTypesPublicSpongebobCharacterQuery {
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
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }

        pub struct VecStringQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_client::sync::Stmt,
            extractor: fn(&postgres::Row) -> cornucopia_client::ArrayIterator<'_, &str>,
            mapper: fn(cornucopia_client::ArrayIterator<'_, &str>) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> VecStringQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(cornucopia_client::ArrayIterator<'_, &str>) -> R,
            ) -> VecStringQuery<'a, C, R, N> {
                VecStringQuery {
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
                    .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(it)
            }
        }
        pub fn authors() -> AuthorsStmt {
            AuthorsStmt(cornucopia_client::sync::Stmt::new(
                "SELECT
    *
FROM
    Author",
            ))
        }
        pub struct AuthorsStmt(cornucopia_client::sync::Stmt);
        impl AuthorsStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
            ) -> AuthorsQuery<'a, C, Authors, 0> {
                AuthorsQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| AuthorsBorrowed {
                        id: row.get(0),
                        name: row.get(1),
                        country: row.get(2),
                    },
                    mapper: |it| <Authors>::from(it),
                }
            }
            pub fn params<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                params: &'a impl cornucopia_client::sync::Params<
                    'a,
                    Self,
                    AuthorsQuery<'a, C, Authors, 0>,
                    C,
                >,
            ) -> AuthorsQuery<'a, C, Authors, 0> {
                params.bind(client, self)
            }
        }
        pub fn books() -> BooksStmt {
            BooksStmt(cornucopia_client::sync::Stmt::new(
                "SELECT
    Title
FROM
    Book",
            ))
        }
        pub struct BooksStmt(cornucopia_client::sync::Stmt);
        impl BooksStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
            ) -> StringQuery<'a, C, String, 0> {
                StringQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it.into(),
                }
            }
            pub fn params<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                params: &'a impl cornucopia_client::sync::Params<
                    'a,
                    Self,
                    StringQuery<'a, C, String, 0>,
                    C,
                >,
            ) -> StringQuery<'a, C, String, 0> {
                params.bind(client, self)
            }
        }
        pub fn books_opt_ret_param() -> BooksOptRetParamStmt {
            BooksOptRetParamStmt(cornucopia_client::sync::Stmt::new(
                "SELECT
    Title
FROM
    Book",
            ))
        }
        pub struct BooksOptRetParamStmt(cornucopia_client::sync::Stmt);
        impl BooksOptRetParamStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
            ) -> StringQuery<'a, C, String, 0> {
                StringQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it.into(),
                }
            }
            pub fn params<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                params: &'a impl cornucopia_client::sync::Params<
                    'a,
                    Self,
                    StringQuery<'a, C, String, 0>,
                    C,
                >,
            ) -> StringQuery<'a, C, String, 0> {
                params.bind(client, self)
            }
        }
        pub fn author_name_by_id() -> AuthorNameByIdStmt {
            AuthorNameByIdStmt(cornucopia_client::sync::Stmt::new(
                "SELECT
    Author.Name
FROM
    Author
WHERE
    Author.Id = $1",
            ))
        }
        pub struct AuthorNameByIdStmt(cornucopia_client::sync::Stmt);
        impl AuthorNameByIdStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                id: &'a i32,
            ) -> StringQuery<'a, C, String, 1> {
                StringQuery {
                    client,
                    params: [id],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it.into(),
                }
            }
        }
        pub fn author_name_starting_with() -> AuthorNameStartingWithStmt {
            AuthorNameStartingWithStmt(cornucopia_client::sync::Stmt::new(
                "SELECT
    BookAuthor.AuthorId,
    Author.Name,
    BookAuthor.BookId,
    Book.Title
FROM
    BookAuthor
    INNER JOIN Author ON Author.id = BookAuthor.AuthorId
    INNER JOIN Book ON Book.Id = BookAuthor.BookId
WHERE
    Author.Name LIKE CONCAT($1::text, '%')",
            ))
        }
        pub struct AuthorNameStartingWithStmt(cornucopia_client::sync::Stmt);
        impl AuthorNameStartingWithStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                start_str: &'a &'a str,
            ) -> AuthorNameStartingWithQuery<'a, C, AuthorNameStartingWith, 1> {
                AuthorNameStartingWithQuery {
                    client,
                    params: [start_str],
                    stmt: &mut self.0,
                    extractor: |row| AuthorNameStartingWithBorrowed {
                        authorid: row.get(0),
                        name: row.get(1),
                        bookid: row.get(2),
                        title: row.get(3),
                    },
                    mapper: |it| <AuthorNameStartingWith>::from(it),
                }
            }
        }
        pub fn return_custom_type() -> ReturnCustomTypeStmt {
            ReturnCustomTypeStmt(cornucopia_client::sync::Stmt::new(
                "SELECT
    col1
FROM
    CustomTable",
            ))
        }
        pub struct ReturnCustomTypeStmt(cornucopia_client::sync::Stmt);
        impl ReturnCustomTypeStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
            ) -> SuperSuperTypesPublicCustomCompositeQuery<
                'a,
                C,
                super::super::types::public::CustomComposite,
                0,
            > {
                SuperSuperTypesPublicCustomCompositeQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it.into(),
                }
            }
            pub fn params<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                params: &'a impl cornucopia_client::sync::Params<
                    'a,
                    Self,
                    SuperSuperTypesPublicCustomCompositeQuery<
                        'a,
                        C,
                        super::super::types::public::CustomComposite,
                        0,
                    >,
                    C,
                >,
            ) -> SuperSuperTypesPublicCustomCompositeQuery<
                'a,
                C,
                super::super::types::public::CustomComposite,
                0,
            > {
                params.bind(client, self)
            }
        }
        pub fn select_where_custom_type() -> SelectWhereCustomTypeStmt {
            SelectWhereCustomTypeStmt(cornucopia_client::sync::Stmt::new(
                "SELECT
    col2
FROM
    CustomTable
WHERE (col1).persona = $1",
            ))
        }
        pub struct SelectWhereCustomTypeStmt(cornucopia_client::sync::Stmt);
        impl SelectWhereCustomTypeStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                spongebob_character: &'a super::super::types::public::SpongebobCharacter,
            ) -> SuperSuperTypesPublicSpongebobCharacterQuery<
                'a,
                C,
                super::super::types::public::SpongebobCharacter,
                1,
            > {
                SuperSuperTypesPublicSpongebobCharacterQuery {
                    client,
                    params: [spongebob_character],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it,
                }
            }
        }
        pub fn select_translations() -> SelectTranslationsStmt {
            SelectTranslationsStmt(cornucopia_client::sync::Stmt::new(
                "SELECT
    Translations
FROM
    Book",
            ))
        }
        pub struct SelectTranslationsStmt(cornucopia_client::sync::Stmt);
        impl SelectTranslationsStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
            ) -> VecStringQuery<'a, C, Vec<String>, 0> {
                VecStringQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it.map(|v| v.into()).collect(),
                }
            }
            pub fn params<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                params: &'a impl cornucopia_client::sync::Params<
                    'a,
                    Self,
                    VecStringQuery<'a, C, Vec<String>, 0>,
                    C,
                >,
            ) -> VecStringQuery<'a, C, Vec<String>, 0> {
                params.bind(client, self)
            }
        }
    }
}
