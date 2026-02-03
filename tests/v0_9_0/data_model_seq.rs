#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        ArrayValue, Value, value::ValueType,
    };
    use serde_firestore_value::{from_value, to_value};

    let o = vec![1, 2, 3];
    let v = Value {
        value_type: Some(ValueType::ArrayValue(ArrayValue {
            values: [1, 2, 3]
                .into_iter()
                .map(|it| Value {
                    value_type: Some(ValueType::IntegerValue(i64::from(it))),
                })
                .collect::<Vec<Value>>(),
        })),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, Vec<i32>>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}
