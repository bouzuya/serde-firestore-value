/// Timestamp
///
/// `timestampValue` inner type.
///
/// <https://protobuf.dev/reference/protobuf/google.protobuf/#timestamp>
/// <https://firebase.google.com/docs/firestore/reference/rest/Shared.Types/ArrayValue#Value>
///
/// # Examples
///
/// ```rust
/// # fn test_timestamp() -> anyhow::Result<()> {
/// #     use google_api_proto::google::firestore::v1::{value::ValueType, Value};
/// #     use serde_firestore_value::{from_value, to_value, Timestamp};
/// let o = Timestamp {
///     seconds: 1_i64,
///     nanos: 2_i32,
/// };
/// let v = Value {
///     value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
///         seconds: 1_i64,
///         nanos: 2_i32,
///     })),
/// };
/// let s = to_value(&o)?;
/// let d = from_value::<'_, Timestamp>(&s)?;
/// assert_eq!(s, v);
/// assert_eq!(d, o);
/// #     Ok(())
/// # }
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename = "$__serde-firestore-value_private_timestamp")]
pub struct Timestamp {
    /// seconds
    pub seconds: i64,
    /// nanoseconds
    pub nanos: i32,
}

impl Timestamp {
    pub(crate) const NAME: &'static str = "$__serde-firestore-value_private_timestamp";
}

impl From<Timestamp> for prost_types::Timestamp {
    fn from(Timestamp { seconds, nanos }: Timestamp) -> Self {
        Self { seconds, nanos }
    }
}

impl From<prost_types::Timestamp> for Timestamp {
    fn from(prost_types::Timestamp { seconds, nanos }: prost_types::Timestamp) -> Self {
        Self { seconds, nanos }
    }
}
