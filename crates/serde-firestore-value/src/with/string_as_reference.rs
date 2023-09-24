// A (de)serializer that serializes a `String` as a `Value` (`ValueType::ReferenceValue(String)`) .

pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    crate::de::with::string_as_reference::deserialize_string_as_reference(deserializer)
}

pub fn serialize<S>(s: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    crate::ser::with::string_as_reference::serialize_string_as_reference(s, serializer)
}
