--! authors()*
SELECT
    *
FROM
    Author;

--! books() {title}*
SELECT
    Title
FROM
    Book;

--! books_from_author_id(id)*
SELECT
    Book.Title
FROM
    BookAuthor
    INNER JOIN Author ON Author.Id = BookAuthor.AuthorId
    INNER JOIN Book ON Book.Id = BookAuthor.BookId
WHERE
    Author.Id = $1;

--! author_name_by_id_opt(id)?
SELECT
    Author.Name
FROM
    Author
WHERE
    Author.Id = $1;

--! author_name_by_id(id)
SELECT
    Author.Name
FROM
    Author
WHERE
    Author.Id = $1;

--! author_name_starting_with(s: TEXT)*
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
    Author.Name LIKE CONCAT($1, '%');
