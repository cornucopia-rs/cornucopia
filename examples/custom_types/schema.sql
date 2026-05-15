CREATE TYPE element AS enum (
    'anemo',
    'cryo',
    'dendro',
    'electro',
    'geo',
    'hydro',
    'pyro',
    'physical'
);

CREATE TYPE quality AS enum (
    'SR',
    'SSR'
);

CREATE TABLE characters (
    id serial NOT NULL,
    name varchar(70) NOT NULL,
    quality quality NOT NULL,
    element element NOT NULL,
    release_date date,
    PRIMARY KEY (id)
);

INSERT INTO characters
    (name, quality, element, release_date)
VALUES
    ('Amber', 'SSR', 'pyro', '2020-09-28'),
    ('Arlecchino', 'SSR', 'pyro', null),
    ('Furina', 'SSR', 'hydro', '2023-09-08'),
    ('Hu Tao', 'SSR', 'pyro', null),
    ('Kujou Sara', 'SR', 'electro', null)
;
