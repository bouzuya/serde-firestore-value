//! (De)serialize `Option<time::OffsetDateTime>` as `timestampValue`.

use prost_types::Timestamp;

/// Deserialize `Option<time::OffsetDateTime>` from `timestampValue` or `nullValue
///
/// # Example
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// use google_api_proto::google::firestore::v1::{value::ValueType, Value};
/// use prost_types::Timestamp;
/// use serde_firestore_value::{from_value, with::option_time_offset_date_time_as_timestamp};
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
///     value_type: Some(ValueType::TimestampValue(Timestamp {
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
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<time::OffsetDateTime>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(
        crate::with::option_timestamp::deserialize(deserializer)?.map(
            |Timestamp { seconds, nanos }| {
                time::OffsetDateTime::from_unix_timestamp_nanos(
                    i128::from(seconds) * 1_000_000_000_i128 + i128::from(nanos),
                )
                .expect("timestamp")
            },
        ),
    )
}

/// Serialize `Option<time::OffsetDateTime>` as `timestampValue` or `nullValue`.
///
/// # Example
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// use google_api_proto::google::firestore::v1::{value::ValueType, Value};
/// use prost_types::Timestamp;
/// use serde_firestore_value::{to_value, with::option_time_offset_date_time_as_timestamp};
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
///     value_type: Some(ValueType::TimestampValue(Timestamp {
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
    option_offset_date_time: &Option<time::OffsetDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let option_timestamp = option_offset_date_time.map(|offset_date_time| Timestamp {
        seconds: offset_date_time.unix_timestamp(),
        nanos: i32::try_from(offset_date_time.unix_timestamp_nanos() % 1_000_000_000_i128)
            .expect("nanos"),
    });
    crate::with::option_timestamp::serialize(&option_timestamp, serializer)
}
