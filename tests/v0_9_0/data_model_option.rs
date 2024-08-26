#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{value::ValueType, Value};
    use serde_firestore_value::{from_value, to_value};

    let o = None;
    let v = Value {
        value_type: Some(ValueType::NullValue(0_i32)),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, Option<i64>>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);

    let o = Some(i64::MAX);
    let v = Value {
        value_type: Some(ValueType::IntegerValue(i64::MAX)),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, Option<i64>>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}
