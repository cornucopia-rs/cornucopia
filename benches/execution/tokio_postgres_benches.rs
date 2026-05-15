use criterion::Bencher;
use futures::executor::block_on;
use futures::{StreamExt, TryStreamExt};
use postgres_types::ToSql;
use std::collections::HashMap;
use std::fmt::Write;
use tokio_postgres::Client;

const NO_PARAMS: Vec<&dyn ToSql> = Vec::new();

#[allow(dead_code)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub hair_color: Option<String>,
}

#[allow(dead_code)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: Option<String>,
}

#[allow(dead_code)]
pub struct Comment {
    pub id: i32,
    pub post_id: i32,
    pub text: String,
}

pub fn bench_trivial_query(b: &mut Bencher, client: &mut Client) {
    let query = block_on(async {
        client
            .prepare("SELECT id, name, hair_color FROM users")
            .await
            .unwrap()
    });

    b.iter(|| {
        block_on(async {
            client
                .query_raw(&query, NO_PARAMS)
                .await
                .unwrap()
                .map(|row| {
                    row.map(|row| User {
                        id: row.get(0),
                        name: row.get(1),
                        hair_color: row.get(2),
                    })
                })
                .try_collect::<Vec<_>>()
                .await
                .unwrap()
        })
    })
}

pub fn bench_medium_complex_query(b: &mut Bencher, client: &mut Client) {
    let query = block_on(async {
        client
            .prepare(
                "SELECT u.id, u.name, u.hair_color, p.id, p.user_id, p.title, p.body \
             FROM users as u LEFT JOIN posts as p on u.id = p.user_id",
            )
            .await
            .unwrap()
    });

    b.iter(|| {
        block_on(async {
            client
                .query_raw(&query, NO_PARAMS)
                .await
                .unwrap()
                .map(|row| {
                    row.map(|row| {
                        (
                            User {
                                id: row.get(0),
                                name: row.get(1),
                                hair_color: row.get(2),
                            },
                            row.get::<_, Option<i32>>(3).map(|id| Post {
                                id,
                                user_id: row.get(4),
                                title: row.get(5),
                                body: row.get(6),
                            }),
                        )
                    })
                })
                .try_collect::<Vec<_>>()
                .await
                .unwrap()
        })
    })
}

pub fn bench_insert(b: &mut Bencher, client: &mut Client, size: usize) {
    b.iter(|| {
        block_on(async {
            let mut query = String::from("INSERT INTO users (name, hair_color) VALUES");

            let mut params = Vec::with_capacity(2 * size);

            for x in 0..size {
                write!(
                    query,
                    "{} (${}, ${})",
                    if x == 0 { "" } else { "," },
                    2 * x + 1,
                    2 * x + 2
                )
                .unwrap();
                params.push((format!("User {x}"), Some("hair_color")));
            }

            let params = params
                .iter()
                .flat_map(|(a, b)| [a as _, b as _])
                .collect::<Vec<_>>();

            client.execute(&query as &str, &params).await.unwrap();
        })
    })
}

pub fn loading_associations_sequentially(b: &mut Bencher, client: &mut Client) {
    let user_query = block_on(async {
        client
            .prepare("SELECT id, name, hair_color FROM users")
            .await
            .unwrap()
    });

    b.iter(|| {
        block_on(async {
            let users = client
                .query_raw(&user_query, NO_PARAMS)
                .await
                .unwrap()
                .map(|row| {
                    row.map(|row| User {
                        id: row.get("id"),
                        name: row.get("name"),
                        hair_color: row.get("hair_color"),
                    })
                })
                .try_collect::<Vec<_>>()
                .await
                .unwrap();

            let mut posts_query =
                String::from("SELECT id, title, user_id, body FROM posts WHERE user_id IN(");

            let user_ids = users
                .iter()
                .enumerate()
                .map(|(i, &User { id, .. })| {
                    posts_query += &format!("{}${}", if i == 0 { "" } else { "," }, i + 1);
                    id
                })
                .collect::<Vec<i32>>();

            posts_query += ")";

            let posts = client
                .query_raw(&posts_query as &str, user_ids)
                .await
                .unwrap()
                .map(|row| {
                    row.map(|row| Post {
                        id: row.get("id"),
                        user_id: row.get("user_id"),
                        title: row.get("title"),
                        body: row.get("body"),
                    })
                })
                .try_collect::<Vec<_>>()
                .await
                .unwrap();

            let mut comments_query =
                String::from("SELECT id, post_id, text FROM comments WHERE post_id IN(");

            let post_ids = posts
                .iter()
                .enumerate()
                .map(|(i, &Post { id, .. })| {
                    comments_query += &format!("{}${}", if i == 0 { "" } else { "," }, i + 1);
                    id
                })
                .collect::<Vec<i32>>();

            comments_query += ")";

            let comments = client
                .query_raw(&comments_query as &str, post_ids)
                .await
                .unwrap()
                .map(|row| {
                    row.map(|row| Comment {
                        id: row.get("id"),
                        post_id: row.get("post_id"),
                        text: row.get("text"),
                    })
                })
                .try_collect::<Vec<_>>()
                .await
                .unwrap();

            let mut posts = posts
                .into_iter()
                .map(|p| (p.id, (p, Vec::new())))
                .collect::<HashMap<_, _>>();

            let mut users = users
                .into_iter()
                .map(|u| (u.id, (u, Vec::new())))
                .collect::<HashMap<_, _>>();

            for comment in comments {
                posts.get_mut(&comment.post_id).unwrap().1.push(comment);
            }

            for (_, post_with_comments) in posts {
                users
                    .get_mut(&post_with_comments.0.user_id)
                    .unwrap()
                    .1
                    .push(post_with_comments);
            }

            users
                .into_values()
                .collect::<Vec<(User, Vec<(Post, Vec<Comment>)>)>>()
        });
    })
}
