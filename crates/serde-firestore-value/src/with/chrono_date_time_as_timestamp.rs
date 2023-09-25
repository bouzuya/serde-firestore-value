// A (de)serializer that serializes a `chrono::DateTime<Tz>` as a `Value` (`ValueType::TimestampValue(Timestamp)`) .

use prost_types::Timestamp;

pub fn deserialize<'de, D>(deserializer: D) -> Result<chrono::DateTime<chrono::Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let Timestamp { seconds, nanos } = crate::with::timestamp::deserialize(deserializer)?;
    Ok(chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
        chrono::NaiveDateTime::from_timestamp_opt(seconds, nanos as u32).expect("timestamp"),
        chrono::Utc,
    ))
}

pub fn serialize<S>(
    date_time: &chrono::DateTime<chrono::Utc>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let timestamp = Timestamp {
        seconds: date_time.timestamp(),
        nanos: date_time.timestamp_subsec_nanos() as i32,
    };
    crate::with::timestamp::serialize(&timestamp, serializer)
}
