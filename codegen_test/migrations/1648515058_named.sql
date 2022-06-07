-- Write your migration SQL here
CREATE TABLE Item (
    id serial NOT NULL,
    name TEXT NOT NULL,
    price FLOAT NOT NULL,
    show BOOLEAN NOT NULL
);