CREATE TYPE nullity_composite AS (
    jsons JSON[],
    id INT
);

CREATE TABLE nullity (
    texts TEXT[],
    name TEXT,
    composite nullity_composite
);