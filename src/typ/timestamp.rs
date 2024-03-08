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

#[cfg(feature = "chrono")]
impl std::convert::TryFrom<Timestamp> for chrono::DateTime<chrono::Utc> {
    type Error = crate::Error;

    fn try_from(Timestamp { seconds, nanos }: Timestamp) -> Result<Self, Self::Error> {
        let nanos = u32::try_from(nanos).map_err(|_| {
            crate::Error::from(crate::error::ErrorCode::Custom(format!(
                "chrono::DateTime::<chrono::Utc>::try_from(Timestamp) / u32::try_from({})",
                nanos
            )))
        })?;
        Self::from_timestamp(seconds, nanos).ok_or_else(|| {
            crate::Error::from(crate::error::ErrorCode::Custom(format!(
                "chrono::DateTime::<chrono::Utc>::try_from(Timestamp) / chrono::DateTime::<chrono::Utc>::from_timestamp({}, {})",
                seconds, nanos
            )))
        })
    }
}

#[cfg(feature = "time")]
impl std::convert::TryFrom<Timestamp> for time::OffsetDateTime {
    type Error = crate::Error;

    fn try_from(Timestamp { seconds, nanos }: Timestamp) -> Result<Self, Self::Error> {
        let timestamp_nanos = i128::from(seconds) * 1_000_000_000_i128 + i128::from(nanos);
        Self::from_unix_timestamp_nanos(timestamp_nanos).map_err(|e| {
            crate::Error::from(crate::error::ErrorCode::Custom(format!(
                "time::OffsetDateTime::try_from(Tiemstamp) / time::OffsetDateTime::from_unix_timestamp_nanos({}) : {}",
                timestamp_nanos,
                e
            )))
        })
    }
}

#[cfg(feature = "chrono")]
impl std::convert::TryFrom<chrono::DateTime<chrono::Utc>> for Timestamp {
    type Error = crate::Error;

    fn try_from(date_time: chrono::DateTime<chrono::Utc>) -> Result<Self, Self::Error> {
        let seconds = date_time.timestamp();
        let nanos = date_time.timestamp_subsec_nanos();
        let nanos = i32::try_from(nanos).map_err(|_| {
            crate::Error::from(crate::error::ErrorCode::Custom(format!(
                "Timestamp::try_from(chrono::DateTime::<chrono::Utc>) / i32::try_from({})",
                nanos
            )))
        })?;
        Ok(Self { seconds, nanos })
    }
}

#[cfg(feature = "time")]
impl std::convert::TryFrom<time::OffsetDateTime> for Timestamp {
    type Error = crate::Error;

    fn try_from(offset_date_time: time::OffsetDateTime) -> Result<Self, Self::Error> {
        let seconds = offset_date_time.unix_timestamp();
        let nanos = offset_date_time.unix_timestamp_nanos() % 1_000_000_000_i128;
        let nanos = i32::try_from(nanos).map_err(|_| {
            crate::Error::from(crate::error::ErrorCode::Custom(format!(
                "Timestamp::try_from(chrono::DateTime::<chrono::Utc>) / i32::try_from({})",
                nanos
            )))
        })?;
        Ok(Self { seconds, nanos })
    }
}
