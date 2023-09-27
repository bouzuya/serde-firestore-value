//! (De)serialize `Timestamp` as `timestampValue`.

use prost_types::Timestamp;

pub fn deserialize<'de, D>(deserializer: D) -> Result<Timestamp, D::Error>
where
    D: serde::Deserializer<'de>,
{
    crate::de::with::timestamp_as_timestamp::deserialize_timestamp(deserializer)
}

pub fn serialize<S>(timestamp: &Timestamp, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    crate::ser::with::timestamp_as_timestamp::serialize_timestamp(timestamp, serializer)
}
