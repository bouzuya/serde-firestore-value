#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        value::ValueType, ArrayValue, Value,
    };
    use serde_firestore_value::{from_value, to_value};

    let o = [0_u8, 1_u8];
    let v = Value {
        // ArrayValue is used instead of BytesValue.
        value_type: Some(ValueType::ArrayValue(ArrayValue {
            values: vec![
                Value {
                    value_type: Some(ValueType::IntegerValue(i64::from(0_u8))),
                },
                Value {
                    value_type: Some(ValueType::IntegerValue(i64::from(1_u8))),
                },
            ],
        })),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, Vec<u8>>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}
