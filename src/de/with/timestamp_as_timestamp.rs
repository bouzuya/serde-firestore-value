use prost_types::Timestamp;
use serde::Deserialize;

use crate::typ::my_timestamp::MyTimestamp;

pub(crate) fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<Timestamp, D::Error>
where
    D: serde::Deserializer<'de>,
{
    MyTimestamp::deserialize(deserializer).map(Timestamp::from)
}

pub(crate) fn deserialize_option_timestamp<'de, D>(
    deserializer: D,
) -> Result<Option<Timestamp>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<MyTimestamp>::deserialize(deserializer).map(|o| o.map(Timestamp::from))
}
