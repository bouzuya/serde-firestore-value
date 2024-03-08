//! (De)serialize `Option<chrono::DateTime<chrono::Utc>>` as `timestampValue`.

/// Deserialize `Option<chrono::DateTime<chrono::Utc>>` from `timestampValue` or `nullValue`
///
/// # Examples
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// use google_api_proto::google::firestore::v1::{value::ValueType, Value};
/// use serde_firestore_value::{from_value, with::option_chrono_date_time_as_timestamp};
///
/// #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
/// struct S(
///     #[serde(deserialize_with = "option_chrono_date_time_as_timestamp::deserialize")]
///     Option<chrono::DateTime<chrono::Utc>>,
/// );
///
/// let o = S(Some(
///     chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339(
///         "1970-01-01T00:00:01.000000002Z",
///     )?
///     .with_timezone(&chrono::Utc),
/// ));
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
pub fn deserialize<'de, D>(
    deserializer: D,
) -> Result<Option<chrono::DateTime<chrono::Utc>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let option_timestamp = crate::with::option_timestamp::deserialize(deserializer)?;
    option_timestamp
        .map(|prost_types::Timestamp { seconds, nanos }| {
            Ok(
                chrono::DateTime::<chrono::Utc>::from_timestamp(seconds, nanos as u32)
                    .expect("timestamp"),
            )
        })
        .transpose()
}

/// Serialize `Option<chrono::DateTime<chrono::Utc>` as `referenceValue` or `nullValue`.
///
/// # Examples
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// use google_api_proto::google::firestore::v1::{value::ValueType, Value};
/// use serde_firestore_value::{to_value, with::option_chrono_date_time_as_timestamp};
///
/// #[derive(Debug, Eq, PartialEq, serde::Serialize)]
/// struct S(
///     #[serde(serialize_with = "option_chrono_date_time_as_timestamp::serialize")]
///     Option<chrono::DateTime<chrono::Utc>>,
/// );
///
/// let o = S(Some(
///     chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339(
///         "1970-01-01T00:00:01.000000002Z",
///     )?
///     .with_timezone(&chrono::Utc),
/// ));
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
    option_date_time: &Option<chrono::DateTime<chrono::Utc>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let option_timestamp = option_date_time
        .as_ref()
        .map(|date_time| prost_types::Timestamp {
            seconds: date_time.timestamp(),
            nanos: date_time.timestamp_subsec_nanos() as i32,
        });
    crate::with::option_timestamp::serialize(&option_timestamp, serializer)
}
