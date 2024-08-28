#[cfg(feature = "hash-map")]
#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{value::ValueType, Value};
    use serde_firestore_value::{to_value, with::timestamp};

    #[derive(Debug, Eq, PartialEq, serde::Serialize)]
    struct S(#[serde(serialize_with = "timestamp::serialize")] prost_types::Timestamp);

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
    let s = to_value(&o)?;
    assert_eq!(s, v);
    Ok(())
}
