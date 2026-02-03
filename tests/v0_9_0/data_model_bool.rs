#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{Value, value::ValueType};
    use serde_firestore_value::{from_value, to_value};

    let o = false;
    let v = Value {
        value_type: Some(ValueType::BooleanValue(false)),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, bool>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);

    let o = true;
    let v = Value {
        value_type: Some(ValueType::BooleanValue(true)),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, bool>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}
