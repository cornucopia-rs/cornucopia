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

--! books_opt_ret_param ?{title}
SELECT
    Title
FROM
    Book;

--! author_name_by_id
SELECT
    Author.Name
FROM
    Author
WHERE
    Author.Id = :id;

--! author_name_starting_with
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
    Author.Name LIKE CONCAT(:start_str::text, '%');

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
WHERE (col1).persona = :spongebob_character;

--! select_translations
SELECT
    Translations
FROM
    Book;

