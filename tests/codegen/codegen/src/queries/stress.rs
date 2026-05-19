// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct EverythingParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
    T6: crate::BytesSql,
    T7: crate::JsonSql,
    T8: crate::JsonSql,
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
    pub name_: T3,
    pub citext_: T4,
    pub ltree_: T5,
    pub bytea_: T6,
    pub timestamp_: chrono::NaiveDateTime,
    pub timestamp_without_time_zone_: chrono::NaiveDateTime,
    pub timestamptz_: chrono::DateTime<chrono::FixedOffset>,
    pub timestamp_with_time_zone_: chrono::DateTime<chrono::FixedOffset>,
    pub date_: chrono::NaiveDate,
    pub time_: chrono::NaiveTime,
    pub json_: T7,
    pub jsonb_: T8,
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
    T18: crate::StringSql,
    T19: crate::ArraySql<Item = T18>,
    T20: crate::StringSql,
    T21: crate::ArraySql<Item = T20>,
    T22: crate::StringSql,
    T23: crate::ArraySql<Item = T22>,
    T24: crate::BytesSql,
    T25: crate::ArraySql<Item = T24>,
    T26: crate::ArraySql<Item = chrono::NaiveDateTime>,
    T27: crate::ArraySql<Item = chrono::NaiveDateTime>,
    T28: crate::ArraySql<Item = chrono::DateTime<chrono::FixedOffset>>,
    T29: crate::ArraySql<Item = chrono::DateTime<chrono::FixedOffset>>,
    T30: crate::ArraySql<Item = chrono::NaiveDate>,
    T31: crate::ArraySql<Item = chrono::NaiveTime>,
    T32: crate::JsonSql,
    T33: crate::ArraySql<Item = T32>,
    T34: crate::JsonSql,
    T35: crate::ArraySql<Item = T34>,
    T36: crate::ArraySql<Item = uuid::Uuid>,
    T37: crate::ArraySql<Item = std::net::IpAddr>,
    T38: crate::ArraySql<Item = eui48::MacAddress>,
    T39: crate::ArraySql<Item = rust_decimal::Decimal>,
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
    pub name_: T19,
    pub citext_: T21,
    pub ltree_: T23,
    pub bytea_: T25,
    pub timestamp_: T26,
    pub timestamp_without_time_zone_: T27,
    pub timestamptz_: T28,
    pub timestamp_with_time_zone_: T29,
    pub date_: T30,
    pub time_: T31,
    pub json_: T33,
    pub jsonb_: T35,
    pub uuid_: T36,
    pub inet_: T37,
    pub macaddr_: T38,
    pub numeric_: T39,
}
#[derive(Debug, Clone, PartialEq)]
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
    pub name_: String,
    pub citext_: String,
    pub ltree_: String,
    pub bytea_: Vec<u8>,
    pub timestamp_: chrono::NaiveDateTime,
    pub timestamp_without_time_zone_: chrono::NaiveDateTime,
    pub timestamptz_: chrono::DateTime<chrono::FixedOffset>,
    pub timestamp_with_time_zone_: chrono::DateTime<chrono::FixedOffset>,
    pub date_: chrono::NaiveDate,
    pub time_: chrono::NaiveTime,
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
    pub name_: &'a str,
    pub citext_: &'a str,
    pub ltree_: &'a str,
    pub bytea_: &'a [u8],
    pub timestamp_: chrono::NaiveDateTime,
    pub timestamp_without_time_zone_: chrono::NaiveDateTime,
    pub timestamptz_: chrono::DateTime<chrono::FixedOffset>,
    pub timestamp_with_time_zone_: chrono::DateTime<chrono::FixedOffset>,
    pub date_: chrono::NaiveDate,
    pub time_: chrono::NaiveTime,
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
            name_,
            citext_,
            ltree_,
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
            name_: name_.into(),
            citext_: citext_.into(),
            ltree_: ltree_.into(),
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
#[derive(Debug, Clone, PartialEq)]
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
    pub name_: Option<String>,
    pub citext_: Option<String>,
    pub ltree_: Option<String>,
    pub bytea_: Option<Vec<u8>>,
    pub timestamp_: Option<chrono::NaiveDateTime>,
    pub timestamp_without_time_zone_: Option<chrono::NaiveDateTime>,
    pub timestamptz_: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub timestamp_with_time_zone_: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub date_: Option<chrono::NaiveDate>,
    pub time_: Option<chrono::NaiveTime>,
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
    pub name_: Option<&'a str>,
    pub citext_: Option<&'a str>,
    pub ltree_: Option<&'a str>,
    pub bytea_: Option<&'a [u8]>,
    pub timestamp_: Option<chrono::NaiveDateTime>,
    pub timestamp_without_time_zone_: Option<chrono::NaiveDateTime>,
    pub timestamptz_: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub timestamp_with_time_zone_: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub date_: Option<chrono::NaiveDate>,
    pub time_: Option<chrono::NaiveTime>,
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
            name_,
            citext_,
            ltree_,
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
            name_: name_.map(|v| v.into()),
            citext_: citext_.map(|v| v.into()),
            ltree_: ltree_.map(|v| v.into()),
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
#[derive(Debug, Clone, PartialEq)]
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
    pub name_: Vec<String>,
    pub citext_: Vec<String>,
    pub ltree_: Vec<String>,
    pub bytea_: Vec<Vec<u8>>,
    pub timestamp_: Vec<chrono::NaiveDateTime>,
    pub timestamp_without_time_zone_: Vec<chrono::NaiveDateTime>,
    pub timestamptz_: Vec<chrono::DateTime<chrono::FixedOffset>>,
    pub timestamp_with_time_zone_: Vec<chrono::DateTime<chrono::FixedOffset>>,
    pub date_: Vec<chrono::NaiveDate>,
    pub time_: Vec<chrono::NaiveTime>,
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
    pub name_: crate::ArrayIterator<'a, &'a str>,
    pub citext_: crate::ArrayIterator<'a, &'a str>,
    pub ltree_: crate::ArrayIterator<'a, &'a str>,
    pub bytea_: crate::ArrayIterator<'a, &'a [u8]>,
    pub timestamp_: crate::ArrayIterator<'a, chrono::NaiveDateTime>,
    pub timestamp_without_time_zone_: crate::ArrayIterator<'a, chrono::NaiveDateTime>,
    pub timestamptz_: crate::ArrayIterator<'a, chrono::DateTime<chrono::FixedOffset>>,
    pub timestamp_with_time_zone_: crate::ArrayIterator<'a, chrono::DateTime<chrono::FixedOffset>>,
    pub date_: crate::ArrayIterator<'a, chrono::NaiveDate>,
    pub time_: crate::ArrayIterator<'a, chrono::NaiveTime>,
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
            name_,
            citext_,
            ltree_,
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
            name_: name_.map(|v| v.into()).collect(),
            citext_: citext_.map(|v| v.into()).collect(),
            ltree_: ltree_.map(|v| v.into()).collect(),
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
#[derive(Debug, Clone, PartialEq)]
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
    pub name_: Option<Vec<String>>,
    pub citext_: Option<Vec<String>>,
    pub ltree_: Option<Vec<String>>,
    pub bytea_: Option<Vec<Vec<u8>>>,
    pub timestamp_: Option<Vec<chrono::NaiveDateTime>>,
    pub timestamp_without_time_zone_: Option<Vec<chrono::NaiveDateTime>>,
    pub timestamptz_: Option<Vec<chrono::DateTime<chrono::FixedOffset>>>,
    pub timestamp_with_time_zone_: Option<Vec<chrono::DateTime<chrono::FixedOffset>>>,
    pub date_: Option<Vec<chrono::NaiveDate>>,
    pub time_: Option<Vec<chrono::NaiveTime>>,
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
    pub name_: Option<crate::ArrayIterator<'a, &'a str>>,
    pub citext_: Option<crate::ArrayIterator<'a, &'a str>>,
    pub ltree_: Option<crate::ArrayIterator<'a, &'a str>>,
    pub bytea_: Option<crate::ArrayIterator<'a, &'a [u8]>>,
    pub timestamp_: Option<crate::ArrayIterator<'a, chrono::NaiveDateTime>>,
    pub timestamp_without_time_zone_: Option<crate::ArrayIterator<'a, chrono::NaiveDateTime>>,
    pub timestamptz_: Option<crate::ArrayIterator<'a, chrono::DateTime<chrono::FixedOffset>>>,
    pub timestamp_with_time_zone_:
        Option<crate::ArrayIterator<'a, chrono::DateTime<chrono::FixedOffset>>>,
    pub date_: Option<crate::ArrayIterator<'a, chrono::NaiveDate>>,
    pub time_: Option<crate::ArrayIterator<'a, chrono::NaiveTime>>,
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
            name_,
            citext_,
            ltree_,
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
            name_: name_.map(|v| v.map(|v| v.into()).collect()),
            citext_: citext_.map(|v| v.map(|v| v.into()).collect()),
            ltree_: ltree_.map(|v| v.map(|v| v.into()).collect()),
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
    use crate::client::sync::GenericClient;
    use postgres::fallible_iterator::FallibleIterator;
    pub struct EverythingQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor: fn(&postgres::Row) -> Result<super::EverythingBorrowed, postgres::Error>,
        mapper: fn(super::EverythingBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> EverythingQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::EverythingBorrowed) -> R,
        ) -> EverythingQuery<'c, 'a, 's, C, R, N> {
            EverythingQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let row = crate::client::sync::one(self.client, self.query, &self.params, self.cached)?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let opt_row =
                crate::client::sync::opt(self.client, self.query, &self.params, self.cached)?;
            Ok(opt_row
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'c, postgres::Error>
        {
            let stream = crate::client::sync::raw(
                self.client,
                self.query,
                crate::slice_iter(&self.params),
                self.cached,
            )?;
            let mapped = stream.iterator().map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            });
            Ok(mapped)
        }
    }
    pub struct EverythingNullQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor: fn(&postgres::Row) -> Result<super::EverythingNullBorrowed, postgres::Error>,
        mapper: fn(super::EverythingNullBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> EverythingNullQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::EverythingNullBorrowed) -> R,
        ) -> EverythingNullQuery<'c, 'a, 's, C, R, N> {
            EverythingNullQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let row = crate::client::sync::one(self.client, self.query, &self.params, self.cached)?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let opt_row =
                crate::client::sync::opt(self.client, self.query, &self.params, self.cached)?;
            Ok(opt_row
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'c, postgres::Error>
        {
            let stream = crate::client::sync::raw(
                self.client,
                self.query,
                crate::slice_iter(&self.params),
                self.cached,
            )?;
            let mapped = stream.iterator().map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            });
            Ok(mapped)
        }
    }
    pub struct StringQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor: fn(&postgres::Row) -> Result<&str, postgres::Error>,
        mapper: fn(&str) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> StringQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(&str) -> R) -> StringQuery<'c, 'a, 's, C, R, N> {
            StringQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let row = crate::client::sync::one(self.client, self.query, &self.params, self.cached)?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let opt_row =
                crate::client::sync::opt(self.client, self.query, &self.params, self.cached)?;
            Ok(opt_row
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'c, postgres::Error>
        {
            let stream = crate::client::sync::raw(
                self.client,
                self.query,
                crate::slice_iter(&self.params),
                self.cached,
            )?;
            let mapped = stream.iterator().map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            });
            Ok(mapped)
        }
    }
    pub struct EverythingArrayQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor: fn(&postgres::Row) -> Result<super::EverythingArrayBorrowed, postgres::Error>,
        mapper: fn(super::EverythingArrayBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> EverythingArrayQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::EverythingArrayBorrowed) -> R,
        ) -> EverythingArrayQuery<'c, 'a, 's, C, R, N> {
            EverythingArrayQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let row = crate::client::sync::one(self.client, self.query, &self.params, self.cached)?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let opt_row =
                crate::client::sync::opt(self.client, self.query, &self.params, self.cached)?;
            Ok(opt_row
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'c, postgres::Error>
        {
            let stream = crate::client::sync::raw(
                self.client,
                self.query,
                crate::slice_iter(&self.params),
                self.cached,
            )?;
            let mapped = stream.iterator().map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            });
            Ok(mapped)
        }
    }
    pub struct EverythingArrayNullQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor:
            fn(&postgres::Row) -> Result<super::EverythingArrayNullBorrowed, postgres::Error>,
        mapper: fn(super::EverythingArrayNullBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> EverythingArrayNullQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::EverythingArrayNullBorrowed) -> R,
        ) -> EverythingArrayNullQuery<'c, 'a, 's, C, R, N> {
            EverythingArrayNullQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let row = crate::client::sync::one(self.client, self.query, &self.params, self.cached)?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let opt_row =
                crate::client::sync::opt(self.client, self.query, &self.params, self.cached)?;
            Ok(opt_row
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'c, postgres::Error>
        {
            let stream = crate::client::sync::raw(
                self.client,
                self.query,
                crate::slice_iter(&self.params),
                self.cached,
            )?;
            let mapped = stream.iterator().map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            });
            Ok(mapped)
        }
    }
    pub struct NightmareCompositeQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor:
            fn(&postgres::Row) -> Result<crate::types::NightmareCompositeBorrowed, postgres::Error>,
        mapper: fn(crate::types::NightmareCompositeBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> NightmareCompositeQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(crate::types::NightmareCompositeBorrowed) -> R,
        ) -> NightmareCompositeQuery<'c, 'a, 's, C, R, N> {
            NightmareCompositeQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let row = crate::client::sync::one(self.client, self.query, &self.params, self.cached)?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let opt_row =
                crate::client::sync::opt(self.client, self.query, &self.params, self.cached)?;
            Ok(opt_row
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'c, postgres::Error>
        {
            let stream = crate::client::sync::raw(
                self.client,
                self.query,
                crate::slice_iter(&self.params),
                self.cached,
            )?;
            let mapped = stream.iterator().map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            });
            Ok(mapped)
        }
    }
    pub struct SchemaNightmareCompositeQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s postgres::Statement>,
        extractor: fn(
            &postgres::Row,
        )
            -> Result<crate::types::schema::NightmareCompositeBorrowed, postgres::Error>,
        mapper: fn(crate::types::schema::NightmareCompositeBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> SchemaNightmareCompositeQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(crate::types::schema::NightmareCompositeBorrowed) -> R,
        ) -> SchemaNightmareCompositeQuery<'c, 'a, 's, C, R, N> {
            SchemaNightmareCompositeQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let row = crate::client::sync::one(self.client, self.query, &self.params, self.cached)?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let opt_row =
                crate::client::sync::opt(self.client, self.query, &self.params, self.cached)?;
            Ok(opt_row
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'c, postgres::Error>
        {
            let stream = crate::client::sync::raw(
                self.client,
                self.query,
                crate::slice_iter(&self.params),
                self.cached,
            )?;
            let mapped = stream.iterator().map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            });
            Ok(mapped)
        }
    }
    pub struct SelectEverythingStmt(&'static str, Option<postgres::Statement>);
    pub fn select_everything() -> SelectEverythingStmt {
        SelectEverythingStmt("SELECT * FROM Everything", None)
    }
    impl SelectEverythingStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s self,
            client: &'c mut C,
        ) -> EverythingQuery<'c, 'a, 's, C, super::Everything, 0> {
            EverythingQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor:
                    |row: &postgres::Row| -> Result<super::EverythingBorrowed, postgres::Error> {
                        Ok(super::EverythingBorrowed {
                            bool_: row.try_get(0)?,
                            boolean_: row.try_get(1)?,
                            char_: row.try_get(2)?,
                            smallint_: row.try_get(3)?,
                            int2_: row.try_get(4)?,
                            smallserial_: row.try_get(5)?,
                            serial2_: row.try_get(6)?,
                            int_: row.try_get(7)?,
                            int4_: row.try_get(8)?,
                            serial_: row.try_get(9)?,
                            serial4_: row.try_get(10)?,
                            bingint_: row.try_get(11)?,
                            int8_: row.try_get(12)?,
                            bigserial_: row.try_get(13)?,
                            serial8_: row.try_get(14)?,
                            float4_: row.try_get(15)?,
                            real_: row.try_get(16)?,
                            float8_: row.try_get(17)?,
                            double_precision_: row.try_get(18)?,
                            text_: row.try_get(19)?,
                            varchar_: row.try_get(20)?,
                            name_: row.try_get(21)?,
                            citext_: row.try_get(22)?,
                            ltree_: row.try_get(23)?,
                            bytea_: row.try_get(24)?,
                            timestamp_: row.try_get(25)?,
                            timestamp_without_time_zone_: row.try_get(26)?,
                            timestamptz_: row.try_get(27)?,
                            timestamp_with_time_zone_: row.try_get(28)?,
                            date_: row.try_get(29)?,
                            time_: row.try_get(30)?,
                            json_: row.try_get(31)?,
                            jsonb_: row.try_get(32)?,
                            uuid_: row.try_get(33)?,
                            inet_: row.try_get(34)?,
                            macaddr_: row.try_get(35)?,
                            numeric_: row.try_get(36)?,
                        })
                    },
                mapper: |it| super::Everything::from(it),
            }
        }
    }
    pub struct SelectEverythingNullStmt(&'static str, Option<postgres::Statement>);
    pub fn select_everything_null() -> SelectEverythingNullStmt {
        SelectEverythingNullStmt("SELECT * FROM Everything", None)
    }
    impl SelectEverythingNullStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s self,
            client: &'c mut C,
        ) -> EverythingNullQuery<'c, 'a, 's, C, super::EverythingNull, 0> {
            EverythingNullQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor:
                    |row: &postgres::Row| -> Result<super::EverythingNullBorrowed, postgres::Error> {
                        Ok(super::EverythingNullBorrowed {
                            bool_: row.try_get(0)?,
                            boolean_: row.try_get(1)?,
                            char_: row.try_get(2)?,
                            smallint_: row.try_get(3)?,
                            int2_: row.try_get(4)?,
                            smallserial_: row.try_get(5)?,
                            serial2_: row.try_get(6)?,
                            int_: row.try_get(7)?,
                            int4_: row.try_get(8)?,
                            serial_: row.try_get(9)?,
                            serial4_: row.try_get(10)?,
                            bingint_: row.try_get(11)?,
                            int8_: row.try_get(12)?,
                            bigserial_: row.try_get(13)?,
                            serial8_: row.try_get(14)?,
                            float4_: row.try_get(15)?,
                            real_: row.try_get(16)?,
                            float8_: row.try_get(17)?,
                            double_precision_: row.try_get(18)?,
                            text_: row.try_get(19)?,
                            varchar_: row.try_get(20)?,
                            name_: row.try_get(21)?,
                            citext_: row.try_get(22)?,
                            ltree_: row.try_get(23)?,
                            bytea_: row.try_get(24)?,
                            timestamp_: row.try_get(25)?,
                            timestamp_without_time_zone_: row.try_get(26)?,
                            timestamptz_: row.try_get(27)?,
                            timestamp_with_time_zone_: row.try_get(28)?,
                            date_: row.try_get(29)?,
                            time_: row.try_get(30)?,
                            json_: row.try_get(31)?,
                            jsonb_: row.try_get(32)?,
                            uuid_: row.try_get(33)?,
                            inet_: row.try_get(34)?,
                            macaddr_: row.try_get(35)?,
                            numeric_: row.try_get(36)?,
                        })
                    },
                mapper: |it| super::EverythingNull::from(it),
            }
        }
    }
    pub struct InsertEverythingStmt(&'static str, Option<postgres::Statement>);
    pub fn insert_everything() -> InsertEverythingStmt {
        InsertEverythingStmt(
            "INSERT INTO Everything (bool_, boolean_, char_, smallint_, int2_, smallserial_, serial2_, int_, int4_, serial_, serial4_, bingint_, int8_, bigserial_, serial8_, float4_, real_, float8_, double_precision_, text_, varchar_, name_, citext_, ltree_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_, numeric_) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37)",
            None,
        )
    }
    impl InsertEverythingStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<
            'c,
            'a,
            's,
            C: GenericClient,
            T1: crate::StringSql,
            T2: crate::StringSql,
            T3: crate::StringSql,
            T4: crate::StringSql,
            T5: crate::StringSql,
            T6: crate::BytesSql,
            T7: crate::JsonSql,
            T8: crate::JsonSql,
        >(
            &'s self,
            client: &'c mut C,
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
            name_: &'a T3,
            citext_: &'a T4,
            ltree_: &'a T5,
            bytea_: &'a T6,
            timestamp_: &'a chrono::NaiveDateTime,
            timestamp_without_time_zone_: &'a chrono::NaiveDateTime,
            timestamptz_: &'a chrono::DateTime<chrono::FixedOffset>,
            timestamp_with_time_zone_: &'a chrono::DateTime<chrono::FixedOffset>,
            date_: &'a chrono::NaiveDate,
            time_: &'a chrono::NaiveTime,
            json_: &'a T7,
            jsonb_: &'a T8,
            uuid_: &'a uuid::Uuid,
            inet_: &'a std::net::IpAddr,
            macaddr_: &'a eui48::MacAddress,
            numeric_: &'a rust_decimal::Decimal,
        ) -> Result<u64, postgres::Error> {
            client.execute(
                self.0,
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
                    name_,
                    citext_,
                    ltree_,
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
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
        T5: crate::StringSql,
        T6: crate::BytesSql,
        T7: crate::JsonSql,
        T8: crate::JsonSql,
    >
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::EverythingParams<T1, T2, T3, T4, T5, T6, T7, T8>,
            Result<u64, postgres::Error>,
            C,
        > for InsertEverythingStmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
            params: &'a super::EverythingParams<T1, T2, T3, T4, T5, T6, T7, T8>,
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
                &params.name_,
                &params.citext_,
                &params.ltree_,
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
    pub struct SelectLtreeStmt(&'static str, Option<postgres::Statement>);
    pub fn select_ltree() -> SelectLtreeStmt {
        SelectLtreeStmt("SELECT ltree_ FROM Everything where $1 @> ltree_", None)
    }
    impl SelectLtreeStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
            &'s self,
            client: &'c mut C,
            path: &'a T1,
        ) -> StringQuery<'c, 'a, 's, C, String, 1> {
            StringQuery {
                client,
                params: [path],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it.into(),
            }
        }
    }
    pub struct SelectEverythingArrayStmt(&'static str, Option<postgres::Statement>);
    pub fn select_everything_array() -> SelectEverythingArrayStmt {
        SelectEverythingArrayStmt("SELECT * FROM EverythingArray", None)
    }
    impl SelectEverythingArrayStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s self,
            client: &'c mut C,
        ) -> EverythingArrayQuery<'c, 'a, 's, C, super::EverythingArray, 0> {
            EverythingArrayQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |
                    row: &postgres::Row,
                | -> Result<super::EverythingArrayBorrowed, postgres::Error> {
                    Ok(super::EverythingArrayBorrowed {
                        bool_: row.try_get(0)?,
                        boolean_: row.try_get(1)?,
                        char_: row.try_get(2)?,
                        smallint_: row.try_get(3)?,
                        int2_: row.try_get(4)?,
                        int_: row.try_get(5)?,
                        int4_: row.try_get(6)?,
                        bingint_: row.try_get(7)?,
                        int8_: row.try_get(8)?,
                        float4_: row.try_get(9)?,
                        real_: row.try_get(10)?,
                        float8_: row.try_get(11)?,
                        double_precision_: row.try_get(12)?,
                        text_: row.try_get(13)?,
                        varchar_: row.try_get(14)?,
                        name_: row.try_get(15)?,
                        citext_: row.try_get(16)?,
                        ltree_: row.try_get(17)?,
                        bytea_: row.try_get(18)?,
                        timestamp_: row.try_get(19)?,
                        timestamp_without_time_zone_: row.try_get(20)?,
                        timestamptz_: row.try_get(21)?,
                        timestamp_with_time_zone_: row.try_get(22)?,
                        date_: row.try_get(23)?,
                        time_: row.try_get(24)?,
                        json_: row.try_get(25)?,
                        jsonb_: row.try_get(26)?,
                        uuid_: row.try_get(27)?,
                        inet_: row.try_get(28)?,
                        macaddr_: row.try_get(29)?,
                        numeric_: row.try_get(30)?,
                    })
                },
                mapper: |it| super::EverythingArray::from(it),
            }
        }
    }
    pub struct SelectEverythingArrayNullStmt(&'static str, Option<postgres::Statement>);
    pub fn select_everything_array_null() -> SelectEverythingArrayNullStmt {
        SelectEverythingArrayNullStmt("SELECT * FROM EverythingArray", None)
    }
    impl SelectEverythingArrayNullStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s self,
            client: &'c mut C,
        ) -> EverythingArrayNullQuery<'c, 'a, 's, C, super::EverythingArrayNull, 0> {
            EverythingArrayNullQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |
                    row: &postgres::Row,
                | -> Result<super::EverythingArrayNullBorrowed, postgres::Error> {
                    Ok(super::EverythingArrayNullBorrowed {
                        bool_: row.try_get(0)?,
                        boolean_: row.try_get(1)?,
                        char_: row.try_get(2)?,
                        smallint_: row.try_get(3)?,
                        int2_: row.try_get(4)?,
                        int_: row.try_get(5)?,
                        int4_: row.try_get(6)?,
                        bingint_: row.try_get(7)?,
                        int8_: row.try_get(8)?,
                        float4_: row.try_get(9)?,
                        real_: row.try_get(10)?,
                        float8_: row.try_get(11)?,
                        double_precision_: row.try_get(12)?,
                        text_: row.try_get(13)?,
                        varchar_: row.try_get(14)?,
                        name_: row.try_get(15)?,
                        citext_: row.try_get(16)?,
                        ltree_: row.try_get(17)?,
                        bytea_: row.try_get(18)?,
                        timestamp_: row.try_get(19)?,
                        timestamp_without_time_zone_: row.try_get(20)?,
                        timestamptz_: row.try_get(21)?,
                        timestamp_with_time_zone_: row.try_get(22)?,
                        date_: row.try_get(23)?,
                        time_: row.try_get(24)?,
                        json_: row.try_get(25)?,
                        jsonb_: row.try_get(26)?,
                        uuid_: row.try_get(27)?,
                        inet_: row.try_get(28)?,
                        macaddr_: row.try_get(29)?,
                        numeric_: row.try_get(30)?,
                    })
                },
                mapper: |it| super::EverythingArrayNull::from(it),
            }
        }
    }
    pub struct InsertEverythingArrayStmt(&'static str, Option<postgres::Statement>);
    pub fn insert_everything_array() -> InsertEverythingArrayStmt {
        InsertEverythingArrayStmt(
            "INSERT INTO EverythingArray (bool_, boolean_, char_, smallint_, int2_, int_, int4_, bingint_, int8_, float4_, real_, float8_, double_precision_, text_, varchar_, name_, citext_, ltree_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_, numeric_) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31)",
            None,
        )
    }
    impl InsertEverythingArrayStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<
            'c,
            'a,
            's,
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
            T18: crate::StringSql,
            T19: crate::ArraySql<Item = T18>,
            T20: crate::StringSql,
            T21: crate::ArraySql<Item = T20>,
            T22: crate::StringSql,
            T23: crate::ArraySql<Item = T22>,
            T24: crate::BytesSql,
            T25: crate::ArraySql<Item = T24>,
            T26: crate::ArraySql<Item = chrono::NaiveDateTime>,
            T27: crate::ArraySql<Item = chrono::NaiveDateTime>,
            T28: crate::ArraySql<Item = chrono::DateTime<chrono::FixedOffset>>,
            T29: crate::ArraySql<Item = chrono::DateTime<chrono::FixedOffset>>,
            T30: crate::ArraySql<Item = chrono::NaiveDate>,
            T31: crate::ArraySql<Item = chrono::NaiveTime>,
            T32: crate::JsonSql,
            T33: crate::ArraySql<Item = T32>,
            T34: crate::JsonSql,
            T35: crate::ArraySql<Item = T34>,
            T36: crate::ArraySql<Item = uuid::Uuid>,
            T37: crate::ArraySql<Item = std::net::IpAddr>,
            T38: crate::ArraySql<Item = eui48::MacAddress>,
            T39: crate::ArraySql<Item = rust_decimal::Decimal>,
        >(
            &'s self,
            client: &'c mut C,
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
            name_: &'a T19,
            citext_: &'a T21,
            ltree_: &'a T23,
            bytea_: &'a T25,
            timestamp_: &'a T26,
            timestamp_without_time_zone_: &'a T27,
            timestamptz_: &'a T28,
            timestamp_with_time_zone_: &'a T29,
            date_: &'a T30,
            time_: &'a T31,
            json_: &'a T33,
            jsonb_: &'a T35,
            uuid_: &'a T36,
            inet_: &'a T37,
            macaddr_: &'a T38,
            numeric_: &'a T39,
        ) -> Result<u64, postgres::Error> {
            client.execute(
                self.0,
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
                    name_,
                    citext_,
                    ltree_,
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
        'c,
        'a,
        's,
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
        T18: crate::StringSql,
        T19: crate::ArraySql<Item = T18>,
        T20: crate::StringSql,
        T21: crate::ArraySql<Item = T20>,
        T22: crate::StringSql,
        T23: crate::ArraySql<Item = T22>,
        T24: crate::BytesSql,
        T25: crate::ArraySql<Item = T24>,
        T26: crate::ArraySql<Item = chrono::NaiveDateTime>,
        T27: crate::ArraySql<Item = chrono::NaiveDateTime>,
        T28: crate::ArraySql<Item = chrono::DateTime<chrono::FixedOffset>>,
        T29: crate::ArraySql<Item = chrono::DateTime<chrono::FixedOffset>>,
        T30: crate::ArraySql<Item = chrono::NaiveDate>,
        T31: crate::ArraySql<Item = chrono::NaiveTime>,
        T32: crate::JsonSql,
        T33: crate::ArraySql<Item = T32>,
        T34: crate::JsonSql,
        T35: crate::ArraySql<Item = T34>,
        T36: crate::ArraySql<Item = uuid::Uuid>,
        T37: crate::ArraySql<Item = std::net::IpAddr>,
        T38: crate::ArraySql<Item = eui48::MacAddress>,
        T39: crate::ArraySql<Item = rust_decimal::Decimal>,
    >
        crate::client::sync::Params<
            'c,
            'a,
            's,
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
                T34,
                T35,
                T36,
                T37,
                T38,
                T39,
            >,
            Result<u64, postgres::Error>,
            C,
        > for InsertEverythingArrayStmt
    {
        fn params(
            &'s self,
            client: &'c mut C,
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
                T34,
                T35,
                T36,
                T37,
                T38,
                T39,
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
                &params.name_,
                &params.citext_,
                &params.ltree_,
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
    pub struct SelectNightmareStmt(&'static str, Option<postgres::Statement>);
    pub fn select_nightmare() -> SelectNightmareStmt {
        SelectNightmareStmt("SELECT * FROM nightmare", None)
    }
    impl SelectNightmareStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s self,
            client: &'c mut C,
        ) -> NightmareCompositeQuery<'c, 'a, 's, C, crate::types::NightmareComposite, 0> {
            NightmareCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it.into(),
            }
        }
    }
    pub struct InsertNightmareStmt(&'static str, Option<postgres::Statement>);
    pub fn insert_nightmare() -> InsertNightmareStmt {
        InsertNightmareStmt("INSERT INTO nightmare (composite) VALUES ($1)", None)
    }
    impl InsertNightmareStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s self,
            client: &'c mut C,
            composite: &'a crate::types::NightmareCompositeParams<'a>,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[composite])
        }
    }
    pub struct SelectSchemaNightmareStmt(&'static str, Option<postgres::Statement>);
    pub fn select_schema_nightmare() -> SelectSchemaNightmareStmt {
        SelectSchemaNightmareStmt("SELECT * FROM schema.nightmare", None)
    }
    impl SelectSchemaNightmareStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s self,
            client: &'c mut C,
        ) -> SchemaNightmareCompositeQuery<'c, 'a, 's, C, crate::types::schema::NightmareComposite, 0>
        {
            SchemaNightmareCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it.into(),
            }
        }
    }
    pub struct InsertSchemaNightmareStmt(&'static str, Option<postgres::Statement>);
    pub fn insert_schema_nightmare() -> InsertSchemaNightmareStmt {
        InsertSchemaNightmareStmt("INSERT INTO schema.nightmare (composite) VALUES ($1)", None)
    }
    impl InsertSchemaNightmareStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s self,
            client: &'c mut C,
            composite: &'a crate::types::schema::NightmareCompositeParams<'a>,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[composite])
        }
    }
}
pub mod async_ {
    use crate::client::async_::GenericClient;
    use futures::{self, StreamExt, TryStreamExt};
    pub struct EverythingQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor:
            fn(&tokio_postgres::Row) -> Result<super::EverythingBorrowed, tokio_postgres::Error>,
        mapper: fn(super::EverythingBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> EverythingQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::EverythingBorrowed) -> R,
        ) -> EverythingQuery<'c, 'a, 's, C, R, N> {
            EverythingQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let row =
                crate::client::async_::one(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let opt_row =
                crate::client::async_::opt(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok(opt_row
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
            tokio_postgres::Error,
        > {
            let stream = crate::client::async_::raw(
                self.client,
                self.query,
                crate::slice_iter(&self.params),
                self.cached,
            )
            .await?;
            let mapped = stream
                .map(move |res| {
                    res.and_then(|row| {
                        let extracted = (self.extractor)(&row)?;
                        Ok((self.mapper)(extracted))
                    })
                })
                .into_stream();
            Ok(mapped)
        }
    }
    pub struct EverythingNullQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor: fn(
            &tokio_postgres::Row,
        ) -> Result<super::EverythingNullBorrowed, tokio_postgres::Error>,
        mapper: fn(super::EverythingNullBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> EverythingNullQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::EverythingNullBorrowed) -> R,
        ) -> EverythingNullQuery<'c, 'a, 's, C, R, N> {
            EverythingNullQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let row =
                crate::client::async_::one(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let opt_row =
                crate::client::async_::opt(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok(opt_row
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
            tokio_postgres::Error,
        > {
            let stream = crate::client::async_::raw(
                self.client,
                self.query,
                crate::slice_iter(&self.params),
                self.cached,
            )
            .await?;
            let mapped = stream
                .map(move |res| {
                    res.and_then(|row| {
                        let extracted = (self.extractor)(&row)?;
                        Ok((self.mapper)(extracted))
                    })
                })
                .into_stream();
            Ok(mapped)
        }
    }
    pub struct StringQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor: fn(&tokio_postgres::Row) -> Result<&str, tokio_postgres::Error>,
        mapper: fn(&str) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> StringQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(&str) -> R) -> StringQuery<'c, 'a, 's, C, R, N> {
            StringQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let row =
                crate::client::async_::one(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let opt_row =
                crate::client::async_::opt(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok(opt_row
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
            tokio_postgres::Error,
        > {
            let stream = crate::client::async_::raw(
                self.client,
                self.query,
                crate::slice_iter(&self.params),
                self.cached,
            )
            .await?;
            let mapped = stream
                .map(move |res| {
                    res.and_then(|row| {
                        let extracted = (self.extractor)(&row)?;
                        Ok((self.mapper)(extracted))
                    })
                })
                .into_stream();
            Ok(mapped)
        }
    }
    pub struct EverythingArrayQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor: fn(
            &tokio_postgres::Row,
        ) -> Result<super::EverythingArrayBorrowed, tokio_postgres::Error>,
        mapper: fn(super::EverythingArrayBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> EverythingArrayQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::EverythingArrayBorrowed) -> R,
        ) -> EverythingArrayQuery<'c, 'a, 's, C, R, N> {
            EverythingArrayQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let row =
                crate::client::async_::one(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let opt_row =
                crate::client::async_::opt(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok(opt_row
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
            tokio_postgres::Error,
        > {
            let stream = crate::client::async_::raw(
                self.client,
                self.query,
                crate::slice_iter(&self.params),
                self.cached,
            )
            .await?;
            let mapped = stream
                .map(move |res| {
                    res.and_then(|row| {
                        let extracted = (self.extractor)(&row)?;
                        Ok((self.mapper)(extracted))
                    })
                })
                .into_stream();
            Ok(mapped)
        }
    }
    pub struct EverythingArrayNullQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor: fn(
            &tokio_postgres::Row,
        ) -> Result<super::EverythingArrayNullBorrowed, tokio_postgres::Error>,
        mapper: fn(super::EverythingArrayNullBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> EverythingArrayNullQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::EverythingArrayNullBorrowed) -> R,
        ) -> EverythingArrayNullQuery<'c, 'a, 's, C, R, N> {
            EverythingArrayNullQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let row =
                crate::client::async_::one(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let opt_row =
                crate::client::async_::opt(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok(opt_row
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
            tokio_postgres::Error,
        > {
            let stream = crate::client::async_::raw(
                self.client,
                self.query,
                crate::slice_iter(&self.params),
                self.cached,
            )
            .await?;
            let mapped = stream
                .map(move |res| {
                    res.and_then(|row| {
                        let extracted = (self.extractor)(&row)?;
                        Ok((self.mapper)(extracted))
                    })
                })
                .into_stream();
            Ok(mapped)
        }
    }
    pub struct NightmareCompositeQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor: fn(
            &tokio_postgres::Row,
        )
            -> Result<crate::types::NightmareCompositeBorrowed, tokio_postgres::Error>,
        mapper: fn(crate::types::NightmareCompositeBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> NightmareCompositeQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(crate::types::NightmareCompositeBorrowed) -> R,
        ) -> NightmareCompositeQuery<'c, 'a, 's, C, R, N> {
            NightmareCompositeQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let row =
                crate::client::async_::one(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let opt_row =
                crate::client::async_::opt(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok(opt_row
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
            tokio_postgres::Error,
        > {
            let stream = crate::client::async_::raw(
                self.client,
                self.query,
                crate::slice_iter(&self.params),
                self.cached,
            )
            .await?;
            let mapped = stream
                .map(move |res| {
                    res.and_then(|row| {
                        let extracted = (self.extractor)(&row)?;
                        Ok((self.mapper)(extracted))
                    })
                })
                .into_stream();
            Ok(mapped)
        }
    }
    pub struct SchemaNightmareCompositeQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'s tokio_postgres::Statement>,
        extractor:
            fn(
                &tokio_postgres::Row,
            )
                -> Result<crate::types::schema::NightmareCompositeBorrowed, tokio_postgres::Error>,
        mapper: fn(crate::types::schema::NightmareCompositeBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> SchemaNightmareCompositeQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(crate::types::schema::NightmareCompositeBorrowed) -> R,
        ) -> SchemaNightmareCompositeQuery<'c, 'a, 's, C, R, N> {
            SchemaNightmareCompositeQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let row =
                crate::client::async_::one(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let opt_row =
                crate::client::async_::opt(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok(opt_row
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
            tokio_postgres::Error,
        > {
            let stream = crate::client::async_::raw(
                self.client,
                self.query,
                crate::slice_iter(&self.params),
                self.cached,
            )
            .await?;
            let mapped = stream
                .map(move |res| {
                    res.and_then(|row| {
                        let extracted = (self.extractor)(&row)?;
                        Ok((self.mapper)(extracted))
                    })
                })
                .into_stream();
            Ok(mapped)
        }
    }
    pub struct SelectEverythingStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn select_everything() -> SelectEverythingStmt {
        SelectEverythingStmt("SELECT * FROM Everything", None)
    }
    impl SelectEverythingStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s self,
            client: &'c C,
        ) -> EverythingQuery<'c, 'a, 's, C, super::Everything, 0> {
            EverythingQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |
                    row: &tokio_postgres::Row,
                | -> Result<super::EverythingBorrowed, tokio_postgres::Error> {
                    Ok(super::EverythingBorrowed {
                        bool_: row.try_get(0)?,
                        boolean_: row.try_get(1)?,
                        char_: row.try_get(2)?,
                        smallint_: row.try_get(3)?,
                        int2_: row.try_get(4)?,
                        smallserial_: row.try_get(5)?,
                        serial2_: row.try_get(6)?,
                        int_: row.try_get(7)?,
                        int4_: row.try_get(8)?,
                        serial_: row.try_get(9)?,
                        serial4_: row.try_get(10)?,
                        bingint_: row.try_get(11)?,
                        int8_: row.try_get(12)?,
                        bigserial_: row.try_get(13)?,
                        serial8_: row.try_get(14)?,
                        float4_: row.try_get(15)?,
                        real_: row.try_get(16)?,
                        float8_: row.try_get(17)?,
                        double_precision_: row.try_get(18)?,
                        text_: row.try_get(19)?,
                        varchar_: row.try_get(20)?,
                        name_: row.try_get(21)?,
                        citext_: row.try_get(22)?,
                        ltree_: row.try_get(23)?,
                        bytea_: row.try_get(24)?,
                        timestamp_: row.try_get(25)?,
                        timestamp_without_time_zone_: row.try_get(26)?,
                        timestamptz_: row.try_get(27)?,
                        timestamp_with_time_zone_: row.try_get(28)?,
                        date_: row.try_get(29)?,
                        time_: row.try_get(30)?,
                        json_: row.try_get(31)?,
                        jsonb_: row.try_get(32)?,
                        uuid_: row.try_get(33)?,
                        inet_: row.try_get(34)?,
                        macaddr_: row.try_get(35)?,
                        numeric_: row.try_get(36)?,
                    })
                },
                mapper: |it| super::Everything::from(it),
            }
        }
    }
    pub struct SelectEverythingNullStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn select_everything_null() -> SelectEverythingNullStmt {
        SelectEverythingNullStmt("SELECT * FROM Everything", None)
    }
    impl SelectEverythingNullStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s self,
            client: &'c C,
        ) -> EverythingNullQuery<'c, 'a, 's, C, super::EverythingNull, 0> {
            EverythingNullQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |
                    row: &tokio_postgres::Row,
                | -> Result<super::EverythingNullBorrowed, tokio_postgres::Error> {
                    Ok(super::EverythingNullBorrowed {
                        bool_: row.try_get(0)?,
                        boolean_: row.try_get(1)?,
                        char_: row.try_get(2)?,
                        smallint_: row.try_get(3)?,
                        int2_: row.try_get(4)?,
                        smallserial_: row.try_get(5)?,
                        serial2_: row.try_get(6)?,
                        int_: row.try_get(7)?,
                        int4_: row.try_get(8)?,
                        serial_: row.try_get(9)?,
                        serial4_: row.try_get(10)?,
                        bingint_: row.try_get(11)?,
                        int8_: row.try_get(12)?,
                        bigserial_: row.try_get(13)?,
                        serial8_: row.try_get(14)?,
                        float4_: row.try_get(15)?,
                        real_: row.try_get(16)?,
                        float8_: row.try_get(17)?,
                        double_precision_: row.try_get(18)?,
                        text_: row.try_get(19)?,
                        varchar_: row.try_get(20)?,
                        name_: row.try_get(21)?,
                        citext_: row.try_get(22)?,
                        ltree_: row.try_get(23)?,
                        bytea_: row.try_get(24)?,
                        timestamp_: row.try_get(25)?,
                        timestamp_without_time_zone_: row.try_get(26)?,
                        timestamptz_: row.try_get(27)?,
                        timestamp_with_time_zone_: row.try_get(28)?,
                        date_: row.try_get(29)?,
                        time_: row.try_get(30)?,
                        json_: row.try_get(31)?,
                        jsonb_: row.try_get(32)?,
                        uuid_: row.try_get(33)?,
                        inet_: row.try_get(34)?,
                        macaddr_: row.try_get(35)?,
                        numeric_: row.try_get(36)?,
                    })
                },
                mapper: |it| super::EverythingNull::from(it),
            }
        }
    }
    pub struct InsertEverythingStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn insert_everything() -> InsertEverythingStmt {
        InsertEverythingStmt(
            "INSERT INTO Everything (bool_, boolean_, char_, smallint_, int2_, smallserial_, serial2_, int_, int4_, serial_, serial4_, bingint_, int8_, bigserial_, serial8_, float4_, real_, float8_, double_precision_, text_, varchar_, name_, citext_, ltree_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_, numeric_) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37)",
            None,
        )
    }
    impl InsertEverythingStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub async fn bind<
            'c,
            'a,
            's,
            C: GenericClient,
            T1: crate::StringSql,
            T2: crate::StringSql,
            T3: crate::StringSql,
            T4: crate::StringSql,
            T5: crate::StringSql,
            T6: crate::BytesSql,
            T7: crate::JsonSql,
            T8: crate::JsonSql,
        >(
            &'s self,
            client: &'c C,
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
            name_: &'a T3,
            citext_: &'a T4,
            ltree_: &'a T5,
            bytea_: &'a T6,
            timestamp_: &'a chrono::NaiveDateTime,
            timestamp_without_time_zone_: &'a chrono::NaiveDateTime,
            timestamptz_: &'a chrono::DateTime<chrono::FixedOffset>,
            timestamp_with_time_zone_: &'a chrono::DateTime<chrono::FixedOffset>,
            date_: &'a chrono::NaiveDate,
            time_: &'a chrono::NaiveTime,
            json_: &'a T7,
            jsonb_: &'a T8,
            uuid_: &'a uuid::Uuid,
            inet_: &'a std::net::IpAddr,
            macaddr_: &'a eui48::MacAddress,
            numeric_: &'a rust_decimal::Decimal,
        ) -> Result<u64, tokio_postgres::Error> {
            client
                .execute(
                    self.0,
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
                        name_,
                        citext_,
                        ltree_,
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
        T3: crate::StringSql,
        T4: crate::StringSql,
        T5: crate::StringSql,
        T6: crate::BytesSql,
        T7: crate::JsonSql,
        T8: crate::JsonSql,
    >
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::EverythingParams<T1, T2, T3, T4, T5, T6, T7, T8>,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for InsertEverythingStmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::EverythingParams<T1, T2, T3, T4, T5, T6, T7, T8>,
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
                &params.name_,
                &params.citext_,
                &params.ltree_,
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
    pub struct SelectLtreeStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn select_ltree() -> SelectLtreeStmt {
        SelectLtreeStmt("SELECT ltree_ FROM Everything where $1 @> ltree_", None)
    }
    impl SelectLtreeStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
            &'s self,
            client: &'c C,
            path: &'a T1,
        ) -> StringQuery<'c, 'a, 's, C, String, 1> {
            StringQuery {
                client,
                params: [path],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it.into(),
            }
        }
    }
    pub struct SelectEverythingArrayStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn select_everything_array() -> SelectEverythingArrayStmt {
        SelectEverythingArrayStmt("SELECT * FROM EverythingArray", None)
    }
    impl SelectEverythingArrayStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s self,
            client: &'c C,
        ) -> EverythingArrayQuery<'c, 'a, 's, C, super::EverythingArray, 0> {
            EverythingArrayQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |
                    row: &tokio_postgres::Row,
                | -> Result<super::EverythingArrayBorrowed, tokio_postgres::Error> {
                    Ok(super::EverythingArrayBorrowed {
                        bool_: row.try_get(0)?,
                        boolean_: row.try_get(1)?,
                        char_: row.try_get(2)?,
                        smallint_: row.try_get(3)?,
                        int2_: row.try_get(4)?,
                        int_: row.try_get(5)?,
                        int4_: row.try_get(6)?,
                        bingint_: row.try_get(7)?,
                        int8_: row.try_get(8)?,
                        float4_: row.try_get(9)?,
                        real_: row.try_get(10)?,
                        float8_: row.try_get(11)?,
                        double_precision_: row.try_get(12)?,
                        text_: row.try_get(13)?,
                        varchar_: row.try_get(14)?,
                        name_: row.try_get(15)?,
                        citext_: row.try_get(16)?,
                        ltree_: row.try_get(17)?,
                        bytea_: row.try_get(18)?,
                        timestamp_: row.try_get(19)?,
                        timestamp_without_time_zone_: row.try_get(20)?,
                        timestamptz_: row.try_get(21)?,
                        timestamp_with_time_zone_: row.try_get(22)?,
                        date_: row.try_get(23)?,
                        time_: row.try_get(24)?,
                        json_: row.try_get(25)?,
                        jsonb_: row.try_get(26)?,
                        uuid_: row.try_get(27)?,
                        inet_: row.try_get(28)?,
                        macaddr_: row.try_get(29)?,
                        numeric_: row.try_get(30)?,
                    })
                },
                mapper: |it| super::EverythingArray::from(it),
            }
        }
    }
    pub struct SelectEverythingArrayNullStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn select_everything_array_null() -> SelectEverythingArrayNullStmt {
        SelectEverythingArrayNullStmt("SELECT * FROM EverythingArray", None)
    }
    impl SelectEverythingArrayNullStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s self,
            client: &'c C,
        ) -> EverythingArrayNullQuery<'c, 'a, 's, C, super::EverythingArrayNull, 0> {
            EverythingArrayNullQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row: &tokio_postgres::Row| -> Result<
                    super::EverythingArrayNullBorrowed,
                    tokio_postgres::Error,
                > {
                    Ok(super::EverythingArrayNullBorrowed {
                        bool_: row.try_get(0)?,
                        boolean_: row.try_get(1)?,
                        char_: row.try_get(2)?,
                        smallint_: row.try_get(3)?,
                        int2_: row.try_get(4)?,
                        int_: row.try_get(5)?,
                        int4_: row.try_get(6)?,
                        bingint_: row.try_get(7)?,
                        int8_: row.try_get(8)?,
                        float4_: row.try_get(9)?,
                        real_: row.try_get(10)?,
                        float8_: row.try_get(11)?,
                        double_precision_: row.try_get(12)?,
                        text_: row.try_get(13)?,
                        varchar_: row.try_get(14)?,
                        name_: row.try_get(15)?,
                        citext_: row.try_get(16)?,
                        ltree_: row.try_get(17)?,
                        bytea_: row.try_get(18)?,
                        timestamp_: row.try_get(19)?,
                        timestamp_without_time_zone_: row.try_get(20)?,
                        timestamptz_: row.try_get(21)?,
                        timestamp_with_time_zone_: row.try_get(22)?,
                        date_: row.try_get(23)?,
                        time_: row.try_get(24)?,
                        json_: row.try_get(25)?,
                        jsonb_: row.try_get(26)?,
                        uuid_: row.try_get(27)?,
                        inet_: row.try_get(28)?,
                        macaddr_: row.try_get(29)?,
                        numeric_: row.try_get(30)?,
                    })
                },
                mapper: |it| super::EverythingArrayNull::from(it),
            }
        }
    }
    pub struct InsertEverythingArrayStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn insert_everything_array() -> InsertEverythingArrayStmt {
        InsertEverythingArrayStmt(
            "INSERT INTO EverythingArray (bool_, boolean_, char_, smallint_, int2_, int_, int4_, bingint_, int8_, float4_, real_, float8_, double_precision_, text_, varchar_, name_, citext_, ltree_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_, numeric_) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31)",
            None,
        )
    }
    impl InsertEverythingArrayStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub async fn bind<
            'c,
            'a,
            's,
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
            T18: crate::StringSql,
            T19: crate::ArraySql<Item = T18>,
            T20: crate::StringSql,
            T21: crate::ArraySql<Item = T20>,
            T22: crate::StringSql,
            T23: crate::ArraySql<Item = T22>,
            T24: crate::BytesSql,
            T25: crate::ArraySql<Item = T24>,
            T26: crate::ArraySql<Item = chrono::NaiveDateTime>,
            T27: crate::ArraySql<Item = chrono::NaiveDateTime>,
            T28: crate::ArraySql<Item = chrono::DateTime<chrono::FixedOffset>>,
            T29: crate::ArraySql<Item = chrono::DateTime<chrono::FixedOffset>>,
            T30: crate::ArraySql<Item = chrono::NaiveDate>,
            T31: crate::ArraySql<Item = chrono::NaiveTime>,
            T32: crate::JsonSql,
            T33: crate::ArraySql<Item = T32>,
            T34: crate::JsonSql,
            T35: crate::ArraySql<Item = T34>,
            T36: crate::ArraySql<Item = uuid::Uuid>,
            T37: crate::ArraySql<Item = std::net::IpAddr>,
            T38: crate::ArraySql<Item = eui48::MacAddress>,
            T39: crate::ArraySql<Item = rust_decimal::Decimal>,
        >(
            &'s self,
            client: &'c C,
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
            name_: &'a T19,
            citext_: &'a T21,
            ltree_: &'a T23,
            bytea_: &'a T25,
            timestamp_: &'a T26,
            timestamp_without_time_zone_: &'a T27,
            timestamptz_: &'a T28,
            timestamp_with_time_zone_: &'a T29,
            date_: &'a T30,
            time_: &'a T31,
            json_: &'a T33,
            jsonb_: &'a T35,
            uuid_: &'a T36,
            inet_: &'a T37,
            macaddr_: &'a T38,
            numeric_: &'a T39,
        ) -> Result<u64, tokio_postgres::Error> {
            client
                .execute(
                    self.0,
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
                        name_,
                        citext_,
                        ltree_,
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
        T18: crate::StringSql,
        T19: crate::ArraySql<Item = T18>,
        T20: crate::StringSql,
        T21: crate::ArraySql<Item = T20>,
        T22: crate::StringSql,
        T23: crate::ArraySql<Item = T22>,
        T24: crate::BytesSql,
        T25: crate::ArraySql<Item = T24>,
        T26: crate::ArraySql<Item = chrono::NaiveDateTime>,
        T27: crate::ArraySql<Item = chrono::NaiveDateTime>,
        T28: crate::ArraySql<Item = chrono::DateTime<chrono::FixedOffset>>,
        T29: crate::ArraySql<Item = chrono::DateTime<chrono::FixedOffset>>,
        T30: crate::ArraySql<Item = chrono::NaiveDate>,
        T31: crate::ArraySql<Item = chrono::NaiveTime>,
        T32: crate::JsonSql,
        T33: crate::ArraySql<Item = T32>,
        T34: crate::JsonSql,
        T35: crate::ArraySql<Item = T34>,
        T36: crate::ArraySql<Item = uuid::Uuid>,
        T37: crate::ArraySql<Item = std::net::IpAddr>,
        T38: crate::ArraySql<Item = eui48::MacAddress>,
        T39: crate::ArraySql<Item = rust_decimal::Decimal>,
    >
        crate::client::async_::Params<
            'a,
            'a,
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
                T34,
                T35,
                T36,
                T37,
                T38,
                T39,
            >,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for InsertEverythingArrayStmt
    {
        fn params(
            &'a self,
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
                T34,
                T35,
                T36,
                T37,
                T38,
                T39,
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
                &params.name_,
                &params.citext_,
                &params.ltree_,
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
    pub struct SelectNightmareStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn select_nightmare() -> SelectNightmareStmt {
        SelectNightmareStmt("SELECT * FROM nightmare", None)
    }
    impl SelectNightmareStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s self,
            client: &'c C,
        ) -> NightmareCompositeQuery<'c, 'a, 's, C, crate::types::NightmareComposite, 0> {
            NightmareCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it.into(),
            }
        }
    }
    pub struct InsertNightmareStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn insert_nightmare() -> InsertNightmareStmt {
        InsertNightmareStmt("INSERT INTO nightmare (composite) VALUES ($1)", None)
    }
    impl InsertNightmareStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub async fn bind<'c, 'a, 's, C: GenericClient>(
            &'s self,
            client: &'c C,
            composite: &'a crate::types::NightmareCompositeParams<'a>,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[composite]).await
        }
    }
    pub struct SelectSchemaNightmareStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn select_schema_nightmare() -> SelectSchemaNightmareStmt {
        SelectSchemaNightmareStmt("SELECT * FROM schema.nightmare", None)
    }
    impl SelectSchemaNightmareStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s self,
            client: &'c C,
        ) -> SchemaNightmareCompositeQuery<'c, 'a, 's, C, crate::types::schema::NightmareComposite, 0>
        {
            SchemaNightmareCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it.into(),
            }
        }
    }
    pub struct InsertSchemaNightmareStmt(&'static str, Option<tokio_postgres::Statement>);
    pub fn insert_schema_nightmare() -> InsertSchemaNightmareStmt {
        InsertSchemaNightmareStmt("INSERT INTO schema.nightmare (composite) VALUES ($1)", None)
    }
    impl InsertSchemaNightmareStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub async fn bind<'c, 'a, 's, C: GenericClient>(
            &'s self,
            client: &'c C,
            composite: &'a crate::types::schema::NightmareCompositeParams<'a>,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[composite]).await
        }
    }
}
