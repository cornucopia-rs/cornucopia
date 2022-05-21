--! authors
SELECT
    *
FROM
    Author;

--! books
SELECT
    Title
FROM
    Book;

--! books_opt_ret ?{1}
SELECT
    Title
FROM
    Book;

--! books_from_author_id(id)
SELECT
    Book.Title
FROM
    BookAuthor
    INNER JOIN Author ON Author.Id = BookAuthor.AuthorId
    INNER JOIN Book ON Book.Id = BookAuthor.BookId
WHERE
    Author.Id = $1;

--! author_name_starting_with ?{authorid}
SELECT
    BookAuthor.AuthorId,
    Author.Name,
    BookAuthor.BookId,
    Book.Title
FROM
    BookAuthor
    INNER JOIN Author ON Author.id = BookAuthor.AuthorId
    INNER JOIN Book ON Book.Id = BookAuthor.BookId
WHERE
    Author.Name LIKE CONCAT(:s::text, '%');

--! return_custom_type
SELECT
    col1
FROM
    CustomTable;

--! select_where_custom_type
SELECT
    col2
FROM
    CustomTable
WHERE (col1).nice = :spongebob_character;

--! select_everything
SELECT
    *
FROM
    Everything;

--! insert_everything
INSERT INTO Everything (domain_, array_, custom_array_, bool_, boolean_, char_, smallint_, int2_, smallserial_, serial2_, int_, int4_, serial_, serial4_, bingint_, int8_, bigserial_, serial8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_)
    VALUES (:domain_, :array_, :custom_array_, :bool_, :boolean_, :char_, :smallint_, :int2_, :smallserial_, :serial2_, :int_, :int4_, :serial_, :serial4_, :bingint_, :int8_, :bigserial_, :serial8_, :float4_, :real_, :float8_, :double_precision_, :text_, :varchar_, :bytea_, :timestamp_, :timestamp_without_time_zone_, :timestamptz_, :timestamp_with_time_zone_, :date_, :time_, :json_, :jsonb_, :uuid_, :inet_, :macaddr_);

