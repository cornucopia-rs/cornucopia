use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};
use eui48::MacAddress;
use postgres::{Client, Config, NoTls};
use rust_decimal::Decimal;
use serde_json::Value;
use std::{
    borrow::Cow,
    collections::HashMap,
    net::{IpAddr, Ipv4Addr},
};
use uuid::Uuid;

use codegen::{
    IterSql,
    client::sync::Params,
    queries::{
        copy::sync::{insert_clone, insert_copy, select_copy},
        domain::{
            InsertNightmareDomainParams, SelectNightmareDomain, SelectNightmareDomainNull,
            sync::{
                insert_nightmare_domain, select_nightmare_domain, select_nightmare_domain_null,
            },
        },
        named::{
            Named, NamedComplex, NamedComplexParams, NamedParams,
            sync::{
                named, named_by_id, named_complex, new_named_complex, new_named_hidden,
                new_named_visible,
            },
        },
        nullity::{
            Nullity, NullityParams,
            sync::{
                new_nullity, nullity, test_direct_nullity, test_named_direct, test_named_nested,
                test_nested_nullity, test_single_direct, test_single_nested,
            },
        },
        params::{
            SelectBook,
            sync::{find_books, insert_book, params_use_twice, select_book},
        },
        stress::{
            Everything, EverythingArray, EverythingArrayParams, EverythingParams,
            sync::{
                insert_everything, insert_everything_array, insert_nightmare,
                insert_schema_nightmare, select_everything, select_everything_array, select_ltree,
                select_nightmare, select_schema_nightmare,
            },
        },
        syntax::{
            SelectInlineComment, TrickySql10Params,
            sync::{select_inline_comment, tricky_sql10, r#typeof},
        },
    },
    types::{
        CloneCompositeBorrowed, CopyComposite, CustomComposite, CustomCompositeBorrowed,
        DomainComposite, DomainCompositeParams, EnumWithDot, NamedComposite,
        NamedCompositeBorrowed, NamedCompositeWithDot, NightmareComposite,
        NightmareCompositeParams, NullityComposite, NullityCompositeParams, SpongebobCharacter,
        SyntaxComposite, SyntaxEnum, schema,
    },
};

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
    test_inline_comment(client);
}

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

    let map: HashMap<&str, &str> = HashMap::from_iter([("one", "1"), ("two", "2"), ("three", "3")]);

    // Old way with allocation
    let vec: Vec<_> = map.values().collect();
    find_books().bind(client, &vec.as_slice()).all().unwrap();
    // A little more ergonomic
    find_books().bind(client, &vec).all().unwrap();
    // Zero allocation
    find_books()
        .bind(client, &IterSql(|| map.values()))
        .all()
        .unwrap();
}

pub fn test_nullity(client: &mut Client) {
    new_nullity()
        .params(
            client,
            &NullityParams {
                composite: Some(NullityCompositeParams {
                    jsons: Some(&[]),
                    id: Some(42),
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
                jsons: Some(vec![]),
                id: Some(42),
            }),
            name: "James Bond".to_string(),
            texts: vec![Some("Hello".to_string()), Some("world".to_string()), None],
        }
    );

    let result = test_nested_nullity().bind(client).one().unwrap();
    assert!(result.jsons.is_some());
    assert!(result.id.is_none() || result.id.is_some());

    let result = test_single_nested().bind(client).one().unwrap();
    assert!(result.jsons.is_some());
    assert!(result.id.is_none() || result.id.is_some());

    let result = test_direct_nullity().bind(client).one().unwrap();
    if let Some(composite) = result {
        assert!(composite.jsons.is_some());
        assert!(composite.id.is_none() || composite.id.is_some());
    }

    let result = test_single_direct().bind(client).one().unwrap();
    if let Some(composite) = result {
        assert!(composite.jsons.is_some());
        assert!(composite.id.is_none() || composite.id.is_some());
    }

    let result = test_named_nested().bind(client).one().unwrap();
    assert!(result.composite.jsons.is_some());
    assert!(result.composite.id.is_none() || result.composite.id.is_some());

    let result = test_named_direct().bind(client).one().unwrap();
    if let Some(composite) = result.composite {
        assert!(composite.jsons.is_some());
        assert!(composite.id.is_none() || composite.id.is_some());
    }

    new_nullity()
        .params(
            client,
            &NullityParams {
                composite: Some(NullityCompositeParams {
                    jsons: None, // Test null jsons
                    id: Some(0), // id will be treated as nullable due to nested specifications
                }),
                name: "Null Test",
                texts: [None::<&str>].as_slice(),
            },
        )
        .unwrap();

    let results = nullity().bind(client).all().unwrap();
    assert!(results.len() >= 2);

    let null_test_record = results.iter().find(|r| r.name == "Null Test").unwrap();
    if let Some(composite) = &null_test_record.composite {
        assert!(composite.jsons.is_none());
    }
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
                named_with_dot: Some(NamedCompositeWithDot {
                    this_is_inconceivable: Some(EnumWithDot::variant_with_dot),
                }),
            },
        )
        .unwrap();

    new_named_complex()
        .params(
            client,
            &NamedComplexParams {
                named: NamedCompositeBorrowed {
                    wow: Some("Hello world, again"),
                    such_cool: None,
                },
                named_with_dot: None,
            },
        )
        .unwrap();

    assert_eq!(
        named_complex().bind(client).all().unwrap(),
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
#[allow(dropping_copy_types)]
pub fn test_copy(client: &mut Client) {
    // Test copy
    let copy_params = CopyComposite {
        first: 42,
        second: 4.2,
    };
    drop(copy_params); // Ignore if copied
    insert_copy().bind(client, &copy_params).unwrap();
    let copy_row = select_copy().bind(client).one().unwrap();
    drop(copy_row); // Ignore if copied
    drop(copy_row);

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
    let naive_datetime =
        NaiveDateTime::parse_from_str("2020-01-02 03:04:05", "%Y-%m-%d %H:%M:%S").unwrap();
    let offset_datetime =
        DateTime::<FixedOffset>::parse_from_rfc3339("1985-04-12T23:20:50.52Z").unwrap();

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
        citext_: String::from("hello"),
        ltree_: String::from("Teyvat.Fontaine.Court"),
        bytea_: vec![222u8, 173u8, 190u8, 239u8],
        timestamp_: naive_datetime,
        timestamp_without_time_zone_: naive_datetime,
        timestamptz_: offset_datetime,
        timestamp_with_time_zone_: offset_datetime,
        date_: NaiveDate::from_ymd_opt(1999, 1, 8).unwrap(),
        time_: NaiveTime::from_hms_milli_opt(4, 5, 6, 789).unwrap(),
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
        timestamp_: naive_datetime,
        timestamp_with_time_zone_: offset_datetime,
        timestamp_without_time_zone_: naive_datetime,
        timestamptz_: offset_datetime,
        uuid_: expected.uuid_,
        varchar_: &expected.varchar_,
        citext_: &expected.citext_,
        ltree_: &expected.ltree_,
        numeric_: Decimal::new(202, 2),
    };

    assert_eq!(1, insert_everything().params(client, &params).unwrap());
    let actual = select_everything().bind(client).one().unwrap();
    assert_eq!(expected, actual);

    // ltree query
    let ltree_select = select_ltree().bind(client, &"Teyvat").one().unwrap();
    assert_eq!("Teyvat.Fontaine.Court", ltree_select);

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
        citext_: vec![String::from("hello")],
        ltree_: vec![String::from("Teyvat.Fontaine.Court")],
        bytea_: vec![vec![222u8, 173u8, 190u8, 239u8]],
        timestamp_: vec![naive_datetime],
        timestamp_without_time_zone_: vec![naive_datetime],
        timestamptz_: vec![offset_datetime],
        timestamp_with_time_zone_: vec![offset_datetime],
        date_: vec![NaiveDate::from_ymd_opt(1999, 1, 8).unwrap()],
        time_: vec![NaiveTime::from_hms_milli_opt(4, 5, 6, 789).unwrap()],
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

    let ltree = &expected
        .ltree_
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
        citext_: &txt,
        ltree_: &ltree,
        numeric_: &expected.numeric_,
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

    // In a named schema
    let expected = schema::NightmareComposite {
        custom: vec![CustomComposite {
            wow: "Bob".to_string(),
            such_cool: 42,
            nice: SpongebobCharacter::Squidward,
        }],
        spongebob: vec![SpongebobCharacter::Bob, SpongebobCharacter::Patrick],
        domain: "Hello".to_string(),
    };

    let params = schema::NightmareCompositeParams {
        custom: &[CustomCompositeBorrowed {
            wow: "Bob",
            such_cool: 42,
            nice: SpongebobCharacter::Squidward,
        }],
        spongebob: &[SpongebobCharacter::Bob, SpongebobCharacter::Patrick],
        domain: "Hello",
    };

    assert_eq!(1, insert_schema_nightmare().bind(client, &params).unwrap());
    let actual = select_schema_nightmare().bind(client).one().unwrap();
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

// Test inline comment removing
pub fn test_inline_comment(client: &mut Client) {
    let expected = SelectInlineComment {
        c1: "-- dont remove this\\n".to_string(),
        c2: "-- or this".to_string(),
        c3: "-- escape string here".to_string(),
        c4: "-- another escape string".to_string(),
        c5: "-- dollar quoted here".to_string(),
    };
    let actual = select_inline_comment().bind(client).one().unwrap();
    assert_eq!(expected, actual);
}
