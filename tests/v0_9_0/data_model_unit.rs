#[allow(clippy::let_unit_value)]
#[allow(clippy::unit_cmp)]
#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{value::ValueType, Value};
    use serde_firestore_value::{from_value, to_value};

    let o = ();
    let v = Value {
        value_type: Some(ValueType::NullValue(0)),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, ()>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}
