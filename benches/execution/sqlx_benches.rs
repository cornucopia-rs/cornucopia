use criterion::Bencher;
use sqlx::{Row, postgres::PgPool, query, query_as};
use tokio::runtime::Runtime;

#[derive(sqlx::FromRow, Debug, Clone)]
#[allow(dead_code)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub hair_color: Option<String>,
}

#[derive(sqlx::FromRow, Debug, Clone)]
#[allow(dead_code)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: Option<String>,
}

#[derive(sqlx::FromRow, Debug, Clone)]
#[allow(dead_code)]
pub struct Comment {
    pub id: i32,
    pub post_id: i32,
    pub text: String,
}

pub fn bench_trivial_query(b: &mut Bencher, pool: &PgPool, rt: &Runtime) {
    b.iter(|| {
        rt.block_on(async {
            query_as::<_, User>("SELECT id, name, hair_color FROM users")
                .fetch_all(pool)
                .await
                .unwrap()
        })
    })
}

pub fn bench_medium_complex_query(b: &mut Bencher, pool: &PgPool, rt: &Runtime) {
    b.iter(|| {
        rt.block_on(async {
            query(
                "SELECT u.id, u.name, u.hair_color, p.id, p.user_id, p.title, p.body
                 FROM users as u LEFT JOIN posts as p on u.id = p.user_id",
            )
            .map(|row: sqlx::postgres::PgRow| {
                (
                    User {
                        id: row.get(0),
                        name: row.get(1),
                        hair_color: row.get(2),
                    },
                    row.get::<Option<i32>, _>(3).map(|id| Post {
                        id,
                        user_id: row.get(4),
                        title: row.get(5),
                        body: row.get(6),
                    }),
                )
            })
            .fetch_all(pool)
            .await
            .unwrap()
        })
    })
}

pub fn bench_insert(b: &mut Bencher, pool: &PgPool, rt: &Runtime, size: usize) {
    b.iter(|| {
        rt.block_on(async {
            let mut query_str = String::from("INSERT INTO users (name, hair_color) VALUES ");
            let mut bindings = Vec::new();

            for i in 0..size {
                if i > 0 {
                    query_str.push_str(", ");
                }
                query_str.push_str(&format!("(${},${})", i * 2 + 1, i * 2 + 2));
                bindings.push(format!("User {i}"));
                bindings.push("hair_color".to_string());
            }

            let mut q = query(&query_str);
            for binding in bindings {
                q = q.bind(binding);
            }

            q.execute(pool).await.unwrap()
        })
    })
}

pub fn loading_associations_sequentially(b: &mut Bencher, pool: &PgPool, rt: &Runtime) {
    b.iter(|| {
        rt.block_on(async {
            let users = query_as::<_, User>("SELECT id, name, hair_color FROM users")
                .fetch_all(pool)
                .await
                .unwrap();

            let user_ids: Vec<i32> = users.iter().map(|u| u.id).collect();
            let posts = query_as::<_, Post>(
                "SELECT id, user_id, title, body FROM posts WHERE user_id = ANY($1)",
            )
            .bind(&user_ids)
            .fetch_all(pool)
            .await
            .unwrap();

            let post_ids: Vec<i32> = posts.iter().map(|p| p.id).collect();
            let comments = query_as::<_, Comment>(
                "SELECT id, post_id, text FROM comments WHERE post_id = ANY($1)",
            )
            .bind(&post_ids)
            .fetch_all(pool)
            .await
            .unwrap();

            let mut result = Vec::new();
            for user in users {
                let user_posts: Vec<(Post, Vec<Comment>)> = posts
                    .iter()
                    .filter(|p| p.user_id == user.id)
                    .map(|post| {
                        let post_comments: Vec<Comment> = comments
                            .iter()
                            .filter(|c| c.post_id == post.id)
                            .cloned()
                            .collect();
                        (post.clone(), post_comments)
                    })
                    .collect();
                result.push((user, user_posts));
            }
            result
        })
    })
}
