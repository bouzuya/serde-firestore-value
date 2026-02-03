#[cfg(feature = "time")]
#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{Value, value::ValueType};
    use serde_firestore_value::{from_value, with::option_time_offset_date_time_as_timestamp};

    #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
    struct S(
        #[serde(deserialize_with = "option_time_offset_date_time_as_timestamp::deserialize")]
        Option<time::OffsetDateTime>,
    );

    let o = S(Some(time::OffsetDateTime::from_unix_timestamp_nanos(
        1_000_000_002_i128,
    )?));
    let v = Value {
        value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
            seconds: 1_i64,
            nanos: 2_i32,
        })),
    };
    let d = from_value::<'_, S>(&v)?;
    assert_eq!(d, o);

    let o = S(None);
    let v = Value {
        value_type: Some(ValueType::NullValue(0)),
    };
    let d = from_value::<'_, S>(&v)?;
    assert_eq!(d, o);
    Ok(())
}
