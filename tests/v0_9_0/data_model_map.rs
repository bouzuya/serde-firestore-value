#[cfg(feature = "hash-map")]
#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        MapValue, Value, value::ValueType,
    };
    use serde_firestore_value::{from_value, to_value};

    let o = [("k1".to_owned(), 1_i64), ("k2".to_owned(), 2_i64)]
        .into_iter()
        .collect::<std::collections::BTreeMap<String, i64>>();
    let v = Value {
        value_type: Some(ValueType::MapValue(MapValue {
            fields: [("k1".to_owned(), 1_i64), ("k2".to_owned(), 2_i64)]
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Value {
                            value_type: Some(ValueType::IntegerValue(v)),
                        },
                    )
                })
                .collect::<std::collections::HashMap<String, Value>>(),
        })),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, std::collections::BTreeMap<String, i64>>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}
