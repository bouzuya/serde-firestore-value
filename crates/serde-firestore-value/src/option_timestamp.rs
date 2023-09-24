use prost_types::Timestamp;

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Timestamp>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    crate::de::with::timestamp_as_timestamp::deserialize_option_timestamp(deserializer)
}

pub fn serialize<S>(option_timestamp: &Option<Timestamp>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    crate::ser::with::timestamp_as_timestamp::serialize_option_timestamp(
        option_timestamp,
        serializer,
    )
}
