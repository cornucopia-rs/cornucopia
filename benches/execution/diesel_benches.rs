use criterion::Bencher;
use diesel::*;

table! {
    users {
        id -> Integer,
        name -> Text,
        hair_color -> Nullable<Text>,
    }
}

table! {
    posts {
        id -> Integer,
        user_id -> Integer,
        title -> Text,
        body -> Nullable<Text>,
    }
}

table! {
    comments {
        id -> Integer,
        post_id -> Integer,
        text -> Text,
    }
}

joinable!(comments -> posts (post_id));
joinable!(posts -> users (user_id));
allow_tables_to_appear_in_same_query!(users, posts, comments);

#[derive(
    PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Insertable, AsChangeset, QueryableByName,
)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub hair_color: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Queryable, Clone, Insertable, AsChangeset)]
#[diesel(table_name = users)]
#[diesel(treat_none_as_default_value = false)]
pub struct NewUser {
    pub name: String,
    pub hair_color: Option<String>,
}

impl NewUser {
    pub fn new(name: &str, hair_color: Option<&str>) -> Self {
        NewUser {
            name: name.to_string(),
            hair_color: hair_color.map(|s| s.to_string()),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Associations, QueryableByName)]
#[diesel(belongs_to(User))]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    user_id: i32,
    title: String,
    body: Option<String>,
}

impl NewPost {
    pub fn new(user_id: i32, title: &str, body: Option<&str>) -> Self {
        NewPost {
            user_id,
            title: title.into(),
            body: body.map(|b| b.into()),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Post))]
pub struct Comment {
    id: i32,
    post_id: i32,
    text: String,
}

#[derive(Debug, Clone, Copy, Insertable)]
#[diesel(table_name = comments)]
pub struct NewComment<'a>(
    #[diesel(column_name = post_id)] pub i32,
    #[diesel(column_name = text)] pub &'a str,
);

fn insert_users<F: Fn(usize) -> Option<&'static str>, const N: usize>(
    conn: &mut PgConnection,
    hair_color_init: F,
) {
    const DUMMY_USER: NewUser = NewUser {
        name: String::new(),
        hair_color: None,
    };

    // There are stackoverflows on windows otherwise
    if N > 1_000 {
        let mut data = Box::new([DUMMY_USER; N]);

        for (idx, user) in data.iter_mut().enumerate() {
            *user = NewUser::new(&format!("User {idx}"), hair_color_init(idx));
        }

        insert_into(users::table)
            .values(data)
            .execute(conn)
            .unwrap();
    } else {
        let mut data = [DUMMY_USER; N];

        for (idx, user) in data.iter_mut().enumerate() {
            *user = NewUser::new(&format!("User {idx}"), hair_color_init(idx));
        }

        insert_into(users::table)
            .values(data)
            .execute(conn)
            .unwrap();
    }
}

pub fn bench_trivial_query(b: &mut Bencher, conn: &mut PgConnection) {
    b.iter(|| users::table.load::<User>(conn).unwrap())
}

pub fn bench_medium_complex_query(b: &mut Bencher, conn: &mut PgConnection) {
    b.iter(|| {
        use self::users::dsl::*;
        let target = users.left_outer_join(posts::table);
        target.load::<(User, Option<Post>)>(conn).unwrap()
    })
}

pub fn bench_insert(b: &mut Bencher, conn: &mut PgConnection, size: usize) {
    #[inline(always)]
    fn hair_color_callback(_: usize) -> Option<&'static str> {
        Some("hair_color")
    }

    let insert: fn(&mut PgConnection) = match size {
        1 => |conn| insert_users::<_, 1>(conn, hair_color_callback),
        10 => |conn| insert_users::<_, 10>(conn, hair_color_callback),
        25 => |conn| insert_users::<_, 25>(conn, hair_color_callback),
        50 => |conn| insert_users::<_, 50>(conn, hair_color_callback),
        100 => |conn| insert_users::<_, 100>(conn, hair_color_callback),
        1000 => |conn| insert_users::<_, 1000>(conn, hair_color_callback),
        _ => unimplemented!(),
    };

    b.iter(|| insert(conn))
}

pub fn loading_associations_sequentially(b: &mut Bencher, conn: &mut PgConnection) {
    b.iter(|| {
        let users = users::table.load::<User>(conn).unwrap();
        let posts = Post::belonging_to(&users).load::<Post>(conn).unwrap();
        let comments = Comment::belonging_to(&posts)
            .load::<Comment>(conn)
            .unwrap()
            .grouped_by(&posts);
        let posts_and_comments = posts.into_iter().zip(comments).grouped_by(&users);
        users
            .into_iter()
            .zip(posts_and_comments)
            .collect::<Vec<(User, Vec<(Post, Vec<Comment>)>)>>()
    })
}
