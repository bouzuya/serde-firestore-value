#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{Value, value::ValueType};
    use serde_firestore_value::{from_value, to_value};

    let o = i64::MAX;
    let v = Value {
        value_type: Some(ValueType::IntegerValue(i64::MAX)),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, i64>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);

    let o = i64::MIN;
    let v = Value {
        value_type: Some(ValueType::IntegerValue(i64::MIN)),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, i64>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}
