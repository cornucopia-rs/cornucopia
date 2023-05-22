// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct EverythingParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::BytesSql,
    T4: crate::JsonSql,
    T5: crate::JsonSql,
> {
    pub bool_: bool,
    pub boolean_: bool,
    pub char_: i8,
    pub smallint_: i16,
    pub int2_: i16,
    pub smallserial_: i16,
    pub serial2_: i16,
    pub int_: i32,
    pub int4_: i32,
    pub serial_: i32,
    pub serial4_: i32,
    pub bingint_: i64,
    pub int8_: i64,
    pub bigserial_: i64,
    pub serial8_: i64,
    pub float4_: f32,
    pub real_: f32,
    pub float8_: f64,
    pub double_precision_: f64,
    pub text_: T1,
    pub varchar_: T2,
    pub bytea_: T3,
    pub timestamp_: time::PrimitiveDateTime,
    pub timestamp_without_time_zone_: time::PrimitiveDateTime,
    pub timestamptz_: time::OffsetDateTime,
    pub timestamp_with_time_zone_: time::OffsetDateTime,
    pub date_: time::Date,
    pub time_: time::Time,
    pub json_: T4,
    pub jsonb_: T5,
    pub uuid_: uuid::Uuid,
    pub inet_: std::net::IpAddr,
    pub macaddr_: eui48::MacAddress,
    pub numeric_: rust_decimal::Decimal,
}
#[derive(Debug)]
pub struct EverythingArrayParams<
    T1: crate::ArraySql<Item = bool>,
    T2: crate::ArraySql<Item = bool>,
    T3: crate::ArraySql<Item = i8>,
    T4: crate::ArraySql<Item = i16>,
    T5: crate::ArraySql<Item = i16>,
    T6: crate::ArraySql<Item = i32>,
    T7: crate::ArraySql<Item = i32>,
    T8: crate::ArraySql<Item = i64>,
    T9: crate::ArraySql<Item = i64>,
    T10: crate::ArraySql<Item = f32>,
    T11: crate::ArraySql<Item = f32>,
    T12: crate::ArraySql<Item = f64>,
    T13: crate::ArraySql<Item = f64>,
    T14: crate::StringSql,
    T15: crate::ArraySql<Item = T14>,
    T16: crate::StringSql,
    T17: crate::ArraySql<Item = T16>,
    T18: crate::BytesSql,
    T19: crate::ArraySql<Item = T18>,
    T20: crate::ArraySql<Item = time::PrimitiveDateTime>,
    T21: crate::ArraySql<Item = time::PrimitiveDateTime>,
    T22: crate::ArraySql<Item = time::OffsetDateTime>,
    T23: crate::ArraySql<Item = time::OffsetDateTime>,
    T24: crate::ArraySql<Item = time::Date>,
    T25: crate::ArraySql<Item = time::Time>,
    T26: crate::JsonSql,
    T27: crate::ArraySql<Item = T26>,
    T28: crate::JsonSql,
    T29: crate::ArraySql<Item = T28>,
    T30: crate::ArraySql<Item = uuid::Uuid>,
    T31: crate::ArraySql<Item = std::net::IpAddr>,
    T32: crate::ArraySql<Item = eui48::MacAddress>,
    T33: crate::ArraySql<Item = rust_decimal::Decimal>,
> {
    pub bool_: T1,
    pub boolean_: T2,
    pub char_: T3,
    pub smallint_: T4,
    pub int2_: T5,
    pub int_: T6,
    pub int4_: T7,
    pub bingint_: T8,
    pub int8_: T9,
    pub float4_: T10,
    pub real_: T11,
    pub float8_: T12,
    pub double_precision_: T13,
    pub text_: T15,
    pub varchar_: T17,
    pub bytea_: T19,
    pub timestamp_: T20,
    pub timestamp_without_time_zone_: T21,
    pub timestamptz_: T22,
    pub timestamp_with_time_zone_: T23,
    pub date_: T24,
    pub time_: T25,
    pub json_: T27,
    pub jsonb_: T29,
    pub uuid_: T30,
    pub inet_: T31,
    pub macaddr_: T32,
    pub numeric_: T33,
}
#[derive(serde::Serialize, Debug, Clone, PartialEq)]
pub struct Everything {
    pub bool_: bool,
    pub boolean_: bool,
    pub char_: i8,
    pub smallint_: i16,
    pub int2_: i16,
    pub smallserial_: i16,
    pub serial2_: i16,
    pub int_: i32,
    pub int4_: i32,
    pub serial_: i32,
    pub serial4_: i32,
    pub bingint_: i64,
    pub int8_: i64,
    pub bigserial_: i64,
    pub serial8_: i64,
    pub float4_: f32,
    pub real_: f32,
    pub float8_: f64,
    pub double_precision_: f64,
    pub text_: String,
    pub varchar_: String,
    pub bytea_: Vec<u8>,
    pub timestamp_: time::PrimitiveDateTime,
    pub timestamp_without_time_zone_: time::PrimitiveDateTime,
    pub timestamptz_: time::OffsetDateTime,
    pub timestamp_with_time_zone_: time::OffsetDateTime,
    pub date_: time::Date,
    pub time_: time::Time,
    pub json_: serde_json::Value,
    pub jsonb_: serde_json::Value,
    pub uuid_: uuid::Uuid,
    pub inet_: std::net::IpAddr,
    pub macaddr_: eui48::MacAddress,
    pub numeric_: rust_decimal::Decimal,
}
pub struct EverythingBorrowed<'a> {
    pub bool_: bool,
    pub boolean_: bool,
    pub char_: i8,
    pub smallint_: i16,
    pub int2_: i16,
    pub smallserial_: i16,
    pub serial2_: i16,
    pub int_: i32,
    pub int4_: i32,
    pub serial_: i32,
    pub serial4_: i32,
    pub bingint_: i64,
    pub int8_: i64,
    pub bigserial_: i64,
    pub serial8_: i64,
    pub float4_: f32,
    pub real_: f32,
    pub float8_: f64,
    pub double_precision_: f64,
    pub text_: &'a str,
    pub varchar_: &'a str,
    pub bytea_: &'a [u8],
    pub timestamp_: time::PrimitiveDateTime,
    pub timestamp_without_time_zone_: time::PrimitiveDateTime,
    pub timestamptz_: time::OffsetDateTime,
    pub timestamp_with_time_zone_: time::OffsetDateTime,
    pub date_: time::Date,
    pub time_: time::Time,
    pub json_: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub jsonb_: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub uuid_: uuid::Uuid,
    pub inet_: std::net::IpAddr,
    pub macaddr_: eui48::MacAddress,
    pub numeric_: rust_decimal::Decimal,
}
impl<'a> From<EverythingBorrowed<'a>> for Everything {
    fn from(
        EverythingBorrowed {
            bool_,
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
            macaddr_,
            numeric_,
        }: EverythingBorrowed<'a>,
    ) -> Self {
        Self {
            bool_,
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
            text_: text_.into(),
            varchar_: varchar_.into(),
            bytea_: bytea_.into(),
            timestamp_,
            timestamp_without_time_zone_,
            timestamptz_,
            timestamp_with_time_zone_,
            date_,
            time_,
            json_: serde_json::from_str(json_.0.get()).unwrap(),
            jsonb_: serde_json::from_str(jsonb_.0.get()).unwrap(),
            uuid_,
            inet_,
            macaddr_,
            numeric_,
        }
    }
}
#[derive(serde::Serialize, Debug, Clone, PartialEq)]
pub struct EverythingNull {
    pub bool_: Option<bool>,
    pub boolean_: Option<bool>,
    pub char_: Option<i8>,
    pub smallint_: Option<i16>,
    pub int2_: Option<i16>,
    pub smallserial_: Option<i16>,
    pub serial2_: Option<i16>,
    pub int_: Option<i32>,
    pub int4_: Option<i32>,
    pub serial_: Option<i32>,
    pub serial4_: Option<i32>,
    pub bingint_: Option<i64>,
    pub int8_: Option<i64>,
    pub bigserial_: Option<i64>,
    pub serial8_: Option<i64>,
    pub float4_: Option<f32>,
    pub real_: Option<f32>,
    pub float8_: Option<f64>,
    pub double_precision_: Option<f64>,
    pub text_: Option<String>,
    pub varchar_: Option<String>,
    pub bytea_: Option<Vec<u8>>,
    pub timestamp_: Option<time::PrimitiveDateTime>,
    pub timestamp_without_time_zone_: Option<time::PrimitiveDateTime>,
    pub timestamptz_: Option<time::OffsetDateTime>,
    pub timestamp_with_time_zone_: Option<time::OffsetDateTime>,
    pub date_: Option<time::Date>,
    pub time_: Option<time::Time>,
    pub json_: Option<serde_json::Value>,
    pub jsonb_: Option<serde_json::Value>,
    pub uuid_: Option<uuid::Uuid>,
    pub inet_: Option<std::net::IpAddr>,
    pub macaddr_: Option<eui48::MacAddress>,
    pub numeric_: Option<rust_decimal::Decimal>,
}
pub struct EverythingNullBorrowed<'a> {
    pub bool_: Option<bool>,
    pub boolean_: Option<bool>,
    pub char_: Option<i8>,
    pub smallint_: Option<i16>,
    pub int2_: Option<i16>,
    pub smallserial_: Option<i16>,
    pub serial2_: Option<i16>,
    pub int_: Option<i32>,
    pub int4_: Option<i32>,
    pub serial_: Option<i32>,
    pub serial4_: Option<i32>,
    pub bingint_: Option<i64>,
    pub int8_: Option<i64>,
    pub bigserial_: Option<i64>,
    pub serial8_: Option<i64>,
    pub float4_: Option<f32>,
    pub real_: Option<f32>,
    pub float8_: Option<f64>,
    pub double_precision_: Option<f64>,
    pub text_: Option<&'a str>,
    pub varchar_: Option<&'a str>,
    pub bytea_: Option<&'a [u8]>,
    pub timestamp_: Option<time::PrimitiveDateTime>,
    pub timestamp_without_time_zone_: Option<time::PrimitiveDateTime>,
    pub timestamptz_: Option<time::OffsetDateTime>,
    pub timestamp_with_time_zone_: Option<time::OffsetDateTime>,
    pub date_: Option<time::Date>,
    pub time_: Option<time::Time>,
    pub json_: Option<postgres_types::Json<&'a serde_json::value::RawValue>>,
    pub jsonb_: Option<postgres_types::Json<&'a serde_json::value::RawValue>>,
    pub uuid_: Option<uuid::Uuid>,
    pub inet_: Option<std::net::IpAddr>,
    pub macaddr_: Option<eui48::MacAddress>,
    pub numeric_: Option<rust_decimal::Decimal>,
}
impl<'a> From<EverythingNullBorrowed<'a>> for EverythingNull {
    fn from(
        EverythingNullBorrowed {
            bool_,
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
            macaddr_,
            numeric_,
        }: EverythingNullBorrowed<'a>,
    ) -> Self {
        Self {
            bool_,
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
            text_: text_.map(|v| v.into()),
            varchar_: varchar_.map(|v| v.into()),
            bytea_: bytea_.map(|v| v.into()),
            timestamp_,
            timestamp_without_time_zone_,
            timestamptz_,
            timestamp_with_time_zone_,
            date_,
            time_,
            json_: json_.map(|v| serde_json::from_str(v.0.get()).unwrap()),
            jsonb_: jsonb_.map(|v| serde_json::from_str(v.0.get()).unwrap()),
            uuid_,
            inet_,
            macaddr_,
            numeric_,
        }
    }
}
#[derive(serde::Serialize, Debug, Clone, PartialEq)]
pub struct EverythingArray {
    pub bool_: Vec<bool>,
    pub boolean_: Vec<bool>,
    pub char_: Vec<i8>,
    pub smallint_: Vec<i16>,
    pub int2_: Vec<i16>,
    pub int_: Vec<i32>,
    pub int4_: Vec<i32>,
    pub bingint_: Vec<i64>,
    pub int8_: Vec<i64>,
    pub float4_: Vec<f32>,
    pub real_: Vec<f32>,
    pub float8_: Vec<f64>,
    pub double_precision_: Vec<f64>,
    pub text_: Vec<String>,
    pub varchar_: Vec<String>,
    pub bytea_: Vec<Vec<u8>>,
    pub timestamp_: Vec<time::PrimitiveDateTime>,
    pub timestamp_without_time_zone_: Vec<time::PrimitiveDateTime>,
    pub timestamptz_: Vec<time::OffsetDateTime>,
    pub timestamp_with_time_zone_: Vec<time::OffsetDateTime>,
    pub date_: Vec<time::Date>,
    pub time_: Vec<time::Time>,
    pub json_: Vec<serde_json::Value>,
    pub jsonb_: Vec<serde_json::Value>,
    pub uuid_: Vec<uuid::Uuid>,
    pub inet_: Vec<std::net::IpAddr>,
    pub macaddr_: Vec<eui48::MacAddress>,
    pub numeric_: Vec<rust_decimal::Decimal>,
}
pub struct EverythingArrayBorrowed<'a> {
    pub bool_: crate::ArrayIterator<'a, bool>,
    pub boolean_: crate::ArrayIterator<'a, bool>,
    pub char_: crate::ArrayIterator<'a, i8>,
    pub smallint_: crate::ArrayIterator<'a, i16>,
    pub int2_: crate::ArrayIterator<'a, i16>,
    pub int_: crate::ArrayIterator<'a, i32>,
    pub int4_: crate::ArrayIterator<'a, i32>,
    pub bingint_: crate::ArrayIterator<'a, i64>,
    pub int8_: crate::ArrayIterator<'a, i64>,
    pub float4_: crate::ArrayIterator<'a, f32>,
    pub real_: crate::ArrayIterator<'a, f32>,
    pub float8_: crate::ArrayIterator<'a, f64>,
    pub double_precision_: crate::ArrayIterator<'a, f64>,
    pub text_: crate::ArrayIterator<'a, &'a str>,
    pub varchar_: crate::ArrayIterator<'a, &'a str>,
    pub bytea_: crate::ArrayIterator<'a, &'a [u8]>,
    pub timestamp_: crate::ArrayIterator<'a, time::PrimitiveDateTime>,
    pub timestamp_without_time_zone_: crate::ArrayIterator<'a, time::PrimitiveDateTime>,
    pub timestamptz_: crate::ArrayIterator<'a, time::OffsetDateTime>,
    pub timestamp_with_time_zone_: crate::ArrayIterator<'a, time::OffsetDateTime>,
    pub date_: crate::ArrayIterator<'a, time::Date>,
    pub time_: crate::ArrayIterator<'a, time::Time>,
    pub json_: crate::ArrayIterator<'a, postgres_types::Json<&'a serde_json::value::RawValue>>,
    pub jsonb_: crate::ArrayIterator<'a, postgres_types::Json<&'a serde_json::value::RawValue>>,
    pub uuid_: crate::ArrayIterator<'a, uuid::Uuid>,
    pub inet_: crate::ArrayIterator<'a, std::net::IpAddr>,
    pub macaddr_: crate::ArrayIterator<'a, eui48::MacAddress>,
    pub numeric_: crate::ArrayIterator<'a, rust_decimal::Decimal>,
}
impl<'a> From<EverythingArrayBorrowed<'a>> for EverythingArray {
    fn from(
        EverythingArrayBorrowed {
            bool_,
            boolean_,
            char_,
            smallint_,
            int2_,
            int_,
            int4_,
            bingint_,
            int8_,
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
            macaddr_,
            numeric_,
        }: EverythingArrayBorrowed<'a>,
    ) -> Self {
        Self {
            bool_: bool_.map(|v| v).collect(),
            boolean_: boolean_.map(|v| v).collect(),
            char_: char_.map(|v| v).collect(),
            smallint_: smallint_.map(|v| v).collect(),
            int2_: int2_.map(|v| v).collect(),
            int_: int_.map(|v| v).collect(),
            int4_: int4_.map(|v| v).collect(),
            bingint_: bingint_.map(|v| v).collect(),
            int8_: int8_.map(|v| v).collect(),
            float4_: float4_.map(|v| v).collect(),
            real_: real_.map(|v| v).collect(),
            float8_: float8_.map(|v| v).collect(),
            double_precision_: double_precision_.map(|v| v).collect(),
            text_: text_.map(|v| v.into()).collect(),
            varchar_: varchar_.map(|v| v.into()).collect(),
            bytea_: bytea_.map(|v| v.into()).collect(),
            timestamp_: timestamp_.map(|v| v).collect(),
            timestamp_without_time_zone_: timestamp_without_time_zone_.map(|v| v).collect(),
            timestamptz_: timestamptz_.map(|v| v).collect(),
            timestamp_with_time_zone_: timestamp_with_time_zone_.map(|v| v).collect(),
            date_: date_.map(|v| v).collect(),
            time_: time_.map(|v| v).collect(),
            json_: json_
                .map(|v| serde_json::from_str(v.0.get()).unwrap())
                .collect(),
            jsonb_: jsonb_
                .map(|v| serde_json::from_str(v.0.get()).unwrap())
                .collect(),
            uuid_: uuid_.map(|v| v).collect(),
            inet_: inet_.map(|v| v).collect(),
            macaddr_: macaddr_.map(|v| v).collect(),
            numeric_: numeric_.map(|v| v).collect(),
        }
    }
}
#[derive(serde::Serialize, Debug, Clone, PartialEq)]
pub struct EverythingArrayNull {
    pub bool_: Option<Vec<bool>>,
    pub boolean_: Option<Vec<bool>>,
    pub char_: Option<Vec<i8>>,
    pub smallint_: Option<Vec<i16>>,
    pub int2_: Option<Vec<i16>>,
    pub int_: Option<Vec<i32>>,
    pub int4_: Option<Vec<i32>>,
    pub bingint_: Option<Vec<i64>>,
    pub int8_: Option<Vec<i64>>,
    pub float4_: Option<Vec<f32>>,
    pub real_: Option<Vec<f32>>,
    pub float8_: Option<Vec<f64>>,
    pub double_precision_: Option<Vec<f64>>,
    pub text_: Option<Vec<String>>,
    pub varchar_: Option<Vec<String>>,
    pub bytea_: Option<Vec<Vec<u8>>>,
    pub timestamp_: Option<Vec<time::PrimitiveDateTime>>,
    pub timestamp_without_time_zone_: Option<Vec<time::PrimitiveDateTime>>,
    pub timestamptz_: Option<Vec<time::OffsetDateTime>>,
    pub timestamp_with_time_zone_: Option<Vec<time::OffsetDateTime>>,
    pub date_: Option<Vec<time::Date>>,
    pub time_: Option<Vec<time::Time>>,
    pub json_: Option<Vec<serde_json::Value>>,
    pub jsonb_: Option<Vec<serde_json::Value>>,
    pub uuid_: Option<Vec<uuid::Uuid>>,
    pub inet_: Option<Vec<std::net::IpAddr>>,
    pub macaddr_: Option<Vec<eui48::MacAddress>>,
    pub numeric_: Option<Vec<rust_decimal::Decimal>>,
}
pub struct EverythingArrayNullBorrowed<'a> {
    pub bool_: Option<crate::ArrayIterator<'a, bool>>,
    pub boolean_: Option<crate::ArrayIterator<'a, bool>>,
    pub char_: Option<crate::ArrayIterator<'a, i8>>,
    pub smallint_: Option<crate::ArrayIterator<'a, i16>>,
    pub int2_: Option<crate::ArrayIterator<'a, i16>>,
    pub int_: Option<crate::ArrayIterator<'a, i32>>,
    pub int4_: Option<crate::ArrayIterator<'a, i32>>,
    pub bingint_: Option<crate::ArrayIterator<'a, i64>>,
    pub int8_: Option<crate::ArrayIterator<'a, i64>>,
    pub float4_: Option<crate::ArrayIterator<'a, f32>>,
    pub real_: Option<crate::ArrayIterator<'a, f32>>,
    pub float8_: Option<crate::ArrayIterator<'a, f64>>,
    pub double_precision_: Option<crate::ArrayIterator<'a, f64>>,
    pub text_: Option<crate::ArrayIterator<'a, &'a str>>,
    pub varchar_: Option<crate::ArrayIterator<'a, &'a str>>,
    pub bytea_: Option<crate::ArrayIterator<'a, &'a [u8]>>,
    pub timestamp_: Option<crate::ArrayIterator<'a, time::PrimitiveDateTime>>,
    pub timestamp_without_time_zone_: Option<crate::ArrayIterator<'a, time::PrimitiveDateTime>>,
    pub timestamptz_: Option<crate::ArrayIterator<'a, time::OffsetDateTime>>,
    pub timestamp_with_time_zone_: Option<crate::ArrayIterator<'a, time::OffsetDateTime>>,
    pub date_: Option<crate::ArrayIterator<'a, time::Date>>,
    pub time_: Option<crate::ArrayIterator<'a, time::Time>>,
    pub json_:
        Option<crate::ArrayIterator<'a, postgres_types::Json<&'a serde_json::value::RawValue>>>,
    pub jsonb_:
        Option<crate::ArrayIterator<'a, postgres_types::Json<&'a serde_json::value::RawValue>>>,
    pub uuid_: Option<crate::ArrayIterator<'a, uuid::Uuid>>,
    pub inet_: Option<crate::ArrayIterator<'a, std::net::IpAddr>>,
    pub macaddr_: Option<crate::ArrayIterator<'a, eui48::MacAddress>>,
    pub numeric_: Option<crate::ArrayIterator<'a, rust_decimal::Decimal>>,
}
impl<'a> From<EverythingArrayNullBorrowed<'a>> for EverythingArrayNull {
    fn from(
        EverythingArrayNullBorrowed {
            bool_,
            boolean_,
            char_,
            smallint_,
            int2_,
            int_,
            int4_,
            bingint_,
            int8_,
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
            macaddr_,
            numeric_,
        }: EverythingArrayNullBorrowed<'a>,
    ) -> Self {
        Self {
            bool_: bool_.map(|v| v.map(|v| v).collect()),
            boolean_: boolean_.map(|v| v.map(|v| v).collect()),
            char_: char_.map(|v| v.map(|v| v).collect()),
            smallint_: smallint_.map(|v| v.map(|v| v).collect()),
            int2_: int2_.map(|v| v.map(|v| v).collect()),
            int_: int_.map(|v| v.map(|v| v).collect()),
            int4_: int4_.map(|v| v.map(|v| v).collect()),
            bingint_: bingint_.map(|v| v.map(|v| v).collect()),
            int8_: int8_.map(|v| v.map(|v| v).collect()),
            float4_: float4_.map(|v| v.map(|v| v).collect()),
            real_: real_.map(|v| v.map(|v| v).collect()),
            float8_: float8_.map(|v| v.map(|v| v).collect()),
            double_precision_: double_precision_.map(|v| v.map(|v| v).collect()),
            text_: text_.map(|v| v.map(|v| v.into()).collect()),
            varchar_: varchar_.map(|v| v.map(|v| v.into()).collect()),
            bytea_: bytea_.map(|v| v.map(|v| v.into()).collect()),
            timestamp_: timestamp_.map(|v| v.map(|v| v).collect()),
            timestamp_without_time_zone_: timestamp_without_time_zone_
                .map(|v| v.map(|v| v).collect()),
            timestamptz_: timestamptz_.map(|v| v.map(|v| v).collect()),
            timestamp_with_time_zone_: timestamp_with_time_zone_.map(|v| v.map(|v| v).collect()),
            date_: date_.map(|v| v.map(|v| v).collect()),
            time_: time_.map(|v| v.map(|v| v).collect()),
            json_: json_.map(|v| {
                v.map(|v| serde_json::from_str(v.0.get()).unwrap())
                    .collect()
            }),
            jsonb_: jsonb_.map(|v| {
                v.map(|v| serde_json::from_str(v.0.get()).unwrap())
                    .collect()
            }),
            uuid_: uuid_.map(|v| v.map(|v| v).collect()),
            inet_: inet_.map(|v| v.map(|v| v).collect()),
            macaddr_: macaddr_.map(|v| v.map(|v| v).collect()),
            numeric_: numeric_.map(|v| v.map(|v| v).collect()),
        }
    }
}
pub mod sync {
    use postgres::{fallible_iterator::FallibleIterator, GenericClient};
    pub struct EverythingQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> super::EverythingBorrowed,
        mapper: fn(super::EverythingBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> EverythingQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::EverythingBorrowed) -> R,
        ) -> EverythingQuery<'a, C, R, N> {
            EverythingQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            let row = self.client.query_one(stmt, &self.params)?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)?
                .map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
        {
            let stmt = self.stmt.prepare(self.client)?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))?
                .iterator()
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
            Ok(it)
        }
    }
    pub struct EverythingNullQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> super::EverythingNullBorrowed,
        mapper: fn(super::EverythingNullBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> EverythingNullQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::EverythingNullBorrowed) -> R,
        ) -> EverythingNullQuery<'a, C, R, N> {
            EverythingNullQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            let row = self.client.query_one(stmt, &self.params)?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)?
                .map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
        {
            let stmt = self.stmt.prepare(self.client)?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))?
                .iterator()
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
            Ok(it)
        }
    }
    pub struct EverythingArrayQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> super::EverythingArrayBorrowed,
        mapper: fn(super::EverythingArrayBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> EverythingArrayQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::EverythingArrayBorrowed) -> R,
        ) -> EverythingArrayQuery<'a, C, R, N> {
            EverythingArrayQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            let row = self.client.query_one(stmt, &self.params)?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)?
                .map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
        {
            let stmt = self.stmt.prepare(self.client)?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))?
                .iterator()
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
            Ok(it)
        }
    }
    pub struct EverythingArrayNullQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> super::EverythingArrayNullBorrowed,
        mapper: fn(super::EverythingArrayNullBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> EverythingArrayNullQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::EverythingArrayNullBorrowed) -> R,
        ) -> EverythingArrayNullQuery<'a, C, R, N> {
            EverythingArrayNullQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            let row = self.client.query_one(stmt, &self.params)?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)?
                .map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
        {
            let stmt = self.stmt.prepare(self.client)?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))?
                .iterator()
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
            Ok(it)
        }
    }
    pub struct NightmareCompositeQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> crate::types::NightmareCompositeBorrowed,
        mapper: fn(crate::types::NightmareCompositeBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> NightmareCompositeQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(crate::types::NightmareCompositeBorrowed) -> R,
        ) -> NightmareCompositeQuery<'a, C, R, N> {
            NightmareCompositeQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            let row = self.client.query_one(stmt, &self.params)?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)?
                .map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
        {
            let stmt = self.stmt.prepare(self.client)?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))?
                .iterator()
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
            Ok(it)
        }
    }
    pub fn select_everything() -> SelectEverythingStmt {
        SelectEverythingStmt(crate::client::sync::Stmt::new(
            "SELECT
    *
FROM
    Everything",
        ))
    }
    pub struct SelectEverythingStmt(crate::client::sync::Stmt);
    impl SelectEverythingStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
        ) -> EverythingQuery<'a, C, super::Everything, 0> {
            EverythingQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| super::EverythingBorrowed {
                    bool_: row.get(0),
                    boolean_: row.get(1),
                    char_: row.get(2),
                    smallint_: row.get(3),
                    int2_: row.get(4),
                    smallserial_: row.get(5),
                    serial2_: row.get(6),
                    int_: row.get(7),
                    int4_: row.get(8),
                    serial_: row.get(9),
                    serial4_: row.get(10),
                    bingint_: row.get(11),
                    int8_: row.get(12),
                    bigserial_: row.get(13),
                    serial8_: row.get(14),
                    float4_: row.get(15),
                    real_: row.get(16),
                    float8_: row.get(17),
                    double_precision_: row.get(18),
                    text_: row.get(19),
                    varchar_: row.get(20),
                    bytea_: row.get(21),
                    timestamp_: row.get(22),
                    timestamp_without_time_zone_: row.get(23),
                    timestamptz_: row.get(24),
                    timestamp_with_time_zone_: row.get(25),
                    date_: row.get(26),
                    time_: row.get(27),
                    json_: row.get(28),
                    jsonb_: row.get(29),
                    uuid_: row.get(30),
                    inet_: row.get(31),
                    macaddr_: row.get(32),
                    numeric_: row.get(33),
                },
                mapper: |it| <super::Everything>::from(it),
            }
        }
    }
    pub fn select_everything_null() -> SelectEverythingNullStmt {
        SelectEverythingNullStmt(crate::client::sync::Stmt::new(
            "SELECT
    *
FROM
    Everything",
        ))
    }
    pub struct SelectEverythingNullStmt(crate::client::sync::Stmt);
    impl SelectEverythingNullStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
        ) -> EverythingNullQuery<'a, C, super::EverythingNull, 0> {
            EverythingNullQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| super::EverythingNullBorrowed {
                    bool_: row.get(0),
                    boolean_: row.get(1),
                    char_: row.get(2),
                    smallint_: row.get(3),
                    int2_: row.get(4),
                    smallserial_: row.get(5),
                    serial2_: row.get(6),
                    int_: row.get(7),
                    int4_: row.get(8),
                    serial_: row.get(9),
                    serial4_: row.get(10),
                    bingint_: row.get(11),
                    int8_: row.get(12),
                    bigserial_: row.get(13),
                    serial8_: row.get(14),
                    float4_: row.get(15),
                    real_: row.get(16),
                    float8_: row.get(17),
                    double_precision_: row.get(18),
                    text_: row.get(19),
                    varchar_: row.get(20),
                    bytea_: row.get(21),
                    timestamp_: row.get(22),
                    timestamp_without_time_zone_: row.get(23),
                    timestamptz_: row.get(24),
                    timestamp_with_time_zone_: row.get(25),
                    date_: row.get(26),
                    time_: row.get(27),
                    json_: row.get(28),
                    jsonb_: row.get(29),
                    uuid_: row.get(30),
                    inet_: row.get(31),
                    macaddr_: row.get(32),
                    numeric_: row.get(33),
                },
                mapper: |it| <super::EverythingNull>::from(it),
            }
        }
    }
    pub fn insert_everything() -> InsertEverythingStmt {
        InsertEverythingStmt(crate::client::sync :: Stmt :: new("INSERT INTO Everything (bool_, boolean_, char_, smallint_, int2_, smallserial_, serial2_, int_, int4_, serial_, serial4_, bingint_, int8_, bigserial_, serial8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_, numeric_)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34)"))
    }
    pub struct InsertEverythingStmt(crate::client::sync::Stmt);
    impl InsertEverythingStmt {
        pub fn bind<
            'a,
            C: GenericClient,
            T1: crate::StringSql,
            T2: crate::StringSql,
            T3: crate::BytesSql,
            T4: crate::JsonSql,
            T5: crate::JsonSql,
        >(
            &'a mut self,
            client: &'a mut C,
            bool_: &'a bool,
            boolean_: &'a bool,
            char_: &'a i8,
            smallint_: &'a i16,
            int2_: &'a i16,
            smallserial_: &'a i16,
            serial2_: &'a i16,
            int_: &'a i32,
            int4_: &'a i32,
            serial_: &'a i32,
            serial4_: &'a i32,
            bingint_: &'a i64,
            int8_: &'a i64,
            bigserial_: &'a i64,
            serial8_: &'a i64,
            float4_: &'a f32,
            real_: &'a f32,
            float8_: &'a f64,
            double_precision_: &'a f64,
            text_: &'a T1,
            varchar_: &'a T2,
            bytea_: &'a T3,
            timestamp_: &'a time::PrimitiveDateTime,
            timestamp_without_time_zone_: &'a time::PrimitiveDateTime,
            timestamptz_: &'a time::OffsetDateTime,
            timestamp_with_time_zone_: &'a time::OffsetDateTime,
            date_: &'a time::Date,
            time_: &'a time::Time,
            json_: &'a T4,
            jsonb_: &'a T5,
            uuid_: &'a uuid::Uuid,
            inet_: &'a std::net::IpAddr,
            macaddr_: &'a eui48::MacAddress,
            numeric_: &'a rust_decimal::Decimal,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(
                stmt,
                &[
                    bool_,
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
                    macaddr_,
                    numeric_,
                ],
            )
        }
    }
    impl<
            'a,
            C: GenericClient,
            T1: crate::StringSql,
            T2: crate::StringSql,
            T3: crate::BytesSql,
            T4: crate::JsonSql,
            T5: crate::JsonSql,
        >
        crate::client::sync::Params<
            'a,
            super::EverythingParams<T1, T2, T3, T4, T5>,
            Result<u64, postgres::Error>,
            C,
        > for InsertEverythingStmt
    {
        fn params(
            &'a mut self,
            client: &'a mut C,
            params: &'a super::EverythingParams<T1, T2, T3, T4, T5>,
        ) -> Result<u64, postgres::Error> {
            self.bind(
                client,
                &params.bool_,
                &params.boolean_,
                &params.char_,
                &params.smallint_,
                &params.int2_,
                &params.smallserial_,
                &params.serial2_,
                &params.int_,
                &params.int4_,
                &params.serial_,
                &params.serial4_,
                &params.bingint_,
                &params.int8_,
                &params.bigserial_,
                &params.serial8_,
                &params.float4_,
                &params.real_,
                &params.float8_,
                &params.double_precision_,
                &params.text_,
                &params.varchar_,
                &params.bytea_,
                &params.timestamp_,
                &params.timestamp_without_time_zone_,
                &params.timestamptz_,
                &params.timestamp_with_time_zone_,
                &params.date_,
                &params.time_,
                &params.json_,
                &params.jsonb_,
                &params.uuid_,
                &params.inet_,
                &params.macaddr_,
                &params.numeric_,
            )
        }
    }
    pub fn select_everything_array() -> SelectEverythingArrayStmt {
        SelectEverythingArrayStmt(crate::client::sync::Stmt::new(
            "SELECT
    *
FROM
    EverythingArray",
        ))
    }
    pub struct SelectEverythingArrayStmt(crate::client::sync::Stmt);
    impl SelectEverythingArrayStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
        ) -> EverythingArrayQuery<'a, C, super::EverythingArray, 0> {
            EverythingArrayQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| super::EverythingArrayBorrowed {
                    bool_: row.get(0),
                    boolean_: row.get(1),
                    char_: row.get(2),
                    smallint_: row.get(3),
                    int2_: row.get(4),
                    int_: row.get(5),
                    int4_: row.get(6),
                    bingint_: row.get(7),
                    int8_: row.get(8),
                    float4_: row.get(9),
                    real_: row.get(10),
                    float8_: row.get(11),
                    double_precision_: row.get(12),
                    text_: row.get(13),
                    varchar_: row.get(14),
                    bytea_: row.get(15),
                    timestamp_: row.get(16),
                    timestamp_without_time_zone_: row.get(17),
                    timestamptz_: row.get(18),
                    timestamp_with_time_zone_: row.get(19),
                    date_: row.get(20),
                    time_: row.get(21),
                    json_: row.get(22),
                    jsonb_: row.get(23),
                    uuid_: row.get(24),
                    inet_: row.get(25),
                    macaddr_: row.get(26),
                    numeric_: row.get(27),
                },
                mapper: |it| <super::EverythingArray>::from(it),
            }
        }
    }
    pub fn select_everything_array_null() -> SelectEverythingArrayNullStmt {
        SelectEverythingArrayNullStmt(crate::client::sync::Stmt::new(
            "SELECT
    *
FROM
    EverythingArray",
        ))
    }
    pub struct SelectEverythingArrayNullStmt(crate::client::sync::Stmt);
    impl SelectEverythingArrayNullStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
        ) -> EverythingArrayNullQuery<'a, C, super::EverythingArrayNull, 0> {
            EverythingArrayNullQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| super::EverythingArrayNullBorrowed {
                    bool_: row.get(0),
                    boolean_: row.get(1),
                    char_: row.get(2),
                    smallint_: row.get(3),
                    int2_: row.get(4),
                    int_: row.get(5),
                    int4_: row.get(6),
                    bingint_: row.get(7),
                    int8_: row.get(8),
                    float4_: row.get(9),
                    real_: row.get(10),
                    float8_: row.get(11),
                    double_precision_: row.get(12),
                    text_: row.get(13),
                    varchar_: row.get(14),
                    bytea_: row.get(15),
                    timestamp_: row.get(16),
                    timestamp_without_time_zone_: row.get(17),
                    timestamptz_: row.get(18),
                    timestamp_with_time_zone_: row.get(19),
                    date_: row.get(20),
                    time_: row.get(21),
                    json_: row.get(22),
                    jsonb_: row.get(23),
                    uuid_: row.get(24),
                    inet_: row.get(25),
                    macaddr_: row.get(26),
                    numeric_: row.get(27),
                },
                mapper: |it| <super::EverythingArrayNull>::from(it),
            }
        }
    }
    pub fn insert_everything_array() -> InsertEverythingArrayStmt {
        InsertEverythingArrayStmt(crate::client::sync :: Stmt :: new("INSERT INTO EverythingArray (bool_, boolean_, char_, smallint_, int2_, int_, int4_, bingint_, int8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_, numeric_)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28)"))
    }
    pub struct InsertEverythingArrayStmt(crate::client::sync::Stmt);
    impl InsertEverythingArrayStmt {
        pub fn bind<
            'a,
            C: GenericClient,
            T1: crate::ArraySql<Item = bool>,
            T2: crate::ArraySql<Item = bool>,
            T3: crate::ArraySql<Item = i8>,
            T4: crate::ArraySql<Item = i16>,
            T5: crate::ArraySql<Item = i16>,
            T6: crate::ArraySql<Item = i32>,
            T7: crate::ArraySql<Item = i32>,
            T8: crate::ArraySql<Item = i64>,
            T9: crate::ArraySql<Item = i64>,
            T10: crate::ArraySql<Item = f32>,
            T11: crate::ArraySql<Item = f32>,
            T12: crate::ArraySql<Item = f64>,
            T13: crate::ArraySql<Item = f64>,
            T14: crate::StringSql,
            T15: crate::ArraySql<Item = T14>,
            T16: crate::StringSql,
            T17: crate::ArraySql<Item = T16>,
            T18: crate::BytesSql,
            T19: crate::ArraySql<Item = T18>,
            T20: crate::ArraySql<Item = time::PrimitiveDateTime>,
            T21: crate::ArraySql<Item = time::PrimitiveDateTime>,
            T22: crate::ArraySql<Item = time::OffsetDateTime>,
            T23: crate::ArraySql<Item = time::OffsetDateTime>,
            T24: crate::ArraySql<Item = time::Date>,
            T25: crate::ArraySql<Item = time::Time>,
            T26: crate::JsonSql,
            T27: crate::ArraySql<Item = T26>,
            T28: crate::JsonSql,
            T29: crate::ArraySql<Item = T28>,
            T30: crate::ArraySql<Item = uuid::Uuid>,
            T31: crate::ArraySql<Item = std::net::IpAddr>,
            T32: crate::ArraySql<Item = eui48::MacAddress>,
            T33: crate::ArraySql<Item = rust_decimal::Decimal>,
        >(
            &'a mut self,
            client: &'a mut C,
            bool_: &'a T1,
            boolean_: &'a T2,
            char_: &'a T3,
            smallint_: &'a T4,
            int2_: &'a T5,
            int_: &'a T6,
            int4_: &'a T7,
            bingint_: &'a T8,
            int8_: &'a T9,
            float4_: &'a T10,
            real_: &'a T11,
            float8_: &'a T12,
            double_precision_: &'a T13,
            text_: &'a T15,
            varchar_: &'a T17,
            bytea_: &'a T19,
            timestamp_: &'a T20,
            timestamp_without_time_zone_: &'a T21,
            timestamptz_: &'a T22,
            timestamp_with_time_zone_: &'a T23,
            date_: &'a T24,
            time_: &'a T25,
            json_: &'a T27,
            jsonb_: &'a T29,
            uuid_: &'a T30,
            inet_: &'a T31,
            macaddr_: &'a T32,
            numeric_: &'a T33,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(
                stmt,
                &[
                    bool_,
                    boolean_,
                    char_,
                    smallint_,
                    int2_,
                    int_,
                    int4_,
                    bingint_,
                    int8_,
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
                    macaddr_,
                    numeric_,
                ],
            )
        }
    }
    impl<
            'a,
            C: GenericClient,
            T1: crate::ArraySql<Item = bool>,
            T2: crate::ArraySql<Item = bool>,
            T3: crate::ArraySql<Item = i8>,
            T4: crate::ArraySql<Item = i16>,
            T5: crate::ArraySql<Item = i16>,
            T6: crate::ArraySql<Item = i32>,
            T7: crate::ArraySql<Item = i32>,
            T8: crate::ArraySql<Item = i64>,
            T9: crate::ArraySql<Item = i64>,
            T10: crate::ArraySql<Item = f32>,
            T11: crate::ArraySql<Item = f32>,
            T12: crate::ArraySql<Item = f64>,
            T13: crate::ArraySql<Item = f64>,
            T14: crate::StringSql,
            T15: crate::ArraySql<Item = T14>,
            T16: crate::StringSql,
            T17: crate::ArraySql<Item = T16>,
            T18: crate::BytesSql,
            T19: crate::ArraySql<Item = T18>,
            T20: crate::ArraySql<Item = time::PrimitiveDateTime>,
            T21: crate::ArraySql<Item = time::PrimitiveDateTime>,
            T22: crate::ArraySql<Item = time::OffsetDateTime>,
            T23: crate::ArraySql<Item = time::OffsetDateTime>,
            T24: crate::ArraySql<Item = time::Date>,
            T25: crate::ArraySql<Item = time::Time>,
            T26: crate::JsonSql,
            T27: crate::ArraySql<Item = T26>,
            T28: crate::JsonSql,
            T29: crate::ArraySql<Item = T28>,
            T30: crate::ArraySql<Item = uuid::Uuid>,
            T31: crate::ArraySql<Item = std::net::IpAddr>,
            T32: crate::ArraySql<Item = eui48::MacAddress>,
            T33: crate::ArraySql<Item = rust_decimal::Decimal>,
        >
        crate::client::sync::Params<
            'a,
            super::EverythingArrayParams<
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                T24,
                T25,
                T26,
                T27,
                T28,
                T29,
                T30,
                T31,
                T32,
                T33,
            >,
            Result<u64, postgres::Error>,
            C,
        > for InsertEverythingArrayStmt
    {
        fn params(
            &'a mut self,
            client: &'a mut C,
            params: &'a super::EverythingArrayParams<
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                T24,
                T25,
                T26,
                T27,
                T28,
                T29,
                T30,
                T31,
                T32,
                T33,
            >,
        ) -> Result<u64, postgres::Error> {
            self.bind(
                client,
                &params.bool_,
                &params.boolean_,
                &params.char_,
                &params.smallint_,
                &params.int2_,
                &params.int_,
                &params.int4_,
                &params.bingint_,
                &params.int8_,
                &params.float4_,
                &params.real_,
                &params.float8_,
                &params.double_precision_,
                &params.text_,
                &params.varchar_,
                &params.bytea_,
                &params.timestamp_,
                &params.timestamp_without_time_zone_,
                &params.timestamptz_,
                &params.timestamp_with_time_zone_,
                &params.date_,
                &params.time_,
                &params.json_,
                &params.jsonb_,
                &params.uuid_,
                &params.inet_,
                &params.macaddr_,
                &params.numeric_,
            )
        }
    }
    pub fn select_nightmare() -> SelectNightmareStmt {
        SelectNightmareStmt(crate::client::sync::Stmt::new(
            "SELECT
    *
FROM
    nightmare",
        ))
    }
    pub struct SelectNightmareStmt(crate::client::sync::Stmt);
    impl SelectNightmareStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
        ) -> NightmareCompositeQuery<'a, C, crate::types::NightmareComposite, 0> {
            NightmareCompositeQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| row.get(0),
                mapper: |it| it.into(),
            }
        }
    }
    pub fn insert_nightmare() -> InsertNightmareStmt {
        InsertNightmareStmt(crate::client::sync::Stmt::new(
            "INSERT INTO nightmare (composite)
    VALUES ($1)",
        ))
    }
    pub struct InsertNightmareStmt(crate::client::sync::Stmt);
    impl InsertNightmareStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a mut C,
            composite: &'a crate::types::NightmareCompositeParams<'a>,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[composite])
        }
    }
}
pub mod async_ {
    use crate::client::async_::GenericClient;
    use futures::{self, StreamExt, TryStreamExt};
    pub struct EverythingQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::async_::Stmt,
        extractor: fn(&tokio_postgres::Row) -> super::EverythingBorrowed,
        mapper: fn(super::EverythingBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> EverythingQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::EverythingBorrowed) -> R,
        ) -> EverythingQuery<'a, C, R, N> {
            EverythingQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            let row = self.client.query_one(stmt, &self.params).await?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)
                .await?
                .map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
            tokio_postgres::Error,
        > {
            let stmt = self.stmt.prepare(self.client).await?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))
                .await?
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                .into_stream();
            Ok(it)
        }
    }
    pub struct EverythingNullQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::async_::Stmt,
        extractor: fn(&tokio_postgres::Row) -> super::EverythingNullBorrowed,
        mapper: fn(super::EverythingNullBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> EverythingNullQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::EverythingNullBorrowed) -> R,
        ) -> EverythingNullQuery<'a, C, R, N> {
            EverythingNullQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            let row = self.client.query_one(stmt, &self.params).await?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)
                .await?
                .map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
            tokio_postgres::Error,
        > {
            let stmt = self.stmt.prepare(self.client).await?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))
                .await?
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                .into_stream();
            Ok(it)
        }
    }
    pub struct EverythingArrayQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::async_::Stmt,
        extractor: fn(&tokio_postgres::Row) -> super::EverythingArrayBorrowed,
        mapper: fn(super::EverythingArrayBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> EverythingArrayQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::EverythingArrayBorrowed) -> R,
        ) -> EverythingArrayQuery<'a, C, R, N> {
            EverythingArrayQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            let row = self.client.query_one(stmt, &self.params).await?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)
                .await?
                .map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
            tokio_postgres::Error,
        > {
            let stmt = self.stmt.prepare(self.client).await?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))
                .await?
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                .into_stream();
            Ok(it)
        }
    }
    pub struct EverythingArrayNullQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::async_::Stmt,
        extractor: fn(&tokio_postgres::Row) -> super::EverythingArrayNullBorrowed,
        mapper: fn(super::EverythingArrayNullBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> EverythingArrayNullQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::EverythingArrayNullBorrowed) -> R,
        ) -> EverythingArrayNullQuery<'a, C, R, N> {
            EverythingArrayNullQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            let row = self.client.query_one(stmt, &self.params).await?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)
                .await?
                .map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
            tokio_postgres::Error,
        > {
            let stmt = self.stmt.prepare(self.client).await?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))
                .await?
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                .into_stream();
            Ok(it)
        }
    }
    pub struct NightmareCompositeQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut crate::client::async_::Stmt,
        extractor: fn(&tokio_postgres::Row) -> crate::types::NightmareCompositeBorrowed,
        mapper: fn(crate::types::NightmareCompositeBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> NightmareCompositeQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(crate::types::NightmareCompositeBorrowed) -> R,
        ) -> NightmareCompositeQuery<'a, C, R, N> {
            NightmareCompositeQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            let row = self.client.query_one(stmt, &self.params).await?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)
                .await?
                .map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
            tokio_postgres::Error,
        > {
            let stmt = self.stmt.prepare(self.client).await?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))
                .await?
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                .into_stream();
            Ok(it)
        }
    }
    pub fn select_everything() -> SelectEverythingStmt {
        SelectEverythingStmt(crate::client::async_::Stmt::new(
            "SELECT
    *
FROM
    Everything",
        ))
    }
    pub struct SelectEverythingStmt(crate::client::async_::Stmt);
    impl SelectEverythingStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
        ) -> EverythingQuery<'a, C, super::Everything, 0> {
            EverythingQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| super::EverythingBorrowed {
                    bool_: row.get(0),
                    boolean_: row.get(1),
                    char_: row.get(2),
                    smallint_: row.get(3),
                    int2_: row.get(4),
                    smallserial_: row.get(5),
                    serial2_: row.get(6),
                    int_: row.get(7),
                    int4_: row.get(8),
                    serial_: row.get(9),
                    serial4_: row.get(10),
                    bingint_: row.get(11),
                    int8_: row.get(12),
                    bigserial_: row.get(13),
                    serial8_: row.get(14),
                    float4_: row.get(15),
                    real_: row.get(16),
                    float8_: row.get(17),
                    double_precision_: row.get(18),
                    text_: row.get(19),
                    varchar_: row.get(20),
                    bytea_: row.get(21),
                    timestamp_: row.get(22),
                    timestamp_without_time_zone_: row.get(23),
                    timestamptz_: row.get(24),
                    timestamp_with_time_zone_: row.get(25),
                    date_: row.get(26),
                    time_: row.get(27),
                    json_: row.get(28),
                    jsonb_: row.get(29),
                    uuid_: row.get(30),
                    inet_: row.get(31),
                    macaddr_: row.get(32),
                    numeric_: row.get(33),
                },
                mapper: |it| <super::Everything>::from(it),
            }
        }
    }
    pub fn select_everything_null() -> SelectEverythingNullStmt {
        SelectEverythingNullStmt(crate::client::async_::Stmt::new(
            "SELECT
    *
FROM
    Everything",
        ))
    }
    pub struct SelectEverythingNullStmt(crate::client::async_::Stmt);
    impl SelectEverythingNullStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
        ) -> EverythingNullQuery<'a, C, super::EverythingNull, 0> {
            EverythingNullQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| super::EverythingNullBorrowed {
                    bool_: row.get(0),
                    boolean_: row.get(1),
                    char_: row.get(2),
                    smallint_: row.get(3),
                    int2_: row.get(4),
                    smallserial_: row.get(5),
                    serial2_: row.get(6),
                    int_: row.get(7),
                    int4_: row.get(8),
                    serial_: row.get(9),
                    serial4_: row.get(10),
                    bingint_: row.get(11),
                    int8_: row.get(12),
                    bigserial_: row.get(13),
                    serial8_: row.get(14),
                    float4_: row.get(15),
                    real_: row.get(16),
                    float8_: row.get(17),
                    double_precision_: row.get(18),
                    text_: row.get(19),
                    varchar_: row.get(20),
                    bytea_: row.get(21),
                    timestamp_: row.get(22),
                    timestamp_without_time_zone_: row.get(23),
                    timestamptz_: row.get(24),
                    timestamp_with_time_zone_: row.get(25),
                    date_: row.get(26),
                    time_: row.get(27),
                    json_: row.get(28),
                    jsonb_: row.get(29),
                    uuid_: row.get(30),
                    inet_: row.get(31),
                    macaddr_: row.get(32),
                    numeric_: row.get(33),
                },
                mapper: |it| <super::EverythingNull>::from(it),
            }
        }
    }
    pub fn insert_everything() -> InsertEverythingStmt {
        InsertEverythingStmt(crate::client::async_ :: Stmt :: new("INSERT INTO Everything (bool_, boolean_, char_, smallint_, int2_, smallserial_, serial2_, int_, int4_, serial_, serial4_, bingint_, int8_, bigserial_, serial8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_, numeric_)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34)"))
    }
    pub struct InsertEverythingStmt(crate::client::async_::Stmt);
    impl InsertEverythingStmt {
        pub async fn bind<
            'a,
            C: GenericClient,
            T1: crate::StringSql,
            T2: crate::StringSql,
            T3: crate::BytesSql,
            T4: crate::JsonSql,
            T5: crate::JsonSql,
        >(
            &'a mut self,
            client: &'a C,
            bool_: &'a bool,
            boolean_: &'a bool,
            char_: &'a i8,
            smallint_: &'a i16,
            int2_: &'a i16,
            smallserial_: &'a i16,
            serial2_: &'a i16,
            int_: &'a i32,
            int4_: &'a i32,
            serial_: &'a i32,
            serial4_: &'a i32,
            bingint_: &'a i64,
            int8_: &'a i64,
            bigserial_: &'a i64,
            serial8_: &'a i64,
            float4_: &'a f32,
            real_: &'a f32,
            float8_: &'a f64,
            double_precision_: &'a f64,
            text_: &'a T1,
            varchar_: &'a T2,
            bytea_: &'a T3,
            timestamp_: &'a time::PrimitiveDateTime,
            timestamp_without_time_zone_: &'a time::PrimitiveDateTime,
            timestamptz_: &'a time::OffsetDateTime,
            timestamp_with_time_zone_: &'a time::OffsetDateTime,
            date_: &'a time::Date,
            time_: &'a time::Time,
            json_: &'a T4,
            jsonb_: &'a T5,
            uuid_: &'a uuid::Uuid,
            inet_: &'a std::net::IpAddr,
            macaddr_: &'a eui48::MacAddress,
            numeric_: &'a rust_decimal::Decimal,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client
                .execute(
                    stmt,
                    &[
                        bool_,
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
                        macaddr_,
                        numeric_,
                    ],
                )
                .await
        }
    }
    impl<
            'a,
            C: GenericClient + Send + Sync,
            T1: crate::StringSql,
            T2: crate::StringSql,
            T3: crate::BytesSql,
            T4: crate::JsonSql,
            T5: crate::JsonSql,
        >
        crate::client::async_::Params<
            'a,
            super::EverythingParams<T1, T2, T3, T4, T5>,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for InsertEverythingStmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::EverythingParams<T1, T2, T3, T4, T5>,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(
                client,
                &params.bool_,
                &params.boolean_,
                &params.char_,
                &params.smallint_,
                &params.int2_,
                &params.smallserial_,
                &params.serial2_,
                &params.int_,
                &params.int4_,
                &params.serial_,
                &params.serial4_,
                &params.bingint_,
                &params.int8_,
                &params.bigserial_,
                &params.serial8_,
                &params.float4_,
                &params.real_,
                &params.float8_,
                &params.double_precision_,
                &params.text_,
                &params.varchar_,
                &params.bytea_,
                &params.timestamp_,
                &params.timestamp_without_time_zone_,
                &params.timestamptz_,
                &params.timestamp_with_time_zone_,
                &params.date_,
                &params.time_,
                &params.json_,
                &params.jsonb_,
                &params.uuid_,
                &params.inet_,
                &params.macaddr_,
                &params.numeric_,
            ))
        }
    }
    pub fn select_everything_array() -> SelectEverythingArrayStmt {
        SelectEverythingArrayStmt(crate::client::async_::Stmt::new(
            "SELECT
    *
FROM
    EverythingArray",
        ))
    }
    pub struct SelectEverythingArrayStmt(crate::client::async_::Stmt);
    impl SelectEverythingArrayStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
        ) -> EverythingArrayQuery<'a, C, super::EverythingArray, 0> {
            EverythingArrayQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| super::EverythingArrayBorrowed {
                    bool_: row.get(0),
                    boolean_: row.get(1),
                    char_: row.get(2),
                    smallint_: row.get(3),
                    int2_: row.get(4),
                    int_: row.get(5),
                    int4_: row.get(6),
                    bingint_: row.get(7),
                    int8_: row.get(8),
                    float4_: row.get(9),
                    real_: row.get(10),
                    float8_: row.get(11),
                    double_precision_: row.get(12),
                    text_: row.get(13),
                    varchar_: row.get(14),
                    bytea_: row.get(15),
                    timestamp_: row.get(16),
                    timestamp_without_time_zone_: row.get(17),
                    timestamptz_: row.get(18),
                    timestamp_with_time_zone_: row.get(19),
                    date_: row.get(20),
                    time_: row.get(21),
                    json_: row.get(22),
                    jsonb_: row.get(23),
                    uuid_: row.get(24),
                    inet_: row.get(25),
                    macaddr_: row.get(26),
                    numeric_: row.get(27),
                },
                mapper: |it| <super::EverythingArray>::from(it),
            }
        }
    }
    pub fn select_everything_array_null() -> SelectEverythingArrayNullStmt {
        SelectEverythingArrayNullStmt(crate::client::async_::Stmt::new(
            "SELECT
    *
FROM
    EverythingArray",
        ))
    }
    pub struct SelectEverythingArrayNullStmt(crate::client::async_::Stmt);
    impl SelectEverythingArrayNullStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
        ) -> EverythingArrayNullQuery<'a, C, super::EverythingArrayNull, 0> {
            EverythingArrayNullQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| super::EverythingArrayNullBorrowed {
                    bool_: row.get(0),
                    boolean_: row.get(1),
                    char_: row.get(2),
                    smallint_: row.get(3),
                    int2_: row.get(4),
                    int_: row.get(5),
                    int4_: row.get(6),
                    bingint_: row.get(7),
                    int8_: row.get(8),
                    float4_: row.get(9),
                    real_: row.get(10),
                    float8_: row.get(11),
                    double_precision_: row.get(12),
                    text_: row.get(13),
                    varchar_: row.get(14),
                    bytea_: row.get(15),
                    timestamp_: row.get(16),
                    timestamp_without_time_zone_: row.get(17),
                    timestamptz_: row.get(18),
                    timestamp_with_time_zone_: row.get(19),
                    date_: row.get(20),
                    time_: row.get(21),
                    json_: row.get(22),
                    jsonb_: row.get(23),
                    uuid_: row.get(24),
                    inet_: row.get(25),
                    macaddr_: row.get(26),
                    numeric_: row.get(27),
                },
                mapper: |it| <super::EverythingArrayNull>::from(it),
            }
        }
    }
    pub fn insert_everything_array() -> InsertEverythingArrayStmt {
        InsertEverythingArrayStmt(crate::client::async_ :: Stmt :: new("INSERT INTO EverythingArray (bool_, boolean_, char_, smallint_, int2_, int_, int4_, bingint_, int8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_, numeric_)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28)"))
    }
    pub struct InsertEverythingArrayStmt(crate::client::async_::Stmt);
    impl InsertEverythingArrayStmt {
        pub async fn bind<
            'a,
            C: GenericClient,
            T1: crate::ArraySql<Item = bool>,
            T2: crate::ArraySql<Item = bool>,
            T3: crate::ArraySql<Item = i8>,
            T4: crate::ArraySql<Item = i16>,
            T5: crate::ArraySql<Item = i16>,
            T6: crate::ArraySql<Item = i32>,
            T7: crate::ArraySql<Item = i32>,
            T8: crate::ArraySql<Item = i64>,
            T9: crate::ArraySql<Item = i64>,
            T10: crate::ArraySql<Item = f32>,
            T11: crate::ArraySql<Item = f32>,
            T12: crate::ArraySql<Item = f64>,
            T13: crate::ArraySql<Item = f64>,
            T14: crate::StringSql,
            T15: crate::ArraySql<Item = T14>,
            T16: crate::StringSql,
            T17: crate::ArraySql<Item = T16>,
            T18: crate::BytesSql,
            T19: crate::ArraySql<Item = T18>,
            T20: crate::ArraySql<Item = time::PrimitiveDateTime>,
            T21: crate::ArraySql<Item = time::PrimitiveDateTime>,
            T22: crate::ArraySql<Item = time::OffsetDateTime>,
            T23: crate::ArraySql<Item = time::OffsetDateTime>,
            T24: crate::ArraySql<Item = time::Date>,
            T25: crate::ArraySql<Item = time::Time>,
            T26: crate::JsonSql,
            T27: crate::ArraySql<Item = T26>,
            T28: crate::JsonSql,
            T29: crate::ArraySql<Item = T28>,
            T30: crate::ArraySql<Item = uuid::Uuid>,
            T31: crate::ArraySql<Item = std::net::IpAddr>,
            T32: crate::ArraySql<Item = eui48::MacAddress>,
            T33: crate::ArraySql<Item = rust_decimal::Decimal>,
        >(
            &'a mut self,
            client: &'a C,
            bool_: &'a T1,
            boolean_: &'a T2,
            char_: &'a T3,
            smallint_: &'a T4,
            int2_: &'a T5,
            int_: &'a T6,
            int4_: &'a T7,
            bingint_: &'a T8,
            int8_: &'a T9,
            float4_: &'a T10,
            real_: &'a T11,
            float8_: &'a T12,
            double_precision_: &'a T13,
            text_: &'a T15,
            varchar_: &'a T17,
            bytea_: &'a T19,
            timestamp_: &'a T20,
            timestamp_without_time_zone_: &'a T21,
            timestamptz_: &'a T22,
            timestamp_with_time_zone_: &'a T23,
            date_: &'a T24,
            time_: &'a T25,
            json_: &'a T27,
            jsonb_: &'a T29,
            uuid_: &'a T30,
            inet_: &'a T31,
            macaddr_: &'a T32,
            numeric_: &'a T33,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client
                .execute(
                    stmt,
                    &[
                        bool_,
                        boolean_,
                        char_,
                        smallint_,
                        int2_,
                        int_,
                        int4_,
                        bingint_,
                        int8_,
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
                        macaddr_,
                        numeric_,
                    ],
                )
                .await
        }
    }
    impl<
            'a,
            C: GenericClient + Send + Sync,
            T1: crate::ArraySql<Item = bool>,
            T2: crate::ArraySql<Item = bool>,
            T3: crate::ArraySql<Item = i8>,
            T4: crate::ArraySql<Item = i16>,
            T5: crate::ArraySql<Item = i16>,
            T6: crate::ArraySql<Item = i32>,
            T7: crate::ArraySql<Item = i32>,
            T8: crate::ArraySql<Item = i64>,
            T9: crate::ArraySql<Item = i64>,
            T10: crate::ArraySql<Item = f32>,
            T11: crate::ArraySql<Item = f32>,
            T12: crate::ArraySql<Item = f64>,
            T13: crate::ArraySql<Item = f64>,
            T14: crate::StringSql,
            T15: crate::ArraySql<Item = T14>,
            T16: crate::StringSql,
            T17: crate::ArraySql<Item = T16>,
            T18: crate::BytesSql,
            T19: crate::ArraySql<Item = T18>,
            T20: crate::ArraySql<Item = time::PrimitiveDateTime>,
            T21: crate::ArraySql<Item = time::PrimitiveDateTime>,
            T22: crate::ArraySql<Item = time::OffsetDateTime>,
            T23: crate::ArraySql<Item = time::OffsetDateTime>,
            T24: crate::ArraySql<Item = time::Date>,
            T25: crate::ArraySql<Item = time::Time>,
            T26: crate::JsonSql,
            T27: crate::ArraySql<Item = T26>,
            T28: crate::JsonSql,
            T29: crate::ArraySql<Item = T28>,
            T30: crate::ArraySql<Item = uuid::Uuid>,
            T31: crate::ArraySql<Item = std::net::IpAddr>,
            T32: crate::ArraySql<Item = eui48::MacAddress>,
            T33: crate::ArraySql<Item = rust_decimal::Decimal>,
        >
        crate::client::async_::Params<
            'a,
            super::EverythingArrayParams<
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                T24,
                T25,
                T26,
                T27,
                T28,
                T29,
                T30,
                T31,
                T32,
                T33,
            >,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for InsertEverythingArrayStmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::EverythingArrayParams<
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                T24,
                T25,
                T26,
                T27,
                T28,
                T29,
                T30,
                T31,
                T32,
                T33,
            >,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(
                client,
                &params.bool_,
                &params.boolean_,
                &params.char_,
                &params.smallint_,
                &params.int2_,
                &params.int_,
                &params.int4_,
                &params.bingint_,
                &params.int8_,
                &params.float4_,
                &params.real_,
                &params.float8_,
                &params.double_precision_,
                &params.text_,
                &params.varchar_,
                &params.bytea_,
                &params.timestamp_,
                &params.timestamp_without_time_zone_,
                &params.timestamptz_,
                &params.timestamp_with_time_zone_,
                &params.date_,
                &params.time_,
                &params.json_,
                &params.jsonb_,
                &params.uuid_,
                &params.inet_,
                &params.macaddr_,
                &params.numeric_,
            ))
        }
    }
    pub fn select_nightmare() -> SelectNightmareStmt {
        SelectNightmareStmt(crate::client::async_::Stmt::new(
            "SELECT
    *
FROM
    nightmare",
        ))
    }
    pub struct SelectNightmareStmt(crate::client::async_::Stmt);
    impl SelectNightmareStmt {
        pub fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
        ) -> NightmareCompositeQuery<'a, C, crate::types::NightmareComposite, 0> {
            NightmareCompositeQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| row.get(0),
                mapper: |it| it.into(),
            }
        }
    }
    pub fn insert_nightmare() -> InsertNightmareStmt {
        InsertNightmareStmt(crate::client::async_::Stmt::new(
            "INSERT INTO nightmare (composite)
    VALUES ($1)",
        ))
    }
    pub struct InsertNightmareStmt(crate::client::async_::Stmt);
    impl InsertNightmareStmt {
        pub async fn bind<'a, C: GenericClient>(
            &'a mut self,
            client: &'a C,
            composite: &'a crate::types::NightmareCompositeParams<'a>,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[composite]).await
        }
    }
}
