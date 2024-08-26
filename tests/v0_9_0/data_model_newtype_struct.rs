#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{value::ValueType, Value};
    use serde_firestore_value::{from_value, to_value};

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct Millimeters(u8);

    let o = Millimeters(u8::MAX);
    let v = Value {
        value_type: Some(ValueType::IntegerValue(i64::from(u8::MAX))),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, Millimeters>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}
