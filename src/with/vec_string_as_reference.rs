//! (De)serialize `Vec<String>` as `arrayValue` of `referenceValue`.

/// Deserialize `Vec<String>` from `arrayValue` of `referenceValue`.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    crate::de::with::string_as_reference::deserialize_vec_string_as_reference(deserializer)
}

/// Serialize `Vec<String>` as `arrayValue` of `referenceValue`.
pub fn serialize<S>(vec_string: &[String], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    crate::ser::with::string_as_reference::serialize_vec_string_as_reference(vec_string, serializer)
}
