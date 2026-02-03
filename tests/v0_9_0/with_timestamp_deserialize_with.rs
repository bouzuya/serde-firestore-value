#[cfg(feature = "hash-map")]
#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{Value, value::ValueType};
    use serde_firestore_value::{from_value, with::timestamp};

    #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
    struct S(#[serde(deserialize_with = "timestamp::deserialize")] prost_types::Timestamp);

    let o = S(prost_types::Timestamp {
        seconds: 1_i64,
        nanos: 2_i32,
    });
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
