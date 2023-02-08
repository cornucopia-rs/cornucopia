use std::collections::HashMap;

use criterion::Bencher;
use futures::executor::block_on;
use tokio_postgres::Client;

use self::generated::queries::bench::{
    async_::{comments_by_post_id, insert_user, post_by_user_ids, select_complex, users},
    Comment, Post, User,
};

mod generated;

pub fn bench_trivial_query(b: &mut Bencher, client: &Client) {
    let mut stmt = users();
    b.iter(|| block_on(async { stmt.bind(client).all().await.unwrap() }))
}

pub fn bench_medium_complex_query(b: &mut Bencher, client: &Client) {
    let mut stmt = select_complex();
    b.iter(|| {
        block_on(async {
            stmt.bind(client)
                .map(|it| {
                    (
                        User {
                            id: it.myuser_id,
                            name: it.name.to_string(),
                            hair_color: it.hair_color.map(|it| it.to_string()),
                        },
                        it.post_id.map(|id| Post {
                            id,
                            user_id: it.user_id.unwrap(),
                            title: it.title.unwrap().to_string(),
                            body: it.body.map(|it| it.to_string()),
                        }),
                    )
                })
                .all()
                .await
                .unwrap()
        })
    })
}

pub fn bench_insert(b: &mut Bencher, client: &mut Client, size: usize) {
    let mut stmt = insert_user();
    b.iter(|| {
        block_on(async {
            let tx = client.transaction().await.unwrap();
            for x in 0..size {
                stmt.bind(&tx, &format!("User {x}").as_str(), &Some("hair_color"))
                    .await
                    .unwrap();
            }
            tx.commit().await.unwrap();
        })
    })
}

pub fn loading_associations_sequentially(b: &mut Bencher, client: &Client) {
    let mut user_stmt = users();
    let mut post_stmt = post_by_user_ids();
    let mut comment_stmt = comments_by_post_id();
    b.iter(|| {
        block_on(async {
            let users = user_stmt.bind(client).all().await.unwrap();
            let users_ids: Vec<i32> = users.iter().map(|it| it.id).collect();
            let posts = post_stmt
                .bind(client, &users_ids.as_slice())
                .all()
                .await
                .unwrap();
            let posts_ids: Vec<i32> = posts.iter().map(|it| it.id).collect();
            let comments = comment_stmt
                .bind(client, &posts_ids.as_slice())
                .all()
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
        })
    })
}

pub mod sync {
    use std::collections::HashMap;

    use criterion::Bencher;
    use postgres::Client;

    use super::generated::queries::bench::{
        sync::{comments_by_post_id, insert_user, post_by_user_ids, select_complex, users},
        Comment, Post, User,
    };
    pub fn bench_trivial_query(b: &mut Bencher, client: &mut Client) {
        let mut stmt = users();
        b.iter(|| stmt.bind(client).all().unwrap())
    }

    pub fn bench_medium_complex_query(b: &mut Bencher, client: &mut Client) {
        let mut stmt = select_complex();
        b.iter(|| {
            stmt.bind(client)
                .map(|it| {
                    (
                        User {
                            id: it.myuser_id,
                            name: it.name.to_string(),
                            hair_color: it.hair_color.map(|it| it.to_string()),
                        },
                        it.post_id.map(|id| Post {
                            id,
                            user_id: it.user_id.unwrap(),
                            title: it.title.unwrap().to_string(),
                            body: it.body.map(|it| it.to_string()),
                        }),
                    )
                })
                .all()
                .unwrap()
        })
    }

    pub fn bench_insert(b: &mut Bencher, client: &mut Client, size: usize) {
        let mut stmt = insert_user();
        b.iter(|| {
            let mut tx = client.transaction().unwrap();
            for x in 0..size {
                stmt.bind(&mut tx, &format!("User {x}").as_str(), &Some("hair_color"))
                    .unwrap();
            }
            tx.commit().unwrap();
        })
    }

    pub fn loading_associations_sequentially(b: &mut Bencher, client: &mut Client) {
        let mut user_stmt = users();
        let mut post_stmt = post_by_user_ids();
        let mut comment_stmt = comments_by_post_id();

        b.iter(|| {
            let users = user_stmt.bind(client).all().unwrap();
            let users_ids: Vec<i32> = users.iter().map(|it| it.id).collect();
            let posts = post_stmt.bind(client, &users_ids.as_slice()).all().unwrap();
            let posts_ids: Vec<i32> = posts.iter().map(|it| it.id).collect();
            let comments = comment_stmt
                .bind(client, &posts_ids.as_slice())
                .all()
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
}
