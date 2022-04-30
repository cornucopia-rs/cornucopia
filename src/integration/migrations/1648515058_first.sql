-- Write your migration SQL here
CREATE TYPE spongebob_character AS enum (
    'Bob',
    'Patrick',
    'Squidward'
);

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
    array_ bool[],
    custom_array_ spongebob_character[],
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

INSERT INTO Everything (array_, custom_array_, bool_, boolean_, char_, smallint_, int2_, smallserial_, serial2_, int_, int4_, serial_, serial4_, bingint_, int8_, bigserial_, serial8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_)
    VALUES ('{TRUE, FALSE}', '{"Bob", "Patrick"}', TRUE, TRUE, 42::"char", 300, 300, 300, 300, 100000, 100000, 100000, 100000, 10000000000, 10000000000, 10000000000, 10000000000, 1.12, 1.12, 1.1231231231, 1.1231231231, 'hello', 'hello', '\xDEADBEEF', '2020-01-02 03:04:05', '2020-01-02 03:04:05', '1985-04-12T23:20:50.52Z', '1985-04-12T23:20:50.52Z', '1999-01-08', '04:05:06.789', '{}', '{}', 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', '127.0.0.1', '08:00:2b:01:02:03');

