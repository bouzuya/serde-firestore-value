#[cfg(feature = "chrono")]
#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{Value, value::ValueType};
    use serde_firestore_value::{to_value, with::chrono_date_time_as_timestamp};

    #[derive(Debug, Eq, PartialEq, serde::Serialize)]
    struct S(
        #[serde(serialize_with = "chrono_date_time_as_timestamp::serialize")]
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
    let s = to_value(&o)?;
    assert_eq!(s, v);
    Ok(())
}
