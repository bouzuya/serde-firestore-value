#[cfg(feature = "chrono")]
#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{value::ValueType, Value};
    use serde_firestore_value::{from_value, to_value, with::chrono_date_time_as_timestamp};

    #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
    struct S(
        #[serde(deserialize_with = "chrono_date_time_as_timestamp::deserialize")]
        chrono::DateTime<chrono::Utc>,
    );

    let o = S(chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339(
        "1970-01-01T00:00:01.000000002Z",
    )?
    .with_timezone(&chrono::Utc));
    let v = Value {
        value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
            seconds: 1_i64,
            nanos: 2_i32,
        })),
    };
    let d = from_value::<'_, S>(&v)?;
    assert_eq!(d, o);
    Ok(())
}
