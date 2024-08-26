#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        value::ValueType, MapValue, Value,
    };
    use serde_firestore_value::{from_value, to_value};

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct S {
        r: u8,
        g: u8,
        b: u8,
    }

    let o = S { r: 1, g: 2, b: 3 };
    let v = Value {
        value_type: Some(ValueType::MapValue(MapValue {
            fields: [
                ("r".to_owned(), 1_i64),
                ("g".to_owned(), 2_i64),
                ("b".to_owned(), 3_i64),
            ]
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    Value {
                        value_type: Some(ValueType::IntegerValue(i64::from(v))),
                    },
                )
            })
            .collect::<std::collections::HashMap<String, Value>>(),
        })),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, S>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}
