use std::net::{IpAddr, Ipv4Addr};

use cornucopia_sync::{
    queries::copy::{select_copy, InsertCloneParams, InsertCopyParams},
    types::public::{CloneCompositeBorrowed, CopyComposite},
};
use eui48::MacAddress;
use postgres::{Client, Config, NoTls};
use postgres_types::Json;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

use crate::cornucopia_sync::{
    queries::stress::{nightmare, select_everything, InsertEverythingParams, SelectEverything},
    types::public::{
        CustomComposite, CustomCompositeBorrowed, CustomDomainParams, MyDomainBorrowed,
        SpongebobCharacter,
    },
};

mod cornucopia_async;
mod cornucopia_sync;

pub fn main() {
    let client = &mut Config::new()
        .user("postgres")
        .password("postgres")
        .host("127.0.0.1")
        .port(5432)
        .dbname("postgres")
        .connect(NoTls)
        .unwrap();
    test_copy(client);
    test_stress(client);
}

// Test we correctly implement borrowed version and copy derive
pub fn test_copy(client: &mut Client) {
    // Test copy
    let copy_params = InsertCopyParams {
        composite: CopyComposite {
            first: 42,
            second: 4.2,
        },
    };
    std::mem::drop(copy_params); // Ignore if copied
    copy_params.insert_copy(client).unwrap();
    let copy_row = select_copy(client).one().unwrap();
    std::mem::drop(copy_row); // Ignore if copied
    std::mem::drop(copy_row);

    // Test clone
    let clone_params = InsertCloneParams {
        composite: CloneCompositeBorrowed {
            first: 42,
            second: "Hello world",
        },
    };
    std::mem::drop(copy_params); // Ignore if copied
    clone_params.insert_clone(client).unwrap();
    select_copy(client).one().unwrap();
}

// Test hard cases
pub fn test_stress(client: &mut Client) {
    let primitive_datetime_format =
        time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
    let primitive_datetime =
        PrimitiveDateTime::parse("2020-01-02 03:04:05", &primitive_datetime_format).unwrap();
    let offset_datetime = OffsetDateTime::parse(
        "1985-04-12T23:20:50.52Z",
        &time::format_description::well_known::Rfc3339,
    )
    .unwrap();
    let tmp = &[CustomCompositeBorrowed {
        wow: &"",
        such_cool: 3,
        nice: SpongebobCharacter::Bob,
    }];
    let params = InsertEverythingParams {
        custom_domain_: CustomDomainParams(tmp),
        domain_: MyDomainBorrowed(&"hello"),
        array_: &[true, false],
        custom_array_: &[SpongebobCharacter::Bob, SpongebobCharacter::Patrick],
        bool_: true,
        boolean_: true,
        char_: 42i8,
        smallint_: 300i16,
        int2_: 300i16,
        smallserial_: 300i16,
        serial2_: 300i16,
        int_: 100000i32,
        int4_: 100000i32,
        serial_: 100000i32,
        serial4_: 100000i32,
        bingint_: 10000000000i64,
        int8_: 10000000000i64,
        bigserial_: 10000000000i64,
        serial8_: 10000000000i64,
        float4_: 1.12f32,
        real_: 1.12f32,
        float8_: 1.1231231231f64,
        double_precision_: 1.1231231231f64,
        text_: "hello",
        varchar_: "hello",
        bytea_: &[222u8, 173u8, 190u8, 239u8],
        timestamp_: primitive_datetime.clone(),
        timestamp_without_time_zone_: primitive_datetime,
        timestamptz_: offset_datetime.clone(),
        timestamp_with_time_zone_: offset_datetime,
        date_: time::Date::from_calendar_date(1999, time::Month::January, 8).unwrap(),
        time_: time::Time::from_hms_milli(4, 5, 6, 789).unwrap(),
        json_: Json(serde_json::from_str("{}").unwrap()),
        jsonb_: Json(serde_json::from_str("{}").unwrap()),
        uuid_: Uuid::parse_str("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11").unwrap(),
        inet_: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        macaddr_: MacAddress::new([8, 0, 43, 1, 2, 3]),
    };

    assert_eq!(1, params.insert_everything(client).unwrap());

    let expected = SelectEverything {
        custom_domain_: vec![CustomComposite {
            wow: String::from(""),
            such_cool: 3,
            nice: SpongebobCharacter::Bob,
        }],
        domain_: String::from("hello"),
        array_: vec![true, false],
        custom_array_: vec![SpongebobCharacter::Bob, SpongebobCharacter::Patrick],
        bool_: true,
        bool_opt: Some(true),
        boolean_: true,
        char_: 42i8,
        smallint_: 300i16,
        int2_: 300i16,
        smallserial_: 300i16,
        serial2_: 300i16,
        int_: 100000i32,
        int4_: 100000i32,
        serial_: 100000i32,
        serial4_: 100000i32,
        bingint_: 10000000000i64,
        int8_: 10000000000i64,
        bigserial_: 10000000000i64,
        serial8_: 10000000000i64,
        float4_: 1.12f32,
        real_: 1.12f32,
        float8_: 1.1231231231f64,
        double_precision_: 1.1231231231f64,
        text_: String::from("hello"),
        varchar_: String::from("hello"),
        bytea_: vec![222u8, 173u8, 190u8, 239u8],
        timestamp_: primitive_datetime.clone(),
        timestamp_without_time_zone_: primitive_datetime,
        timestamptz_: offset_datetime.clone(),
        timestamp_with_time_zone_: offset_datetime,
        date_: time::Date::from_calendar_date(1999, time::Month::January, 8).unwrap(),
        time_: time::Time::from_hms_milli(4, 5, 6, 789).unwrap(),
        json_: Json(serde_json::from_str("{}").unwrap()),
        jsonb_: Json(serde_json::from_str("{}").unwrap()),
        uuid_: Uuid::parse_str("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11").unwrap(),
        inet_: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        macaddr_: MacAddress::new([8, 0, 43, 1, 2, 3]),
    };
    let actual = select_everything(client).one().unwrap();

    assert_eq!(expected, actual);

    nightmare(client).one().unwrap();
}
