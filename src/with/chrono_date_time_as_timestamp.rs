//! (De)serialize `chrono::DateTime<chrono::Utc>` as `timestampValue`.

/// Deserialize `chrono::DateTime<chrono::Utc>` from `timestampValue`.
///
/// # Examples
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// use google_api_proto::google::firestore::v1::{value::ValueType, Value};
/// use serde_firestore_value::{from_value, with::chrono_date_time_as_timestamp};
///
/// #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
/// struct S(
///     #[serde(deserialize_with = "chrono_date_time_as_timestamp::deserialize")]
///     chrono::DateTime<chrono::Utc>,
/// );
///
/// let o = S(chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339(
///     "1970-01-01T00:00:01.000000002Z",
/// )?
/// .with_timezone(&chrono::Utc));
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
pub fn deserialize<'de, D>(deserializer: D) -> Result<chrono::DateTime<chrono::Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let prost_types::Timestamp { seconds, nanos } =
        crate::with::timestamp::deserialize(deserializer)?;
    Ok(chrono::DateTime::<chrono::Utc>::from_timestamp(seconds, nanos as u32).expect("timestamp"))
}

/// Serialize `chrono::DateTime<chrono::Utc>` as `timestampValue`.
///
/// # Examples
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// use google_api_proto::google::firestore::v1::{value::ValueType, Value};
/// use serde_firestore_value::{to_value, with::chrono_date_time_as_timestamp};
///
/// #[derive(Debug, Eq, PartialEq, serde::Serialize)]
/// struct S(
///     #[serde(serialize_with = "chrono_date_time_as_timestamp::serialize")]
///     chrono::DateTime<chrono::Utc>,
/// );
///
/// let o = S(chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339(
///     "1970-01-01T00:00:01.000000002Z",
/// )?
/// .with_timezone(&chrono::Utc));
/// let v = Value {
///     value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
///         seconds: 1_i64,
///         nanos: 2_i32,
///     })),
/// };
/// let s = to_value(&o)?;
/// assert_eq!(s, v);
/// #     Ok(())
/// # }
/// ```
pub fn serialize<S>(
    date_time: &chrono::DateTime<chrono::Utc>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let timestamp = prost_types::Timestamp {
        seconds: date_time.timestamp(),
        nanos: date_time.timestamp_subsec_nanos() as i32,
    };
    crate::with::timestamp::serialize(&timestamp, serializer)
}
