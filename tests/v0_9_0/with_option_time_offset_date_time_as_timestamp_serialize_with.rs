#[cfg(feature = "time")]
#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{value::ValueType, Value};
    use serde_firestore_value::{to_value, with::option_time_offset_date_time_as_timestamp};

    #[derive(Debug, Eq, PartialEq, serde::Serialize)]
    struct S(
        #[serde(serialize_with = "option_time_offset_date_time_as_timestamp::serialize")]
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
    let s = to_value(&o)?;
    assert_eq!(s, v);

    let o = S(None);
    let v = Value {
        value_type: Some(ValueType::NullValue(0)),
    };
    let s = to_value(&o)?;
    assert_eq!(s, v);
    Ok(())
}
