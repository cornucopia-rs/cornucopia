-- Write your migration SQL here
CREATE TYPE spongebob_character AS enum (
    'Bob',
    'Patrick',
    'Squidward'
);

CREATE TYPE custom_composite AS (
    name text,
    age integer,
    persona spongebob_character
);

CREATE TABLE CustomTable (
    col1 custom_composite,
    col2 spongebob_character
);

INSERT INTO CustomTable (col1, col2)
    VALUES (ROW ('incredible', 42, 'Patrick'), 'Bob');

CREATE TABLE BookAuthor (
    AuthorId int NOT NULL,
    BookId int NOT NULL,
    FOREIGN KEY (AuthorId) REFERENCES Author (Id),
    FOREIGN KEY (BookId) REFERENCES Book (Id)
);

INSERT INTO BookAuthor (AuthorId, BookId)
    VALUES (1, 1), (1, 2), (2, 3), (2, 4);

