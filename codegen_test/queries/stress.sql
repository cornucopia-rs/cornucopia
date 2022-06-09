--! select_everything
SELECT * FROM Everything;

--! select_everything_null: (bool_?, boolean_?, char_?, smallint_?, int2_?, smallserial_?, serial2_?, int_?, int4_?, serial_?, serial4_?, bingint_?, int8_?, bigserial_?, serial8_?, float4_?, real_?, float8_?, double_precision_?, text_?, varchar_?, bytea_?, timestamp_?, timestamp_without_time_zone_?, timestamptz_?, timestamp_with_time_zone_?, date_?, time_?, json_?, jsonb_?, uuid_?, inet_?, macaddr_?)
SELECT * FROM Everything;

--! insert_everything
INSERT INTO Everything (bool_, boolean_, char_, smallint_, int2_, smallserial_, serial2_, int_, int4_, serial_, serial4_, bingint_, int8_, bigserial_, serial8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_)
    VALUES (:bool_, :boolean_, :char_, :smallint_, :int2_, :smallserial_, :serial2_, :int_, :int4_, :serial_, :serial4_, :bingint_, :int8_, :bigserial_, :serial8_, :float4_, :real_, :float8_, :double_precision_, :text_, :varchar_, :bytea_, :timestamp_, :timestamp_without_time_zone_, :timestamptz_, :timestamp_with_time_zone_, :date_, :time_, :json_, :jsonb_, :uuid_, :inet_, :macaddr_);

--! select_everything_array
SELECT * FROM EverythingArray;

--! select_everything_array_null: (bool_?, boolean_?, char_?, smallint_?, int2_?, int_?, int4_?, bingint_?, int8_?, float4_?, real_?, float8_?, double_precision_?, text_?, varchar_?, bytea_?, timestamp_?, timestamp_without_time_zone_?, timestamptz_?, timestamp_with_time_zone_?, date_?, time_?, json_?, jsonb_?, uuid_?, inet_?, macaddr_?)
SELECT * FROM EverythingArray;

--! insert_everything_array
INSERT INTO EverythingArray (bool_, boolean_, char_, smallint_, int2_, int_, int4_, bingint_, int8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_)
    VALUES (:bool_, :boolean_, :char_, :smallint_, :int2_, :int_, :int4_, :bingint_, :int8_, :float4_, :real_, :float8_, :double_precision_, :text_, :varchar_, :bytea_, :timestamp_, :timestamp_without_time_zone_, :timestamptz_, :timestamp_with_time_zone_, :date_, :time_, :json_, :jsonb_, :uuid_, :inet_, :macaddr_);

--! select_nightmare
SELECT * FROM nightmare;

--! insert_nightmare
INSERT INTO nightmare (composite) VALUES (:composite);