use bytes::BytesMut;
use postgres_protocol::types;
use postgres_types::{FromSql, IsNull, ToSql, Type, to_sql_checked};
use std::error::Error;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// A wrapper that can be used to represent infinity with `Type::Date` types.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Date {
    /// Represents `infinity`, a date that is later than all other dates.
    #[default]
    PosInfinity,
    /// Represents `-infinity`, a date that is earlier than all other dates.
    NegInfinity,
    /// The wrapped date.
    Value(SystemTime),
}

impl<'a> FromSql<'a> for Date {
    fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        match types::date_from_sql(raw)? {
            i32::MAX => Ok(Date::PosInfinity),
            i32::MIN => Ok(Date::NegInfinity),
            days_since_2000 => {
                // PostgreSQL epoch (2000-01-01) is 10957 days after Unix epoch (1970-01-01)
                const DAYS_2000_TO_1970: i32 = -10957;

                // Convert days since 2000 to days since Unix epoch
                let days_since_unix_epoch = days_since_2000 + DAYS_2000_TO_1970;

                let timestamp = if days_since_unix_epoch >= 0 {
                    UNIX_EPOCH + Duration::from_secs((days_since_unix_epoch as u64) * 86400)
                } else {
                    UNIX_EPOCH - Duration::from_secs((-days_since_unix_epoch as u64) * 86400)
                };

                Ok(Date::Value(timestamp))
            }
        }
    }

    fn accepts(ty: &Type) -> bool {
        *ty == Type::DATE
    }
}

impl ToSql for Date {
    fn to_sql(
        &self,
        _ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        let value = match *self {
            Date::PosInfinity => i32::MAX,
            Date::NegInfinity => i32::MIN,
            Date::Value(time) => {
                // PostgreSQL epoch (2000-01-01) is 10957 days after Unix epoch (1970-01-01)
                const DAYS_2000_TO_1970: i32 = -10957;

                let unix_epoch_days = match time.duration_since(UNIX_EPOCH) {
                    Ok(duration) => (duration.as_secs() / 86400) as i32,
                    Err(err) => {
                        // If time is before Unix epoch, calculate negative days
                        let duration = err.duration();
                        -((duration.as_secs() / 86400) as i32)
                    }
                };

                // Convert days since Unix epoch to days since PostgreSQL epoch (2000-01-01)
                unix_epoch_days - DAYS_2000_TO_1970
            }
        };
        types::date_to_sql(value, out);
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        *ty == Type::DATE
    }

    to_sql_checked!();
}
