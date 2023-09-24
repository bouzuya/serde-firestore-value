// A serializer that serializes a `String` as a `ValueType::ReferenceValue(String)` .

pub fn serialize<S>(s: &String, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    crate::ser::with::string_as_reference::serialize_string_as_reference(s, serializer)
}
