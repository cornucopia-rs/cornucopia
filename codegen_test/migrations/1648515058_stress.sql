-- Write your migration SQL here
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
    macaddr_ macaddr
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
    macaddr_ macaddr[]
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