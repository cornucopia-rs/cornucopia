--! author_name_starting_with
SELECT
    name
FROM
    Author
WHERE
    name LIKE CONCAT(:start_str::text, '%');

--! select_everything ?{bool_opt}
SELECT
    custom_domain_,
    custom_array_,
    domain_,
    array_,
    bool_,
    bool_ AS bool_opt,
    boolean_,
    char_,
    smallint_,
    int2_,
    smallserial_,
    serial2_,
    int_,
    int4_,
    serial_,
    serial4_,
    bingint_,
    int8_,
    bigserial_,
    serial8_,
    float4_,
    real_,
    float8_,
    double_precision_,
    text_,
    varchar_,
    bytea_,
    timestamp_,
    timestamp_without_time_zone_,
    timestamptz_,
    timestamp_with_time_zone_,
    date_,
    time_,
    json_,
    jsonb_,
    uuid_,
    inet_,
    macaddr_
FROM
    Everything;

--! insert_everything
INSERT INTO Everything (custom_domain_, custom_array_, domain_, array_, bool_, boolean_, char_, smallint_, int2_, smallserial_, serial2_, int_, int4_, serial_, serial4_, bingint_, int8_, bigserial_, serial8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_)
    VALUES (:custom_domain_, :custom_array_, :domain_, :array_, :bool_, :boolean_, :char_, :smallint_, :int2_, :smallserial_, :serial2_, :int_, :int4_, :serial_, :serial4_, :bingint_, :int8_, :bigserial_, :serial8_, :float4_, :real_, :float8_, :double_precision_, :text_, :varchar_, :bytea_, :timestamp_, :timestamp_without_time_zone_, :timestamptz_, :timestamp_with_time_zone_, :date_, :time_, :json_, :jsonb_, :uuid_, :inet_, :macaddr_);

