#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{value::ValueType, Value};
    use serde_firestore_value::{from_value, to_value};

    let o = char::MAX;
    let v = Value {
        value_type: Some(ValueType::StringValue(char::MAX.to_string())),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, char>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);

    // char::MIN is unstable
    let o = ' ';
    let v = Value {
        value_type: Some(ValueType::StringValue(' '.to_string())),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, char>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}
