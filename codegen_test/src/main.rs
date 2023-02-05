mod cornucopia;

use ::cornucopia_async::IterSql;
use eui48::MacAddress;
use postgres::{Client, Config, NoTls};
use rust_decimal::Decimal;
use serde_json::Value;
use std::{
    borrow::Cow,
    collections::HashMap,
    net::{IpAddr, Ipv4Addr},
};
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

use crate::cornucopia::{
    queries::{
        copy::sync::{insert_clone, insert_copy, select_copy},
        domain::{
            sync::{
                insert_nightmare_domain, select_nightmare_domain, select_nightmare_domain_null,
            },
            InsertNightmareDomainParams, SelectNightmareDomain, SelectNightmareDomainNull,
        },
        named::sync::{
            named, named_by_id, named_complex, new_named_complex, new_named_hidden,
            new_named_visible,
        },
        named::{Named, NamedComplex, NamedComplexParams, NamedParams},
        nullity::sync::{new_nullity, nullity},
        nullity::{Nullity, NullityParams},
        params::sync::insert_book,
        params::{
            sync::{find_books, params_use_twice, select_book},
            SelectBook,
        },
        stress::{
            sync::{
                insert_everything, insert_everything_array, insert_nightmare, select_everything,
                select_everything_array, select_nightmare,
            },
            Everything, EverythingArray, EverythingArrayParams, EverythingParams,
        },
        syntax::{
            sync::{r#typeof, tricky_sql10},
            TrickySql10Params,
        },
    },
    types::public::{
        CloneCompositeBorrowed, CopyComposite, CustomComposite, CustomCompositeBorrowed,
        DomainComposite, DomainCompositeParams, EnumWithDot, NamedComposite,
        NamedCompositeBorrowed, NamedCompositeWithDot, NightmareComposite,
        NightmareCompositeParams, NullityComposite, NullityCompositeParams, SpongebobCharacter,
        SyntaxComposite, SyntaxEnum,
    },
};
use cornucopia_sync::Params;

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
        insert_book(client)
            .bind(&None::<&str>, &"Necronomicon")
            .unwrap()
    );
    assert_eq!(
        1,
        insert_book(client)
            .bind(&Some("Marcel Proust"), &"In Search of Lost Time")
            .unwrap()
    );
    assert_eq!(
        select_book(client).bind().all().unwrap(),
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
    params_use_twice(client).bind(&"name").unwrap();
}

pub fn test_trait_sql(client: &mut Client) {
    let str = "hello world";
    insert_book(client).bind(&Some(str), &str).unwrap();
    find_books(client).bind(&[str].as_slice()).all().unwrap();

    let string = str.to_string();
    insert_book(client)
        .bind(&Some(string.clone()), &string)
        .unwrap();
    find_books(client)
        .bind(&vec![string.clone()])
        .all()
        .unwrap();

    let boxed = string.into_boxed_str();
    insert_book(client)
        .bind(&Some(boxed.clone()), &boxed)
        .unwrap();
    find_books(client).bind(&vec![boxed]).all().unwrap();

    let cow = Cow::Borrowed(str);
    insert_book(client).bind(&Some(cow.clone()), &cow).unwrap();
    find_books(client).bind(&vec![cow]).all().unwrap();

    let map: HashMap<&str, &str> =
        HashMap::from_iter([("one", "1"), ("two", "2"), ("three", "3")].into_iter());

    // Old way with allocation
    let vec: Vec<_> = map.values().collect();
    find_books(client).bind(&vec.as_slice()).all().unwrap();
    // A little more ergonomic
    find_books(client).bind(&vec).all().unwrap();
    // Zero allocation
    find_books(client)
        .bind(&IterSql(|| map.values()))
        .all()
        .unwrap();
}

pub fn test_nullity(client: &mut Client) {
    new_nullity(client)
        .params(&NullityParams {
            composite: Some(NullityCompositeParams {
                jsons: Some(&[None]),
                id: 42,
            }),
            name: "James Bond",
            texts: [Some("Hello"), Some("world"), None].as_slice(),
        })
        .unwrap();
    assert_eq!(
        nullity(client).bind().one().unwrap(),
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
    let hidden_id = new_named_hidden(client)
        .params(&NamedParams {
            name: "secret",
            price: Some(42.0),
        })
        .one()
        .unwrap()
        .id;
    let visible_id = new_named_visible(client)
        .params(&NamedParams {
            name: "stuff",
            price: Some(84.0),
        })
        .one()
        .unwrap()
        .id;
    let last_id = new_named_visible(client)
        .bind(&"can't by me", &None)
        .one()
        .unwrap()
        .id;
    assert_eq!(
        named(client).bind().all().unwrap(),
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
        named_by_id(client).bind(&hidden_id).one().unwrap(),
        Named {
            id: hidden_id,
            name: "secret".into(),
            price: Some(42.0),
            show: false
        }
    );
    assert_eq!(
        named_by_id(client).bind(&visible_id).one().unwrap(),
        Named {
            id: visible_id,
            name: "stuff".into(),
            price: Some(84.0),
            show: true
        }
    );
    assert_eq!(
        named(client).bind().map(|it| it.id).all().unwrap(),
        &[hidden_id, visible_id, last_id]
    );

    new_named_complex(client)
        .params(&NamedComplexParams {
            named: NamedCompositeBorrowed {
                wow: Some("Hello world"),
                such_cool: None,
            },
            named_with_dot: Some(NamedCompositeWithDot {
                this_is_inconceivable: Some(EnumWithDot::variant_with_dot),
            }),
        })
        .unwrap();

    new_named_complex(client)
        .params(&NamedComplexParams {
            named: NamedCompositeBorrowed {
                wow: Some("Hello world, again"),
                such_cool: None,
            },
            named_with_dot: None,
        })
        .unwrap();

    assert_eq!(
        named_complex(client).bind().all().unwrap(),
        vec![
            NamedComplex {
                named: NamedComposite {
                    wow: Some("Hello world".into()),
                    such_cool: None,
                },
                named_with_dot: Some(NamedCompositeWithDot {
                    this_is_inconceivable: Some(EnumWithDot::variant_with_dot),
                }),
            },
            NamedComplex {
                named: NamedComposite {
                    wow: Some("Hello world, again".into()),
                    such_cool: None,
                },
                named_with_dot: None,
            }
        ],
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
    insert_copy(client).bind(&copy_params).unwrap();
    let copy_row = select_copy(client).bind().one().unwrap();
    moving(copy_row); // Ignore if copied
    moving(copy_row);

    // Test clone
    let clone_params = CloneCompositeBorrowed {
        first: 42,
        second: "Hello world",
    };
    insert_clone(client).bind(&clone_params).unwrap();
    select_copy(client).bind().one().unwrap();
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
    assert_eq!(1, insert_nightmare_domain(client).params(&params).unwrap());
    let actual = select_nightmare_domain(client).bind().one().unwrap();
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
    let actual = select_nightmare_domain_null(client).bind().one().unwrap();
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
        numeric_: Decimal::new(202, 2),
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
        numeric_: Decimal::new(202, 2),
    };
    assert_eq!(1, insert_everything(client).params(&params).unwrap());
    let actual = select_everything(client).bind().one().unwrap();
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
        numeric_: vec![Decimal::new(202, 2)],
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
        numeric_: &expected.numeric_,
    };
    assert_eq!(1, insert_everything_array(client).params(&params).unwrap());
    let actual = select_everything_array(client).bind().one().unwrap();
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

    assert_eq!(1, insert_nightmare(client).bind(&params).unwrap());
    let actual = select_nightmare(client).bind().one().unwrap();
    assert_eq!(expected, actual);
}

// Test keyword escaping
pub fn test_keyword_escaping(client: &mut Client) {
    let params = TrickySql10Params {
        r#async: SyntaxComposite { r#async: 34 },
        r#enum: SyntaxEnum::r#box,
    };
    tricky_sql10(client).params(&params).unwrap();
    r#typeof(client).bind().all().unwrap();
}
