use prost_types::Timestamp;

use super::super::firestore_value_serializer::FirestoreValueSerializer;

pub fn serialize_timestamp<S>(timestamp: &Timestamp, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut s = serializer.serialize_struct(FirestoreValueSerializer::TIMESTAMP_STRUCT_NAME, 2)?;
    serde::ser::SerializeStruct::serialize_field(&mut s, "seconds", &timestamp.seconds)?;
    serde::ser::SerializeStruct::serialize_field(&mut s, "nanos", &timestamp.nanos)?;
    serde::ser::SerializeStruct::end(s)
}

pub fn serialize_option_timestamp<S>(
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
