-- Write your migration SQL here

CREATE TABLE CustomTable (
    col1 custom_composite,
    col2 spongebob_character
);

INSERT INTO CustomTable (col1, col2)
    VALUES (ROW('incredible', 42, 'Patrick'), 'Bob');

CREATE TABLE BookAuthor (
    AuthorId int NOT NULL,
    BookId int NOT NULL,
    FOREIGN KEY (AuthorId) REFERENCES Author (Id),
    FOREIGN KEY (BookId) REFERENCES Book (Id)
);

INSERT INTO BookAuthor (AuthorId, BookId)
    VALUES (1, 1), (1, 2), (2, 3), (2, 4);

CREATE TYPE nightmare_composite AS (
    custom custom_composite[],
    spongebob spongebob_character[]
);

CREATE TABLE nightmare (
    composite nightmare_composite NOT NULL,
    name TEXT NOT NULL,
    names TEXT[] NOT NULL,
    data BYTEA,
    datas BYTEA[]
);

INSERT INTO nightmare (composite, name, names)
    VALUES (
        ROW(
            CAST (ARRAY[ROW('incredible', 42, 'Patrick')] as custom_composite[]),
            CAST (ARRAY['Bob', 'Patrick'] as spongebob_character[])
            ), 
        'Bob', 
        ARRAY['I', 'Love', 'Chocolate']
    );
    