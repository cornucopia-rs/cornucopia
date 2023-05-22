-- Copy

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

-- Domain

CREATE DOMAIN domain_txt AS TEXT;
CREATE DOMAIN domain_json AS JSON;
CREATE DOMAIN domain_nb AS INT;
CREATE DOMAIN domain_array AS domain_json[];

CREATE TYPE domain_composite AS (
    txt domain_txt,
    json domain_json,
    nb domain_nb,
    arr domain_array
);

CREATE TABLE nightmare_domain (
    txt domain_txt,
    json domain_json,
    nb domain_nb,
    arr domain_array,
    composite domain_composite
);

-- Named

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

CREATE TYPE "enum.with_dot" AS Enum('variant.with_dot');

CREATE TYPE "named_composite.with_dot" AS (
    "this.is.inconceivable" "enum.with_dot"
);

CREATE TABLE named_complex (
    named named_composite,
    "named.with_dot" "named_composite.with_dot"
);

-- Nullity

CREATE TYPE nullity_composite AS (
    jsons JSON[],
    id INT
);

CREATE TABLE nullity (
    texts TEXT[],
    name TEXT,
    composite nullity_composite
);

-- Params

CREATE TABLE Book (
    name TEXT NOT NULL,
    author TEXT
);

CREATE TABLE imaginary (
    a SERIAL,
    c SERIAL,
    z SERIAL,
    r SERIAL
);

-- Stress

CREATE TABLE Everything (
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
    macaddr_ macaddr,
    numeric_ numeric
);

CREATE TABLE EverythingArray (
    bool_ bool[],
    boolean_ boolean[],
    char_ "char"[],
    smallint_ smallint[],
    int2_ int2[],
    int_ int[],
    int4_ int4[],
    bingint_ bigint[],
    int8_ int8[],
    float4_ float4[],
    real_ real[],
    float8_ float8[],
    double_precision_ double precision[],
    text_ text[],
    varchar_ varchar[],
    bytea_ bytea[],
    timestamp_ timestamp[],
    timestamp_without_time_zone_ timestamp without time zone[],
    timestamptz_ timestamptz[],
    timestamp_with_time_zone_ timestamp with time zone[],
    date_ date[],
    time_ time[],
    json_ json[],
    jsonb_ jsonb[],
    uuid_ uuid[],
    inet_ inet[],
    macaddr_ macaddr[],
    numeric_ numeric[]

);

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

CREATE TYPE nightmare_composite AS (
    custom custom_composite[],
    spongebob spongebob_character[],
    domain my_domain
);

CREATE TABLE nightmare (
    composite nightmare_composite NOT NULL
);

-- Schema

CREATE SCHEMA schema;

CREATE TYPE schema.nightmare_composite AS (
    custom custom_composite[],
    spongebob spongebob_character[],
    domain my_domain
);

CREATE TABLE schema.nightmare (
    composite schema.nightmare_composite NOT NULL
);

-- Syntax

CREATE TYPE syntax_composite AS (
    async INT
);
CREATE TYPE syntax_enum AS Enum('async', 'box', 'I Love Chocolate');
CREATE TABLE Syntax (
    "trick:y" TEXT,
    async syntax_composite,
    enum syntax_enum
);
