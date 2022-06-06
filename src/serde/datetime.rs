use bson;
use chrono;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

pub fn chrono_as_bson<S>(
    data: &chrono::DateTime<chrono::Utc>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    bson::UtcDateTime::from(*data).serialize(serializer)
}

pub fn bson_as_chrono<'de, D>(data: D) -> Result<chrono::DateTime<chrono::Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    bson::UtcDateTime::deserialize(data).and_then(|d| Ok(chrono::DateTime::from(d.clone())))
}

pub fn chrono_as_bson_optional<S>(
    data: &Option<chrono::DateTime<chrono::Utc>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match data {
        Some(d) => bson::UtcDateTime::from(*d).serialize(serializer),
        _ => serializer.serialize_none(),
    }
}

pub fn bson_as_chrono_optional<'de, D>(
    data: D,
) -> Result<Option<chrono::DateTime<chrono::Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    match bson::UtcDateTime::deserialize(data) {
        Ok(d) => Ok(Some(chrono::DateTime::from(d))),
        _ => Ok(None),
    }
}
