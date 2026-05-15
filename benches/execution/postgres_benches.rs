use criterion::Bencher;
use postgres::types::ToSql;
use postgres::{Client, fallible_iterator::FallibleIterator};
use std::collections::HashMap;
use std::fmt::Write;

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
    let query = client
        .prepare("SELECT id, name, hair_color FROM users")
        .unwrap();

    b.iter(|| {
        client
            .query_raw(&query, NO_PARAMS)
            .unwrap()
            .map(|row| {
                Ok(User {
                    id: row.get(0),
                    name: row.get(1),
                    hair_color: row.get(2),
                })
            })
            .collect::<Vec<_>>()
            .unwrap()
    })
}

pub fn bench_medium_complex_query(b: &mut Bencher, client: &mut Client) {
    let query = client
        .prepare(
            "SELECT u.id, u.name, u.hair_color, p.id, p.user_id, p.title, p.body \
             FROM users as u LEFT JOIN posts as p on u.id = p.user_id",
        )
        .unwrap();

    b.iter(|| {
        client
            .query_raw(&query, NO_PARAMS)
            .unwrap()
            .map(|row| {
                let user = User {
                    id: row.get(0),
                    name: row.get(1),
                    hair_color: row.get(2),
                };
                let post = row.get::<_, Option<i32>>(3).map(|id| Post {
                    id,
                    user_id: row.get(4),
                    title: row.get(5),
                    body: row.get(6),
                });
                Ok((user, post))
            })
            .collect::<Vec<_>>()
            .unwrap()
    })
}

pub fn bench_insert(b: &mut Bencher, client: &mut Client, size: usize) {
    b.iter(|| {
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

        client.execute(&query as &str, &params).unwrap();
    })
}

pub fn loading_associations_sequentially(b: &mut Bencher, client: &mut Client) {
    let user_query = client
        .prepare("SELECT id, name, hair_color FROM users")
        .unwrap();

    b.iter(|| {
        let users = client
            .query_raw(&user_query, NO_PARAMS)
            .unwrap()
            .map(|row| {
                Ok(User {
                    id: row.get("id"),
                    name: row.get("name"),
                    hair_color: row.get("hair_color"),
                })
            })
            .collect::<Vec<_>>()
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
            .unwrap()
            .map(|row| {
                Ok(Post {
                    id: row.get("id"),
                    user_id: row.get("user_id"),
                    title: row.get("title"),
                    body: row.get("body"),
                })
            })
            .collect::<Vec<_>>()
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
            .unwrap()
            .map(|row| {
                Ok(Comment {
                    id: row.get("id"),
                    post_id: row.get("post_id"),
                    text: row.get("text"),
                })
            })
            .collect::<Vec<_>>()
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
    })
}
