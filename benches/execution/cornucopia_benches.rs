use std::collections::HashMap;

use criterion::Bencher;
use futures::executor::block_on;
use tokio_postgres::Client;

use generated::queries::bench::{
    Comment, Post, User,
    async_::{comments_by_post_id, insert_user, post_by_user_ids, select_complex, users},
};

pub fn bench_trivial_query(b: &mut Bencher, client: &Client) {
    let stmt = block_on(async { users().prepare(client).await.unwrap() });
    b.iter(|| block_on(async { stmt.bind(client).all().await.unwrap() }))
}

pub fn bench_medium_complex_query(b: &mut Bencher, client: &Client) {
    let stmt = block_on(async { select_complex().prepare(client).await.unwrap() });
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
    let stmt = block_on(async { insert_user().prepare(client).await.unwrap() });

    b.iter(|| {
        block_on(async {
            let names: Vec<String> = (0..size).map(|x| format!("User {x}")).collect();
            let hair_colors: Vec<String> = (0..size).map(|_| "hair_color".to_string()).collect();

            stmt.bind(client, &names, &hair_colors).await.unwrap();
        })
    })
}

pub fn loading_associations_sequentially(b: &mut Bencher, client: &Client) {
    let user_stmt = block_on(async { users().prepare(client).await.unwrap() });
    let post_stmt = block_on(async { post_by_user_ids().prepare(client).await.unwrap() });
    let comment_stmt = block_on(async { comments_by_post_id().prepare(client).await.unwrap() });
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

    use generated::queries::bench::{
        Comment, Post, User,
        sync::{comments_by_post_id, insert_user, post_by_user_ids, select_complex, users},
    };
    pub fn bench_trivial_query(b: &mut Bencher, client: &mut Client) {
        let stmt = users().prepare(client).unwrap();
        b.iter(|| stmt.bind(client).all().unwrap())
    }

    pub fn bench_medium_complex_query(b: &mut Bencher, client: &mut Client) {
        let stmt = select_complex().prepare(client).unwrap();
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
        let stmt = insert_user().prepare(client).unwrap();
        b.iter(|| {
            let names: Vec<String> = (0..size).map(|x| format!("User {x}")).collect();
            let hair_colors: Vec<String> = (0..size).map(|_| "hair_color".to_string()).collect();
            stmt.bind(client, &names, &hair_colors).unwrap();
        })
    }

    pub fn loading_associations_sequentially(b: &mut Bencher, client: &mut Client) {
        let user_stmt = users().prepare(client).unwrap();
        let post_stmt = post_by_user_ids().prepare(client).unwrap();
        let comment_stmt = comments_by_post_id().prepare(client).unwrap();

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
