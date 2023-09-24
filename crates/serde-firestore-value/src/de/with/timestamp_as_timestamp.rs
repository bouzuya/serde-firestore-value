use prost_types::Timestamp;
use serde::Deserialize;

#[derive(Debug, serde::Deserialize)]
#[serde(rename = "$__serde-firestore-value_private_timestamp")]
struct MyTimestamp {
    seconds: i64,
    nanos: i32,
}

pub(crate) fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<Timestamp, D::Error>
where
    D: serde::Deserializer<'de>,
{
    MyTimestamp::deserialize(deserializer)
        .map(|MyTimestamp { seconds, nanos }| Timestamp { seconds, nanos })
}

pub(crate) fn deserialize_option_timestamp<'de, D>(
    deserializer: D,
) -> Result<Option<Timestamp>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<MyTimestamp>::deserialize(deserializer)
        .map(|o| o.map(|MyTimestamp { seconds, nanos }| Timestamp { seconds, nanos }))
}
