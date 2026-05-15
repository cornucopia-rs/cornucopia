--! authors
SELECT
    *
FROM
    authors;

--! books
SELECT
    title
FROM
    books;

--! author_name_by_id
SELECT
    authors.name
FROM
    authors
WHERE
    authors.id = :id;

--! author_name_starting_with AuthorNameStartingWithParams()
SELECT
    book_authors.author_id,
    authors.name,
    book_authors.book_id,
    books.title
FROM
    book_authors
    INNER JOIN authors ON authors.id = book_authors.author_id
    INNER JOIN books ON books.id = book_authors.book_id
WHERE
    authors.name LIKE CONCAT(:start_str::text, '%');

--! select_voice_actor_with_character
SELECT
    voice_actor
FROM
    spongebob_voice_actors
WHERE
    character = :spongebob_character;

--! select_translations
SELECT
    title,
    translations
FROM
    books;
