use std::net::{IpAddr, Ipv4Addr};

use cornucopia_client::Json;
use cornucopia_sync::{
    queries::{
        copy::{select_copy, InsertCloneParams, InsertCopyParams},
        named::items,
        params::insert_book,
    },
    types::public::{CloneCompositeBorrowed, CopyComposite},
};
use eui48::MacAddress;
use postgres::{Client, Config, NoTls};
use serde_json::Value;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

use crate::cornucopia_sync::{
    queries::{
        domain::{
            select_nightmare_domain, select_nightmare_domain_null, InsertNightmareDomainParams,
            SelectNightmareDomain, SelectNightmareDomainNull,
        },
        named::{item_by_id, new_item_visible, Item, ItemParams},
        params::{params_use_twice, select_book, SelectBook},
        stress::{
            select_everything, select_everything_array, select_nightmare,
            InsertEverythingArrayParams, InsertEverythingParams, InsertNightmareParams,
            SelectEverything, SelectEverythingArray, SelectNightmare,
        },
    },
    types::public::{
        CustomComposite, CustomCompositeBorrowed, NightmareComposite, NightmareCompositeParams,
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
    test_params(client);
    test_named(client);
    test_stress(client);
}

pub fn moving<T>(_item: T) {}

pub fn test_params(client: &mut Client) {
    assert_eq!(1, insert_book(client, &None, &"Necronomicon").unwrap());
    assert_eq!(
        1,
        insert_book(client, &Some("Marcel Proust"), &"In Search of Lost Time").unwrap()
    );
    assert_eq!(
        select_book(client).vec().unwrap(),
        &[
            SelectBook {
                author: None,
                name: "Necronomicon".into()
            },
            SelectBook {
                author: Some("Marcel Proust".into()),
                name: "In Search of Lost Time".into()
            }
        ]
    );
    params_use_twice(client, &"name").unwrap();
}

pub fn test_named(client: &mut Client) {
    let hidden_id = ItemParams {
        name: "secret",
        price: Some(42.0),
    }
    .new_item_hidden(client)
    .one()
    .unwrap()
    .id;
    let visible_id = ItemParams {
        name: "stuff",
        price: Some(84.0),
    }
    .new_item_visible(client)
    .one()
    .unwrap()
    .id;
    let last_id = new_item_visible(client, &"can't by me", &None)
        .one()
        .unwrap()
        .id;
    assert_eq!(
        items(client).vec().unwrap(),
        &[
            Item {
                id: hidden_id,
                name: "secret".into(),
                price: Some(42.0),
                show: false
            },
            Item {
                id: visible_id,
                name: "stuff".into(),
                price: Some(84.0),
                show: true
            },
            Item {
                id: last_id,
                name: "can't by me".into(),
                price: None,
                show: true
            }
        ]
    );
    assert_eq!(
        item_by_id(client, &hidden_id).one().unwrap(),
        Item {
            id: hidden_id,
            name: "secret".into(),
            price: Some(42.0),
            show: false
        }
    );
    assert_eq!(
        item_by_id(client, &visible_id).one().unwrap(),
        Item {
            id: visible_id,
            name: "stuff".into(),
            price: Some(84.0),
            show: true
        }
    );
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
    moving(copy_params); // Ignore if copied
    copy_params.insert_copy(client).unwrap();
    let copy_row = select_copy(client).one().unwrap();
    moving(copy_row); // Ignore if copied
    moving(copy_row);

    // Test clone
    let clone_params = InsertCloneParams {
        composite: CloneCompositeBorrowed {
            first: 42,
            second: "Hello world",
        },
    };
    clone_params.insert_clone(client).unwrap();
    select_copy(client).one().unwrap();
}

// Test domain erasing
pub fn domain(client: &mut Client) {
    let json: Value = serde_json::from_str("{}").unwrap();
    let raw_json = &*serde_json::value::to_raw_value(&json).unwrap();

    // Erased domain not null
    let params = InsertNightmareDomainParams {
        arr: &[Json::from(raw_json)],
        json: Json::from(raw_json),
        nb: 42,
        txt: "Hello world",
    };
    let expected = SelectNightmareDomain {
        arr: vec![Json::from(json.clone())],
        json: Json::from(json.clone()),
        nb: 42,
        txt: "Hello world".to_string(),
    };
    assert_eq!(1, params.insert_nightmare_domain(client).unwrap());
    let actual = select_nightmare_domain(client).one().unwrap();
    assert_eq!(expected, actual);
    let expected = SelectNightmareDomainNull {
        arr: Some(vec![Json::from(json.clone())]),
        json: Some(Json::from(json.clone())),
        nb: Some(42),
        txt: Some("Hello world".to_string()),
    };
    let actual = select_nightmare_domain_null(client).one().unwrap();
    assert_eq!(expected, actual);
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
    let json: Value = serde_json::from_str("{}").unwrap();
    let raw_json = &*serde_json::value::to_raw_value(&json).unwrap();

    // Every supported type
    let expected = SelectEverything {
        bool_: true,
        boolean_: false,
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
        timestamp_: primitive_datetime,
        timestamp_without_time_zone_: primitive_datetime,
        timestamptz_: offset_datetime,
        timestamp_with_time_zone_: offset_datetime,
        date_: time::Date::from_calendar_date(1999, time::Month::January, 8).unwrap(),
        time_: time::Time::from_hms_milli(4, 5, 6, 789).unwrap(),
        json_: Json::from(json.clone()),
        jsonb_: Json::from(json.clone()),
        uuid_: Uuid::parse_str("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11").unwrap(),
        inet_: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        macaddr_: MacAddress::new([8, 0, 43, 1, 2, 3]),
    };
    let params = InsertEverythingParams {
        bigserial_: expected.bigserial_,
        bingint_: expected.bingint_,
        bool_: expected.bool_,
        boolean_: expected.boolean_,
        bytea_: &expected.bytea_,
        char_: expected.char_,
        date_: expected.date_,
        double_precision_: expected.double_precision_,
        float4_: expected.float4_,
        float8_: expected.float8_,
        inet_: expected.inet_,
        int2_: expected.int2_,
        int4_: expected.int4_,
        int8_: expected.int8_,
        int_: expected.int_,
        json_: Json::from(raw_json),
        jsonb_: Json::from(raw_json),
        macaddr_: expected.macaddr_,
        real_: expected.real_,
        serial2_: expected.serial2_,
        serial4_: expected.serial4_,
        serial8_: expected.serial8_,
        serial_: expected.serial_,
        smallint_: expected.smallint_,
        smallserial_: expected.smallserial_,
        text_: &expected.text_,
        time_: expected.time_,
        timestamp_: expected.timestamp_,
        timestamp_with_time_zone_: expected.timestamp_with_time_zone_,
        timestamp_without_time_zone_: expected.timestamp_without_time_zone_,
        timestamptz_: expected.timestamptz_,
        uuid_: expected.uuid_,
        varchar_: &expected.varchar_,
    };
    assert_eq!(1, params.insert_everything(client).unwrap());
    let actual = select_everything(client).one().unwrap();
    assert_eq!(expected, actual);

    // Every supported array type
    let expected = SelectEverythingArray {
        bool_: vec![true],
        boolean_: vec![true],
        char_: vec![42i8],
        smallint_: vec![300i16],
        int2_: vec![300i16],
        int_: vec![100000i32],
        int4_: vec![100000i32],
        bingint_: vec![10000000000i64],
        int8_: vec![10000000000i64],
        float4_: vec![1.12f32],
        real_: vec![1.12f32],
        float8_: vec![1.1231231231f64],
        double_precision_: vec![1.1231231231f64],
        text_: vec![String::from("hello")],
        varchar_: vec![String::from("hello")],
        bytea_: vec![vec![222u8, 173u8, 190u8, 239u8]],
        timestamp_: vec![primitive_datetime],
        timestamp_without_time_zone_: vec![primitive_datetime],
        timestamptz_: vec![offset_datetime],
        timestamp_with_time_zone_: vec![offset_datetime],
        date_: vec![time::Date::from_calendar_date(1999, time::Month::January, 8).unwrap()],
        time_: vec![time::Time::from_hms_milli(4, 5, 6, 789).unwrap()],
        json_: vec![Json::from(json.clone())],
        jsonb_: vec![Json::from(json)],
        uuid_: vec![Uuid::parse_str("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11").unwrap()],
        inet_: vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))],
        macaddr_: vec![MacAddress::new([8, 0, 43, 1, 2, 3])],
    };
    let params = InsertEverythingArrayParams {
        bingint_: &expected.bingint_,
        bool_: &expected.bool_,
        boolean_: &expected.boolean_,
        bytea_: &expected
            .bytea_
            .iter()
            .map(|v| v.as_slice())
            .collect::<Vec<_>>(),
        char_: &expected.char_,
        date_: &expected.date_,
        double_precision_: &expected.double_precision_,
        float4_: &expected.float4_,
        float8_: &expected.float8_,
        inet_: &expected.inet_,
        int2_: &expected.int2_,
        int4_: &expected.int4_,
        int8_: &expected.int8_,
        int_: &expected.int_,
        json_: &[Json::from(raw_json)],
        jsonb_: &[Json::from(raw_json)],
        macaddr_: &expected.macaddr_,
        real_: &expected.real_,
        smallint_: &expected.smallint_,
        text_: &expected
            .text_
            .iter()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        time_: &expected.time_,
        timestamp_: &expected.timestamp_,
        timestamp_with_time_zone_: &expected.timestamp_with_time_zone_,
        timestamp_without_time_zone_: &expected.timestamp_without_time_zone_,
        timestamptz_: &expected.timestamptz_,
        uuid_: &expected.uuid_,
        varchar_: &expected
            .varchar_
            .iter()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
    };
    assert_eq!(1, params.insert_everything_array(client).unwrap());
    let actual = select_everything_array(client).one().unwrap();
    assert_eq!(expected, actual);

    // Complex mix of enum, domain and composite types
    let expected = SelectNightmare {
        composite: NightmareComposite {
            custom: vec![CustomComposite {
                wow: "Bob".to_string(),
                such_cool: 42,
                nice: SpongebobCharacter::Squidward,
            }],
            spongebob: vec![SpongebobCharacter::Bob, SpongebobCharacter::Patrick],
            domain: "Hello".to_string(),
        },
    };
    let params = InsertNightmareParams {
        composite: NightmareCompositeParams {
            custom: &[CustomCompositeBorrowed {
                wow: "Bob",
                such_cool: 42,
                nice: SpongebobCharacter::Squidward,
            }],
            spongebob: &[SpongebobCharacter::Bob, SpongebobCharacter::Patrick],
            domain: "Hello",
        },
    };

    assert_eq!(1, params.insert_nightmare(client).unwrap());
    let actual = select_nightmare(client).one().unwrap();
    assert_eq!(expected, actual);
}
