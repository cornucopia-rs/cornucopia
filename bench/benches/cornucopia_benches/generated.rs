// This file was generated with `cornucopia`. Do not modify.
    #![allow(clippy::all)]
    #![allow(unused_variables)]
    #![allow(unused_imports)]
    #![allow(dead_code)]
    pub mod types {  }pub mod queries { pub mod bench { use postgres::{{fallible_iterator::FallibleIterator,GenericClient}}; #[derive(Debug)]
        pub struct InsertUserParams<'a> { pub hair_color : Option<&'a str>,pub name : &'a str }impl <'a, C: GenericClient> cornucopia_client::sync::Params<'a, InsertUserStmt, Result<u64, postgres::Error>, C> for InsertUserParams<'a>  { 
                fn bind(&'a self, client: &'a mut C, stmt: &'a mut InsertUserStmt) -> Result<u64, postgres::Error> {
                    stmt.bind(client, &self.hair_color,&self.name)
                }
            }
#[derive(Debug)]
        pub struct InsertPostParams<'a> { pub body : Option<&'a str>,pub title : &'a str,pub user_id : i32 }impl <'a, C: GenericClient> cornucopia_client::sync::Params<'a, InsertPostStmt, Result<u64, postgres::Error>, C> for InsertPostParams<'a>  { 
                fn bind(&'a self, client: &'a mut C, stmt: &'a mut InsertPostStmt) -> Result<u64, postgres::Error> {
                    stmt.bind(client, &self.body,&self.title,&self.user_id)
                }
            }
#[derive(Debug)]
        pub struct PostByUserIdsParams<'a> { pub ids : &'a [i32] }impl <'a, C: GenericClient> cornucopia_client::sync::Params<'a, PostByUserIdsStmt, PostQuery<'a, C, Post, 1>, C> for PostByUserIdsParams<'a>  { 
                fn bind(&'a self, client: &'a mut C, stmt: &'a mut PostByUserIdsStmt) -> PostQuery<'a, C, Post, 1> {
                    stmt.bind(client, &self.ids)
                }
            }
#[derive(Debug)]
        pub struct InsertCommentsParams<'a> { pub post_id : i32,pub text : &'a str }impl <'a, C: GenericClient> cornucopia_client::sync::Params<'a, InsertCommentsStmt, Result<u64, postgres::Error>, C> for InsertCommentsParams<'a>  { 
                fn bind(&'a self, client: &'a mut C, stmt: &'a mut InsertCommentsStmt) -> Result<u64, postgres::Error> {
                    stmt.bind(client, &self.post_id,&self.text)
                }
            }
#[derive(Debug)]
        pub struct CommentsByPostIdParams<'a> { pub ids : &'a [i32] }impl <'a, C: GenericClient> cornucopia_client::sync::Params<'a, CommentsByPostIdStmt, CommentQuery<'a, C, Comment, 1>, C> for CommentsByPostIdParams<'a>  { 
                fn bind(&'a self, client: &'a mut C, stmt: &'a mut CommentsByPostIdStmt) -> CommentQuery<'a, C, Comment, 1> {
                    stmt.bind(client, &self.ids)
                }
            } #[derive( Debug, Clone, PartialEq,)] pub struct User { pub hair_color : Option<String>,pub id : i32,pub name : String }pub struct UserBorrowed<'a> { pub hair_color : Option<&'a str>,pub id : i32,pub name : &'a str }
                impl<'a> From<UserBorrowed<'a>> for User {
                    fn from(UserBorrowed { hair_color,id,name }: UserBorrowed<'a>) -> Self {
                        Self { hair_color: hair_color.map(|v| v.into()),id,name: name.into() }
                    }
                }
            pub struct UserQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_client::sync::Stmt,
                extractor: fn(&postgres::Row) -> UserBorrowed,
                mapper: fn(UserBorrowed) -> T,
            }
            impl<'a, C, T:'a, const N: usize> UserQuery<'a, C, T, N> where C: GenericClient {
                pub fn map<R>(self, mapper: fn(UserBorrowed) -> R) -> UserQuery<'a,C,R,N> {
                    UserQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
            
                pub  fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
            
                pub  fn vec(self) -> Result<Vec<T>, postgres::Error> {
                    self.stream()?.collect()
                }
            
                pub  fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        ?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
            
                pub  fn stream(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let stream = self
                        .client
                        .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                        ?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        ;
                    Ok(stream)
                }
            }
#[derive( Debug, Clone, PartialEq,Copy)] pub struct UsersIds { pub id : i32 }
            pub struct UsersIdsQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_client::sync::Stmt,
                extractor: fn(&postgres::Row) -> UsersIds,
                mapper: fn(UsersIds) -> T,
            }
            impl<'a, C, T:'a, const N: usize> UsersIdsQuery<'a, C, T, N> where C: GenericClient {
                pub fn map<R>(self, mapper: fn(UsersIds) -> R) -> UsersIdsQuery<'a,C,R,N> {
                    UsersIdsQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
            
                pub  fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
            
                pub  fn vec(self) -> Result<Vec<T>, postgres::Error> {
                    self.stream()?.collect()
                }
            
                pub  fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        ?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
            
                pub  fn stream(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let stream = self
                        .client
                        .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                        ?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        ;
                    Ok(stream)
                }
            }
#[derive( Debug, Clone, PartialEq,)] pub struct Post { pub body : Option<String>,pub id : i32,pub title : String,pub user_id : i32 }pub struct PostBorrowed<'a> { pub body : Option<&'a str>,pub id : i32,pub title : &'a str,pub user_id : i32 }
                impl<'a> From<PostBorrowed<'a>> for Post {
                    fn from(PostBorrowed { body,id,title,user_id }: PostBorrowed<'a>) -> Self {
                        Self { body: body.map(|v| v.into()),id,title: title.into(),user_id }
                    }
                }
            pub struct PostQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_client::sync::Stmt,
                extractor: fn(&postgres::Row) -> PostBorrowed,
                mapper: fn(PostBorrowed) -> T,
            }
            impl<'a, C, T:'a, const N: usize> PostQuery<'a, C, T, N> where C: GenericClient {
                pub fn map<R>(self, mapper: fn(PostBorrowed) -> R) -> PostQuery<'a,C,R,N> {
                    PostQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
            
                pub  fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
            
                pub  fn vec(self) -> Result<Vec<T>, postgres::Error> {
                    self.stream()?.collect()
                }
            
                pub  fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        ?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
            
                pub  fn stream(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let stream = self
                        .client
                        .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                        ?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        ;
                    Ok(stream)
                }
            }
#[derive( Debug, Clone, PartialEq,Copy)] pub struct PostIds { pub id : i32 }
            pub struct PostIdsQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_client::sync::Stmt,
                extractor: fn(&postgres::Row) -> PostIds,
                mapper: fn(PostIds) -> T,
            }
            impl<'a, C, T:'a, const N: usize> PostIdsQuery<'a, C, T, N> where C: GenericClient {
                pub fn map<R>(self, mapper: fn(PostIds) -> R) -> PostIdsQuery<'a,C,R,N> {
                    PostIdsQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
            
                pub  fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
            
                pub  fn vec(self) -> Result<Vec<T>, postgres::Error> {
                    self.stream()?.collect()
                }
            
                pub  fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        ?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
            
                pub  fn stream(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let stream = self
                        .client
                        .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                        ?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        ;
                    Ok(stream)
                }
            }
#[derive( Debug, Clone, PartialEq,)] pub struct Comment { pub id : i32,pub post_id : i32,pub text : String }pub struct CommentBorrowed<'a> { pub id : i32,pub post_id : i32,pub text : &'a str }
                impl<'a> From<CommentBorrowed<'a>> for Comment {
                    fn from(CommentBorrowed { id,post_id,text }: CommentBorrowed<'a>) -> Self {
                        Self { id,post_id,text: text.into() }
                    }
                }
            pub struct CommentQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_client::sync::Stmt,
                extractor: fn(&postgres::Row) -> CommentBorrowed,
                mapper: fn(CommentBorrowed) -> T,
            }
            impl<'a, C, T:'a, const N: usize> CommentQuery<'a, C, T, N> where C: GenericClient {
                pub fn map<R>(self, mapper: fn(CommentBorrowed) -> R) -> CommentQuery<'a,C,R,N> {
                    CommentQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
            
                pub  fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
            
                pub  fn vec(self) -> Result<Vec<T>, postgres::Error> {
                    self.stream()?.collect()
                }
            
                pub  fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        ?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
            
                pub  fn stream(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let stream = self
                        .client
                        .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                        ?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        ;
                    Ok(stream)
                }
            }
#[derive( Debug, Clone, PartialEq,)] pub struct SelectComplex { pub body : Option<String>,pub hair_color : Option<String>,pub myuser_id : i32,pub name : String,pub post_id : Option<i32>,pub title : Option<String>,pub user_id : Option<i32> }pub struct SelectComplexBorrowed<'a> { pub body : Option<&'a str>,pub hair_color : Option<&'a str>,pub myuser_id : i32,pub name : &'a str,pub post_id : Option<i32>,pub title : Option<&'a str>,pub user_id : Option<i32> }
                impl<'a> From<SelectComplexBorrowed<'a>> for SelectComplex {
                    fn from(SelectComplexBorrowed { body,hair_color,myuser_id,name,post_id,title,user_id }: SelectComplexBorrowed<'a>) -> Self {
                        Self { body: body.map(|v| v.into()),hair_color: hair_color.map(|v| v.into()),myuser_id,name: name.into(),post_id,title: title.map(|v| v.into()),user_id }
                    }
                }
            pub struct SelectComplexQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_client::sync::Stmt,
                extractor: fn(&postgres::Row) -> SelectComplexBorrowed,
                mapper: fn(SelectComplexBorrowed) -> T,
            }
            impl<'a, C, T:'a, const N: usize> SelectComplexQuery<'a, C, T, N> where C: GenericClient {
                pub fn map<R>(self, mapper: fn(SelectComplexBorrowed) -> R) -> SelectComplexQuery<'a,C,R,N> {
                    SelectComplexQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
            
                pub  fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
            
                pub  fn vec(self) -> Result<Vec<T>, postgres::Error> {
                    self.stream()?.collect()
                }
            
                pub  fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        ?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
            
                pub  fn stream(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let stream = self
                        .client
                        .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                        ?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        ;
                    Ok(stream)
                }
            } pub fn insert_user() -> InsertUserStmt {
                InsertUserStmt(cornucopia_client::sync::Stmt::new("INSERT INTO users (name, hair_color) VALUES ($2, $1)"))
            }
            pub struct InsertUserStmt(cornucopia_client::sync::Stmt);
            impl InsertUserStmt {pub  fn bind<'a, C: GenericClient>(&'a mut self, client: &'a mut C, hair_color : &'a Option<&'a str>,name : &'a &'a str) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(stmt, &[hair_color,name])
            }pub  fn params<'a, C: GenericClient>(&'a mut self, client: &'a mut C, params: &'a impl cornucopia_client::sync::Params<'a, Self, Result<u64, postgres::Error>, C>) -> Result<u64, postgres::Error> {
                    params.bind(client, self)
                }}
pub fn users() -> UsersStmt {
                UsersStmt(cornucopia_client::sync::Stmt::new("SELECT * FROM users"))
            }
            pub struct UsersStmt(cornucopia_client::sync::Stmt);
            impl UsersStmt {pub fn bind<'a, C: GenericClient>(&'a mut self, client: &'a mut C, ) -> UserQuery<'a,C, User, 0> {
                UserQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| { UserBorrowed {hair_color: row.get(2),id: row.get(0),name: row.get(1)} },
                    mapper: |it| User::from(it),
                }
            }}
pub fn users_ids() -> UsersIdsStmt {
                UsersIdsStmt(cornucopia_client::sync::Stmt::new("SELECT id from users"))
            }
            pub struct UsersIdsStmt(cornucopia_client::sync::Stmt);
            impl UsersIdsStmt {pub fn bind<'a, C: GenericClient>(&'a mut self, client: &'a mut C, ) -> UsersIdsQuery<'a,C, UsersIds, 0> {
                UsersIdsQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| { UsersIds {id: row.get(0)} },
                    mapper: |it| UsersIds::from(it),
                }
            }}
pub fn insert_post() -> InsertPostStmt {
                InsertPostStmt(cornucopia_client::sync::Stmt::new("INSERT INTO posts(title, user_id, body) VALUES ($2, $3, $1)"))
            }
            pub struct InsertPostStmt(cornucopia_client::sync::Stmt);
            impl InsertPostStmt {pub  fn bind<'a, C: GenericClient>(&'a mut self, client: &'a mut C, body : &'a Option<&'a str>,title : &'a &'a str,user_id : &'a i32) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(stmt, &[body,title,user_id])
            }pub  fn params<'a, C: GenericClient>(&'a mut self, client: &'a mut C, params: &'a impl cornucopia_client::sync::Params<'a, Self, Result<u64, postgres::Error>, C>) -> Result<u64, postgres::Error> {
                    params.bind(client, self)
                }}
pub fn posts() -> PostsStmt {
                PostsStmt(cornucopia_client::sync::Stmt::new("SELECT * FROM posts"))
            }
            pub struct PostsStmt(cornucopia_client::sync::Stmt);
            impl PostsStmt {pub fn bind<'a, C: GenericClient>(&'a mut self, client: &'a mut C, ) -> PostQuery<'a,C, Post, 0> {
                PostQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| { PostBorrowed {body: row.get(3),id: row.get(0),title: row.get(2),user_id: row.get(1)} },
                    mapper: |it| Post::from(it),
                }
            }}
pub fn post_ids() -> PostIdsStmt {
                PostIdsStmt(cornucopia_client::sync::Stmt::new("SELECT id from posts"))
            }
            pub struct PostIdsStmt(cornucopia_client::sync::Stmt);
            impl PostIdsStmt {pub fn bind<'a, C: GenericClient>(&'a mut self, client: &'a mut C, ) -> PostIdsQuery<'a,C, PostIds, 0> {
                PostIdsQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| { PostIds {id: row.get(0)} },
                    mapper: |it| PostIds::from(it),
                }
            }}
pub fn post_by_user_ids() -> PostByUserIdsStmt {
                PostByUserIdsStmt(cornucopia_client::sync::Stmt::new("SELECT * FROM posts WHERE user_id = ANY($1)"))
            }
            pub struct PostByUserIdsStmt(cornucopia_client::sync::Stmt);
            impl PostByUserIdsStmt {pub fn bind<'a, C: GenericClient>(&'a mut self, client: &'a mut C, ids : &'a &'a [i32]) -> PostQuery<'a,C, Post, 1> {
                PostQuery {
                    client,
                    params: [ids],
                    stmt: &mut self.0,
                    extractor: |row| { PostBorrowed {body: row.get(3),id: row.get(0),title: row.get(2),user_id: row.get(1)} },
                    mapper: |it| Post::from(it),
                }
            }pub fn params<'a, C: GenericClient>(&'a mut self, client: &'a mut C, params: &'a impl cornucopia_client::sync::Params<'a, Self, PostQuery<'a,C, Post, 1>, C>) -> PostQuery<'a,C, Post, 1> {
                    params.bind(client, self)
                }}
pub fn insert_comments() -> InsertCommentsStmt {
                InsertCommentsStmt(cornucopia_client::sync::Stmt::new("INSERT INTO comments(text, post_id) VALUES ($2, $1)"))
            }
            pub struct InsertCommentsStmt(cornucopia_client::sync::Stmt);
            impl InsertCommentsStmt {pub  fn bind<'a, C: GenericClient>(&'a mut self, client: &'a mut C, post_id : &'a i32,text : &'a &'a str) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(stmt, &[post_id,text])
            }pub  fn params<'a, C: GenericClient>(&'a mut self, client: &'a mut C, params: &'a impl cornucopia_client::sync::Params<'a, Self, Result<u64, postgres::Error>, C>) -> Result<u64, postgres::Error> {
                    params.bind(client, self)
                }}
pub fn comments() -> CommentsStmt {
                CommentsStmt(cornucopia_client::sync::Stmt::new("SELECT * FROM comments"))
            }
            pub struct CommentsStmt(cornucopia_client::sync::Stmt);
            impl CommentsStmt {pub fn bind<'a, C: GenericClient>(&'a mut self, client: &'a mut C, ) -> CommentQuery<'a,C, Comment, 0> {
                CommentQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| { CommentBorrowed {id: row.get(0),post_id: row.get(1),text: row.get(2)} },
                    mapper: |it| Comment::from(it),
                }
            }}
pub fn comments_by_post_id() -> CommentsByPostIdStmt {
                CommentsByPostIdStmt(cornucopia_client::sync::Stmt::new("SELECT * FROM comments WHERE post_id = ANY($1)"))
            }
            pub struct CommentsByPostIdStmt(cornucopia_client::sync::Stmt);
            impl CommentsByPostIdStmt {pub fn bind<'a, C: GenericClient>(&'a mut self, client: &'a mut C, ids : &'a &'a [i32]) -> CommentQuery<'a,C, Comment, 1> {
                CommentQuery {
                    client,
                    params: [ids],
                    stmt: &mut self.0,
                    extractor: |row| { CommentBorrowed {id: row.get(0),post_id: row.get(1),text: row.get(2)} },
                    mapper: |it| Comment::from(it),
                }
            }pub fn params<'a, C: GenericClient>(&'a mut self, client: &'a mut C, params: &'a impl cornucopia_client::sync::Params<'a, Self, CommentQuery<'a,C, Comment, 1>, C>) -> CommentQuery<'a,C, Comment, 1> {
                    params.bind(client, self)
                }}
pub fn select_complex() -> SelectComplexStmt {
                SelectComplexStmt(cornucopia_client::sync::Stmt::new("SELECT u.id as myuser_id, u.name, u.hair_color, p.id as post_id, p.user_id, p.title, p.body FROM users as u LEFT JOIN posts as p on u.id = p.user_id"))
            }
            pub struct SelectComplexStmt(cornucopia_client::sync::Stmt);
            impl SelectComplexStmt {pub fn bind<'a, C: GenericClient>(&'a mut self, client: &'a mut C, ) -> SelectComplexQuery<'a,C, SelectComplex, 0> {
                SelectComplexQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| { SelectComplexBorrowed {body: row.get(6),hair_color: row.get(2),myuser_id: row.get(0),name: row.get(1),post_id: row.get(3),title: row.get(5),user_id: row.get(4)} },
                    mapper: |it| SelectComplex::from(it),
                }
            }} } }