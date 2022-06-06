use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

pub fn objectid_as_string<'de, D>(data: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    bson::oid::ObjectId::deserialize(data).and_then(|oid| Ok(oid.to_hex()))
}

pub fn string_as_objectid<S>(data: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    bson::oid::ObjectId::with_string(data)
        .expect("invalid_object_id")
        .serialize(serializer)
}

pub fn objectid_as_string_optional<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    match bson::oid::ObjectId::deserialize(deserializer) {
        Ok(oid) => Ok(Some(oid.to_hex())),
        _ => Ok(None),
    }
}

pub fn string_as_objectid_optional<S>(
    data: &Option<String>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match data {
        Some(d) => bson::oid::ObjectId::with_string(d.as_str())
            .expect("invalid_object_id")
            .serialize(serializer),
        _ => serializer.serialize_none(),
    }
}
