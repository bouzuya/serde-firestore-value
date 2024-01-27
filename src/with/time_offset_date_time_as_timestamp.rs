//! (De)serialize `time::OffsetDateTime` as `timestampValue`.

/// Deserialize `time::OffsetDateTime` from `timestampValue`.
///
/// # Examples
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// use google_api_proto::google::firestore::v1::{value::ValueType, Value};
/// use serde_firestore_value::{
///     from_value, with::option_time_offset_date_time_as_timestamp,
/// };
///
/// #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
/// struct S(
///     #[serde(deserialize_with = "option_time_offset_date_time_as_timestamp::deserialize")]
///     Option<time::OffsetDateTime>,
/// );
///
/// let o = S(Some(time::OffsetDateTime::from_unix_timestamp_nanos(
///     1_000_000_002_i128,
/// )?));
/// let v = Value {
///     value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
///         seconds: 1_i64,
///         nanos: 2_i32,
///     })),
/// };
/// let d = from_value::<'_, S>(&v)?;
/// assert_eq!(d, o);
///
/// let o = S(None);
/// let v = Value {
///     value_type: Some(ValueType::NullValue(0)),
/// };
/// let d = from_value::<'_, S>(&v)?;
/// assert_eq!(d, o);
/// #     Ok(())
/// # }
/// ```
pub fn deserialize<'de, D>(deserializer: D) -> Result<time::OffsetDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let prost_types::Timestamp { seconds, nanos } =
        crate::with::timestamp::deserialize(deserializer)?;
    Ok(time::OffsetDateTime::from_unix_timestamp_nanos(
        i128::from(seconds) * 1_000_000_000_i128 + i128::from(nanos),
    )
    .expect("timestamp"))
}

/// Serialize `time::OffsetDateTime` as `timestampValue`.
///
/// # Examples
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// use google_api_proto::google::firestore::v1::{value::ValueType, Value};
/// use serde_firestore_value::{
///     to_value, with::option_time_offset_date_time_as_timestamp,
/// };
///
/// #[derive(Debug, Eq, PartialEq, serde::Serialize)]
/// struct S(
///     #[serde(serialize_with = "option_time_offset_date_time_as_timestamp::serialize")]
///     Option<time::OffsetDateTime>,
/// );
///
/// let o = S(Some(time::OffsetDateTime::from_unix_timestamp_nanos(
///     1_000_000_002_i128,
/// )?));
/// let v = Value {
///     value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
///         seconds: 1_i64,
///         nanos: 2_i32,
///     })),
/// };
/// let s = to_value(&o)?;
/// assert_eq!(s, v);
///
/// let o = S(None);
/// let v = Value {
///     value_type: Some(ValueType::NullValue(0)),
/// };
/// let s = to_value(&o)?;
/// assert_eq!(s, v);
/// #     Ok(())
/// # }
/// ```
pub fn serialize<S>(
    offset_date_time: &time::OffsetDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let timestamp = prost_types::Timestamp {
        seconds: offset_date_time.unix_timestamp(),
        nanos: i32::try_from(offset_date_time.unix_timestamp_nanos() % 1_000_000_000_i128)
            .expect("nanos"),
    };
    crate::with::timestamp::serialize(&timestamp, serializer)
}
