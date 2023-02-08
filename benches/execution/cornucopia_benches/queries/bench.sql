--: User(hair_color?)
--: Post(body?)
--: Comment()

--! users: User
SELECT * FROM users;
--! insert_user (hair_color?)
INSERT INTO users (name, hair_color) VALUES (:name, :hair_color);

--! posts: Post
SELECT * FROM posts;
--! post_by_user_ids: Post
SELECT * FROM posts WHERE user_id = ANY(:ids);

--! comments: Comment
SELECT * FROM comments;
--! comments_by_post_id: Comment
SELECT * FROM comments WHERE post_id = ANY(:ids);

--! select_complex: (hair_color?, post_id?, user_id?, title?, body?)
SELECT u.id as myuser_id, u.name, u.hair_color, p.id as post_id, p.user_id, p.title, p.body FROM users as u LEFT JOIN posts as p on u.id = p.user_id;
