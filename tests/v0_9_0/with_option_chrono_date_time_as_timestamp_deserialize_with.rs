#[cfg(feature = "chrono")]
#[test]
fn test() -> anyhow::Result<()> {
    use google_api_proto::google::firestore::v1::{value::ValueType, Value};
    use serde_firestore_value::{from_value, to_value, with::option_chrono_date_time_as_timestamp};

    #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
    struct S(
        #[serde(deserialize_with = "option_chrono_date_time_as_timestamp::deserialize")]
        Option<chrono::DateTime<chrono::Utc>>,
    );

    let o = S(Some(
        chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339(
            "1970-01-01T00:00:01.000000002Z",
        )?
        .with_timezone(&chrono::Utc),
    ));
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
