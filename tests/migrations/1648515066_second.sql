-- Write your migration SQL here
CREATE TYPE custom_composite_type AS (
    wow text,
    such_cool integer
);

CREATE TABLE CustomTable (
    my_column custom_composite_type
);

CREATE TABLE BookAuthor (
    AuthorId int NOT NULL,
    BookId int NOT NULL,
    FOREIGN KEY (AuthorId) REFERENCES Author (Id),
    FOREIGN KEY (BookId) REFERENCES Book (Id)
);

INSERT INTO BookAuthor (AuthorId, BookId)
    VALUES (1, 1), (1, 2), (2, 3), (2, 4);

