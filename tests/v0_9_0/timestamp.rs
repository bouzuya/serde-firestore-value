#[test]
fn test() -> anyhow::Result<()> {
    use serde_firestore_value::google::firestore::v1::{Value, value::ValueType};
    use serde_firestore_value::{Timestamp, from_value, to_value};

    let o = Timestamp {
        seconds: 1_i64,
        nanos: 2_i32,
    };
    let v = Value {
        value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
            seconds: 1_i64,
            nanos: 2_i32,
        })),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, Timestamp>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);

    let json = serde_json::to_string(&o)?;
    assert_eq!(json, r#"{"seconds":1,"nanos":2}"#);
    Ok(())
}
