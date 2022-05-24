-- Write your migration SQL here
CREATE TYPE spongebob_character AS enum (
    'Bob',
    'Patrick',
    'Squidward'
);

CREATE TYPE custom_composite AS (
    wow text,
    such_cool integer,
    nice spongebob_character
);

CREATE DOMAIN my_domain AS TEXT CHECK (value ~ '^\w{5}$');

CREATE DOMAIN custom_domain AS custom_composite[];

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
    PRIMARY KEY (Id)
);

INSERT INTO Book (Title)
    VALUES ('Murder on the Orient Express'), ('Death on the Nile'), ('The Hobbit'), ('The Silmarillion');

CREATE TABLE Everything (
    custom_domain_ custom_domain,
    domain_ my_domain,
    custom_array_ spongebob_character[],
    array_ bool[],
    bool_ bool,
    boolean_ boolean,
    char_ "char",
    smallint_ smallint,
    int2_ int2,
    smallserial_ smallserial,
    serial2_ serial2,
    int_ int,
    int4_ int4,
    serial_ serial,
    serial4_ serial4,
    bingint_ bigint,
    int8_ int8,
    bigserial_ bigserial,
    serial8_ serial8,
    float4_ float4,
    real_ real,
    float8_ float8,
    double_precision_ double precision,
    text_ text,
    varchar_ varchar,
    bytea_ bytea,
    timestamp_ timestamp,
    timestamp_without_time_zone_ timestamp without time zone,
    timestamptz_ timestamptz,
    timestamp_with_time_zone_ timestamp with time zone,
    date_ date,
    time_ time,
    json_ json,
    jsonb_ jsonb,
    uuid_ uuid,
    inet_ inet,
    macaddr_ macaddr
);

