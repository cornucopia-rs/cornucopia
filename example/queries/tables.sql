--! authors()*
SELECT
    *
FROM
    Authors;

--! books()
SELECT
    *
FROM
    Books;

--! books_authors_one()
SELECT
    *
FROM
    BooksAuthors;

--! books_authors_zero_or_one()?
SELECT
    *
FROM
    BooksAuthors;

--! books_authors_one_or_more()*
SELECT
    *
FROM
    BooksAuthors;

--! insert_book_one()
INSERT INTO Books (title)
    VALUES ('bob')
    --! insert_book_zero_or_one()?
    INSERT INTO Books (title)
        VALUES ('bob')
        --! insert_book_zero_or_more()*
        INSERT INTO Books (title)
            VALUES ('bob')
