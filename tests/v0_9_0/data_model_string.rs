#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{Value, value::ValueType};
    use serde_firestore_value::{from_value, to_value};

    let o = "abc".to_owned();
    let v = Value {
        value_type: Some(ValueType::StringValue(o.clone())),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, String>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);

    let o = "x".repeat(1_048_487);
    let v = Value {
        value_type: Some(ValueType::StringValue(o.clone())),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, String>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);

    let o = "x".repeat(1_048_487 + 1);
    assert_eq!(
        to_value(&o).unwrap_err().to_string(),
        "maximum byte length (1,048,487 bytes = 1MiB - 89 bytes) exceeded"
    );

    Ok(())
}
