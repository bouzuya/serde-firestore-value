//! (De)serialize `Option<String>` as `referenceValue` or `nullValue`.

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    crate::de::with::string_as_reference::deserialize_option_string_as_reference(deserializer)
}

pub fn serialize<S>(option_string: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    crate::ser::with::string_as_reference::serialize_option_string_as_reference(
        option_string,
        serializer,
    )
}
