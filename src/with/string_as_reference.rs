//! (De)serialize `String` as `referenceValue`.

/// Deserialize `String` from `referenceValue`.
///
/// # Examples
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
///
/// use serde_firestore_value::google::firestore::v1::{value::ValueType, MapValue, Value};
/// use serde_firestore_value::{from_value, with::string_as_reference};
///
/// #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
/// struct S {
///     #[serde(deserialize_with = "string_as_reference::deserialize")]
///     r: String,
///     s: String,
/// }
///
/// let o = S {
///     r: "projects/p/databases/d/documents/c/1".to_string(),
///     s: "s1".to_string(),
/// };
/// let v = Value {
///     value_type: Some(ValueType::MapValue(MapValue {
///         fields: {
///             let mut fields = std::collections::HashMap::new();
///             fields.insert(
///                 "r".to_string(),
///                 Value {
///                     value_type: Some(ValueType::ReferenceValue(
///                         "projects/p/databases/d/documents/c/1".to_string(),
///                     )),
///                 },
///             );
///             fields.insert(
///                 "s".to_string(),
///                 Value {
///                     value_type: Some(ValueType::StringValue("s1".to_string())),
///                 },
///             );
///             fields
///         },
///     })),
/// };
/// let d = from_value::<'_, S>(&v)?;
/// assert_eq!(d, o);
/// #     Ok(())
/// # }
/// ```
pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    crate::de::with::string_as_reference::deserialize_string_as_reference(deserializer)
}

/// Serialize `String` as `referenceValue`.
///
/// # Examples
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// use serde_firestore_value::google::firestore::v1::{value::ValueType, MapValue, Value};
/// use serde_firestore_value::{to_value, with::string_as_reference};
///
/// #[derive(Debug, Eq, PartialEq, serde::Serialize)]
/// struct S {
///     #[serde(serialize_with = "string_as_reference::serialize")]
///     r: String,
///     s: String,
/// }
///
/// let o = S {
///     r: "projects/p/databases/d/documents/c/1".to_string(),
///     s: "s1".to_string(),
/// };
/// let v = Value {
///     value_type: Some(ValueType::MapValue(MapValue {
///         fields: {
///             let mut fields = std::collections::HashMap::new();
///             fields.insert(
///                 "r".to_string(),
///                 Value {
///                     value_type: Some(ValueType::ReferenceValue(
///                         "projects/p/databases/d/documents/c/1".to_string(),
///                     )),
///                 },
///             );
///             fields.insert(
///                 "s".to_string(),
///                 Value {
///                     value_type: Some(ValueType::StringValue("s1".to_string())),
///                 },
///             );
///             fields
///         },
///     })),
/// };
/// let s = to_value(&o)?;
/// assert_eq!(s, v);
/// #     Ok(())
/// # }
/// ```
pub fn serialize<S>(s: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    crate::ser::with::string_as_reference::serialize_string_as_reference(s, serializer)
}
