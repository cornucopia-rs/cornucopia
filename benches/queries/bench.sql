--: User(hair_color?)
--: Post(body?)
--: Comment()

--! users: User
SELECT * FROM users;

--! insert_user
--- Performs a bulk insert of multiple users.
---
--- Cornucopia doesn't support multi-value inserts, so we use `unnest` to transform two arrays
--- (names and hair_colors) into rows of values that can be inserted together.
INSERT INTO users (name, hair_color)
SELECT unnest(:names::text[]) as name,
       unnest(:hair_colors::text[]) as hair_color;

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
