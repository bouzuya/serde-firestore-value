#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        value::ValueType, ArrayValue, Value,
    };
    use serde_firestore_value::{from_value, to_value};

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct Rgb(u8, u8, u8);

    let o = Rgb(1, 2, 3);
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
    let d = from_value::<'_, Rgb>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}
