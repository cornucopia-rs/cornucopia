-- Write your migration SQL here
CREATE DOMAIN clone_domain AS TEXT CHECK (value ~ '^\w{5}$');
CREATE DOMAIN copy_domain AS INTEGER CHECK (value > 0);

CREATE TYPE clone_composite AS (
    first INTEGER,
    second TEXT--clone_domain
);
CREATE TYPE copy_composite AS (
    first INTEGER,
    second FLOAT-- copy_domain
);

CREATE TABLE clone (
    composite clone_composite
);
CREATE TABLE copy (
    composite copy_composite
);