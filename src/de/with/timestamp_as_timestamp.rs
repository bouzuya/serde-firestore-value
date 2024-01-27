use serde::Deserialize;

use crate::typ::my_timestamp::MyTimestamp;

pub(crate) fn deserialize_timestamp<'de, D>(
    deserializer: D,
) -> Result<prost_types::Timestamp, D::Error>
where
    D: serde::Deserializer<'de>,
{
    MyTimestamp::deserialize(deserializer).map(prost_types::Timestamp::from)
}

pub(crate) fn deserialize_option_timestamp<'de, D>(
    deserializer: D,
) -> Result<Option<prost_types::Timestamp>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<MyTimestamp>::deserialize(deserializer).map(|o| o.map(prost_types::Timestamp::from))
}
