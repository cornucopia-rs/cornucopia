-- Write your migration SQL here
CREATE TABLE BooksAuthors (
    AuthorId int NOT NULL,
    BookId int NOT NULL,
    FOREIGN KEY (AuthorId) REFERENCES Authors (Id),
    FOREIGN KEY (BookId) REFERENCES Books (Id)
);

