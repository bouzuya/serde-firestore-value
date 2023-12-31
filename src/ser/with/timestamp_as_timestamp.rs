use prost_types::Timestamp;

use crate::typ::my_timestamp::MyTimestamp;

pub(crate) fn serialize_timestamp<S>(
    timestamp: &Timestamp,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let timestamp = MyTimestamp::from(timestamp.clone());
    serde::Serialize::serialize(&timestamp, serializer)
}

pub(crate) fn serialize_option_timestamp<S>(
    timestamp: &Option<Timestamp>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match timestamp {
        Some(timestamp) => serialize_timestamp(timestamp, serializer),
        None => serializer.serialize_none(),
    }
}
