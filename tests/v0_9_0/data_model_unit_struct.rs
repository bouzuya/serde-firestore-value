#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{value::ValueType, Value};
    use serde_firestore_value::{from_value, to_value};

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct Unit;

    let o = Unit;
    let v = Value {
        value_type: Some(ValueType::NullValue(0_i32)),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, Unit>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}
