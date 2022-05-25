use std::net::{IpAddr, Ipv4Addr};

//use error::Error;
use cornucopia_client::types::Json;
use eui48::MacAddress;
use postgres::Client;
use serde_json::Map;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;
/*
use self::error::Error;

use crate::integration::cornucopia::{
    self,
    queries::module_2::{select_everything, InsertEverythingParams, SelectEverything},
    types::public::{CustomComposite, CustomDomain, MyDomain, SpongebobCharacter},
};

fn setup() -> Result<Client, crate::error::Error> {
    use crate::run_migrations::run_migrations;
    use crate::{conn::cornucopia_conn, container};

    container::setup(true)?;
    let mut client = cornucopia_conn()?;
    run_migrations(&mut client, "src/integration/migrations")?;

    Ok(client)
}

fn teardown() -> Result<(), crate::error::Error> {
    use crate::container;
    container::cleanup(true)?;
    Ok(())
}

fn integration() -> Result<(), Error> {
    let mut client = setup()?;
    select_everything_test(&mut client)?;
    teardown()?;

    Ok(())
}

#[test]

fn integration_test() {
    if let Err(e) = integration() {
        let _ = teardown();
        panic!("{:?}", e)
    }
}

fn select_everything_test(client: &mut Client) -> Result<(), Error> {
    let primitive_datetime_format =
        time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
    let primitive_datetime =
        PrimitiveDateTime::parse("2020-01-02 03:04:05", &primitive_datetime_format).unwrap();
    let offset_datetime = OffsetDateTime::parse(
        "1985-04-12T23:20:50.52Z",
        &time::format_description::well_known::Rfc3339,
    )
    .unwrap();

    let params = InsertEverythingParams {
        custom_domain_: CustomDomain(vec![CustomComposite {
            wow: String::from(""),
            such_cool: 3,
            nice: SpongebobCharacter::Bob,
        }]),
        domain_: MyDomain(String::from("hello")),
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

    assert_eq!(1, params.query(client).exec()?, "inserting one row");

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
    let actual = select_everything(client).one()?;

    assert_eq!(expected, actual);

    Ok(())
}

fn assert_eq<T: std::fmt::Debug + PartialEq>(expected: T, actual: T) -> Result<(), Error> {
    if actual != expected {
        Err(Error::Integration {
            expected: format!("{:?}", expected),
            actual: format!("{:?}", actual),
        })
    } else {
        Ok(())
    }
}

mod error {
    use crate::error::Error as CornucopiaError;
    use postgres::Error as DbError;
    use thiserror::Error as ThisError;
    #[derive(Debug, ThisError)]
    #[error("error occured during integration testing")]
    pub(crate) enum Error {
        #[error("expected {expected}, got {actual}")]
        Integration {
            expected: String,
            actual: String,
        },
        Db(#[from] DbError),
        Cornucopia(#[from] CornucopiaError),
    }
}
*/
