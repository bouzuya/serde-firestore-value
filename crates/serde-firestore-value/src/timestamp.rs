use prost_types::Timestamp;

pub fn deserialize<'de, D>(deserializer: D) -> Result<Timestamp, D::Error>
where
    D: serde::Deserializer<'de>,
{
    crate::de::timestamp::deserialize(deserializer)
}

pub fn serialize<S>(timestamp: &Timestamp, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    crate::serializer::timestamp::serialize(timestamp, serializer)
}
