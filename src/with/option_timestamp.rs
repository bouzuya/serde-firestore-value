//! (De)serialize `Option<Timestamp>` as `timestampValue` or `nullValue`.

/// Deserialize `Option<Timestamp>` from `timestampValue` or `nullValue`.
///
/// # Examples
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// # use std::collections::BTreeMap;
/// use serde_firestore_value::google::firestore::v1::{value::ValueType, MapValue, Value};
/// use serde_firestore_value::{from_value, with::option_timestamp};
///
/// #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
/// struct S(#[serde(deserialize_with = "option_timestamp::deserialize")] Option<prost_types::Timestamp>);
///
/// // some
/// {
///     let o = S(Some(prost_types::Timestamp {
///         seconds: 1_i64,
///         nanos: 2_i32,
///     }));
///     let v = Value {
///         value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
///             seconds: 1_i64,
///             nanos: 2_i32,
///         })),
///     };
///     let d = from_value::<'_, S>(&v)?;
///     assert_eq!(d, o);
/// }
///
/// // none
/// {
///     let o = S(None);
///     let v = Value {
///         value_type: Some(ValueType::NullValue(0_i32)),
///     };
///     let d = from_value::<'_, S>(&v)?;
///     assert_eq!(d, o);
/// }
/// #     Ok(())
/// # }
/// ```
///
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<prost_types::Timestamp>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    crate::de::with::timestamp_as_timestamp::deserialize_option_timestamp(deserializer)
}

/// Serialize `Option<Timestamp>` as `timestampValue` or `nullValue`.
///
/// # Examples
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// #     use std::collections::BTreeMap;
/// use serde_firestore_value::google::firestore::v1::{value::ValueType, MapValue, Value};
/// use serde_firestore_value::{from_value, to_value, with::option_timestamp};
///
/// #[derive(Debug, Eq, PartialEq, serde::Serialize)]
/// struct S(#[serde(serialize_with = "option_timestamp::serialize")] Option<prost_types::Timestamp>);
///
/// // some
/// {
///     let o = S(Some(prost_types::Timestamp {
///         seconds: 1_i64,
///         nanos: 2_i32,
///     }));
///     let v = Value {
///         value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
///             seconds: 1_i64,
///             nanos: 2_i32,
///         })),
///     };
///     let s = to_value(&o)?;
///     assert_eq!(s, v);
/// }
///
/// // none
/// {
///     let o = S(None);
///     let v = Value {
///         value_type: Some(ValueType::NullValue(0)),
///     };
///     let s = to_value(&o)?;
///     assert_eq!(s, v);
/// }
/// #     Ok(())
/// # }
/// ```
pub fn serialize<S>(
    option_timestamp: &Option<prost_types::Timestamp>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    crate::ser::with::timestamp_as_timestamp::serialize_option_timestamp(
        option_timestamp,
        serializer,
    )
}
