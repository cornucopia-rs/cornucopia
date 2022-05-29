#![allow(clippy::all)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
pub mod types {
    pub mod public {
        #[derive(
            Debug,
            postgres_types::ToSql,
            postgres_types::FromSql,
            Clone,
            Copy,
            PartialEq,
            Eq
        )]
        #[postgres(name = "spongebob_character")]
        pub enum SpongebobCharacter {
            Bob,
            Patrick,
            Squidward,
        }
        #[derive(
            Debug,
            postgres_types::ToSql,
            postgres_types::FromSql,
            Clone,
            PartialEq
        )]
        #[postgres(name = "custom_composite")]
        pub struct CustomComposite {
            pub name: String,
            pub age: i32,
            pub persona: super::public::SpongebobCharacter,
        }
        #[derive(Debug)]
        pub struct CustomCompositeBorrowed<'a> {
            pub name: &'a str,
            pub age: i32,
            pub persona: super::super::types::public::SpongebobCharacter,
        }
        impl<'a> From<CustomCompositeBorrowed<'a>> for CustomComposite {
            fn from(
                CustomCompositeBorrowed {
                    name,
                    age,
                    persona,
                }: CustomCompositeBorrowed<'a>,
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
                _type: &postgres_types::Type,
                buf: &'a [u8],
            ) -> Result<
                CustomCompositeBorrowed<'a>,
                std::boxed::Box<dyn std::error::Error + Sync + Send>,
            > {
                let fields = match *_type.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut buf = buf;
                let num_fields = postgres_types::private::read_be_i32(&mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let name = postgres_types::private::read_value(
                    fields[0].type_(),
                    &mut buf,
                )?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let age = postgres_types::private::read_value(
                    fields[1].type_(),
                    &mut buf,
                )?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let persona = postgres_types::private::read_value(
                    fields[2].type_(),
                    &mut buf,
                )?;
                Result::Ok(CustomCompositeBorrowed {
                    name,
                    age,
                    persona,
                })
            }
            fn accepts(type_: &postgres_types::Type) -> bool {
                type_.name() == "custom_composite" && type_.schema() == "public"
            }
        }
        impl<'a> postgres_types::ToSql for CustomCompositeBorrowed<'a> {
            fn to_sql(
                &self,
                _type: &postgres_types::Type,
                buf: &mut postgres_types::private::BytesMut,
            ) -> std::result::Result<
                postgres_types::IsNull,
                std::boxed::Box<dyn std::error::Error + Sync + Send>,
            > {
                let fields = match *_type.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                buf.extend_from_slice(&(fields.len() as i32).to_be_bytes());
                for field in fields {
                    buf.extend_from_slice(&field.type_().oid().to_be_bytes());
                    let base = buf.len();
                    buf.extend_from_slice(&[0; 4]);
                    let r = match field.name() {
                        "name" => {
                            postgres_types::ToSql::to_sql(&self.name, field.type_(), buf)
                        }
                        "age" => {
                            postgres_types::ToSql::to_sql(&self.age, field.type_(), buf)
                        }
                        "persona" => {
                            postgres_types::ToSql::to_sql(
                                &self.persona,
                                field.type_(),
                                buf,
                            )
                        }
                        _ => unreachable!(),
                    };
                    let count = match r? {
                        postgres_types::IsNull::Yes => -1,
                        postgres_types::IsNull::No => {
                            let len = buf.len() - base - 4;
                            if len > i32::max_value() as usize {
                                return std::result::Result::Err(
                                    std::convert::Into::into("value too large to transmit"),
                                );
                            }
                            len as i32
                        }
                    };
                    buf[base..base + 4].copy_from_slice(&count.to_be_bytes());
                }
                std::result::Result::Ok(postgres_types::IsNull::No)
            }
            fn accepts(type_: &postgres_types::Type) -> bool {
                if type_.name() != "custom_composite" {
                    return false;
                }
                match *type_.kind() {
                    postgres_types::Kind::Composite(ref fields) => {
                        if fields.len() != 3usize {
                            return false;
                        }
                        fields
                            .iter()
                            .all(|f| match f.name() {
                                "name" => {
                                    <&'a str as postgres_types::ToSql>::accepts(f.type_())
                                }
                                "age" => <i32 as postgres_types::ToSql>::accepts(f.type_()),
                                "persona" => {
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
            ) -> std::result::Result<
                postgres_types::IsNull,
                Box<dyn std::error::Error + Sync + Send>,
            > {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
    }
}
pub mod queries {
    pub mod module_2 {
        use postgres::fallible_iterator::FallibleIterator;
        use postgres::GenericClient;
        #[derive(Debug, Clone, Copy)]
        pub struct AuthorNameByIdParams {
            pub id: i32,
        }
        impl AuthorNameByIdParams {
            pub fn author_name_by_id<'a, C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> AuthorNameByIdQuery<'a, C, AuthorNameById, 1> {
                author_name_by_id(client, &self.id)
            }
        }
        #[derive(Debug)]
        pub struct AuthorNameStartingWithParams<'a> {
            pub start_str: &'a str,
        }
        impl<'a> AuthorNameStartingWithParams<'a> {
            pub fn author_name_starting_with<C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> AuthorNameStartingWithQuery<'a, C, AuthorNameStartingWith, 1> {
                author_name_starting_with(client, &self.start_str)
            }
        }
        #[derive(Debug, Clone, Copy)]
        pub struct SelectWhereCustomTypeParams {
            pub spongebob_character: super::super::types::public::SpongebobCharacter,
        }
        impl SelectWhereCustomTypeParams {
            pub fn select_where_custom_type<'a, C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> SelectWhereCustomTypeQuery<'a, C, SelectWhereCustomType, 1> {
                select_where_custom_type(client, &self.spongebob_character)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Authors {
            pub country: String,
            pub id: i32,
            pub name: String,
        }
        pub struct AuthorsBorrowed<'a> {
            pub country: &'a str,
            pub id: i32,
            pub name: &'a str,
        }
        impl<'a> From<AuthorsBorrowed<'a>> for Authors {
            fn from(AuthorsBorrowed { country, id, name }: AuthorsBorrowed<'a>) -> Self {
                Self {
                    country: country.into(),
                    id,
                    name: name.into(),
                }
            }
        }
        pub struct AuthorsQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            extractor: fn(&postgres::Row) -> AuthorsBorrowed,
            mapper: fn(AuthorsBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> AuthorsQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(AuthorsBorrowed) -> R,
            ) -> AuthorsQuery<'a, C, R, N> {
                AuthorsQuery {
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
                Ok(
                    self
                        .client
                        .query_opt(&stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))),
                )
            }
            pub fn stream(
                mut self,
            ) -> Result<
                impl Iterator<Item = Result<T, postgres::Error>> + 'a,
                postgres::Error,
            > {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res
                        .map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Books {
            pub title: String,
        }
        pub struct BooksBorrowed<'a> {
            pub title: &'a str,
        }
        impl<'a> From<BooksBorrowed<'a>> for Books {
            fn from(BooksBorrowed { title }: BooksBorrowed<'a>) -> Self {
                Self { title: title.into() }
            }
        }
        pub struct BooksQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            extractor: fn(&postgres::Row) -> BooksBorrowed,
            mapper: fn(BooksBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> BooksQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(BooksBorrowed) -> R,
            ) -> BooksQuery<'a, C, R, N> {
                BooksQuery {
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
                Ok(
                    self
                        .client
                        .query_opt(&stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))),
                )
            }
            pub fn stream(
                mut self,
            ) -> Result<
                impl Iterator<Item = Result<T, postgres::Error>> + 'a,
                postgres::Error,
            > {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res
                        .map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct BooksOptRetParam {
            pub title: Option<String>,
        }
        pub struct BooksOptRetParamBorrowed<'a> {
            pub title: Option<&'a str>,
        }
        impl<'a> From<BooksOptRetParamBorrowed<'a>> for BooksOptRetParam {
            fn from(
                BooksOptRetParamBorrowed { title }: BooksOptRetParamBorrowed<'a>,
            ) -> Self {
                Self {
                    title: title.map(|v| v.into()),
                }
            }
        }
        pub struct BooksOptRetParamQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            extractor: fn(&postgres::Row) -> BooksOptRetParamBorrowed,
            mapper: fn(BooksOptRetParamBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> BooksOptRetParamQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(BooksOptRetParamBorrowed) -> R,
            ) -> BooksOptRetParamQuery<'a, C, R, N> {
                BooksOptRetParamQuery {
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
                Ok(
                    self
                        .client
                        .query_opt(&stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))),
                )
            }
            pub fn stream(
                mut self,
            ) -> Result<
                impl Iterator<Item = Result<T, postgres::Error>> + 'a,
                postgres::Error,
            > {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res
                        .map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct AuthorNameById {
            pub name: String,
        }
        pub struct AuthorNameByIdBorrowed<'a> {
            pub name: &'a str,
        }
        impl<'a> From<AuthorNameByIdBorrowed<'a>> for AuthorNameById {
            fn from(
                AuthorNameByIdBorrowed { name }: AuthorNameByIdBorrowed<'a>,
            ) -> Self {
                Self { name: name.into() }
            }
        }
        pub struct AuthorNameByIdQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            extractor: fn(&postgres::Row) -> AuthorNameByIdBorrowed,
            mapper: fn(AuthorNameByIdBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> AuthorNameByIdQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(AuthorNameByIdBorrowed) -> R,
            ) -> AuthorNameByIdQuery<'a, C, R, N> {
                AuthorNameByIdQuery {
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
                Ok(
                    self
                        .client
                        .query_opt(&stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))),
                )
            }
            pub fn stream(
                mut self,
            ) -> Result<
                impl Iterator<Item = Result<T, postgres::Error>> + 'a,
                postgres::Error,
            > {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res
                        .map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct AuthorNameStartingWith {
            pub authorid: i32,
            pub bookid: i32,
            pub name: String,
            pub title: String,
        }
        pub struct AuthorNameStartingWithBorrowed<'a> {
            pub authorid: i32,
            pub bookid: i32,
            pub name: &'a str,
            pub title: &'a str,
        }
        impl<'a> From<AuthorNameStartingWithBorrowed<'a>> for AuthorNameStartingWith {
            fn from(
                AuthorNameStartingWithBorrowed {
                    authorid,
                    bookid,
                    name,
                    title,
                }: AuthorNameStartingWithBorrowed<'a>,
            ) -> Self {
                Self {
                    authorid,
                    bookid,
                    name: name.into(),
                    title: title.into(),
                }
            }
        }
        pub struct AuthorNameStartingWithQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
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
                Ok(
                    self
                        .client
                        .query_opt(&stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))),
                )
            }
            pub fn stream(
                mut self,
            ) -> Result<
                impl Iterator<Item = Result<T, postgres::Error>> + 'a,
                postgres::Error,
            > {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res
                        .map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ReturnCustomType {
            pub col1: super::super::types::public::CustomComposite,
        }
        pub struct ReturnCustomTypeBorrowed<'a> {
            pub col1: super::super::types::public::CustomCompositeBorrowed<'a>,
        }
        impl<'a> From<ReturnCustomTypeBorrowed<'a>> for ReturnCustomType {
            fn from(
                ReturnCustomTypeBorrowed { col1 }: ReturnCustomTypeBorrowed<'a>,
            ) -> Self {
                Self { col1: col1.into() }
            }
        }
        pub struct ReturnCustomTypeQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            extractor: fn(&postgres::Row) -> ReturnCustomTypeBorrowed,
            mapper: fn(ReturnCustomTypeBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> ReturnCustomTypeQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(ReturnCustomTypeBorrowed) -> R,
            ) -> ReturnCustomTypeQuery<'a, C, R, N> {
                ReturnCustomTypeQuery {
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
                Ok(
                    self
                        .client
                        .query_opt(&stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))),
                )
            }
            pub fn stream(
                mut self,
            ) -> Result<
                impl Iterator<Item = Result<T, postgres::Error>> + 'a,
                postgres::Error,
            > {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res
                        .map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        #[derive(Debug, Clone, PartialEq, Copy)]
        pub struct SelectWhereCustomType {
            pub col2: super::super::types::public::SpongebobCharacter,
        }
        pub struct SelectWhereCustomTypeQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            extractor: fn(&postgres::Row) -> SelectWhereCustomType,
            mapper: fn(SelectWhereCustomType) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SelectWhereCustomTypeQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(SelectWhereCustomType) -> R,
            ) -> SelectWhereCustomTypeQuery<'a, C, R, N> {
                SelectWhereCustomTypeQuery {
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
                Ok(
                    self
                        .client
                        .query_opt(&stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))),
                )
            }
            pub fn stream(
                mut self,
            ) -> Result<
                impl Iterator<Item = Result<T, postgres::Error>> + 'a,
                postgres::Error,
            > {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res
                        .map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SelectTranslations {
            pub translations: Vec<String>,
        }
        pub struct SelectTranslationsBorrowed<'a> {
            pub translations: cornucopia_client::ArrayIterator<'a, &'a str>,
        }
        impl<'a> From<SelectTranslationsBorrowed<'a>> for SelectTranslations {
            fn from(
                SelectTranslationsBorrowed {
                    translations,
                }: SelectTranslationsBorrowed<'a>,
            ) -> Self {
                Self {
                    translations: translations.map(|v| v.into()).collect(),
                }
            }
        }
        pub struct SelectTranslationsQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            extractor: fn(&postgres::Row) -> SelectTranslationsBorrowed,
            mapper: fn(SelectTranslationsBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SelectTranslationsQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(SelectTranslationsBorrowed) -> R,
            ) -> SelectTranslationsQuery<'a, C, R, N> {
                SelectTranslationsQuery {
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
                Ok(
                    self
                        .client
                        .query_opt(&stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))),
                )
            }
            pub fn stream(
                mut self,
            ) -> Result<
                impl Iterator<Item = Result<T, postgres::Error>> + 'a,
                postgres::Error,
            > {
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res
                        .map(|row| (self.mapper)((self.extractor)(&row))));
                Ok(stream)
            }
        }
        pub fn authors<'a, C: GenericClient>(
            client: &'a mut C,
        ) -> AuthorsQuery<'a, C, Authors, 0> {
            AuthorsQuery {
                client,
                params: [],
                query: "SELECT
    *
FROM
    Author;",
                extractor: |row| {
                    AuthorsBorrowed {
                        country: row.get(2),
                        id: row.get(0),
                        name: row.get(1),
                    }
                },
                mapper: |it| Authors::from(it),
            }
        }
        pub fn books<'a, C: GenericClient>(
            client: &'a mut C,
        ) -> BooksQuery<'a, C, Books, 0> {
            BooksQuery {
                client,
                params: [],
                query: "SELECT
    Title
FROM
    Book;",
                extractor: |row| { BooksBorrowed { title: row.get(0) } },
                mapper: |it| Books::from(it),
            }
        }
        pub fn books_opt_ret_param<'a, C: GenericClient>(
            client: &'a mut C,
        ) -> BooksOptRetParamQuery<'a, C, BooksOptRetParam, 0> {
            BooksOptRetParamQuery {
                client,
                params: [],
                query: "SELECT
    Title
FROM
    Book;",
                extractor: |row| {
                    BooksOptRetParamBorrowed {
                        title: row.get(0),
                    }
                },
                mapper: |it| BooksOptRetParam::from(it),
            }
        }
        pub fn author_name_by_id<'a, C: GenericClient>(
            client: &'a mut C,
            id: &'a i32,
        ) -> AuthorNameByIdQuery<'a, C, AuthorNameById, 1> {
            AuthorNameByIdQuery {
                client,
                params: [id],
                query: "SELECT
    Author.Name
FROM
    Author
WHERE
    Author.Id = $1;",
                extractor: |row| {
                    AuthorNameByIdBorrowed {
                        name: row.get(0),
                    }
                },
                mapper: |it| AuthorNameById::from(it),
            }
        }
        pub fn author_name_starting_with<'a, C: GenericClient>(
            client: &'a mut C,
            start_str: &'a &str,
        ) -> AuthorNameStartingWithQuery<'a, C, AuthorNameStartingWith, 1> {
            AuthorNameStartingWithQuery {
                client,
                params: [start_str],
                query: "SELECT
    BookAuthor.AuthorId,
    Author.Name,
    BookAuthor.BookId,
    Book.Title
FROM
    BookAuthor
    INNER JOIN Author ON Author.id = BookAuthor.AuthorId
    INNER JOIN Book ON Book.Id = BookAuthor.BookId
WHERE
    Author.Name LIKE CONCAT($1::text, '%');",
                extractor: |row| {
                    AuthorNameStartingWithBorrowed {
                        authorid: row.get(0),
                        bookid: row.get(2),
                        name: row.get(1),
                        title: row.get(3),
                    }
                },
                mapper: |it| AuthorNameStartingWith::from(it),
            }
        }
        pub fn return_custom_type<'a, C: GenericClient>(
            client: &'a mut C,
        ) -> ReturnCustomTypeQuery<'a, C, ReturnCustomType, 0> {
            ReturnCustomTypeQuery {
                client,
                params: [],
                query: "SELECT
    col1
FROM
    CustomTable;",
                extractor: |row| {
                    ReturnCustomTypeBorrowed {
                        col1: row.get(0),
                    }
                },
                mapper: |it| ReturnCustomType::from(it),
            }
        }
        pub fn select_where_custom_type<'a, C: GenericClient>(
            client: &'a mut C,
            spongebob_character: &'a super::super::types::public::SpongebobCharacter,
        ) -> SelectWhereCustomTypeQuery<'a, C, SelectWhereCustomType, 1> {
            SelectWhereCustomTypeQuery {
                client,
                params: [spongebob_character],
                query: "SELECT
    col2
FROM
    CustomTable
WHERE (col1).persona = $1;",
                extractor: |row| {
                    SelectWhereCustomType {
                        col2: row.get(0),
                    }
                },
                mapper: |it| SelectWhereCustomType::from(it),
            }
        }
        pub fn select_translations<'a, C: GenericClient>(
            client: &'a mut C,
        ) -> SelectTranslationsQuery<'a, C, SelectTranslations, 0> {
            SelectTranslationsQuery {
                client,
                params: [],
                query: "SELECT
    Translations
FROM
    Book;

",
                extractor: |row| {
                    SelectTranslationsBorrowed {
                        translations: row.get(0),
                    }
                },
                mapper: |it| SelectTranslations::from(it),
            }
        }
    }
    pub mod module_1 {
        use postgres::fallible_iterator::FallibleIterator;
        use postgres::GenericClient;
        #[derive(Debug)]
        pub struct InsertBookParams<'a> {
            pub title: &'a str,
        }
        impl<'a> InsertBookParams<'a> {
            pub fn insert_book<C: GenericClient>(
                &'a self,
                client: &'a mut C,
            ) -> Result<u64, postgres::Error> {
                insert_book(client, &self.title)
            }
        }
        pub fn insert_book<'a, C: GenericClient>(
            client: &'a mut C,
            title: &'a &str,
        ) -> Result<u64, postgres::Error> {
            let stmt = client.prepare("INSERT INTO Book (title)
  VALUES ($1);

")?;
            client.execute(&stmt, &[title])
        }
    }
}
