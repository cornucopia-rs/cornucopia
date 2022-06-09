-- Write your migration SQL here
CREATE TABLE Item (
    id serial NOT NULL,
    name TEXT NOT NULL,
    price FLOAT,
    show BOOLEAN NOT NULL
);