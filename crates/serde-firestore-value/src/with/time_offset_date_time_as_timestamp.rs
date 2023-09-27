//! (De)serialize `time::OffsetDateTime` as `timestampValue`.

use prost_types::Timestamp;

pub fn deserialize<'de, D>(deserializer: D) -> Result<time::OffsetDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let Timestamp { seconds, nanos } = crate::with::timestamp::deserialize(deserializer)?;
    Ok(time::OffsetDateTime::from_unix_timestamp_nanos(
        i128::from(seconds) * 1_000_000_000_i128 + i128::from(nanos),
    )
    .expect("timestamp"))
}

pub fn serialize<S>(
    offset_date_time: &time::OffsetDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let timestamp = Timestamp {
        seconds: offset_date_time.unix_timestamp(),
        nanos: i32::try_from(offset_date_time.unix_timestamp_nanos() % 1_000_000_000_i128)
            .expect("nanos"),
    };
    crate::with::timestamp::serialize(&timestamp, serializer)
}
