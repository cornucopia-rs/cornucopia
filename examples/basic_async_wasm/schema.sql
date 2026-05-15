CREATE TABLE authors (
    id serial NOT NULL,
    name varchar(70) NOT NULL,
    country varchar(100) NOT NULL,
    dob date NOT NULL,
    PRIMARY KEY (id)
);

INSERT INTO authors (name, country, dob)
    VALUES ('Agatha Christie', 'United Kingdom', '1999-01-02'), ('John Ronald Reuel Tolkien', 'United Kingdom', '2003-02-1');

CREATE TABLE books (
    id serial NOT NULL,
    title varchar(50) NOT NULL,
    translations text[] NOT NULL DEFAULT ARRAY['french', 'english'],
    PRIMARY KEY (id)
);

INSERT INTO books (title)
    VALUES ('Murder on the Orient Express'), ('Death on the Nile'), ('The Hobbit'), ('The Silmarillion');

CREATE TABLE book_authors (
    author_id int NOT NULL,
    book_id int NOT NULL,
    FOREIGN KEY (author_id) REFERENCES authors (id),
    FOREIGN KEY (book_id) REFERENCES books (id)
);

INSERT INTO book_authors (author_id, book_id)
    VALUES (1, 1), (1, 2), (2, 3), (2, 4);

CREATE TYPE spongebob_character AS enum (
    'Bob',
    'Patrick',
    'Squidward'
);

CREATE TYPE voice_actor AS (
    name text,
    age integer
);

CREATE TABLE spongebob_voice_actors (
    voice_actor voice_actor,
    character spongebob_character
);

INSERT INTO spongebob_voice_actors (voice_actor, character)
    VALUES (ROW ('Bill Fagerbakke', 65), 'Patrick');
