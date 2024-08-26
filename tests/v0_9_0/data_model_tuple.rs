#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        value::ValueType, ArrayValue, Value,
    };
    use serde_firestore_value::{from_value, to_value};

    let o = (true, 1, "abc".to_owned());
    let v = Value {
        value_type: Some(ValueType::ArrayValue(ArrayValue {
            values: vec![
                Value {
                    value_type: Some(ValueType::BooleanValue(true)),
                },
                Value {
                    value_type: Some(ValueType::IntegerValue(i64::from(1))),
                },
                Value {
                    value_type: Some(ValueType::StringValue("abc".to_owned())),
                },
            ],
        })),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, (bool, i32, String)>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}
