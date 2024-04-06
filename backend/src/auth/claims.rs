use crate::users::User;
use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, Duration};


#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub(super) uuid: String,
    
    #[serde(with = "claims_serde")]
    pub(super) iat: OffsetDateTime,

    #[serde(with = "claims_serde")]
    pub(super) exp: OffsetDateTime,
    pub(super) email: String,
    pub(super) is_active: bool
}

type TokenTime = [OffsetDateTime; 2];

impl Claims { 
    pub fn new(user: &User) -> Claims {
        let [iat, exp] = Self::generate_expiry();

        let uuid = user.get_uuid();
        let email = user.get_email();

        Claims {
            uuid: String::from(uuid),
            iat,
            exp,
            email: String::from(email),
            is_active: true
        }
    }

    pub fn custom(uuid: &str, email: &str, iat: OffsetDateTime, exp: OffsetDateTime) -> Claims {
        Claims {
            uuid: String::from(uuid),
            iat,
            exp,
            email: String::from(email),
            is_active: true
        }
    }

    fn generate_expiry() -> TokenTime {
        let iat = OffsetDateTime::now_utc();    
        let exp = iat + Duration::hours(1);

        let iat = iat.date()
            .with_hms_milli(iat.hour(), iat.minute(), iat.second(), 0)
            .unwrap()
            .assume_utc();

        let exp = exp.date()
            .with_hms_milli(exp.hour(), exp.minute(), exp.second(), 0)
            .unwrap()
            .assume_utc();

        [iat, exp]
    } 

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}

mod claims_serde {
    use serde::{Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;
    use std::i64;

    pub(super) fn serialize<S>(value: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where S:
        Serializer
    {
        let timestamp = value.unix_timestamp();
        serializer.serialize_i64(timestamp)
    }

    pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where D: 
        Deserializer<'de>
    {
        OffsetDateTime::from_unix_timestamp(i64::deserialize(deserializer)?)
            .map_err(|_| serde::de::Error::custom("Invalid timestamp passed"))
    }

}
