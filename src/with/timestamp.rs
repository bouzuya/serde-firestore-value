//! (De)serialize `Timestamp` as `timestampValue`.

/// Deserialize `Timestamp` from `timestampValue`.
///
/// # Examples
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// use googleapis_tonic_google_firestore_v1::google::firestore::v1::{value::ValueType, ArrayValue, MapValue, Value};
/// use serde_firestore_value::{from_value, with::timestamp};
///
/// #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
/// struct S(#[serde(deserialize_with = "timestamp::deserialize")] prost_types::Timestamp);
///
/// let o = S(prost_types::Timestamp {
///     seconds: 1_i64,
///     nanos: 2_i32,
/// });
/// let v = Value {
///     value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
///         seconds: 1_i64,
///         nanos: 2_i32,
///     })),
/// };
/// let d = from_value::<'_, S>(&v)?;
/// assert_eq!(d, o);
/// #     Ok(())
/// # }
/// ```
pub fn deserialize<'de, D>(deserializer: D) -> Result<prost_types::Timestamp, D::Error>
where
    D: serde::Deserializer<'de>,
{
    crate::de::with::timestamp_as_timestamp::deserialize_timestamp(deserializer)
}

/// Serialize `Timestamp` as `timestampValue`.
///
/// # Examples
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// use googleapis_tonic_google_firestore_v1::google::firestore::v1::{value::ValueType, ArrayValue, MapValue, Value};
/// use prost_types::Timestamp;
/// use serde_firestore_value::{to_value, with::timestamp};
///
/// #[derive(Debug, Eq, PartialEq, serde::Serialize)]
/// struct S(#[serde(serialize_with = "timestamp::serialize")] Timestamp);
///
/// let o = S(Timestamp {
///     seconds: 1_i64,
///     nanos: 2_i32,
/// });
/// let v = Value {
///     value_type: Some(ValueType::TimestampValue(Timestamp {
///         seconds: 1_i64,
///         nanos: 2_i32,
///     })),
/// };
/// let s = to_value(&o)?;
/// assert_eq!(s, v);
/// #     Ok(())
/// # }
/// ```
pub fn serialize<S>(timestamp: &prost_types::Timestamp, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    crate::ser::with::timestamp_as_timestamp::serialize_timestamp(timestamp, serializer)
}
