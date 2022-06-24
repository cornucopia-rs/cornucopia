// This file was generated with `cornucopia`. Do not modify.
#![allow(clippy::all, clippy::pedantic)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
pub mod types {}
pub mod queries {
    pub mod bench {
        use postgres::{fallible_iterator::FallibleIterator, GenericClient};
        #[derive(Debug)]
        pub struct InsertUserParams<'a> {
            pub name: &'a str,
            pub hair_color: Option<&'a str>,
        }
        impl<'a, C: GenericClient>
            cornucopia_client::sync::Params<'a, InsertUserStmt, Result<u64, postgres::Error>, C>
            for InsertUserParams<'a>
        {
            fn bind(
                &'a self,
                client: &'a mut C,
                stmt: &'a mut InsertUserStmt,
            ) -> Result<u64, postgres::Error> {
                stmt.bind(client, &self.name, &self.hair_color)
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct User {
            pub id: i32,
            pub name: String,
            pub hair_color: Option<String>,
        }
        pub struct UserBorrowed<'a> {
            pub id: i32,
            pub name: &'a str,
            pub hair_color: Option<&'a str>,
        }
        impl<'a> From<UserBorrowed<'a>> for User {
            fn from(
                UserBorrowed {
                    id,
                    name,
                    hair_color,
                }: UserBorrowed<'a>,
            ) -> Self {
                Self {
                    id,
                    name: name.into(),
                    hair_color: hair_color.map(|v| v.into()),
                }
            }
        }
        impl cornucopia_client::Borrow for User {
            type Borrow<'r> = UserBorrowed<'r>;
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Post {
            pub id: i32,
            pub user_id: i32,
            pub title: String,
            pub body: Option<String>,
        }
        pub struct PostBorrowed<'a> {
            pub id: i32,
            pub user_id: i32,
            pub title: &'a str,
            pub body: Option<&'a str>,
        }
        impl<'a> From<PostBorrowed<'a>> for Post {
            fn from(
                PostBorrowed {
                    id,
                    user_id,
                    title,
                    body,
                }: PostBorrowed<'a>,
            ) -> Self {
                Self {
                    id,
                    user_id,
                    title: title.into(),
                    body: body.map(|v| v.into()),
                }
            }
        }
        impl cornucopia_client::Borrow for Post {
            type Borrow<'r> = PostBorrowed<'r>;
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Comment {
            pub id: i32,
            pub post_id: i32,
            pub text: String,
        }
        pub struct CommentBorrowed<'a> {
            pub id: i32,
            pub post_id: i32,
            pub text: &'a str,
        }
        impl<'a> From<CommentBorrowed<'a>> for Comment {
            fn from(CommentBorrowed { id, post_id, text }: CommentBorrowed<'a>) -> Self {
                Self {
                    id,
                    post_id,
                    text: text.into(),
                }
            }
        }
        impl cornucopia_client::Borrow for Comment {
            type Borrow<'r> = CommentBorrowed<'r>;
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SelectComplex {
            pub myuser_id: i32,
            pub name: String,
            pub hair_color: Option<String>,
            pub post_id: Option<i32>,
            pub user_id: Option<i32>,
            pub title: Option<String>,
            pub body: Option<String>,
        }
        pub struct SelectComplexBorrowed<'a> {
            pub myuser_id: i32,
            pub name: &'a str,
            pub hair_color: Option<&'a str>,
            pub post_id: Option<i32>,
            pub user_id: Option<i32>,
            pub title: Option<&'a str>,
            pub body: Option<&'a str>,
        }
        impl<'a> From<SelectComplexBorrowed<'a>> for SelectComplex {
            fn from(
                SelectComplexBorrowed {
                    myuser_id,
                    name,
                    hair_color,
                    post_id,
                    user_id,
                    title,
                    body,
                }: SelectComplexBorrowed<'a>,
            ) -> Self {
                Self {
                    myuser_id,
                    name: name.into(),
                    hair_color: hair_color.map(|v| v.into()),
                    post_id,
                    user_id,
                    title: title.map(|v| v.into()),
                    body: body.map(|v| v.into()),
                }
            }
        }
        impl cornucopia_client::Borrow for SelectComplex {
            type Borrow<'r> = SelectComplexBorrowed<'r>;
        }
        pub fn users() -> UsersStmt {
            UsersStmt(cornucopia_client::sync::Stmt::new("SELECT * FROM users"))
        }
        pub struct UsersStmt(cornucopia_client::sync::Stmt);
        impl UsersStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
            ) -> cornucopia_client::sync::Query<'a, C, User, User, 0> {
                cornucopia_client::sync::Query {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| UserBorrowed {
                        id: row.get(0),
                        name: row.get(1),
                        hair_color: row.get(2),
                    },
                    mapper: |it| <User>::from(it),
                }
            }
        }
        pub fn insert_user() -> InsertUserStmt {
            InsertUserStmt(cornucopia_client::sync::Stmt::new(
                "INSERT INTO users (name, hair_color) VALUES ($1, $2)",
            ))
        }
        pub struct InsertUserStmt(cornucopia_client::sync::Stmt);
        impl InsertUserStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                name: &'a &'a str,
                hair_color: &'a Option<&'a str>,
            ) -> Result<u64, postgres::Error> {
                let stmt = self.0.prepare(client)?;
                client.execute(stmt, &[name, hair_color])
            }
            pub fn params<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                params: &'a impl cornucopia_client::sync::Params<
                    'a,
                    Self,
                    Result<u64, postgres::Error>,
                    C,
                >,
            ) -> Result<u64, postgres::Error> {
                params.bind(client, self)
            }
        }
        pub fn posts() -> PostsStmt {
            PostsStmt(cornucopia_client::sync::Stmt::new("SELECT * FROM posts"))
        }
        pub struct PostsStmt(cornucopia_client::sync::Stmt);
        impl PostsStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
            ) -> cornucopia_client::sync::Query<'a, C, Post, Post, 0> {
                cornucopia_client::sync::Query {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| PostBorrowed {
                        id: row.get(0),
                        user_id: row.get(1),
                        title: row.get(2),
                        body: row.get(3),
                    },
                    mapper: |it| <Post>::from(it),
                }
            }
        }
        pub fn post_by_user_ids() -> PostByUserIdsStmt {
            PostByUserIdsStmt(cornucopia_client::sync::Stmt::new(
                "SELECT * FROM posts WHERE user_id = ANY($1)",
            ))
        }
        pub struct PostByUserIdsStmt(cornucopia_client::sync::Stmt);
        impl PostByUserIdsStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                ids: &'a &'a [i32],
            ) -> cornucopia_client::sync::Query<'a, C, Post, Post, 1> {
                cornucopia_client::sync::Query {
                    client,
                    params: [ids],
                    stmt: &mut self.0,
                    extractor: |row| PostBorrowed {
                        id: row.get(0),
                        user_id: row.get(1),
                        title: row.get(2),
                        body: row.get(3),
                    },
                    mapper: |it| <Post>::from(it),
                }
            }
        }
        pub fn comments() -> CommentsStmt {
            CommentsStmt(cornucopia_client::sync::Stmt::new("SELECT * FROM comments"))
        }
        pub struct CommentsStmt(cornucopia_client::sync::Stmt);
        impl CommentsStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
            ) -> cornucopia_client::sync::Query<'a, C, Comment, Comment, 0> {
                cornucopia_client::sync::Query {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| CommentBorrowed {
                        id: row.get(0),
                        post_id: row.get(1),
                        text: row.get(2),
                    },
                    mapper: |it| <Comment>::from(it),
                }
            }
        }
        pub fn comments_by_post_id() -> CommentsByPostIdStmt {
            CommentsByPostIdStmt(cornucopia_client::sync::Stmt::new(
                "SELECT * FROM comments WHERE post_id = ANY($1)",
            ))
        }
        pub struct CommentsByPostIdStmt(cornucopia_client::sync::Stmt);
        impl CommentsByPostIdStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
                ids: &'a &'a [i32],
            ) -> cornucopia_client::sync::Query<'a, C, Comment, Comment, 1> {
                cornucopia_client::sync::Query {
                    client,
                    params: [ids],
                    stmt: &mut self.0,
                    extractor: |row| CommentBorrowed {
                        id: row.get(0),
                        post_id: row.get(1),
                        text: row.get(2),
                    },
                    mapper: |it| <Comment>::from(it),
                }
            }
        }
        pub fn select_complex() -> SelectComplexStmt {
            SelectComplexStmt(cornucopia_client::sync::Stmt::new("SELECT u.id as myuser_id, u.name, u.hair_color, p.id as post_id, p.user_id, p.title, p.body FROM users as u LEFT JOIN posts as p on u.id = p.user_id"))
        }
        pub struct SelectComplexStmt(cornucopia_client::sync::Stmt);
        impl SelectComplexStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a mut C,
            ) -> cornucopia_client::sync::Query<'a, C, SelectComplex, SelectComplex, 0>
            {
                cornucopia_client::sync::Query {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| SelectComplexBorrowed {
                        myuser_id: row.get(0),
                        name: row.get(1),
                        hair_color: row.get(2),
                        post_id: row.get(3),
                        user_id: row.get(4),
                        title: row.get(5),
                        body: row.get(6),
                    },
                    mapper: |it| <SelectComplex>::from(it),
                }
            }
        }
    }
}
