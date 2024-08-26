#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{value::ValueType, Value};
    use serde_firestore_value::{from_value, to_value};

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    enum E {
        A,
        B,
    }

    let o = E::A;
    let v = Value {
        value_type: Some(ValueType::StringValue("A".to_owned())),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, E>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);

    let o = E::B;
    let v = Value {
        value_type: Some(ValueType::StringValue("B".to_owned())),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, E>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}
