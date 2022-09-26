mod cornucopia_async;
mod cornucopia_sync;

use ::cornucopia_async::IterSql;
use eui48::MacAddress;
use postgres::{Client, Config, NoTls};
use serde_json::Value;
use std::{
    borrow::Cow,
    collections::HashMap,
    net::{IpAddr, Ipv4Addr},
};
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

use crate::cornucopia_sync::{
    queries::{
        copy::{insert_clone, insert_copy, select_copy},
        domain::{
            insert_nightmare_domain, select_nightmare_domain, select_nightmare_domain_null,
            InsertNightmareDomainParams, SelectNightmareDomain, SelectNightmareDomainNull,
        },
        named::{
            named, named_by_id, named_complex, new_named_complex, new_named_hidden,
            new_named_visible, Named, NamedComplexParams, NamedParams,
        },
        nullity::{new_nullity, nullity},
        nullity::{Nullity, NullityParams},
        params::insert_book,
        params::{find_books, params_use_twice, select_book, SelectBook},
        stress::{
            insert_everything, insert_everything_array, insert_nightmare, select_everything,
            select_everything_array, select_nightmare, Everything, EverythingArray,
            EverythingArrayParams, EverythingParams,
        },
        syntax::{r#typeof, tricky_sql10, TrickySql10Params},
    },
    types::public::{
        CloneCompositeBorrowed, CopyComposite, CustomComposite, CustomCompositeBorrowed,
        DomainComposite, DomainCompositeParams, NamedComposite, NamedCompositeBorrowed,
        NightmareComposite, NightmareCompositeParams, NullityComposite, NullityCompositeParams,
        SpongebobCharacter, SyntaxComposite, SyntaxEnum,
    },
};
use ::cornucopia_sync::Params;

pub fn main() {
    let client = &mut Config::new()
        .user("postgres")
        .password("postgres")
        .host("127.0.0.1")
        .port(5435)
        .dbname("postgres")
        .connect(NoTls)
        .unwrap();
    test_copy(client);
    test_params(client);
    test_named(client);
    test_nullity(client);
    test_stress(client);
    test_domain(client);
    test_trait_sql(client);
    test_keyword_escaping(client);
}

pub fn moving<T>(_item: T) {}

pub fn test_params(client: &mut Client) {
    assert_eq!(
        1,
        insert_book()
            .bind(client, &None::<&str>, &"Necronomicon")
            .unwrap()
    );
    assert_eq!(
        1,
        insert_book()
            .bind(client, &Some("Marcel Proust"), &"In Search of Lost Time")
            .unwrap()
    );
    assert_eq!(
        select_book().bind(client).all().unwrap(),
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
    params_use_twice().bind(client, &"name").unwrap();
}

pub fn test_trait_sql(client: &mut Client) {
    let str = "hello world";
    insert_book().bind(client, &Some(str), &str).unwrap();
    find_books().bind(client, &[str].as_slice()).all().unwrap();

    let string = str.to_string();
    insert_book()
        .bind(client, &Some(string.clone()), &string)
        .unwrap();
    find_books()
        .bind(client, &vec![string.clone()])
        .all()
        .unwrap();

    let boxed = string.clone().into_boxed_str();
    insert_book()
        .bind(client, &Some(boxed.clone()), &boxed)
        .unwrap();
    find_books().bind(client, &vec![boxed]).all().unwrap();

    let cow = Cow::Borrowed(str);
    insert_book()
        .bind(client, &Some(cow.clone()), &cow)
        .unwrap();
    find_books().bind(client, &vec![cow]).all().unwrap();

    let map: HashMap<&str, &str> =
        HashMap::from_iter([("one", "1"), ("two", "2"), ("three", "3")].into_iter());

    // Old way with allocation
    let vec: Vec<_> = map.values().copied().collect();
    find_books().bind(client, &vec.as_slice()).all().unwrap();
    // A little more ergonomic
    find_books().bind(client, &vec).all().unwrap();
    // Zero allocation
    find_books()
        .bind(client, &IterSql(|| map.values().copied()))
        .all()
        .unwrap();
}

pub fn test_nullity(client: &mut Client) {
    new_nullity()
        .params(
            client,
            &NullityParams {
                composite: Some(NullityCompositeParams {
                    jsons: Some(&[None]),
                    id: 42,
                }),
                name: "James Bond",
                texts: [Some("Hello"), Some("world"), None].as_slice(),
            },
        )
        .unwrap();
    assert_eq!(
        nullity().bind(client).one().unwrap(),
        Nullity {
            composite: Some(NullityComposite {
                jsons: Some(vec![None]),
                id: 42,
            }),
            name: "James Bond".to_string(),
            texts: vec![Some("Hello".to_string()), Some("world".to_string()), None],
        }
    );
}

pub fn test_named(client: &mut Client) {
    let hidden_id = new_named_hidden()
        .params(
            client,
            &NamedParams {
                name: "secret",
                price: Some(42.0),
            },
        )
        .one()
        .unwrap()
        .id;
    let visible_id = new_named_visible()
        .params(
            client,
            &NamedParams {
                name: "stuff",
                price: Some(84.0),
            },
        )
        .one()
        .unwrap()
        .id;
    let last_id = new_named_visible()
        .bind(client, &"can't by me", &None)
        .one()
        .unwrap()
        .id;
    assert_eq!(
        named().bind(client).all().unwrap(),
        &[
            Named {
                id: hidden_id,
                name: "secret".into(),
                price: Some(42.0),
                show: false
            },
            Named {
                id: visible_id,
                name: "stuff".into(),
                price: Some(84.0),
                show: true
            },
            Named {
                id: last_id,
                name: "can't by me".into(),
                price: None,
                show: true
            }
        ]
    );
    assert_eq!(
        named_by_id().bind(client, &hidden_id).one().unwrap(),
        Named {
            id: hidden_id,
            name: "secret".into(),
            price: Some(42.0),
            show: false
        }
    );
    assert_eq!(
        named_by_id().bind(client, &visible_id).one().unwrap(),
        Named {
            id: visible_id,
            name: "stuff".into(),
            price: Some(84.0),
            show: true
        }
    );
    assert_eq!(
        named().bind(client).map(|it| it.id).all().unwrap(),
        &[hidden_id, visible_id, last_id]
    );

    new_named_complex()
        .params(
            client,
            &NamedComplexParams {
                named: NamedCompositeBorrowed {
                    wow: Some("Hello world"),
                    such_cool: None,
                },
            },
        )
        .unwrap();

    assert_eq!(
        named_complex().bind(client).one().unwrap(),
        NamedComposite {
            wow: Some("Hello world".into()),
            such_cool: None,
        },
    );
}

// Test we correctly implement borrowed version and copy derive
pub fn test_copy(client: &mut Client) {
    // Test copy
    let copy_params = CopyComposite {
        first: 42,
        second: 4.2,
    };
    moving(copy_params); // Ignore if copied
    insert_copy().bind(client, &copy_params).unwrap();
    let copy_row = select_copy().bind(client).one().unwrap();
    moving(copy_row); // Ignore if copied
    moving(copy_row);

    // Test clone
    let clone_params = CloneCompositeBorrowed {
        first: 42,
        second: "Hello world",
    };
    insert_clone().bind(client, &clone_params).unwrap();
    select_copy().bind(client).one().unwrap();
}

// Test domain erasing
pub fn test_domain(client: &mut Client) {
    let json: Value = serde_json::from_str(r#"{"name": "James Bond"}"#).unwrap();

    // Erased domain not null
    let arr = [&json];
    let params = InsertNightmareDomainParams {
        arr: IterSql(|| arr.iter()),
        json: &json,
        nb: 42,
        txt: "Hello world",
        composite: Some(DomainCompositeParams {
            arr: arr.as_slice(),
            json: &json,
            nb: 42,
            txt: "Hello world",
        }),
    };
    let expected = SelectNightmareDomain {
        arr: vec![json.clone()],
        json: json.clone(),
        nb: 42,
        txt: "Hello world".to_string(),
    };
    assert_eq!(
        1,
        insert_nightmare_domain().params(client, &params).unwrap()
    );
    let actual = select_nightmare_domain().bind(client).one().unwrap();
    assert_eq!(expected, actual);
    let expected = SelectNightmareDomainNull {
        arr: Some(vec![Some(json.clone())]),
        json: Some(json.clone()),
        nb: Some(42),
        txt: Some("Hello world".to_string()),
        composite: Some(DomainComposite {
            arr: vec![json.clone()],
            json: json.clone(),
            nb: 42,
            txt: "Hello world".to_string(),
        }),
    };
    let actual = select_nightmare_domain_null().bind(client).one().unwrap();
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

    // Every supported type
    let expected = Everything {
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
        json_: json.clone(),
        jsonb_: json.clone(),
        uuid_: Uuid::parse_str("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11").unwrap(),
        inet_: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        macaddr_: MacAddress::new([8, 0, 43, 1, 2, 3]),
    };
    let params = EverythingParams {
        bigserial_: expected.bigserial_,
        bingint_: expected.bingint_,
        bool_: expected.bool_,
        boolean_: expected.boolean_,
        bytea_: expected.bytea_.as_slice(),
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
        json_: &json,
        jsonb_: &json,
        macaddr_: expected.macaddr_,
        real_: expected.real_,
        serial2_: expected.serial2_,
        serial4_: expected.serial4_,
        serial8_: expected.serial8_,
        serial_: expected.serial_,
        smallint_: expected.smallint_,
        smallserial_: expected.smallserial_,
        text_: expected.text_.as_str(),
        time_: expected.time_,
        timestamp_: expected.timestamp_,
        timestamp_with_time_zone_: expected.timestamp_with_time_zone_,
        timestamp_without_time_zone_: expected.timestamp_without_time_zone_,
        timestamptz_: expected.timestamptz_,
        uuid_: expected.uuid_,
        varchar_: &expected.varchar_,
    };
    assert_eq!(1, insert_everything().params(client, &params).unwrap());
    let actual = select_everything().bind(client).one().unwrap();
    assert_eq!(expected, actual);

    // Every supported array type
    let expected = EverythingArray {
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
        json_: vec![json.clone()],
        jsonb_: vec![json.clone()],
        uuid_: vec![Uuid::parse_str("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11").unwrap()],
        inet_: vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))],
        macaddr_: vec![MacAddress::new([8, 0, 43, 1, 2, 3])],
    };

    let bytea = expected
        .bytea_
        .iter()
        .map(Vec::as_slice)
        .collect::<Vec<_>>();
    let txt = &expected
        .text_
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let jsons = [&json];
    let params = EverythingArrayParams {
        bingint_: &expected.bingint_,
        bool_: &expected.bool_,
        boolean_: &expected.boolean_,
        bytea_: &bytea,
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
        json_: jsons.as_slice(),
        jsonb_: jsons.as_slice(),
        macaddr_: &expected.macaddr_,
        real_: &expected.real_,
        smallint_: &expected.smallint_,
        text_: &txt,
        time_: &expected.time_,
        timestamp_: &expected.timestamp_,
        timestamp_with_time_zone_: &expected.timestamp_with_time_zone_,
        timestamp_without_time_zone_: &expected.timestamp_without_time_zone_,
        timestamptz_: &expected.timestamptz_,
        uuid_: &expected.uuid_,
        varchar_: txt,
    };
    assert_eq!(
        1,
        insert_everything_array().params(client, &params).unwrap()
    );
    let actual = select_everything_array().bind(client).one().unwrap();
    assert_eq!(expected, actual);

    // Complex mix of enum, domain and composite types
    let expected = NightmareComposite {
        custom: vec![CustomComposite {
            wow: "Bob".to_string(),
            such_cool: 42,
            nice: SpongebobCharacter::Squidward,
        }],
        spongebob: vec![SpongebobCharacter::Bob, SpongebobCharacter::Patrick],
        domain: "Hello".to_string(),
    };
    let params = NightmareCompositeParams {
        custom: &[CustomCompositeBorrowed {
            wow: "Bob",
            such_cool: 42,
            nice: SpongebobCharacter::Squidward,
        }],
        spongebob: &[SpongebobCharacter::Bob, SpongebobCharacter::Patrick],
        domain: "Hello",
    };

    assert_eq!(1, insert_nightmare().bind(client, &params).unwrap());
    let actual = select_nightmare().bind(client).one().unwrap();
    assert_eq!(expected, actual);
}

// Test keyword escaping
pub fn test_keyword_escaping(client: &mut Client) {
    let params = TrickySql10Params {
        r#async: SyntaxComposite { r#async: 34 },
        r#enum: SyntaxEnum::r#box,
    };
    tricky_sql10().params(client, &params).unwrap();
    r#typeof().bind(client).all().unwrap();
}
