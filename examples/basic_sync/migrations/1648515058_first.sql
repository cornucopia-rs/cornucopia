-- Write your migration SQL here
CREATE TABLE Author (
    Id serial NOT NULL,
    Name varchar(70) NOT NULL,
    Country varchar(100) NOT NULL,
    PRIMARY KEY (Id)
);

INSERT INTO Author (Name, Country)
    VALUES ('Agatha Christie', 'United Kingdom'), ('John Ronald Reuel Tolkien', 'United Kingdom');

CREATE TABLE Book (
    Id serial NOT NULL,
    Title varchar(50) NOT NULL,
    Translations text[] NOT NULL DEFAULT '{}',
    PRIMARY KEY (Id)
);

INSERT INTO Book (Title)
    VALUES ('Murder on the Orient Express'), ('Death on the Nile'), ('The Hobbit'), ('The Silmarillion');

