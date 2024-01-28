use crate::Timestamp;

pub(crate) fn serialize_timestamp<S>(
    timestamp: &prost_types::Timestamp,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let timestamp = Timestamp::from(timestamp.clone());
    serde::Serialize::serialize(&timestamp, serializer)
}

pub(crate) fn serialize_option_timestamp<S>(
    timestamp: &Option<prost_types::Timestamp>,
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
