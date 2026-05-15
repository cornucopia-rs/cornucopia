--: BookAuthor(id, name?, bio) : serde::Serialize, serde::Deserialize
--# cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))
--& cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))
--: BookAuthor2(id, name?, bio) : serde::Serialize, serde::Deserialize
--# allow(deprecated)

--! get_author_by_id : BookAuthor
--# deprecated = "Use get_author_v2 instead"
--# allow(dead_code)
SELECT id, name, bio FROM book_authors WHERE id = :id;

--! get_author_by_name : BookAuthor2
SELECT id, name, bio FROM book_authors WHERE name = :name;
