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
    b.iter(move || block_on(async { users(client).bind().all().await.unwrap() }))
}

pub fn bench_medium_complex_query(b: &mut Bencher, client: &Client) {
    b.iter(|| {
        block_on(async {
            select_complex(client)
                .bind()
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
    b.iter(|| {
        block_on(async {
            let mut tx = client.transaction().await.unwrap();
            for x in 0..size {
                insert_user(&mut tx)
                    .bind(&format!("User {}", x).as_str(), &Some("hair_color"))
                    .await
                    .unwrap();
            }
            tx.commit().await.unwrap();
        })
    })
}

pub fn loading_associations_sequentially(b: &mut Bencher, client: &Client) {
    b.iter(|| {
        let mut user_stmt = users(client);
        block_on(async {
            let users = user_stmt.bind().all().await.unwrap();
            let users_ids: Vec<i32> = users.iter().map(|it| it.id).collect();
            let posts = post_by_user_ids(client)
                .bind(&users_ids.as_slice())
                .all()
                .await
                .unwrap();
            let posts_ids: Vec<i32> = posts.iter().map(|it| it.id).collect();
            let comments = comments_by_post_id(client)
                .bind(&posts_ids.as_slice())
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
                .into_iter()
                .map(|(_, users_with_post_and_comment)| users_with_post_and_comment)
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
        b.iter(|| users(client).bind().all().unwrap())
    }

    pub fn bench_medium_complex_query(b: &mut Bencher, client: &mut Client) {
        b.iter(|| {
            select_complex(client)
                .bind()
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
        b.iter(|| {
            let mut tx = client.transaction().unwrap();
            for x in 0..size {
                insert_user(&mut tx)
                    .bind(&format!("User {}", x).as_str(), &Some("hair_color"))
                    .unwrap();
            }
            tx.commit().unwrap();
        })
    }

    pub fn loading_associations_sequentially(b: &mut Bencher, client: &mut Client) {
        b.iter(|| {
            let users = users(client).bind().all().unwrap();
            let users_ids: Vec<i32> = users.iter().map(|it| it.id).collect();
            let posts = post_by_user_ids(client)
                .bind(&users_ids.as_slice())
                .all()
                .unwrap();
            let posts_ids: Vec<i32> = posts.iter().map(|it| it.id).collect();
            let comments = comments_by_post_id(client)
                .bind(&posts_ids.as_slice())
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
                .into_iter()
                .map(|(_, users_with_post_and_comment)| users_with_post_and_comment)
                .collect::<Vec<(User, Vec<(Post, Vec<Comment>)>)>>()
        })
    }
}
