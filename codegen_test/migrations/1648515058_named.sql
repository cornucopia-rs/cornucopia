-- Write your migration SQL here
CREATE TABLE named (
    id serial NOT NULL,
    name TEXT NOT NULL,
    price FLOAT,
    show BOOLEAN NOT NULL
);

CREATE TYPE named_composite AS (
    wow text,
    such_cool integer
);

CREATE TABLE named_complex (
    named named_composite
);
